#![allow(dead_code)]
//! Utility functions shared across the keel crate.

use regex::Regex;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

static ANSI_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap());

/// Get the current Git SHA of the repository.
pub fn get_git_sha(repo_path: &Path) -> anyhow::Result<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .current_dir(repo_path)
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "git rev-parse HEAD failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Calculate the SHA-256 hash of a file.
pub fn hash_file(path: &Path) -> anyhow::Result<String> {
    let output = Command::new("sha256sum").arg(path).output()?;

    if !output.status.success() {
        // Try shasum -a 256 for macOS compatibility just in case
        let output = Command::new("shasum")
            .arg("-a")
            .arg("256")
            .arg(path)
            .output()?;

        if !output.status.success() {
            anyhow::bail!("Failed to calculate sha256 hash for {:?}", path);
        }
        let hash = String::from_utf8_lossy(&output.stdout)
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_string();
        return Ok(hash);
    }

    let hash = String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string();
    Ok(hash)
}

/// Returns the width of the string as it would appear in a terminal,
/// ignoring ANSI escape sequences.
pub fn visible_width(s: &str) -> usize {
    ANSI_RE.replace_all(s, "").chars().count()
}

/// Returns the appropriate singular or plural form based on count.
///
/// # Examples
/// ```
/// use keel::infrastructure::utils::pluralize;
/// assert_eq!(pluralize(1, "story", "stories"), "story");
/// assert_eq!(pluralize(2, "story", "stories"), "stories");
/// ```
pub fn pluralize<'a>(count: usize, singular: &'a str, plural: &'a str) -> &'a str {
    if count == 1 { singular } else { plural }
}

/// Convert a title to a URL-friendly slug.
///
/// Non-alphanumeric characters become hyphens; consecutive hyphens are collapsed;
/// leading/trailing hyphens are stripped.
pub fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Check if a title follows Title Case convention.
///
/// Rules:
/// 1. First word must be capitalized.
/// 2. Significant words must be capitalized.
/// 3. Small words (a, an, the, etc.) can be lowercase if not first.
/// 4. Should not contain hyphens unless explicitly part of a word.
pub fn is_title_case(title: &str) -> bool {
    if title.is_empty() {
        return true;
    }

    // Discourage kebab-case or snake_case in titles
    if (title.contains('-') && !title.contains(' ')) || title.contains('_') {
        return false;
    }

    let words: Vec<&str> = title.split_whitespace().collect();
    for (i, word) in words.iter().enumerate() {
        let first_char = word.chars().next();
        if first_char.is_none_or(|c| !c.is_alphabetic()) {
            continue;
        }

        let first_char = first_char.unwrap();

        // First word must be capitalized
        if i == 0 {
            if !first_char.is_uppercase() {
                return false;
            }
        } else {
            // Other words: must be capitalized UNLESS it's a small word
            let lower = word.to_lowercase();
            let small_words = [
                "a", "an", "the", "and", "but", "or", "for", "nor", "on", "at", "to", "by", "of",
                "in", "as", "with", "into", "near", "from",
            ];

            if !first_char.is_uppercase() && !small_words.contains(&lower.as_str()) {
                return false;
            }
        }
    }

    true
}

/// Convert a string to Title Case.
///
/// Rules:
/// 1. First word is always capitalized.
/// 2. Subsequent words are capitalized unless they are common "small words" (a, an, the, etc.).
/// 3. Replaces hyphens and underscores with spaces.
pub fn to_title_case(text: &str) -> String {
    if text.is_empty() {
        return text.to_string();
    }

    let small_words = [
        "a", "an", "the", "and", "but", "or", "for", "nor", "on", "at", "to", "by", "of", "in",
        "as", "with", "into", "near", "from",
    ];

    let cleaned = text.replace(['-', '_'], " ");
    let words: Vec<&str> = cleaned.split_whitespace().collect();
    let mut title_words = Vec::new();

    for (i, word) in words.iter().enumerate() {
        let lower = word.to_lowercase();

        if i == 0 || !small_words.contains(&lower.as_str()) {
            // Capitalize
            let mut chars = word.chars();
            if let Some(first) = chars.next() {
                let capitalized = first.to_uppercase().to_string() + chars.as_str();
                title_words.push(capitalized);
            }
        } else {
            // Keep small word lowercase
            title_words.push(lower);
        }
    }

    title_words.join(" ")
}

/// Open the system editor to get manual input from the user.
pub fn get_manual_input(initial_content: &str) -> anyhow::Result<String> {
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let temp_file = tempfile::Builder::new()
        .prefix("keel-manual-input")
        .suffix(".md")
        .tempfile()?;

    if !initial_content.is_empty() {
        std::fs::write(temp_file.path(), initial_content)?;
    }

    let status = Command::new(&editor).arg(temp_file.path()).status()?;

    if !status.success() {
        anyhow::bail!("Editor exited with error status");
    }

    let content = std::fs::read_to_string(temp_file.path())?;
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pluralize_returns_singular_for_one() {
        assert_eq!(pluralize(1, "story", "stories"), "story");
    }

    #[test]
    fn pluralize_returns_plural_for_zero() {
        assert_eq!(pluralize(0, "story", "stories"), "stories");
    }

    #[test]
    fn pluralize_returns_plural_for_two() {
        assert_eq!(pluralize(2, "story", "stories"), "stories");
    }

    #[test]
    fn pluralize_returns_plural_for_many() {
        assert_eq!(pluralize(100, "item", "items"), "items");
    }

    #[test]
    fn pluralize_works_with_irregular_plurals() {
        assert_eq!(pluralize(1, "child", "children"), "child");
        assert_eq!(pluralize(2, "child", "children"), "children");
    }

    #[test]
    fn slugify_converts_title() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Add Login Form"), "add-login-form");
        assert_eq!(slugify("Fix Bug #123"), "fix-bug-123");
        assert_eq!(slugify("  Trim  Spaces  "), "trim-spaces");
        assert_eq!(
            slugify("Governance Bounded Context"),
            "governance-bounded-context"
        );
        assert_eq!(slugify("2-Queue Pull System"), "2-queue-pull-system");
    }

    #[test]
    fn to_title_case_converts_common_formats() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("kebab-case-title"), "Kebab Case Title");
        assert_eq!(to_title_case("snake_case_title"), "Snake Case Title");
        assert_eq!(
            to_title_case("a story with small words"),
            "A Story with Small Words"
        );
        assert_eq!(
            to_title_case("the beginning of the end"),
            "The Beginning of the End"
        );
    }
}
