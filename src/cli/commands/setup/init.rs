//! Initialize a new keel board

use anyhow::Result;
use std::path::Path;

use crate::infrastructure::config::Config;

/// Create `.keel` and `keel.toml` in the current directory.
pub fn run() -> Result<()> {
    let config = Config::default();
    init_board(Path::new("."), &config)
}

pub(crate) fn init_board(root: &Path, config: &Config) -> Result<()> {
    let config_path = root.join("keel.toml");
    let config_exists = config_path.exists();
    crate::infrastructure::board_init::init_board(root, config)?;
    let board_path = root.join(config.board_dir());

    if !config_exists {
        println!("Created {}", config_path.display());
    } else {
        println!(
            "Found existing {}. Skipped writing defaults.",
            config_path.display()
        );
    }

    println!("Initialized keel board in {}", board_path.display());
    println!("Created subdirectories:");
    for dir in crate::infrastructure::board_init::INIT_SUBDIRS {
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
