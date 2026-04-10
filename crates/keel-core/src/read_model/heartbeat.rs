//! Canonical repository-activity heartbeat projection.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::SystemTime;

use chrono::{DateTime, Duration, Utc};
use serde::Serialize;
use sha2::{Digest, Sha256};
use walkdir::WalkDir;

const IGNORED_ACTIVITY_PREFIXES: &[&str] = &[".keel/cache/", ".keel/inbox/", ".keel/outbox/"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HeartbeatSource {
    DirtyWorktree,
    GitIndex,
    HeadCommit,
    FilesystemFallback,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HeartbeatProjection {
    pub source: HeartbeatSource,
    pub last_activity_at: DateTime<Utc>,
    pub dirty: bool,
    pub dirty_paths: Vec<PathBuf>,
    pub latest_path: Option<PathBuf>,
}

impl Default for HeartbeatProjection {
    fn default() -> Self {
        Self {
            source: HeartbeatSource::Unknown,
            last_activity_at: DateTime::<Utc>::from(SystemTime::UNIX_EPOCH),
            dirty: false,
            dirty_paths: Vec::new(),
            latest_path: None,
        }
    }
}

impl HeartbeatProjection {
    pub fn is_energized(&self, reference_time: DateTime<Utc>, decay_minutes: u64) -> bool {
        reference_time.signed_duration_since(self.last_activity_at)
            < Duration::minutes(decay_minutes as i64)
    }

    pub fn age_seconds(&self, reference_time: DateTime<Utc>) -> i64 {
        reference_time
            .signed_duration_since(self.last_activity_at)
            .num_seconds()
            .max(0)
    }
}

pub fn project(board_dir: &Path, reference_time: DateTime<Utc>) -> HeartbeatProjection {
    let project_root = project_root(board_dir);
    let fallback = || filesystem_fallback(&project_root);

    let Some(repo_root) = git_repo_root(&project_root) else {
        return fallback();
    };

    let dirty_paths = git_dirty_paths(&repo_root);
    if !dirty_paths.is_empty() {
        if let Some((latest_path, last_activity_at)) =
            latest_existing_dirty_path(&repo_root, &dirty_paths)
        {
            return HeartbeatProjection {
                source: HeartbeatSource::DirtyWorktree,
                last_activity_at,
                dirty: true,
                dirty_paths,
                latest_path: Some(latest_path),
            };
        }

        if let Some(last_activity_at) = git_index_modified_at(&repo_root) {
            return HeartbeatProjection {
                source: HeartbeatSource::GitIndex,
                last_activity_at,
                dirty: true,
                dirty_paths,
                latest_path: None,
            };
        }
    }

    if let Some(last_activity_at) = git_head_commit_at(&repo_root) {
        return HeartbeatProjection {
            source: HeartbeatSource::HeadCommit,
            last_activity_at,
            dirty: false,
            dirty_paths: Vec::new(),
            latest_path: None,
        };
    }

    let projection = fallback();
    if projection.source == HeartbeatSource::Unknown {
        return HeartbeatProjection {
            last_activity_at: reference_time - Duration::days(365),
            ..projection
        };
    }
    projection
}

pub fn fingerprint(board_dir: &Path) -> String {
    let project_root = project_root(board_dir);
    let mut hasher = Sha256::new();

    if let Some(repo_root) = git_repo_root(&project_root) {
        hasher.update(b"git");
        hasher.update(repo_root.to_string_lossy().as_bytes());

        let status_output = git_status_output(&repo_root);
        let dirty_paths = dirty_paths_from_porcelain(&status_output);
        for path in &dirty_paths {
            hasher.update(path.to_string_lossy().as_bytes());
        }
        for path in dirty_paths {
            let absolute = repo_root.join(&path);
            if let Ok(metadata) = fs::metadata(&absolute) {
                update_hasher_with_metadata(&mut hasher, &path, &metadata);
            }
        }

        if !dirty_paths_from_porcelain(&status_output).is_empty()
            && let Some(git_dir) = git_dir(&repo_root)
        {
            let index_path = git_dir.join("index");
            if let Ok(metadata) = fs::metadata(&index_path) {
                update_hasher_with_metadata(&mut hasher, Path::new(".git/index"), &metadata);
            }
        }

        if let Some((sha, committed_at)) = git_head_signature(&repo_root) {
            hasher.update(sha.as_bytes());
            hasher.update(committed_at.timestamp().to_le_bytes());
        }
    } else {
        hasher.update(b"filesystem");
        if let Some((path, modified_at)) = latest_filesystem_entry(&project_root) {
            hasher.update(path.to_string_lossy().as_bytes());
            hasher.update(modified_at.timestamp().to_le_bytes());
        }
    }

    format!("{:x}", hasher.finalize())
}

fn filesystem_fallback(project_root: &Path) -> HeartbeatProjection {
    let Some((latest_path, last_activity_at)) = latest_filesystem_entry(project_root) else {
        return HeartbeatProjection::default();
    };

    HeartbeatProjection {
        source: HeartbeatSource::FilesystemFallback,
        last_activity_at,
        dirty: false,
        dirty_paths: Vec::new(),
        latest_path: Some(latest_path),
    }
}

fn latest_existing_dirty_path(
    repo_root: &Path,
    dirty_paths: &[PathBuf],
) -> Option<(PathBuf, DateTime<Utc>)> {
    dirty_paths
        .iter()
        .filter_map(|relative_path| {
            let absolute_path = repo_root.join(relative_path);
            let metadata = fs::metadata(&absolute_path).ok()?;
            let modified = metadata.modified().ok()?;
            Some((relative_path.clone(), DateTime::<Utc>::from(modified)))
        })
        .max_by_key(|(_, modified)| *modified)
}

fn latest_filesystem_entry(project_root: &Path) -> Option<(PathBuf, DateTime<Utc>)> {
    WalkDir::new(project_root)
        .into_iter()
        .filter_entry(|entry| entry.file_name() != ".git")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .filter_map(|entry| {
            let modified = entry.metadata().ok()?.modified().ok()?;
            let relative_path = entry.path().strip_prefix(project_root).ok()?.to_path_buf();
            if is_ignored_activity_path(&relative_path) {
                return None;
            }
            Some((relative_path, DateTime::<Utc>::from(modified)))
        })
        .max_by_key(|(_, modified)| *modified)
}

fn git_command(dir: &Path) -> Command {
    let mut command = Command::new("git");
    command.current_dir(dir);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    command
}

fn git_repo_root(project_root: &Path) -> Option<PathBuf> {
    let output = git_command(project_root)
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8(output.stdout).ok()?;
    Some(PathBuf::from(path.trim()))
}

fn git_dir(repo_root: &Path) -> Option<PathBuf> {
    let output = git_command(repo_root)
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let raw = String::from_utf8(output.stdout).ok()?;
    let path = PathBuf::from(raw.trim());
    Some(if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    })
}

fn git_head_commit_at(repo_root: &Path) -> Option<DateTime<Utc>> {
    git_head_signature(repo_root).map(|(_, committed_at)| committed_at)
}

fn git_head_signature(repo_root: &Path) -> Option<(String, DateTime<Utc>)> {
    let output = git_command(repo_root)
        .args(["log", "-1", "--format=%H%x00%ct"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }

    let mut parts = output.stdout.splitn(2, |byte| *byte == 0);
    let sha = String::from_utf8(parts.next()?.to_vec()).ok()?;
    let timestamp = String::from_utf8(parts.next()?.to_vec()).ok()?;
    let seconds = timestamp.trim().parse::<i64>().ok()?;
    Some((sha, DateTime::<Utc>::from_timestamp(seconds, 0)?))
}

fn git_dirty_paths(repo_root: &Path) -> Vec<PathBuf> {
    dirty_paths_from_porcelain(&git_status_output(repo_root))
}

fn git_status_output(repo_root: &Path) -> Vec<u8> {
    git_command(repo_root)
        .args([
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--no-renames",
        ])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| output.stdout)
        .unwrap_or_default()
}

fn dirty_paths_from_porcelain(output: &[u8]) -> Vec<PathBuf> {
    output
        .split(|byte| *byte == 0)
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| {
            if entry.len() <= 3 {
                return None;
            }
            let path = PathBuf::from(String::from_utf8_lossy(&entry[3..]).to_string());
            (!is_ignored_activity_path(&path)).then_some(path)
        })
        .collect()
}

