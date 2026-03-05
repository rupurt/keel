//! Helpers for markdown artifact frontmatter mutation.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use chrono::NaiveDateTime;

use crate::infrastructure::frontmatter_mutation::{Mutation, apply};

pub fn format_datetime(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M:%S").to_string()
}

pub fn ensure_created_at(path: &Path, created_at: NaiveDateTime) -> Result<bool> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read artifact: {}", path.display()))?;
    if frontmatter_has_key(&content, "created_at") {
        return Ok(false);
    }

    let updated = if has_frontmatter(&content) {
        apply(
            &content,
            &[Mutation::set("created_at", format_datetime(created_at))],
        )
    } else {
        prepend_created_at_frontmatter(&content, created_at)
    };
    if updated == content {
        return Ok(false);
    }

    fs::write(path, updated)
        .with_context(|| format!("Failed to write artifact: {}", path.display()))?;
    Ok(true)
}

fn prepend_created_at_frontmatter(content: &str, created_at: NaiveDateTime) -> String {
    if content.is_empty() {
        return format!("---\ncreated_at: {}\n---\n", format_datetime(created_at));
    }

    format!(
        "---\ncreated_at: {}\n---\n\n{}",
        format_datetime(created_at),
        content
    )
}

fn has_frontmatter(content: &str) -> bool {
    content.trim_start().starts_with("---")
}

fn frontmatter_has_key(content: &str, key: &str) -> bool {
    if !has_frontmatter(content) {
        return false;
    }

    let mut lines = content.trim_start().lines();
    if lines.next() != Some("---") {
        return false;
    }

    for line in lines {
        if line == "---" {
            break;
        }

        let Some((candidate, _)) = line.split_once(':') else {
            continue;
        };
        if candidate.trim() == key {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::{ensure_created_at, format_datetime};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn ensure_created_at_inserts_into_existing_frontmatter() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("artifact.md");
        fs::write(&path, "---\nid: test\n---\n\n# Body\n").unwrap();

        let updated = ensure_created_at(
            &path,
            chrono::NaiveDateTime::parse_from_str("2026-03-05T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
        )
        .unwrap();

        assert!(updated);
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("created_at: 2026-03-05T10:30:00"));
    }

    #[test]
    fn ensure_created_at_prepends_frontmatter_when_missing() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("artifact.md");
        fs::write(&path, "# Body\n").unwrap();

        ensure_created_at(
            &path,
            chrono::NaiveDateTime::parse_from_str("2026-03-05T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
        )
        .unwrap();

        let content = fs::read_to_string(path).unwrap();
        assert!(content.starts_with("---\ncreated_at: 2026-03-05T10:30:00\n---\n\n# Body"));
    }

    #[test]
    fn ensure_created_at_is_noop_when_key_exists() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("artifact.md");
        fs::write(
            &path,
            "---\ncreated_at: 2026-03-05T10:30:00\n---\n\n# Body\n",
        )
        .unwrap();

        let updated = ensure_created_at(
            &path,
            chrono::NaiveDateTime::parse_from_str("2026-03-06T11:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap(),
        )
        .unwrap();

        assert!(!updated);
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("created_at: 2026-03-05T10:30:00"));
        assert!(!content.contains("2026-03-06T11:30:00"));
    }

    #[test]
    fn format_datetime_uses_strict_board_format() {
        let value =
            chrono::NaiveDateTime::parse_from_str("2026-03-05T10:30:00", "%Y-%m-%dT%H:%M:%S")
                .unwrap();
        assert_eq!(format_datetime(value), "2026-03-05T10:30:00");
    }
}
