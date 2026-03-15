//! Shared goal-reference utilities for bearing lineage.

use regex::Regex;
use std::collections::BTreeSet;
use std::sync::LazyLock;

use crate::infrastructure::markdown_sections::extract_section;

/// Matches `GOAL-NN` anywhere in text (unanchored).
static GOAL_REF_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"GOAL-\d+").unwrap());

/// Matches an exact `GOAL-NN` token (anchored).
static GOAL_ID_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^GOAL-\d+$").unwrap());

/// Returns true if `s` is a valid `GOAL-NN` token.
pub fn is_valid_goal_id(s: &str) -> bool {
    GOAL_ID_RE.is_match(s)
}

/// Extract unique `GOAL-*` references from free text (e.g. BRIEF.md Success Criteria).
///
/// Returns a sorted, deduplicated list.
pub fn parse_goal_references(text: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    for mat in GOAL_REF_RE.find_iter(text) {
        seen.insert(mat.as_str().to_string());
    }
    seen.into_iter().collect()
}

/// Extract valid goal IDs from a PRD's `## Goals & Objectives` table.
///
/// Parses the markdown table looking for rows where the first column matches `GOAL-NN`.
pub fn extract_prd_goal_ids(prd_content: &str) -> Vec<String> {
    let goals_section = extract_section(prd_content, "## Goals & Objectives").unwrap_or_default();
    let mut ids = Vec::new();
    let mut has_header = false;

    for line in goals_section.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }

        let cells: Vec<_> = trimmed
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim())
            .collect();
        if cells.is_empty()
            || cells
                .iter()
                .all(|c| c.chars().all(|ch| ch == '-' || ch == ' '))
        {
            continue;
        }

        if cells.iter().any(|c| c.eq_ignore_ascii_case("ID")) {
            has_header = true;
            continue;
        }

        if has_header && !cells.is_empty() && GOAL_ID_RE.is_match(cells[0]) {
            ids.push(cells[0].to_string());
        }
    }
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_goal_references_extracts_unique_sorted() {
        let text = "Linked to GOAL-02 and GOAL-01, also GOAL-02 again";
        assert_eq!(parse_goal_references(text), vec!["GOAL-01", "GOAL-02"]);
    }

    #[test]
    fn parse_goal_references_empty_when_none() {
        assert!(parse_goal_references("no goals here").is_empty());
    }

    #[test]
    fn is_valid_goal_id_accepts_correct_format() {
        assert!(is_valid_goal_id("GOAL-01"));
        assert!(is_valid_goal_id("GOAL-123"));
    }

    #[test]
    fn is_valid_goal_id_rejects_bad_format() {
        assert!(!is_valid_goal_id("GOAL-"));
        assert!(!is_valid_goal_id("goal-01"));
        assert!(!is_valid_goal_id("GOAL-01 extra"));
    }

    #[test]
    fn extract_prd_goal_ids_parses_table() {
        let prd = "\
## Goals & Objectives

| ID | Description |
|----|-------------|
| GOAL-01 | First goal |
| GOAL-02 | Second goal |

## Next Section
";
        assert_eq!(extract_prd_goal_ids(prd), vec!["GOAL-01", "GOAL-02"]);
    }

    #[test]
    fn extract_prd_goal_ids_empty_when_no_section() {
        assert!(extract_prd_goal_ids("# Just a title\n\nSome content.").is_empty());
    }
}
