//! Shared board scaffold helpers.

use anyhow::{Context, Result, anyhow};
use std::path::Path;

use crate::infrastructure::config::Config;

pub const INIT_SUBDIRS: [&str; 4] = ["stories", "epics", "bearings", "adrs"];

/// Create a keel board rooted at `root` using the provided config.
pub fn init_board(root: &Path, config: &Config) -> Result<()> {
    let board_dir_name = config.board_dir();
    let board_path = root.join(board_dir_name);

    if board_path.exists() && !board_path.is_dir() {
        return Err(anyhow!(
            "Board path '{}' exists but is not a directory",
            board_path.display()
        ));
    }

    std::fs::create_dir_all(&board_path)
        .with_context(|| format!("Failed to create board directory {}", board_path.display()))?;

    for dir in INIT_SUBDIRS {
        let dir_path = board_path.join(dir);
        std::fs::create_dir_all(&dir_path).with_context(|| {
            format!(
                "Failed to create board subdirectory {}",
                dir_path.as_os_str().to_string_lossy()
            )
        })?;
    }

    let config_path = root.join("keel.toml");
    if !config_path.exists() {
        let toml = toml::to_string(config)?;
        std::fs::write(&config_path, toml)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn init_board_creates_default_layout() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let config = Config::default();

        init_board(root, &config).unwrap();

        assert!(root.join(".keel").is_dir());
        for dir in INIT_SUBDIRS {
            assert!(root.join(".keel").join(dir).is_dir());
        }
        assert!(root.join("keel.toml").is_file());
    }
}