fn is_ignored_activity_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    IGNORED_ACTIVITY_PREFIXES
        .iter()
        .any(|prefix| normalized == prefix.trim_end_matches('/') || normalized.starts_with(prefix))
}

fn git_index_modified_at(repo_root: &Path) -> Option<DateTime<Utc>> {
    let git_dir = git_dir(repo_root)?;
    let metadata = fs::metadata(git_dir.join("index")).ok()?;
    let modified = metadata.modified().ok()?;
    Some(DateTime::<Utc>::from(modified))
}

fn project_root(board_dir: &Path) -> PathBuf {
    board_dir.parent().unwrap_or(board_dir).to_path_buf()
}

fn update_hasher_with_metadata(hasher: &mut Sha256, path: &Path, metadata: &fs::Metadata) {
    hasher.update(path.to_string_lossy().as_bytes());
    hasher.update(metadata.len().to_le_bytes());
    if let Ok(modified) = metadata.modified()
        && let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH)
    {
        hasher.update(duration.as_secs().to_le_bytes());
        hasher.update(duration.subsec_nanos().to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use chrono::{Duration, Utc};
    use tempfile::TempDir;

    use super::{HeartbeatSource, fingerprint, git_command, project};

    #[test]
    fn heartbeat_prefers_dirty_worktree_activity() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        fs::write(temp.path().join("README.md"), "# Project\n").unwrap();
        git(temp.path(), &["add", "."]);
        git(temp.path(), &["commit", "-m", "seed"]);

        fs::write(temp.path().join("README.md"), "# Project changed\n").unwrap();

        let heartbeat = project(&temp.path().join(".keel"), Utc::now());

        assert_eq!(heartbeat.source, HeartbeatSource::DirtyWorktree);
        assert!(heartbeat.dirty);
        assert_eq!(
            heartbeat.latest_path,
            Some(Path::new("README.md").to_path_buf())
        );
    }

    #[test]
    fn heartbeat_falls_back_to_head_commit_when_clean() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        git(temp.path(), &["add", ".keel/README.md"]);
        git(temp.path(), &["commit", "-m", "seed"]);

        let heartbeat = project(&temp.path().join(".keel"), Utc::now());

        assert_eq!(heartbeat.source, HeartbeatSource::HeadCommit);
        assert!(!heartbeat.dirty);
        assert!(heartbeat.is_energized(Utc::now(), 10));
    }

    #[test]
    fn heartbeat_uses_filesystem_fallback_outside_git() {
        let temp = TempDir::new().unwrap();
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();

        let heartbeat = project(&temp.path().join(".keel"), Utc::now());

        assert_eq!(heartbeat.source, HeartbeatSource::FilesystemFallback);
        assert!(!heartbeat.dirty);
        assert!(heartbeat.age_seconds(Utc::now()) < Duration::minutes(1).num_seconds());
    }

    #[test]
    fn fingerprint_changes_when_worktree_changes() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        git(temp.path(), &["add", ".keel/README.md"]);
        git(temp.path(), &["commit", "-m", "seed"]);

        let before = fingerprint(&temp.path().join(".keel"));
        fs::write(temp.path().join(".keel/README.md"), "# Board changed\n").unwrap();
        let after = fingerprint(&temp.path().join(".keel"));

        assert_ne!(before, after);
    }

    #[test]
    fn heartbeat_ignores_keel_cache_artifacts() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel/cache")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        git(temp.path(), &["add", ".keel/README.md"]);
        git(temp.path(), &["commit", "-m", "seed"]);

        fs::write(
            temp.path().join(".keel/cache/doctor.json"),
            "{\"report\":\"generated\"}\n",
        )
        .unwrap();

        let heartbeat = project(&temp.path().join(".keel"), Utc::now());

        assert_eq!(heartbeat.source, HeartbeatSource::HeadCommit);
        assert!(!heartbeat.dirty);
        assert!(heartbeat.dirty_paths.is_empty());
    }

    #[test]
    fn fingerprint_ignores_keel_cache_artifacts() {
        let temp = TempDir::new().unwrap();
        init_git_repo(temp.path());
        fs::create_dir_all(temp.path().join(".keel/cache")).unwrap();
        fs::write(temp.path().join(".keel/README.md"), "# Board\n").unwrap();
        git(temp.path(), &["add", ".keel/README.md"]);
        git(temp.path(), &["commit", "-m", "seed"]);

        let before = fingerprint(&temp.path().join(".keel"));
        fs::write(
            temp.path().join(".keel/cache/doctor.json"),
            "{\"report\":\"generated\"}\n",
        )
        .unwrap();
        let after = fingerprint(&temp.path().join(".keel"));

        assert_eq!(before, after);
    }

    fn init_git_repo(dir: &Path) {
        git(dir, &["init", "--quiet"]);
        git(dir, &["config", "user.name", "Keel Test"]);
        git(dir, &["config", "user.email", "keel@example.com"]);
        git(dir, &["config", "commit.gpgsign", "false"]);
    }

    fn git(dir: &Path, args: &[&str]) {
        let output = git_command(dir).args(args).output().unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: stdout=`{}` stderr=`{}`",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
