//! Shared filesystem helpers for generated text artifacts.

use std::fs;
use std::path::Path;

use anyhow::Result;

/// Write generated text only when the content actually changed.
pub fn write_if_changed(path: &Path, content: &str) -> Result<bool> {
    let existing = fs::read_to_string(path).ok();
    if existing.as_deref() == Some(content) {
        return Ok(false);
    }

    fs::write(path, content)?;
    Ok(true)
}

/// Remove a generated artifact if it exists.
pub fn remove_if_exists(path: &Path) -> Result<bool> {
    if path.exists() {
        fs::remove_file(path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Read file names from a directory in deterministic filename order.
pub fn read_sorted_file_names(path: &Path) -> Result<Vec<String>> {
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut names = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_file() {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    names.sort();
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn write_if_changed_skips_identical_content() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("artifact.md");
        fs::write(&path, "same").unwrap();

        let changed = write_if_changed(&path, "same").unwrap();

        assert!(!changed);
        assert_eq!(fs::read_to_string(path).unwrap(), "same");
    }

    #[test]
    fn remove_if_exists_is_safe_for_missing_files() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("missing.md");

        let removed = remove_if_exists(&path).unwrap();

        assert!(!removed);
    }

    #[test]
    fn read_sorted_file_names_returns_canonical_order() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("evidence");
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("zeta.log"), "z").unwrap();
        fs::write(path.join("alpha.log"), "a").unwrap();

        let names = read_sorted_file_names(&path).unwrap();

        assert_eq!(names, vec!["alpha.log".to_string(), "zeta.log".to_string()]);
    }
}
