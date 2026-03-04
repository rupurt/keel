//! Shared validation logic for stories

pub mod structural;
pub mod types;

use regex::Regex;
use std::sync::LazyLock;
pub use types::{CheckId, Fix, GapCategory, Problem, Severity};

static AC_REQ_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[(SRS-\d+)/AC-\d+\]").unwrap());

/// Result of acceptance criteria validation
#[derive(Debug, Default)]
pub struct AcceptanceCriteriaResult {
    /// Unchecked criteria (text after `- [ ]`)
    pub unchecked: Vec<String>,
    /// Checked criteria (text after `- [x]` or `- [X]`)
    pub checked: Vec<String>,
    /// Whether an Acceptance Criteria section was found
    pub has_section: bool,
}

impl AcceptanceCriteriaResult {
    /// Returns true if all criteria are checked (or no section exists)
    pub fn is_complete(&self) -> bool {
        self.unchecked.is_empty()
    }

    /// Returns true if any criteria (checked or unchecked) require manual verification
    pub fn requires_manual(&self) -> bool {
        self.unchecked.iter().any(|c| c.contains("verify: manual"))
            || self.checked.iter().any(|c| c.contains("verify: manual"))
    }
}

fn criterion_text_without_annotation(criterion: &str) -> &str {
    criterion.split("<!--").next().unwrap_or(criterion).trim()
}

/// Return acceptance criteria entries that are missing `[SRS-XX/AC-YY]` references.
pub fn missing_srs_references(criteria: &AcceptanceCriteriaResult) -> Vec<String> {
    let mut missing = Vec::new();

    for criterion in criteria.checked.iter().chain(criteria.unchecked.iter()) {
        let text = criterion_text_without_annotation(criterion);
        if text.is_empty() {
            continue;
        }
        if !AC_REQ_RE.is_match(text) {
            missing.push(text.to_string());
        }
    }

    missing
}

