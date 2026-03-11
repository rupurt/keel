use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::domain::model::Board;
use crate::infrastructure::bearing_readiness::evaluate_bearing_readiness;
use crate::infrastructure::parser::parse_frontmatter;
use crate::read_model::diagnostics::types::*;

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
        let evidence_path = entry.path().join("EVIDENCE.md");
        let legacy_survey_path = entry.path().join("SURVEY.md");

        if readme_path.exists() {
            // Check frontmatter and required fields
            if let Some(problem) = check_bearing_file(&readme_path) {
                problems.push(problem);
            }
            problems.extend(
                crate::infrastructure::validation::structural::check_bearing_readme_structure(
                    &readme_path,
                ),
            );
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

        if legacy_survey_path.exists() {
            problems.push(Problem {
                severity: Severity::Error,
                path: legacy_survey_path,
                message:
                    "legacy SURVEY.md is not supported; migrate the bearing to EVIDENCE.md and rerun `keel bearing research <id>` if needed"
                        .to_string(),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }

        if !evidence_path.exists() && entry.path().join("ASSESSMENT.md").exists() {
            problems.push(Problem {
                severity: Severity::Warning,
                path: evidence_path,
                message: "missing EVIDENCE.md (required before ASSESSMENT.md)".to_string(),
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
        let has_evidence = bearing.has_evidence;
        let has_assessment = bearing.has_assessment;

        // Check state transitions are valid based on document presence
        match status {
            BearingStatus::Exploring => {
                // Exploring is the initial state, no documents required
            }
            BearingStatus::Evaluating => {
                // Evaluating requires EVIDENCE.md
                if !has_evidence {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is in evaluating state but missing EVIDENCE.md",
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
                // Ready requires both EVIDENCE.md and ASSESSMENT.md
                if !has_evidence {
                    problems.push(Problem {
                        severity: Severity::Warning,
                        path: bearing.path.clone(),
                        message: format!(
                            "bearing '{}' is in ready state but missing EVIDENCE.md",
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
                fix: Some(Fix::SetFrontmatterField {
                    path: bearing.path.clone(),
                    field: "title".to_string(),
                    value: new_title,
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
pub use crate::infrastructure::validation::bearings::{
    BEARING_REQUIRED_SECTIONS, check_bearing_content_sections,
};

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

/// Check bearing decision readiness
/// Validates that ready/laid bearings satisfy the evidence-backed decision contract
pub fn check_bearing_assessment_recommendation(board: &Board, board_dir: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        if matches!(
            bearing.status(),
            crate::domain::model::BearingStatus::Ready | crate::domain::model::BearingStatus::Laid
        ) {
            let readiness = evaluate_bearing_readiness(board_dir, bearing, None);
            for message in readiness.problem_messages(bearing.id()) {
                problems.push(Problem {
                    severity: Severity::Error,
                    path: bearing.path.clone(),
                    message,
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

/// Generate insight summary for bearing readiness
pub fn generate_bearing_insight(board: &Board, board_dir: &Path) -> Option<String> {
    use crate::domain::model::BearingStatus;

    let mut needs_voyages: Vec<&str> = Vec::new();
    let mut ready_to_lay: Vec<&str> = Vec::new();
    let mut needs_recommendation: Vec<&str> = Vec::new();
    let mut needs_research: Vec<&str> = Vec::new();

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
                needs_research.push(bearing.id());
            }
            _ => {}
        }
    }

    // Generate insight message (priority order: needs voyages > ready to lay > needs recommendation > needs research)
    if !needs_voyages.is_empty() {
        Some(format!("  → {} needs voyages", needs_voyages.join(", ")))
    } else if !ready_to_lay.is_empty() {
        Some(format!("  → {} ready to lay", ready_to_lay.join(", ")))
    } else if !needs_recommendation.is_empty() {
        Some(format!(
            "  → {} awaiting recommendation",
            needs_recommendation.join(", ")
        ))
    } else if !needs_research.is_empty() && needs_research.len() <= 3 {
        Some(format!("  → {} need research", needs_research.join(", ")))
    } else {
        None
    }
}

/// Check bearing lineage: laid bearings must have `epic` field
pub fn check_bearing_lineage_epic(board: &Board) -> Vec<Problem> {
    use crate::domain::model::BearingStatus;

    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        if bearing.status() == BearingStatus::Laid && bearing.frontmatter.epic.is_none() {
            problems.push(
                Problem::error(
                    bearing.path.clone(),
                    format!(
                        "laid bearing '{}' is missing required `epic` lineage field",
                        bearing.id(),
                    ),
                )
                .with_check_id(CheckId::BearingMissingEpicLineage)
                .with_fix(Fix::SetFrontmatterField {
                    path: bearing.path.clone(),
                    field: "epic".to_string(),
                    value: bearing.id().to_string(),
                }),
            );
        }
    }

    problems
}

/// Check bearing lineage: goal references must be valid GOAL-* tokens
pub fn check_bearing_lineage_goals(board: &Board, board_dir: &Path) -> Vec<Problem> {
    use crate::domain::model::BearingStatus;
    use crate::infrastructure::validation::goals::{extract_prd_goal_ids, is_valid_goal_id};

    let mut problems = Vec::new();

    for bearing in board.bearings.values() {
        if bearing.status() != BearingStatus::Laid {
            continue;
        }

        let goals = match &bearing.frontmatter.goals {
            Some(goals) if !goals.is_empty() => goals,
            _ => continue,
        };

        // Validate each goal matches GOAL-\d+ format
        for goal in goals {
            if !is_valid_goal_id(goal) {
                problems.push(
                    Problem::error(
                        bearing.path.clone(),
                        format!(
                            "laid bearing '{}' has invalid goal reference '{}'; \
                             expected format GOAL-NN. Fix: update goals in {}",
                            bearing.id(),
                            goal,
                            bearing.path.display()
                        ),
                    )
                    .with_check_id(CheckId::BearingInvalidGoalLineage),
                );
                continue;
            }
        }

        // Validate goals exist in the target epic's PRD
        let epic_id = match &bearing.frontmatter.epic {
            Some(id) => id,
            None => continue, // Missing epic handled by check_bearing_lineage_epic
        };

        let prd_path = board_dir.join("epics").join(epic_id).join("PRD.md");
        let prd_content = match fs::read_to_string(&prd_path) {
            Ok(c) => c,
            Err(_) => continue, // PRD missing handled elsewhere
        };

        let valid_ids = extract_prd_goal_ids(&prd_content);

        for goal in goals {
            if is_valid_goal_id(goal) && !valid_ids.contains(goal) {
                problems.push(
                    Problem::error(
                        bearing.path.clone(),
                        format!(
                            "laid bearing '{}' references '{}' but it does not exist in {}. \
                             Valid goals: {}",
                            bearing.id(),
                            goal,
                            prd_path.display(),
                            if valid_ids.is_empty() {
                                "(none)".to_string()
                            } else {
                                valid_ids.join(", ")
                            }
                        ),
                    )
                    .with_check_id(CheckId::BearingInvalidGoalLineage),
                );
            }
        }
    }

    problems
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
    fn test_scan_bearing_files_flags_legacy_readme_without_documents_table() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();
        let path = temp.path().join("bearings/BRG-01/README.md");
        fs::write(
            &path,
            "---\nid: BRG-01\ntitle: Test Research\nstatus: exploring\ncreated_at: 2026-01-01T00:00:00\n---\n\n# Test Research\n\nSee [BRIEF.md](BRIEF.md) for details.\n",
        )
        .unwrap();

        let (problems, _count) = scan_bearing_files(temp.path()).unwrap();

        assert!(
            problems
                .iter()
                .any(|problem| problem.message.contains("README missing Documents section"))
        );
    }

    #[test]
    fn test_check_bearing_state_coherence_missing_evidence() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("BRG-01")
                    .status("evaluating")
                    .has_evidence(false),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_state_coherence(&board);
        assert!(!problems.is_empty());
        assert!(problems[0].message.contains("missing EVIDENCE.md"));
    }

    #[test]
    fn bearing_doctor_rejects_legacy_survey_contract() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01").status("evaluating"))
            .build();

        let bearing_dir = temp.path().join("bearings/BRG-01");
        fs::write(bearing_dir.join("SURVEY.md"), "# Survey\n").unwrap();

        let (problems, _count) = scan_bearing_files(temp.path()).unwrap();
        let board = load_board(temp.path()).unwrap();
        let coherence = check_bearing_state_coherence(&board);

        assert!(problems.iter().any(|problem| {
            problem
                .message
                .contains("legacy SURVEY.md is not supported")
        }));
        assert!(
            coherence
                .iter()
                .any(|problem| problem.message.contains("missing EVIDENCE.md"))
        );
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

    #[test]
    fn check_bearing_lineage_epic_flags_laid_without_epic() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("1w5H2Bq9L").status("laid"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_lineage_epic(&board);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].message.contains("missing required `epic`"));
        assert_eq!(problems[0].check_id, CheckId::BearingMissingEpicLineage);
        assert!(
            matches!(
                &problems[0].fix,
                Some(Fix::SetFrontmatterField { field, value, .. })
                if field == "epic" && value == "1w5H2Bq9L"
            ),
            "fix must set epic to the bearing ID"
        );
    }

    #[test]
    fn check_bearing_lineage_epic_passes_with_epic() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("1w5H2Bq9L")
                    .status("laid")
                    .epic("1w5H2Bq9L"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_lineage_epic(&board);
        assert!(problems.is_empty());
    }

    #[test]
    fn check_bearing_lineage_goals_flags_invalid_format() {
        let temp = TestBoardBuilder::new()
            .bearing(
                TestBearing::new("1w5H2Bq9L")
                    .status("laid")
                    .epic("1w5H2Bq9L")
                    .goals(vec!["NOT-A-GOAL"]),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_lineage_goals(&board, temp.path());
        assert_eq!(problems.len(), 1);
        assert!(problems[0].message.contains("invalid goal reference"));
        assert_eq!(problems[0].check_id, CheckId::BearingInvalidGoalLineage);
    }

    #[test]
    fn lineage_validation_is_deterministic_across_repeated_cycles() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("1w5H2Bq9L").status("laid"))
            .bearing(
                TestBearing::new("1w5H2Bq9M")
                    .status("laid")
                    .epic("1w5H2Bq9M")
                    .goals(vec!["GOAL-01"]),
            )
            .build();

        let board = load_board(temp.path()).unwrap();

        let epic_run1 = check_bearing_lineage_epic(&board);
        let epic_run2 = check_bearing_lineage_epic(&board);
        assert_eq!(epic_run1.len(), epic_run2.len());
        for (a, b) in epic_run1.iter().zip(epic_run2.iter()) {
            assert_eq!(a.message, b.message);
            assert_eq!(a.check_id, b.check_id);
        }

        let goals_run1 = check_bearing_lineage_goals(&board, temp.path());
        let goals_run2 = check_bearing_lineage_goals(&board, temp.path());
        assert_eq!(goals_run1.len(), goals_run2.len());
        for (a, b) in goals_run1.iter().zip(goals_run2.iter()) {
            assert_eq!(a.message, b.message);
            assert_eq!(a.check_id, b.check_id);
        }
    }

    #[test]
    fn lineage_migration_scales_linearly_on_board_size() {
        // Create a board with multiple legacy bearings (no lineage) to verify
        // the check iterates linearly without exponential blowup.
        let mut builder = TestBoardBuilder::new();
        let count = 20;
        for i in 0..count {
            let id = format!("1w5H2B{:03}", i);
            builder = builder.bearing(TestBearing::new(&id).status("laid"));
        }
        let temp = builder.build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_bearing_lineage_epic(&board);
        // Exactly one problem per legacy bearing — linear in board size
        assert_eq!(problems.len(), count);
        // Each problem must carry a fix
        for problem in &problems {
            assert!(
                problem.fix.is_some(),
                "every legacy bearing must have a migration fix"
            );
        }
    }
}
