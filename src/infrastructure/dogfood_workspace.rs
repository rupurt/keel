//! Checked-in secondary workspace helpers for dogfood e2e flows.

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::infrastructure::config::Config;
use crate::infrastructure::generate::board_readme;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::throughput_history_store;

pub const DOGFOOD_WORKSPACE_ROOT: &str = "testdata/dogfood/workspace";

pub fn workspace_root(repo_root: &Path) -> PathBuf {
    repo_root.join(DOGFOOD_WORKSPACE_ROOT)
}

pub fn board_dir(repo_root: &Path) -> PathBuf {
    workspace_root(repo_root).join(Config::default().board_dir())
}

pub fn ensure_workspace(repo_root: &Path) -> Result<()> {
    let root = workspace_root(repo_root);
    fs::create_dir_all(&root)
        .with_context(|| format!("Failed to create dogfood workspace {}", root.display()))?;

    crate::infrastructure::board_init::init_board(&root, &Config::default())?;
    regenerate_workspace_artifacts(repo_root)
}

pub fn reset_workspace(repo_root: &Path) -> Result<()> {
    let root = workspace_root(repo_root);
    if root.exists() {
        for entry in fs::read_dir(&root)
            .with_context(|| format!("Failed to read dogfood workspace {}", root.display()))?
        {
            let path = entry?.path();
            if path.is_dir() {
                fs::remove_dir_all(&path).with_context(|| {
                    format!(
                        "Failed to reset dogfood workspace directory {}",
                        path.display()
                    )
                })?;
            } else {
                fs::remove_file(&path).with_context(|| {
                    format!("Failed to reset dogfood workspace file {}", path.display())
                })?;
            }
        }
    }

    ensure_workspace(repo_root)
}

fn regenerate_workspace_artifacts(repo_root: &Path) -> Result<()> {
    let board_dir = board_dir(repo_root);
    let board = load_board(&board_dir)?;
    board_readme::generate(&board_dir, &board)?;
    let history = crate::read_model::throughput_history::project_default(&board);
    throughput_history_store::save_if_changed(&board_dir, &history)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn dogfood_workspace_scaffold_has_secondary_board() {
        let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let workspace = workspace_root(repo_root);
        assert!(
            workspace.join("keel.toml").is_file(),
            "expected checked-in dogfood workspace at {}",
            workspace.display()
        );
        assert!(
            board_dir(repo_root).join("README.md").is_file(),
            "expected checked-in dogfood board README at {}",
            board_dir(repo_root).display()
        );
        for dir in ["stories", "epics", "bearings", "adrs"] {
            assert!(
                board_dir(repo_root).join(dir).is_dir(),
                "expected checked-in dogfood board subdirectory {}",
                dir
            );
        }
    }

    #[test]
    fn dogfood_workspace_reset_preserves_primary_board() {
        let temp = tempdir().unwrap();
        let repo_root = temp.path();
        fs::create_dir_all(repo_root.join(".keel")).unwrap();
        fs::write(repo_root.join(".keel/README.md"), "primary board").unwrap();

        ensure_workspace(repo_root).unwrap();
        fs::write(
            board_dir(repo_root).join("README.md"),
            "mutated dogfood board readme",
        )
        .unwrap();
        fs::write(
            workspace_root(repo_root).join("scratch.txt"),
            "should be removed on reset",
        )
        .unwrap();

        reset_workspace(repo_root).unwrap();

        assert_eq!(
            fs::read_to_string(repo_root.join(".keel/README.md")).unwrap(),
            "primary board"
        );
        assert!(
            !workspace_root(repo_root).join("scratch.txt").exists(),
            "reset should remove transient dogfood workspace files"
        );
        assert!(
            board_dir(repo_root).join("README.md").is_file(),
            "reset should restore dogfood board artifacts"
        );
    }

    #[test]
    fn dogfood_workspace_board_discovery_prefers_nested_board() {
        let temp = tempdir().unwrap();
        let repo_root = temp.path();
        fs::create_dir_all(repo_root.join(".keel")).unwrap();
        fs::write(repo_root.join(".keel/README.md"), "primary board").unwrap();
        ensure_workspace(repo_root).unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(workspace_root(repo_root)).unwrap();
        let found = crate::infrastructure::config::find_board_dir().unwrap();
        std::env::set_current_dir(original_dir).unwrap();

        assert_eq!(found, board_dir(repo_root));
    }
}
