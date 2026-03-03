use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

use super::super::types::*;

/// Known valid frontmatter field names for each entity type.
/// These include both the canonical serde-serialized names and backwards-compatible aliases.
pub const STORY_FIELDS: &[&str] = &[
    "id",
    "title",
    "type",
    "status",
    "scope",
    "milestone",
    "created_at",
    "updated_at",
    "completed_at",
    "submitted_at",
    "index",
    "governed-by",
    "role",
    "priority",
    "depends",
    // Backwards-compat aliases
    "created",
    "updated",
    "completed",
    "submitted",
    "story_type",
    // Legacy fields (not in struct but accepted on disk)
    "estimate",
    "covers",
    "resolved",
];

pub const VOYAGE_FIELDS: &[&str] = &[
    "id",
    "title",
    "status",
    "epic",
    "created_at",
    "started_at",
    "updated_at",
    "completed_at",
    // Date fields managed by update_frontmatter
    "completed",
    // Legacy/backwards-compat
    "created",
];

pub const EPIC_FIELDS: &[&str] = &[
    "id",
    "title",
    // Kept here to avoid duplicate unknown-field warnings; parser/structural
    // checks enforce that epic status is no longer allowed.
    "status",
    "description",
    "created_at",
    // Link to bearing that originated this epic
    "bearing",
    "index",
    // Legacy/backwards-compat
    "created",
];

pub const ADR_FIELDS: &[&str] = &[
    "id",
    "title",
    "status",
    "context",
    "applies-to",
    "supersedes",
    "superseded-by",
    "rejection-reason",
    "deprecation-reason",
    "date",
];

/// Extract raw YAML frontmatter keys from file content.
/// Returns None if no valid frontmatter block is found.
pub fn extract_frontmatter_keys(content: &str) -> Option<Vec<String>> {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return None;
    }
    let after_open = &trimmed[3..];
    let rest = after_open.trim_start_matches([' ', '\t']);
    if !rest.starts_with('\n') && !rest.starts_with("\r\n") {
        return None;
    }
    let yaml_start = rest.find('\n').map(|i| i + 1)?;
    let yaml_rest = &rest[yaml_start..];
    let closing = yaml_rest
        .find("\n---")
        .or_else(|| yaml_rest.find("\r\n---"))?;
    let yaml_content = &yaml_rest[..closing];

    let value: serde_yaml::Value = serde_yaml::from_str(yaml_content).ok()?;
    if let serde_yaml::Value::Mapping(map) = value {
        Some(
            map.keys()
                .filter_map(|k| k.as_str().map(|s| s.to_string()))
                .collect(),
        )
    } else {
        None
    }
}

/// Determine entity type from file path and return the valid field set.
pub fn fields_for_path(path: &Path, board_dir: &Path) -> Option<&'static [&'static str]> {
    let relative = path.strip_prefix(board_dir).ok()?;
    let components: Vec<&str> = relative
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();

    if components.first() == Some(&"stories") {
        return Some(STORY_FIELDS);
    }
    if components.first() == Some(&"adrs") {
        return Some(ADR_FIELDS);
    }
    // epics/<id>/README.md -> epic
    if components.first() == Some(&"epics")
        && components.len() == 3
        && components.last() == Some(&"README.md")
    {
        return Some(EPIC_FIELDS);
    }
    // epics/<id>/voyages/<id>/README.md -> voyage
    if components.first() == Some(&"epics")
        && components.len() == 5
        && components.get(2) == Some(&"voyages")
        && components.last() == Some(&"README.md")
    {
        return Some(VOYAGE_FIELDS);
    }
    None
}

