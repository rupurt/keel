//! Repo-local Mission Stack projection.
//!
//! Mission Stack remains parallel to the board model: stack metadata is loaded
//! from `.keel/stacks/*/manifest.yaml` and combined with current git/worktree
//! state only when present.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MissionStackScan {
    pub stacks: Vec<MissionStackProjection>,
    pub load_problems: Vec<MissionStackLoadProblem>,
}

impl MissionStackScan {
    pub fn active_stack(&self) -> Option<&MissionStackProjection> {
        let mut active = self
            .stacks
            .iter()
            .filter(|stack| stack.lifecycle == MissionStackLifecycle::Active);
        let first = active.next()?;
        if active.next().is_some() {
            return None;
        }
        Some(first)
    }

    pub fn closed_stacks(&self) -> impl Iterator<Item = &MissionStackProjection> {
        self.stacks
            .iter()
            .filter(|stack| stack.lifecycle == MissionStackLifecycle::Closed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionStackLoadProblem {
    pub path: PathBuf,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissionStackProjection {
    pub manifest_path: PathBuf,
    pub id: String,
    pub lifecycle: MissionStackLifecycle,
    pub steward_repo: String,
    pub local_repo: String,
    pub branch: String,
    pub current_branch: Option<String>,
    pub branch_matches: bool,
    pub local_member: MissionStackMemberProjection,
    pub members: Vec<MissionStackMemberProjection>,
    pub mode: MissionStackModeProjection,
    pub checkpoint: Option<MissionStackCheckpointProjection>,
    pub checkout: MissionStackCheckoutProjection,
}

impl MissionStackProjection {
    pub fn mode_label(&self) -> &'static str {
        match self.mode {
            MissionStackModeProjection::Exclusive { .. } => "exclusive",
            MissionStackModeProjection::Shared { .. } => "shared",
            MissionStackModeProjection::Checkpoint { .. } => "checkpoint",
        }
    }

    pub fn waiting_on_checkpoint_members(&self) -> Vec<String> {
        let Some(checkpoint) = &self.checkpoint else {
            return Vec::new();
        };

        checkpoint
            .required_members
            .iter()
            .filter(|repo| {
                !checkpoint
                    .acknowledged_members
                    .iter()
                    .any(|ack| ack == *repo)
            })
            .cloned()
            .collect()
    }

    pub fn pending_negotiation_members(&self) -> Vec<&MissionStackMemberProjection> {
        self.members
            .iter()
            .filter(|member| member.pending_negotiation)
            .collect()
    }

    pub fn waiting_receipt_members(&self) -> Vec<&MissionStackMemberProjection> {
        self.members
            .iter()
            .filter(|member| !member.waiting_for_receipts_from.is_empty())
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MissionStackLifecycle {
    #[default]
    Active,
    Closed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissionStackMemberProjection {
    pub repo: String,
    pub role: MissionStackMemberRole,
    pub state: String,
    pub mission: Option<String>,
    pub pending_negotiation: bool,
    pub waiting_for_receipts_from: Vec<String>,
    pub checkpoint_acknowledged: bool,
    pub receipt: Option<MissionStackReceiptProjection>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MissionStackMemberRole {
    Steward,
    #[default]
    Member,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissionStackReceiptProjection {
    pub branch: String,
    pub head_sha: String,
    pub remote: Option<String>,
    pub checkpoint: Option<String>,
    pub handoff: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MissionStackModeProjection {
    Exclusive {
        active_repo: String,
    },
    Shared {
        active_repos: Vec<String>,
    },
    Checkpoint {
        name: String,
        required_members: Vec<String>,
        acknowledged_members: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissionStackCheckpointProjection {
    pub name: String,
    pub required_members: Vec<String>,
    pub acknowledged_members: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MissionStackCheckoutProjection {
    pub repo_root: Option<PathBuf>,
    pub current_checkout: Option<PathBuf>,
    pub is_linked_worktree: bool,
    pub foreign_execution_required: bool,
    pub foreign_execution_state: MissionStackForeignExecutionState,
    pub managed_root: Option<PathBuf>,
    pub managed_path: Option<PathBuf>,
    pub managed_paths: Vec<PathBuf>,
    pub leftover_managed_paths: Vec<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionStackForeignExecutionState {
    NotRequired,
    Ready,
    MissingManagedPath,
    MissingManagedCheckout,
    WrongCheckout,
    PrimaryCheckoutDisallowed,
}

#[derive(Debug, Deserialize)]
struct ManifestFile {
    id: String,
    #[serde(default)]
    state: MissionStackLifecycle,
    steward_repo: String,
    local_repo: String,
    branch: Option<String>,
    mode: ManifestMode,
    members: Vec<ManifestMember>,
    checkpoint: Option<ManifestCheckpoint>,
    foreign_execution: Option<ManifestForeignExecution>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum ManifestMode {
    Exclusive {
        active_repo: String,
    },
    Shared {
        active_repos: Vec<String>,
    },
    Checkpoint {
        name: String,
        required_members: Vec<String>,
        #[serde(default)]
        acknowledged_members: Vec<String>,
    },
}

#[derive(Debug, Deserialize)]
struct ManifestMember {
    repo: String,
    #[serde(default)]
    role: MissionStackMemberRole,
    #[serde(default = "default_member_state")]
    state: String,
    mission: Option<String>,
    #[serde(default)]
    pending_negotiation: bool,
    #[serde(default)]
    waiting_for_receipts_from: Vec<String>,
    receipt: Option<ManifestReceipt>,
}

#[derive(Debug, Deserialize)]
struct ManifestReceipt {
    branch: Option<String>,
    head_sha: String,
    remote: Option<String>,
    checkpoint: Option<String>,
    handoff: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ManifestCheckpoint {
    name: String,
    required_members: Vec<String>,
    #[serde(default)]
    acknowledged_members: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct ManifestForeignExecution {
    #[serde(default)]
    required: bool,
    managed_root: Option<PathBuf>,
    managed_path: Option<PathBuf>,
    #[serde(default)]
    managed_paths: Vec<PathBuf>,
}

fn default_member_state() -> String {
    "idle".to_string()
}

pub fn scan(board_dir: &Path) -> Result<MissionStackScan> {
    let manifests_dir = board_dir.join("stacks");
    if !manifests_dir.exists() {
        return Ok(MissionStackScan::default());
    }

    let git = GitContext::discover(board_dir);
    let resolution_root = git
        .repo_root
        .clone()
        .unwrap_or_else(|| board_dir.to_path_buf());
    let mut scan = MissionStackScan::default();
    let mut manifest_paths = discover_manifest_paths(&manifests_dir)?;
    manifest_paths.sort();

    for manifest_path in manifest_paths {
        match load_manifest_projection(&manifest_path, &resolution_root, &git) {
            Ok(stack) => scan.stacks.push(stack),
            Err(error) => scan.load_problems.push(MissionStackLoadProblem {
                path: manifest_path,
                message: error.to_string(),
            }),
        }
    }

    let active_ids: Vec<_> = scan
        .stacks
        .iter()
        .filter(|stack| stack.lifecycle == MissionStackLifecycle::Active)
        .map(|stack| stack.id.clone())
        .collect();
    if active_ids.len() > 1 {
        scan.load_problems.push(MissionStackLoadProblem {
            path: manifests_dir,
            message: format!(
                "multiple active Mission Stack manifests found: {}",
                active_ids.join(", ")
            ),
        });
    }

    Ok(scan)
}

fn discover_manifest_paths(stacks_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    for entry in fs::read_dir(stacks_dir)
        .with_context(|| format!("read Mission Stack directory {}", stacks_dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let manifest_path = entry.path().join("manifest.yaml");
        if manifest_path.exists() {
            paths.push(manifest_path);
        }
    }

    Ok(paths)
}

fn load_manifest_projection(
    manifest_path: &Path,
    resolution_root: &Path,
    git: &GitContext,
) -> Result<MissionStackProjection> {
    let raw = fs::read_to_string(manifest_path)
        .with_context(|| format!("read Mission Stack manifest {}", manifest_path.display()))?;
    let manifest: ManifestFile = serde_yaml::from_str(&raw)
        .with_context(|| format!("parse Mission Stack manifest {}", manifest_path.display()))?;
    if manifest.members.is_empty() {
        bail!("manifest must declare at least one member");
    }

    let branch = manifest
        .branch
        .clone()
        .unwrap_or_else(|| format!("stack/{}", manifest.id));
    let checkpoint = merge_checkpoint(&manifest.mode, manifest.checkpoint.as_ref());

    let members = build_members(&manifest, &branch, checkpoint.as_ref())?;
    let local_member = members
        .iter()
        .find(|member| member.repo == manifest.local_repo)
        .cloned()
        .ok_or_else(|| anyhow!("local_repo `{}` missing from members", manifest.local_repo))?;

    let mode = match &manifest.mode {
        ManifestMode::Exclusive { active_repo } => MissionStackModeProjection::Exclusive {
            active_repo: active_repo.clone(),
        },
        ManifestMode::Shared { active_repos } => MissionStackModeProjection::Shared {
            active_repos: active_repos.clone(),
        },
        ManifestMode::Checkpoint {
            name,
            required_members,
            acknowledged_members,
        } => MissionStackModeProjection::Checkpoint {
            name: name.clone(),
            required_members: required_members.clone(),
            acknowledged_members: acknowledged_members.clone(),
        },
    };

    let checkout = build_checkout_projection(
        resolution_root,
        git,
        manifest.foreign_execution.as_ref(),
        manifest.state,
    );

    Ok(MissionStackProjection {
        manifest_path: manifest_path.to_path_buf(),
        id: manifest.id,
        lifecycle: manifest.state,
        steward_repo: manifest.steward_repo,
        local_repo: manifest.local_repo,
        branch: branch.clone(),
        current_branch: git.current_branch.clone(),
        branch_matches: git.current_branch.as_deref() == Some(branch.as_str()),
        local_member,
        members,
        mode,
        checkpoint,
        checkout,
    })
}

fn build_members(
    manifest: &ManifestFile,
    branch: &str,
    checkpoint: Option<&MissionStackCheckpointProjection>,
) -> Result<Vec<MissionStackMemberProjection>> {
    let mut seen = BTreeSet::new();
    let mut members = Vec::new();

    for member in &manifest.members {
        if !seen.insert(member.repo.clone()) {
            bail!("duplicate member repo `{}` in manifest", member.repo);
        }

        let checkpoint_acknowledged = checkpoint.is_some_and(|checkpoint| {
            checkpoint
                .acknowledged_members
                .iter()
                .any(|repo| repo == &member.repo)
        });

        let receipt = member
            .receipt
            .as_ref()
            .map(|receipt| MissionStackReceiptProjection {
                branch: receipt.branch.clone().unwrap_or_else(|| branch.to_string()),
                head_sha: receipt.head_sha.clone(),
                remote: receipt.remote.clone(),
                checkpoint: receipt.checkpoint.clone(),
                handoff: receipt.handoff.clone(),
            });

        members.push(MissionStackMemberProjection {
            repo: member.repo.clone(),
            role: member.role,
            state: member.state.clone(),
            mission: member.mission.clone(),
            pending_negotiation: member.pending_negotiation,
            waiting_for_receipts_from: member.waiting_for_receipts_from.clone(),
            checkpoint_acknowledged,
            receipt,
        });
    }

    members.sort_by(|left, right| left.repo.cmp(&right.repo));
    Ok(members)
}

fn merge_checkpoint(
    mode: &ManifestMode,
    checkpoint: Option<&ManifestCheckpoint>,
) -> Option<MissionStackCheckpointProjection> {
    match mode {
        ManifestMode::Checkpoint {
            name,
            required_members,
            acknowledged_members,
        } => Some(MissionStackCheckpointProjection {
            name: name.clone(),
            required_members: required_members.clone(),
            acknowledged_members: acknowledged_members.clone(),
        }),
        _ => checkpoint.map(|checkpoint| MissionStackCheckpointProjection {
            name: checkpoint.name.clone(),
            required_members: checkpoint.required_members.clone(),
            acknowledged_members: checkpoint.acknowledged_members.clone(),
        }),
    }
}

fn build_checkout_projection(
    resolution_root: &Path,
    git: &GitContext,
    foreign_execution: Option<&ManifestForeignExecution>,
    lifecycle: MissionStackLifecycle,
) -> MissionStackCheckoutProjection {
    let Some(foreign_execution) = foreign_execution else {
        return MissionStackCheckoutProjection {
            repo_root: git.repo_root.clone(),
            current_checkout: git.checkout_root.clone(),
            is_linked_worktree: git.is_linked_worktree,
            foreign_execution_required: false,
            foreign_execution_state: MissionStackForeignExecutionState::NotRequired,
            managed_root: None,
            managed_path: None,
            managed_paths: Vec::new(),
            leftover_managed_paths: Vec::new(),
        };
    };

    let managed_root = foreign_execution
        .managed_root
        .as_ref()
        .map(|path| resolve_stack_path(path, resolution_root));
    let managed_path = foreign_execution
        .managed_path
        .as_ref()
        .map(|path| resolve_stack_path(path, resolution_root));
    let mut managed_paths: Vec<PathBuf> = foreign_execution
        .managed_paths
        .iter()
        .map(|path| resolve_stack_path(path, resolution_root))
        .collect();
    if let Some(path) = &managed_path
        && !managed_paths.iter().any(|existing| existing == path)
    {
        managed_paths.push(path.clone());
    }
    managed_paths.sort();
    managed_paths.dedup();

    let foreign_execution_state = derive_foreign_execution_state(
        foreign_execution.required,
        git.checkout_root.as_ref(),
        git.is_linked_worktree,
        managed_path.as_ref(),
    );

    let leftover_managed_paths = if lifecycle == MissionStackLifecycle::Closed {
        managed_paths
            .iter()
            .filter(|path| git.worktree_paths.iter().any(|worktree| worktree == *path))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };

    MissionStackCheckoutProjection {
        repo_root: git.repo_root.clone(),
        current_checkout: git.checkout_root.clone(),
        is_linked_worktree: git.is_linked_worktree,
        foreign_execution_required: foreign_execution.required,
        foreign_execution_state,
        managed_root,
        managed_path,
        managed_paths,
        leftover_managed_paths,
    }
}

fn resolve_stack_path(path: &Path, root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn derive_foreign_execution_state(
    required: bool,
    current_checkout: Option<&PathBuf>,
    is_linked_worktree: bool,
    managed_path: Option<&PathBuf>,
) -> MissionStackForeignExecutionState {
    if !required {
        return MissionStackForeignExecutionState::NotRequired;
    }

    let Some(managed_path) = managed_path else {
        return MissionStackForeignExecutionState::MissingManagedPath;
    };

    if !managed_path.exists() {
        return MissionStackForeignExecutionState::MissingManagedCheckout;
    }

    let Some(current_checkout) = current_checkout else {
        return MissionStackForeignExecutionState::WrongCheckout;
    };

    if current_checkout != managed_path {
        return MissionStackForeignExecutionState::WrongCheckout;
    }

    if !is_linked_worktree {
        return MissionStackForeignExecutionState::PrimaryCheckoutDisallowed;
    }

    MissionStackForeignExecutionState::Ready
}

#[derive(Debug, Default)]
struct GitContext {
    repo_root: Option<PathBuf>,
    checkout_root: Option<PathBuf>,
    current_branch: Option<String>,
    is_linked_worktree: bool,
    worktree_paths: Vec<PathBuf>,
}

impl GitContext {
    fn discover(board_dir: &Path) -> Self {
        let repo_root = git_stdout(board_dir, &["rev-parse", "--show-toplevel"])
            .map(PathBuf::from)
            .ok();
        let checkout_root = repo_root.clone();
        let current_branch = git_stdout(board_dir, &["branch", "--show-current"])
            .ok()
            .filter(|value| !value.is_empty());

        let git_dir = git_stdout(board_dir, &["rev-parse", "--absolute-git-dir"]).ok();
        let common_dir = git_stdout(
            board_dir,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )
        .ok();
        let is_linked_worktree = match (git_dir.as_deref(), common_dir.as_deref()) {
            (Some(git_dir), Some(common_dir)) => git_dir != common_dir,
            _ => false,
        };

        let worktree_paths = git_worktree_paths(board_dir).unwrap_or_default();

        Self {
            repo_root,
            checkout_root,
            current_branch,
            is_linked_worktree,
            worktree_paths,
        }
    }
}

fn git_worktree_paths(dir: &Path) -> Result<Vec<PathBuf>> {
    let output = git_command(dir, &["worktree", "list", "--porcelain"])?;
    let mut paths = Vec::new();
    for line in output.lines() {
        if let Some(path) = line.strip_prefix("worktree ") {
            paths.push(PathBuf::from(path));
        }
    }
    Ok(paths)
}

fn git_stdout(dir: &Path, args: &[&str]) -> Result<String> {
    Ok(git_command(dir, args)?.trim().to_string())
}

fn git_command(dir: &Path, args: &[&str]) -> Result<String> {
    let mut command = Command::new("git");
    command.args(args).current_dir(dir);
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("GIT_") {
            command.env_remove(key);
        }
    }
    let output = command
        .output()
        .with_context(|| format!("run git command {:?} in {}", args, dir.display()))?;
    if !output.status.success() {
        bail!(
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};
    use std::process::Command;

    #[test]
    fn mission_stack_absent_repo_is_noop() {
        let temp = TestBoardBuilder::new().build();
        let scan = scan(temp.path()).expect("scan without stack manifest");

        assert!(scan.stacks.is_empty());
        assert!(scan.load_problems.is_empty());
        assert!(scan.active_stack().is_none());
    }

    #[test]
    fn mission_stack_loads_projection_from_manifest_and_git_state() {
        let temp = stack_fixture();
        init_git_repo(temp.path());
        git(temp.path(), &["checkout", "-b", "stack/demo-stack"]);

        let manifest = r#"
id: demo-stack
steward_repo: keel
local_repo: keel
mode:
  kind: exclusive
  active_repo: keel
members:
  - repo: keel
    role: steward
    state: active
    mission: M1
    receipt:
      head_sha: deadbeef
  - repo: paddles
    role: member
    state: waiting
    mission: M2
    pending_negotiation: true
    waiting_for_receipts_from:
      - keel
"#;
        write_manifest(temp.path(), "demo-stack", manifest);

        let scan = scan(temp.path()).expect("stack scan");
        let stack = scan.active_stack().expect("active stack");

        assert_eq!(stack.id, "demo-stack");
        assert_eq!(stack.steward_repo, "keel");
        assert_eq!(stack.local_repo, "keel");
        assert_eq!(stack.branch, "stack/demo-stack");
        assert_eq!(stack.current_branch.as_deref(), Some("stack/demo-stack"));
        assert!(stack.branch_matches);
        assert_eq!(stack.local_member.role, MissionStackMemberRole::Steward);
        assert_eq!(stack.local_member.mission.as_deref(), Some("M1"));
        assert_eq!(
            stack.members[1].waiting_for_receipts_from,
            vec!["keel".to_string()]
        );
        assert_eq!(
            stack.members[0].receipt.as_ref().expect("receipt").branch,
            "stack/demo-stack"
        );
    }

    #[test]
    fn mission_stack_derives_branch_and_worktree_state() {
        let temp = stack_fixture();
        init_git_repo(temp.path());
        git(temp.path(), &["checkout", "-b", "stack/demo-stack"]);

        let manifest = format!(
            r#"
id: demo-stack
steward_repo: keel
local_repo: keel
branch: stack/demo-stack
mode:
  kind: shared
  active_repos:
    - keel
members:
  - repo: keel
    role: steward
    state: active
foreign_execution:
  required: true
  managed_path: {}
"#,
            temp.path().display()
        );
        write_manifest(temp.path(), "demo-stack", &manifest);

        let scan = scan(temp.path()).expect("stack scan");
        let stack = scan.active_stack().expect("active stack");

        assert_eq!(stack.current_branch.as_deref(), Some("stack/demo-stack"));
        assert_eq!(
            stack.checkout.current_checkout.as_deref(),
            Some(temp.path())
        );
        assert!(!stack.checkout.is_linked_worktree);
        assert_eq!(
            stack.checkout.foreign_execution_state,
            MissionStackForeignExecutionState::PrimaryCheckoutDisallowed
        );
    }

    fn stack_fixture() -> tempfile::TempDir {
        TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("active")
                    .activated_at(
                        chrono::NaiveDateTime::parse_from_str(
                            "2026-01-01T00:00:00",
                            "%Y-%m-%dT%H:%M:%S",
                        )
                        .unwrap(),
                    ),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("planned"))
            .story(TestStory::new("S1").scope("E1/V1"))
            .build()
    }

    fn write_manifest(board_dir: &Path, id: &str, yaml: &str) {
        let manifest_dir = board_dir.join("stacks").join(id);
        fs::create_dir_all(&manifest_dir).unwrap();
        fs::write(manifest_dir.join("manifest.yaml"), yaml.trim_start()).unwrap();
    }

    fn init_git_repo(repo_root: &Path) {
        git(repo_root, &["init"]);
        git(repo_root, &["config", "user.name", "Keel Test"]);
        git(repo_root, &["config", "user.email", "keel@example.com"]);
        git(repo_root, &["config", "commit.gpgsign", "false"]);
        fs::write(repo_root.join("README.md"), "# Test Repo\n").unwrap();
        git(repo_root, &["add", "."]);
        git(repo_root, &["commit", "-m", "initial"]);
    }

    fn git(repo_root: &Path, args: &[&str]) {
        let mut command = Command::new("git");
        command.args(args).current_dir(repo_root);
        for (key, _) in std::env::vars_os() {
            if key.to_string_lossy().starts_with("GIT_") {
                command.env_remove(key);
            }
        }
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: stdout=`{}` stderr=`{}`",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
