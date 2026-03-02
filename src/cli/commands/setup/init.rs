//! Initialize a new keel board

use anyhow::{Context, Result, anyhow};
use std::path::Path;

use crate::infrastructure::config::Config;

const INIT_SUBDIRS: [&str; 4] = ["stories", "epics", "bearings", "adrs"];

/// Create `.keel` and `keel.toml` in the current directory.
pub fn run() -> Result<()> {
    let config = Config::default();
    init_board(Path::new("."), &config)
}

fn init_board(root: &Path, config: &Config) -> Result<()> {
    let board_dir_name = config.board_dir();
    let board_path = root.join(board_dir_name);

    if board_path.exists() && !board_path.is_dir() {
        return Err(anyhow!(
            "Board path '{}' exists but is not a directory",
            board_path.display()
        ));
    }

    // Create board root directory
    std::fs::create_dir_all(&board_path)
        .with_context(|| format!("Failed to create board directory {}", board_path.display()))?;

    // Create required subdirectories
    for dir in INIT_SUBDIRS {
        let dir_path = board_path.join(dir);
        std::fs::create_dir_all(&dir_path).with_context(|| {
            format!(
                "Failed to create board subdirectory {}",
                dir_path.as_os_str().to_string_lossy()
            )
        })?;
    }

    // Create default project config if missing
    let config_path = root.join("keel.toml");
    if !config_path.exists() {
        // We need to temporarily change dir or modify save_config to accept a path
        // For now, let's just write the string directly since save_config always writes to "keel.toml"
        let toml = toml::to_string(config)?;
        std::fs::write(&config_path, toml)?;
        println!("Created {}", config_path.display());
    } else {
        println!(
            "Found existing {}. Skipped writing defaults.",
            config_path.display()
        );
    }

    println!("Initialized keel board in {}", board_path.display());
    println!("Created subdirectories:");
    for dir in INIT_SUBDIRS {
        println!("  - {}/{}", board_path.display(), dir);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_board() {
        let temp = tempdir().unwrap();
        let root = temp.path();
        let config = Config::default();

        init_board(root, &config).unwrap();

        assert!(root.join(".keel").is_dir());
        assert!(root.join(".keel/stories").is_dir());
        assert!(root.join(".keel/epics").is_dir());
        assert!(root.join(".keel/bearings").is_dir());
        assert!(root.join(".keel/adrs").is_dir());
        assert!(root.join("keel.toml").is_file());
    }
}
