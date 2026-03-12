//! Bearing-specific validation logic

use std::path::Path;

use crate::domain::model::{Bearing, Board};
use crate::infrastructure::markdown_sections::extract_section;
use crate::infrastructure::validation::structural::first_unfilled_placeholder_pattern;
use crate::infrastructure::validation::types::{CheckId, Problem, Severity};
use std::fs;

/// Required sections for a bearing's BRIEF.md
pub const BEARING_REQUIRED_SECTIONS: &[&str] = &[
    "## Hypothesis",
    "## Problem Space",
    "## Success Criteria",
    "## Open Questions",
];

const HYPOTHESIS_SCAFFOLD_MARKERS: &[&str] = &["What we believe and why it might matter."];
const PROBLEM_SPACE_SCAFFOLD_MARKERS: &[&str] =
    &["What problem or opportunity are we investigating?"];
const SUCCESS_CRITERIA_SCAFFOLD_MARKERS: &[&str] = &[
    "How will we know if this research was valuable?",
    "Criterion 1",
    "Criterion 2",
];
const OPEN_QUESTION_SCAFFOLD_MARKERS: &[&str] = &["Question we need to answer"];

/// Check bearing content sections
/// Validates that BRIEF.md contains required markdown sections
pub fn check_bearing_content_sections(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        problems.extend(check_bearing_content_sections_for_bearing(bearing));

        let evidence_path = bearing.path.parent().unwrap().join("EVIDENCE.md");
        if let Ok(content) = fs::read_to_string(&evidence_path) {
            for message in
                crate::infrastructure::bearing_evidence::validate_evidence_document(&content)
            {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: evidence_path.clone(),
                    message: format!(
                        "bearing '{}' evidence contract error: {}",
                        bearing.id(),
                        message
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
        }
    }

    problems
}

pub fn check_bearing_content_sections_for_bearing(bearing: &Bearing) -> Vec<Problem> {
    let brief_path = bearing.path.parent().unwrap().join("BRIEF.md");
    check_bearing_content_sections_for_path(bearing.id(), &brief_path)
}

pub fn check_bearing_content_sections_for_path(
    bearing_id: &str,
    brief_path: &Path,
) -> Vec<Problem> {
    let content = match fs::read_to_string(brief_path) {
        Ok(content) => content,
        Err(_) => return Vec::new(), // File read errors handled elsewhere
    };

    let mut problems = Vec::new();
    for requirement in brief_requirements() {
        let Some(section) = extract_section(&content, requirement.heading) else {
            problems.push(Problem {
                severity: Severity::Error,
                path: brief_path.to_path_buf(),
                message: format!(
                    "bearing '{}' is missing required BRIEF.md section: {}",
                    bearing_id, requirement.label
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
            continue;
        };

        if brief_section_uses_scaffold(&section, requirement.scaffold_markers) {
            problems.push(Problem {
                severity: Severity::Error,
                path: brief_path.to_path_buf(),
                message: format!(
                    "bearing '{}' must replace the BRIEF.md {} scaffold with authored content",
                    bearing_id, requirement.label
                ),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
}

struct BriefRequirement {
    heading: &'static str,
    label: &'static str,
    scaffold_markers: &'static [&'static str],
}

fn brief_requirements() -> [BriefRequirement; 4] {
    [
        BriefRequirement {
            heading: "## Hypothesis",
            label: "Hypothesis",
            scaffold_markers: HYPOTHESIS_SCAFFOLD_MARKERS,
        },
        BriefRequirement {
            heading: "## Problem Space",
            label: "Problem Space",
            scaffold_markers: PROBLEM_SPACE_SCAFFOLD_MARKERS,
        },
        BriefRequirement {
            heading: "## Success Criteria",
            label: "Success Criteria",
            scaffold_markers: SUCCESS_CRITERIA_SCAFFOLD_MARKERS,
        },
        BriefRequirement {
            heading: "## Open Questions",
            label: "Open Questions",
            scaffold_markers: OPEN_QUESTION_SCAFFOLD_MARKERS,
        },
    ]
}

fn brief_section_uses_scaffold(section: &str, scaffold_markers: &[&str]) -> bool {
    first_unfilled_placeholder_pattern(section).is_some()
        || scaffold_markers
            .iter()
            .any(|marker| section.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBearing, TestBoardBuilder};

    #[test]
    fn doctor_flags_scaffold_hypothesis_and_problem_space() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();
        fs::write(
            temp.path().join("bearings/BRG-01/BRIEF.md"),
            r#"# Test Bearing — Brief

## Hypothesis

What we believe and why it might matter.

## Problem Space

What problem or opportunity are we investigating?

## Success Criteria

How will we know if this research was valuable?

- [ ] Criterion 1
- [ ] Criterion 2

## Open Questions

- Question we need to answer
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);

        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("BRIEF.md Hypothesis scaffold"))
        );
        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("BRIEF.md Problem Space scaffold"))
        );
    }

    #[test]
    fn doctor_accepts_authored_bearing_brief_sections() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();

        let brief_path = temp.path().join("bearings/BRG-01/BRIEF.md");
        fs::write(
            &brief_path,
            r#"# Test Bearing — Brief

## Hypothesis

Operators will make faster planning decisions when the research framing is explicit.

## Problem Space

The current bearing loop makes it easy to advance without a concrete hypothesis or problem statement.

## Success Criteria

- [ ] Research framing is authored before the bearing advances.

## Open Questions

- Should lay also surface the same recovery guidance?
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_content_sections(&board);
        assert!(problems.is_empty());
    }
}