/// Check all entity files for unknown frontmatter field names.
pub fn check_frontmatter_field_drift(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut files_checked = 0;

    for entry in WalkDir::new(board_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
    {
        let path = entry.path();
        let valid_fields = match fields_for_path(path, board_dir) {
            Some(f) => f,
            None => continue,
        };

        files_checked += 1;

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let keys = match extract_frontmatter_keys(&content) {
            Some(k) => k,
            None => continue,
        };

        let valid_set: HashSet<&str> = valid_fields.iter().copied().collect();
        for key in &keys {
            if !valid_set.contains(key.as_str()) {
                problems.push(Problem {
                    severity: Severity::Warning,
                    path: path.to_path_buf(),
                    message: format!("unknown frontmatter field: '{}'", key),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
        }
    }

    Ok((problems, files_checked))
}

/// Regex for epic progress line: **Progress:** X/Y voyages complete, X/Y stories done
pub static EPIC_PROGRESS_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^\*\*Progress:\*\*\s+\d+/\d+\s+voyages\s+complete,\s+\d+/\d+\s+stories\s+done$")
        .unwrap()
});

/// Regex for voyage progress line: **Progress:** X/Y stories complete
pub static VOYAGE_PROGRESS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\*\*Progress:\*\*\s+\d+/\d+\s+stories\s+complete$").unwrap());

/// Expected epic table header
pub const EPIC_TABLE_HEADER: &str = "| Voyage | Status | Stories |";

/// Expected voyage table header
pub const VOYAGE_TABLE_HEADER: &str = "| Story | Title | Type | Status |";

/// Check generated README sections for format drift.
pub fn check_generated_section_drift(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut files_checked = 0;

    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Ok((problems, files_checked));
    }

    for epic_entry in fs::read_dir(&epics_dir)? {
        let epic_entry = epic_entry?;
        let epic_path = epic_entry.path();
        if !epic_path.is_dir() {
            continue;
        }

        // Check epic README
        let epic_readme = epic_path.join("README.md");
        if epic_readme.exists()
            && let Ok(content) = fs::read_to_string(&epic_readme)
            && let Some(section) = extract_generated_section_content(&content)
        {
            files_checked += 1;
            check_epic_generated_section(&epic_readme, section, &mut problems);
        }

        // Check voyage READMEs
        let voyages_dir = epic_path.join("voyages");
        if !voyages_dir.exists() {
            continue;
        }
        for voyage_entry in fs::read_dir(&voyages_dir)? {
            let voyage_entry = voyage_entry?;
            let voyage_path = voyage_entry.path();
            if !voyage_path.is_dir() {
                continue;
            }
            let voyage_readme = voyage_path.join("README.md");
            if voyage_readme.exists()
                && let Ok(content) = fs::read_to_string(&voyage_readme)
                && let Some(section) = extract_generated_section_content(&content)
            {
                files_checked += 1;
                check_voyage_generated_section(&voyage_readme, section, &mut problems);
            }
        }
    }

    Ok((problems, files_checked))
}

/// Extract content between BEGIN GENERATED and END GENERATED markers.
pub fn extract_generated_section_content(content: &str) -> Option<&str> {
    let begin = "<!-- BEGIN GENERATED -->";
    let end = "<!-- END GENERATED -->";
    let start_idx = content.find(begin)?;
    let after_begin = start_idx + begin.len();
    let end_idx = content[after_begin..].find(end)?;
    Some(&content[after_begin..after_begin + end_idx])
}

/// Validate an epic's generated section format.
pub fn check_epic_generated_section(path: &Path, section: &str, problems: &mut Vec<Problem>) {
    let trimmed = section.trim();
    if trimmed.is_empty() {
        // Empty generated section is fine (no content generated yet)
        return;
    }

    // Check progress line
    let mut found_progress = false;
    for line in trimmed.lines() {
        let line = line.trim();
        if line.starts_with("**Progress:**") {
            found_progress = true;
            if !EPIC_PROGRESS_RE.is_match(line) {
                problems.push(Problem {
                    severity: Severity::Warning,
                    path: path.to_path_buf(),
                    message: format!(
                        "malformed epic progress line: '{}' (expected '**Progress:** X/Y voyages complete, X/Y stories done')",
                        line
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
        }
    }
    if !found_progress {
        problems.push(Problem {
            severity: Severity::Warning,
            path: path.to_path_buf(),
            message: "generated section missing progress line".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    // Check table header
    let has_table = trimmed.lines().any(|l| l.trim().starts_with("| "));
    if has_table {
        let header_ok = trimmed
            .lines()
            .any(|l| l.trim().starts_with(EPIC_TABLE_HEADER));
        if !header_ok {
            problems.push(Problem {
                severity: Severity::Warning,
                path: path.to_path_buf(),
                message: format!(
                    "epic table header mismatch (expected '{}')",
                    EPIC_TABLE_HEADER
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }
}

/// Validate a voyage's generated section format.
pub fn check_voyage_generated_section(path: &Path, section: &str, problems: &mut Vec<Problem>) {
    let trimmed = section.trim();
    if trimmed.is_empty() {
        // Empty generated section is fine
        return;
    }

    // Check progress line
    let mut found_progress = false;
    for line in trimmed.lines() {
        let line = line.trim();
        if line.starts_with("**Progress:**") {
            found_progress = true;
            if !VOYAGE_PROGRESS_RE.is_match(line) {
                problems.push(Problem {
                    severity: Severity::Warning,
                    path: path.to_path_buf(),
                    message: format!(
                        "malformed voyage progress line: '{}' (expected '**Progress:** X/Y stories complete')",
                        line
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
        }
    }
    if !found_progress {
        problems.push(Problem {
            severity: Severity::Warning,
            path: path.to_path_buf(),
            message: "generated section missing progress line".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    // Check table header
    let has_table = trimmed.lines().any(|l| l.trim().starts_with("| "));
    if has_table {
        let header_ok = trimmed
            .lines()
            .any(|l| l.trim().starts_with(VOYAGE_TABLE_HEADER));
        if !header_ok {
            problems.push(Problem {
                severity: Severity::Warning,
                path: path.to_path_buf(),
                message: format!(
                    "voyage table header mismatch (expected '{}')",
                    VOYAGE_TABLE_HEADER
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn epic_frontmatter_flags_legacy_completed_field_as_unknown() {
        let temp = TempDir::new().unwrap();
        let epic_dir = temp.path().join("epics").join("test-epic");
        fs::create_dir_all(&epic_dir).unwrap();

        fs::write(
            epic_dir.join("README.md"),
            r#"---
id: test-epic
title: Test Epic
completed: 2026-01-01T00:00:00
---

# Test Epic
"#,
        )
        .unwrap();

        let (problems, files_checked) = check_frontmatter_field_drift(temp.path()).unwrap();
        assert_eq!(files_checked, 1);
        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("unknown frontmatter field: 'completed'")),
            "expected unknown-field warning for legacy completed field, got: {:?}",
            problems
                .iter()
                .map(|p| p.message.clone())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn epic_frontmatter_flags_completed_at_field_as_unknown() {
        let temp = TempDir::new().unwrap();
        let epic_dir = temp.path().join("epics").join("test-epic");
        fs::create_dir_all(&epic_dir).unwrap();

        fs::write(
            epic_dir.join("README.md"),
            r#"---
id: test-epic
title: Test Epic
completed_at: 2026-01-01T00:00:00
---

# Test Epic
"#,
        )
        .unwrap();

        let (problems, files_checked) = check_frontmatter_field_drift(temp.path()).unwrap();
        assert_eq!(files_checked, 1);
        assert!(
            problems.iter().any(|p| p
                .message
                .contains("unknown frontmatter field: 'completed_at'")),
            "expected unknown-field warning for completed_at field, got: {:?}",
            problems
                .iter()
                .map(|p| p.message.clone())
                .collect::<Vec<_>>()
        );
    }
}
