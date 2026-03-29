//! `keel hooks install` — install git hooks that enforce the pacemaker protocol.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use anyhow::{Context, Result, anyhow};

const PRE_COMMIT_HOOK: &str = r#"#!/bin/sh
# keel pacemaker protocol — run quality checks and tests at the commit boundary.
# Installed by `keel hooks install`. Do not edit manually.

# Run quality checks
echo "Running quality checks..."
just quality || exit 1

# Run tests
echo "Running tests..."
just test || exit 1

# Skip silently when no board is present.
if [ ! -d ".keel" ] && [ ! -f "keel.toml" ]; then
    # No board found — skip silently
    exit 0
fi
"#;

const COMMIT_MSG_HOOK: &str = r#"#!/bin/sh
# keel pacemaker protocol — auto-append doctor --status to commit messages.
# Installed by `keel hooks install`. Do not edit manually.

KEEL_BIN="${KEEL_BIN:-keel}"
COMMIT_MSG_FILE="$1"

# Skip merge commits and amends that already have status
if grep -q '\[MSN\]' "$COMMIT_MSG_FILE" 2>/dev/null; then
    exit 0
fi

STATUS=$($KEEL_BIN doctor --status 2>/dev/null)
if [ -n "$STATUS" ]; then
    printf '\n%s\n' "$STATUS" >> "$COMMIT_MSG_FILE"
fi
"#;

pub fn run() -> Result<()> {
    run_in(None)
}

pub fn run_in(working_dir: Option<&Path>) -> Result<()> {
    let git_dir = find_git_dir(working_dir)?;
    let hooks_dir = git_dir.join("hooks");

    fs::create_dir_all(&hooks_dir)
        .with_context(|| format!("Failed to create hooks directory: {}", hooks_dir.display()))?;

    install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK)?;
    install_hook(&hooks_dir, "commit-msg", COMMIT_MSG_HOOK)?;

    println!("Installed git hooks:");
    println!("  - pre-commit  (quality + test)");
    println!("  - commit-msg  (auto-append doctor --status)");

    Ok(())
}

fn find_git_dir(working_dir: Option<&Path>) -> Result<std::path::PathBuf> {
    let mut cmd = std::process::Command::new("git");
    cmd.args(["rev-parse", "--git-dir"]);
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }
    let output = cmd.output().context("Failed to run git rev-parse")?;

    if !output.status.success() {
        return Err(anyhow!("Not inside a git repository"));
    }

    let raw = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in git dir path")?
        .trim()
        .to_string();

    let path = std::path::PathBuf::from(&raw);
    if path.is_absolute() {
        Ok(path)
    } else if let Some(dir) = working_dir {
        Ok(dir.join(path))
    } else {
        Ok(path)
    }
}

fn install_hook(hooks_dir: &Path, name: &str, content: &str) -> Result<()> {
    let hook_path = hooks_dir.join(name);

    if hook_path.exists() {
        let existing = fs::read_to_string(&hook_path)
            .with_context(|| format!("Failed to read existing hook: {}", hook_path.display()))?;

        if existing.contains("keel pacemaker protocol") {
            // Our hook — safe to overwrite
        } else {
            return Err(anyhow!(
                "Existing {} hook is not managed by keel. \
                 Remove it manually or merge the pacemaker protocol into your existing hook.",
                name
            ));
        }
    }

    fs::write(&hook_path, content)
        .with_context(|| format!("Failed to write hook: {}", hook_path.display()))?;

    fs::set_permissions(&hook_path, fs::Permissions::from_mode(0o755))
        .with_context(|| format!("Failed to set permissions on: {}", hook_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn init_git_repo(dir: &Path) {
        std::process::Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(dir)
            .output()
            .unwrap();
    }

    #[test]
    fn install_creates_hooks_with_correct_permissions() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        let hooks_dir = temp.path().join(".git/hooks");
        install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK).unwrap();
        install_hook(&hooks_dir, "commit-msg", COMMIT_MSG_HOOK).unwrap();

        let pre_commit = hooks_dir.join("pre-commit");
        let commit_msg = hooks_dir.join("commit-msg");

        assert!(pre_commit.exists());
        assert!(commit_msg.exists());

        let pre_mode = fs::metadata(&pre_commit).unwrap().permissions().mode();
        assert_eq!(pre_mode & 0o755, 0o755);

        let content = fs::read_to_string(&pre_commit).unwrap();
        assert!(content.contains("keel pacemaker protocol"));
        assert!(content.contains("Running quality checks"));
        assert!(!content.contains(".keel/heartbeat"));
        assert!(!content.contains("git add"));
        assert!(!content.contains(" poke "));

        let msg_content = fs::read_to_string(&commit_msg).unwrap();
        assert!(msg_content.contains("doctor --status"));
    }

    #[test]
    fn pre_commit_hook_uses_commit_boundary_without_synthetic_heartbeat_file() {
        assert!(!PRE_COMMIT_HOOK.contains(".keel/heartbeat"));
        assert!(!PRE_COMMIT_HOOK.contains("git add"));
        assert!(!PRE_COMMIT_HOOK.contains("pre-commit auto-poke"));
        assert!(!PRE_COMMIT_HOOK.contains("KEEL_BIN"));
    }

    #[test]
    fn install_overwrites_keel_managed_hook() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        let hooks_dir = temp.path().join(".git/hooks");
        install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK).unwrap();

        // Second install should succeed (overwrites our own hook)
        install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK).unwrap();
    }

    #[test]
    fn install_refuses_to_overwrite_foreign_hook() {
        let temp = tempdir().unwrap();
        init_git_repo(temp.path());

        let hooks_dir = temp.path().join(".git/hooks");
        fs::create_dir_all(&hooks_dir).unwrap();
        fs::write(
            hooks_dir.join("pre-commit"),
            "#!/bin/sh\necho custom hook\n",
        )
        .unwrap();

        let result = install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("not managed by keel")
        );
    }
}
