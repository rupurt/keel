use std::fs;
use std::path::Path;

use anyhow::Result;

use super::super::types::*;
use crate::domain::model::Board;
use crate::infrastructure::parser::parse_frontmatter;

/// Scan bearing files for structural problems
/// Returns (problems, file_count)
pub fn scan_bearing_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut bearing_count = 0;

    let bearings_dir = board_dir.join("bearings");
    if !bearings_dir.exists() {
        return Ok((problems, bearing_count));
    }

    // Find all bearing directories
    for entry in fs::read_dir(&bearings_dir)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            continue;
        }

        bearing_count += 1;
        let readme_path = entry.path().join("README.md");
        let brief_path = entry.path().join("BRIEF.md");

        if readme_path.exists() {
            // Check frontmatter and required fields
            if let Some(problem) = check_bearing_file(&readme_path) {
                problems.push(problem);
            }
        } else {
            problems.push(Problem {
                severity: Severity::Error,
                path: readme_path,
                message: "missing README.md (required bearing frontmatter)".to_string(),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }

        if !brief_path.exists() {
            problems.push(Problem {
                severity: Severity::Warning,
                path: brief_path,
                message: "missing BRIEF.md (bearing research content)".to_string(),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    Ok((problems, bearing_count))
}

/// Check a single bearing file for problems
pub fn check_bearing_file(path: &Path) -> Option<Problem> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return Some(Problem {
                severity: Severity::Error,
                path: path.to_path_buf(),
                message: format!("cannot read file: {}", e),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    };

    // Check for missing frontmatter
    if !content.starts_with("---") {
        return Some(Problem {
            severity: Severity::Error,
            path: path.to_path_buf(),
            message: "missing frontmatter (file doesn't start with ---)".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    // Try to parse frontmatter
    let result: Result<(crate::domain::model::BearingFrontmatter, &str), _> =
        parse_frontmatter(&content);

    match result {
        Err(e) => Some(Problem {
            severity: Severity::Error,
            path: path.to_path_buf(),
            message: format!("invalid YAML: {}", e),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        }),
        Ok((fm, _)) => {
            // Check required fields
            if fm.id.is_empty() {
                return Some(Problem {
                    severity: Severity::Error,
                    path: path.to_path_buf(),
                    message: "missing required field: id".to_string(),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
            if fm.title.is_empty() {
                return Some(Problem {
                    severity: Severity::Error,
                    path: path.to_path_buf(),
                    message: "missing required field: title".to_string(),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
            None
        }
    }
}

/// Check bearing state coherence
/// Validates that bearing state is consistent with documents present
pub fn check_bearing_state_coherence(board: &Board) -> Vec<Problem> {
    use crate::domain::model::BearingStatus;

    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        let status = bearing.status();
        let has_survey = bearing.has_survey;
        let has_assessment = bearing.has_assessment;

        // Check state transitions are valid based on document presence
        match status {
            BearingStatus::Exploring => {
                // Exploring is the initial state, no documents required
            }
            BearingStatus::Evaluating => {
                // Evaluating requires SURVEY.md
                if !has_survey {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is in evaluating state but missing SURVEY.md",
                            bearing.id()
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
            BearingStatus::Ready => {
                // Ready requires both SURVEY.md and ASSESSMENT.md
                if !has_survey {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is in ready state but missing SURVEY.md",
                            bearing.id()
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
                if !has_assessment {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is in ready state but missing ASSESSMENT.md",
                            bearing.id()
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
            BearingStatus::Laid => {
                // Laid bearings should have laid_at date
                if bearing.frontmatter.laid_at.is_none() {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is laid but missing laid_at date",
                            bearing.id()
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
            BearingStatus::Declined => {
                // Declined bearings should have a reason
                if bearing.frontmatter.decline_reason.is_none() {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is declined but missing decline_reason",
                            bearing.id()
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
            BearingStatus::Parked => {
                // Parked bearings have no additional requirements
            }
        }
    }

    problems
}

/// Check bearing ID-directory consistency
/// Validates that bearing directory names match the `id` field in their README.md frontmatter
pub fn check_bearing_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        // Extract directory name from path: bearings/{dir-name}/README.md
        let dir_name = bearing
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        let frontmatter_id = bearing.id();

        if let Some(dir) = dir_name
            && dir != frontmatter_id
        {
            let old_path = bearing.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(Problem {
                severity: Severity::Error,
                path: bearing.path.clone(),
                message: format!(
                    "directory name '{}' differs from frontmatter id '{}'",
                    dir, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: None,
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}

/// Check bearing ID format
/// Validates that bearing IDs use the canonical generated 9-character base62 format.
pub fn check_bearing_id_format(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        let id = bearing.id();
        if !is_canonical_generated_id(id) {
            problems.push(Problem {
                severity: Severity::Error,
                path: bearing.path.clone(),
                message: format!(
                    "bearing id '{}' must use the canonical generated format (9-character base62)",
                    id
                ),
                fix: None,
                scope: None,
                category: Some(GapCategory::Convention),
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}

fn is_canonical_generated_id(id: &str) -> bool {
    id.len() == 9 && id.chars().all(|ch| ch.is_ascii_alphanumeric())
}

/// Check bearing title case
/// Validates that bearing titles follow Title Case convention
pub fn check_bearing_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        let title = &bearing.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(Problem {
                severity: Severity::Warning,
                path: bearing.path.clone(),
                message: format!("title '{}' should use Title Case", title),
                fix: Some(Fix::UpdateTitle {
                    path: bearing.path.clone(),
                    new_title,
                }),
                scope: None,
                category: Some(GapCategory::Convention),
                check_id: CheckId::TitleCaseViolation,
            });
        }
    }

    problems
}

/// Check for duplicate bearing IDs
/// Scans bearing files for duplicate frontmatter IDs.
pub fn check_bearing_duplicates(board_dir: &Path) -> Vec<Problem> {
    crate::infrastructure::duplicate_ids::duplicate_id_problems(
        board_dir,
        crate::infrastructure::duplicate_ids::DuplicateEntity::Bearing,
    )
}

/// Check bearing date fields
/// Validates that created_at is present and follows naming/type conventions
pub fn check_bearing_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        if bearing.frontmatter.created_at.is_none() {
            problems.push(Problem {
                severity: Severity::Warning,
                path: bearing.path.clone(),
                message: format!("bearing '{}' is missing created_at date", bearing.id()),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::BearingDateConsistency,
            });
        }

        // Add structural consistency checks for date naming and datetime type
        problems.extend(
            crate::infrastructure::validation::structural::check_date_consistency(
                &bearing.path,
                CheckId::BearingDateConsistency,
            ),
        );
    }

    problems
}

/// Required sections in bearing BRIEF.md
pub const BEARING_REQUIRED_SECTIONS: &[&str] = &[
    "## Hypothesis",
    "## Problem Space",
    "## Success Criteria",
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
    }

    problems
}

/// Check bearing-epic coherence
/// Validates that laid bearings have corresponding epics
pub fn check_bearing_epic_coherence(board: &Board) -> Vec<Problem> {
    use crate::domain::model::BearingStatus;

    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        if bearing.status() == BearingStatus::Laid {
            // Check if there's an epic with a matching ID
            // Convention: bearing ID should match or be a prefix of epic ID
            let bearing_id = bearing.id();
            let has_matching_epic = board
                .epics
                .values()
                .any(|e| e.id() == bearing_id || e.id().starts_with(&format!("{}-", bearing_id)));

            if !has_matching_epic {
                problems.push(Problem {
                    severity: Severity::Warning,
                    path: bearing.path.clone(),
                    message: format!(
                        "bearing '{}' is laid but no matching epic found",
                        bearing_id
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

/// Check bearing assessment recommendation
/// Validates that evaluating bearings have a marked recommendation in ASSESSMENT.md
pub fn check_bearing_assessment_recommendation(board: &Board, board_dir: &Path) -> Vec<Problem> {
    use crate::domain::model::BearingStatus;

    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        // Only check bearings that are evaluating and have an assessment
        if bearing.status() == BearingStatus::Evaluating && bearing.has_assessment {
            let assessment_path = board_dir
                .join("bearings")
                .join(bearing.id())
                .join("ASSESSMENT.md");

            if let Ok(content) = fs::read_to_string(&assessment_path) {
                // Check if any recommendation checkbox is marked
                let has_marked_recommendation = content.contains("[x] Proceed")
                    || content.contains("[x] Park")
                    || content.contains("[x] Decline")
                    || content.contains("[X] Proceed")
                    || content.contains("[X] Park")
                    || content.contains("[X] Decline");

                if !has_marked_recommendation {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: "assessment has no recommendation marked".to_string(),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id: CheckId::Unknown,
                    });
                }
            }
        }
    }

    problems
}

/// Generate insight summary for bearing readiness
pub fn generate_bearing_insight(board: &Board, board_dir: &Path) -> Option<String> {
    use crate::domain::model::BearingStatus;

    let mut needs_voyages: Vec<&str> = Vec::new();
    let mut ready_to_lay: Vec<&str> = Vec::new();
    let mut needs_recommendation: Vec<&str> = Vec::new();
    let mut needs_survey: Vec<&str> = Vec::new();

    for bearing in board.bearings.values() {
        match bearing.status() {
            BearingStatus::Laid => {
                // Check if epic has voyages
                let has_voyages = board
                    .voyages
                    .values()
                    .any(|v| v.frontmatter.epic.as_deref() == Some(bearing.id()));
                if !has_voyages {
                    needs_voyages.push(bearing.id());
                }
            }
            BearingStatus::Evaluating => {
                if bearing.has_assessment {
                    // Check if recommendation is marked
                    let assessment_path = board_dir
                        .join("bearings")
                        .join(bearing.id())
                        .join("ASSESSMENT.md");

                    if let Ok(content) = fs::read_to_string(&assessment_path) {
                        let has_marked = content.contains("[x] Proceed")
                            || content.contains("[x] Park")
                            || content.contains("[x] Decline")
                            || content.contains("[X] Proceed")
                            || content.contains("[X] Park")
                            || content.contains("[X] Decline");

                        if has_marked {
                            ready_to_lay.push(bearing.id());
                        } else {
                            needs_recommendation.push(bearing.id());
                        }
                    }
                }
            }
            BearingStatus::Exploring => {
                needs_survey.push(bearing.id());
            }
            _ => {}
        }
    }

    // Generate insight message (priority order: needs voyages > ready to lay > needs recommendation > needs survey)
    if !needs_voyages.is_empty() {
        Some(format!("  → {} needs voyages", needs_voyages.join(", ")))
    } else if !ready_to_lay.is_empty() {
        Some(format!("  → {} ready to lay", ready_to_lay.join(", ")))
    } else if !needs_recommendation.is_empty() {
        Some(format!(
            "  → {} awaiting recommendation",
            needs_recommendation.join(", ")
        ))
    } else if !needs_survey.is_empty() && needs_survey.len() <= 3 {
        Some(format!("  → {} need survey", needs_survey.join(", ")))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBearing, TestBoardBuilder};

    #[test]
    fn test_scan_bearing_files_empty() {
        let temp = TestBoardBuilder::new().build();
        let (problems, count) = scan_bearing_files(temp.path()).unwrap();
        assert_eq!(count, 0);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_scan_bearing_files_valid() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();
        let (problems, count) = scan_bearing_files(temp.path()).unwrap();
        assert_eq!(count, 1);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_check_bearing_file_valid() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();
        let path = temp.path().join("bearings/BRG-01/README.md");
        let problem = check_bearing_file(&path);
        assert!(problem.is_none());
    }

    #[test]
    fn test_check_bearing_state_coherence_missing_survey() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("BRG-01")
                    .status("evaluating")
                    .has_survey(false),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_state_coherence(&board);
        assert!(!problems.is_empty());
        assert!(problems[0].message.contains("missing SURVEY.md"));
    }

    #[test]
    fn check_bearing_id_format_flags_legacy_slug_ids() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("semantic-search-research"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_id_format(&board);
        assert_eq!(problems.len(), 1);
        assert!(
            problems[0]
                .message
                .contains("must use the canonical generated format")
        );
    }

    #[test]
    fn check_bearing_id_format_accepts_generated_ids() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("1w5H2Bq9L"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_id_format(&board);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_is_title_case() {
        use crate::infrastructure::utils::is_title_case;
        assert!(is_title_case("My Bearing Title"));
        assert!(is_title_case("A Bearing with a Small Word"));
        assert!(!is_title_case("my bearing title"));
        assert!(!is_title_case("My bearing Title"));
        assert!(!is_title_case("kebab-case-title"));
    }
}