/// Parse acceptance criteria from story content
///
/// Looks for `## Acceptance Criteria` section and extracts checkbox items.
/// Returns unchecked items (`- [ ]`) and checked items (`- [x]`/`- [X]`).
pub fn parse_acceptance_criteria(content: &str) -> AcceptanceCriteriaResult {
    let mut result = AcceptanceCriteriaResult::default();
    let mut in_section = false;

    for line in content.lines() {
        // Check for section start
        if line.starts_with("## Acceptance Criteria") {
            in_section = true;
            result.has_section = true;
            continue;
        }

        // Check for next section (end of acceptance criteria)
        if in_section && line.starts_with("## ") {
            break;
        }

        if in_section {
            let trimmed = line.trim();

            // Unchecked: `- [ ]`
            if trimmed.starts_with("- [ ]") {
                let text = trimmed.strip_prefix("- [ ]").unwrap_or("").trim();
                if !text.is_empty() {
                    result.unchecked.push(text.to_string());
                }
            }
            // Checked: `- [x]` or `- [X]`
            else if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
                let text = trimmed
                    .strip_prefix("- [x]")
                    .or_else(|| trimmed.strip_prefix("- [X]"))
                    .unwrap_or("")
                    .trim();
                if !text.is_empty() {
                    result.checked.push(text.to_string());
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_finds_unchecked_criteria() {
        let content = r#"# Story

## Acceptance Criteria

- [ ] First criterion
- [ ] Second criterion
"#;
        let result = parse_acceptance_criteria(content);

        assert!(result.has_section);
        assert_eq!(result.unchecked.len(), 2);
        assert_eq!(result.unchecked[0], "First criterion");
        assert_eq!(result.unchecked[1], "Second criterion");
        assert!(result.checked.is_empty());
        assert!(!result.is_complete());
    }

    #[test]
    fn parse_finds_checked_criteria() {
        let content = r#"# Story

## Acceptance Criteria

- [x] Done criterion
- [X] Also done (uppercase)
"#;
        let result = parse_acceptance_criteria(content);

        assert!(result.has_section);
        assert!(result.unchecked.is_empty());
        assert_eq!(result.checked.len(), 2);
        assert_eq!(result.checked[0], "Done criterion");
        assert_eq!(result.checked[1], "Also done (uppercase)");
        assert!(result.is_complete());
    }

    #[test]
    fn parse_handles_mixed_criteria() {
        let content = r#"# Story

## Acceptance Criteria

- [x] Completed item
- [ ] Incomplete item
- [X] Another completed
"#;
        let result = parse_acceptance_criteria(content);

        assert_eq!(result.unchecked.len(), 1);
        assert_eq!(result.unchecked[0], "Incomplete item");
        assert_eq!(result.checked.len(), 2);
        assert!(!result.is_complete());
    }

    #[test]
    fn parse_stops_at_next_section() {
        let content = r#"# Story

## Acceptance Criteria

- [ ] Real criterion

## Implementation Notes

- [ ] This is not a criterion
"#;
        let result = parse_acceptance_criteria(content);

        assert_eq!(result.unchecked.len(), 1);
        assert_eq!(result.unchecked[0], "Real criterion");
    }

    #[test]
    fn parse_handles_no_section() {
        let content = r#"# Story

## Summary

Just some content without acceptance criteria.
"#;
        let result = parse_acceptance_criteria(content);

        assert!(!result.has_section);
        assert!(result.unchecked.is_empty());
        assert!(result.checked.is_empty());
        assert!(result.is_complete()); // No section = complete
    }

    #[test]
    fn parse_handles_empty_section() {
        let content = r#"# Story

## Acceptance Criteria

## Next Section
"#;
        let result = parse_acceptance_criteria(content);

        assert!(result.has_section);
        assert!(result.unchecked.is_empty());
        assert!(result.is_complete());
    }

    #[test]
    fn parse_ignores_empty_checkbox_text() {
        let content = r#"# Story

## Acceptance Criteria

- [ ]
- [x] Valid item
"#;
        let result = parse_acceptance_criteria(content);

        assert!(result.unchecked.is_empty()); // Empty text ignored
        assert_eq!(result.checked.len(), 1);
    }

    #[test]
    fn requires_manual_detects_marker() {
        let content = "## Acceptance Criteria\n\n- [ ] Task <!-- verify: manual -->";
        let result = parse_acceptance_criteria(content);
        assert!(result.requires_manual());

        let content2 = "## Acceptance Criteria\n\n- [x] Task <!-- verify: manual -->";
        let result2 = parse_acceptance_criteria(content2);
        assert!(result2.requires_manual());

        let content3 = "## Acceptance Criteria\n\n- [ ] Task <!-- verify: echo ok -->";
        let result3 = parse_acceptance_criteria(content3);
        assert!(!result3.requires_manual());
    }

    #[test]
    fn parse_handles_no_checkboxes_in_section() {
        let content =
            "## Acceptance Criteria\n\nJust some text without bullets.\nNo checkboxes here.";
        let result = parse_acceptance_criteria(content);
        assert!(result.has_section);
        assert!(result.unchecked.is_empty());
        assert!(result.checked.is_empty());
    }

    #[test]
    fn parse_handles_indented_checkboxes() {
        let content = "## Acceptance Criteria\n\n  - [ ] Indented item\n\t- [x] Tab indented";
        let result = parse_acceptance_criteria(content);
        assert_eq!(result.unchecked.len(), 1);
        assert_eq!(result.unchecked[0], "Indented item");
        assert_eq!(result.checked.len(), 1);
        assert_eq!(result.checked[0], "Tab indented");
    }

    #[test]
    fn parse_handles_malformed_checkboxes() {
        let content =
            "## Acceptance Criteria\n\n- [] No space\n- [  ] Extra space\n- [y] Invalid char";
        let result = parse_acceptance_criteria(content);
        // None of these match the current regex/starts_with logic strictly
        assert!(result.unchecked.is_empty());
        assert!(result.checked.is_empty());
    }

    #[test]
    fn missing_srs_references_ignores_criteria_with_refs() {
        let content = "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Good criterion";
        let criteria = parse_acceptance_criteria(content);
        let missing = missing_srs_references(&criteria);
        assert!(missing.is_empty());
    }

    #[test]
    fn missing_srs_references_flags_criteria_without_refs() {
        let content =
            "## Acceptance Criteria\n\n- [ ] Criterion without ref <!-- verify: manual -->";
        let criteria = parse_acceptance_criteria(content);
        let missing = missing_srs_references(&criteria);
        assert_eq!(missing, vec!["Criterion without ref".to_string()]);
    }
}
