//! Bearing-specific validation logic

use crate::domain::model::{Board, Entity};
use crate::infrastructure::validation::types::{Problem, CheckId, Severity};
use std::fs;

/// Required sections for a bearing's BRIEF.md
pub const BEARING_REQUIRED_SECTIONS: &[&str] = &[
    "## Context",
    "## Objectives",
    "## Scope",
    "## Research Questions",
    "## Open Questions",
];

/// Check bearing content sections
/// Validates that BRIEF.md contains required markdown sections
pub fn check_bearing_content_sections(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        // Read the BRIEF.md content
        let brief_path = bearing.path.parent().unwrap().join("BRIEF.md");
        let content = match fs::read_to_string(&brief_path) {
            Ok(c) => c,
            Err(_) => continue, // File read errors handled elsewhere
        };

        for section in BEARING_REQUIRED_SECTIONS {
            if !content.contains(section) {
                let section_name = section.trim_start_matches("## ");
                problems.push(Problem {
                    severity: Severity::Warning,
                    path: brief_path.clone(),
                    message: format!(
                        "bearing '{}' is missing required section: {}",
                        bearing.id(),
                        section_name
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
        }

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
