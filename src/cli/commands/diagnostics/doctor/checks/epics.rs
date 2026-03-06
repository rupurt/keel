use std::path::Path;

use anyhow::Result;

use super::super::types::*;
use crate::domain::model::{Board, EpicState};
use crate::domain::state_machine::invariants;
use crate::infrastructure::validation::structural;

/// Scan epic files for structural problems
/// Returns (problems, epic_count)
pub fn scan_epic_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    structural::scan_epic_files(board_dir)
}

/// Check a single epic file for problems
pub fn check_epic_file(path: &Path) -> Option<Problem> {
    structural::check_epic_file(path)
}

/// Check epic README structure for rendering issues
pub fn check_epic_readme_structure(path: &Path) -> Vec<Problem> {
    structural::check_epic_readme_structure(path)
}

/// Check epic PRD.md structure for section markers and content
pub fn check_epic_prd_structure(path: &Path) -> Vec<Problem> {
    structural::check_epic_prd_structure(path)
}

/// Check epic title case
pub fn check_epic_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let title = &epic.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(Problem {
                severity: Severity::Warning,
                path: epic.path.clone(),
                message: format!("title '{}' should use Title Case", title),
                fix: Some(Fix::UpdateTitle {
                    path: epic.path.clone(),
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

/// Check epic date field consistency
pub fn check_epic_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        problems.extend(structural::check_date_consistency(
            &epic.path,
            CheckId::EpicDateConsistency,
        ));
    }

    problems
}

/// Check coherence between epic derived status and voyage states.
pub fn check_epic_status_drift(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let voyage_states: Vec<_> = board
            .voyages_for_epic(epic)
            .into_iter()
            .map(|voyage| voyage.status())
            .collect();

        let violations =
            invariants::validate_epic_voyage_coherence(epic.id(), epic.status(), &voyage_states);

        for violation in violations {
            problems.push(Problem {
                severity: Severity::Error,
                path: epic.path.clone(),
                message: violation.message(),
                fix: None,
                scope: Some(epic.id().to_string()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::EpicStatusDrift,
            });
        }
    }

    problems
}

/// Check completion gates for done epics
pub fn check_epic_done_gates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        if epic.status() == EpicState::Done {
            problems.extend(crate::domain::state_machine::evaluate_epic_done(
                board, epic,
            ));
        }
    }

    problems
}

/// Check goal-to-requirement lineage inside epic PRDs.
pub fn check_epic_goal_lineage_coherence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        problems.extend(invariants::epic_goal_lineage_problems(
            epic,
            CheckId::EpicGoalLineageCoherence,
        ));
    }

    problems
}

/// Check epic ID-directory consistency
pub fn check_epic_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        // Extract directory name from path: epics/{dir-name}/README.md
        let dir_name = epic
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        let frontmatter_id = epic.id();

        if let Some(dir) = dir_name
            && dir != frontmatter_id
        {
            let old_path = epic.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(Problem {
                severity: Severity::Error,
                path: epic.path.clone(),
                message: format!(
                    "directory name '{}' differs from frontmatter id '{}'",
                    dir, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: Some(epic.id().to_string()),
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}

/// Check for duplicate epic IDs
pub fn check_epic_duplicates(board_dir: &Path) -> Vec<Problem> {
    crate::infrastructure::duplicate_ids::duplicate_id_problems(
        board_dir,
        crate::infrastructure::duplicate_ids::DuplicateEntity::Epic,
    )
}

/// Check optional epic PRESS_RELEASE.md files for unresolved scaffold text.
pub fn check_epic_press_release(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let pr_path = epic.path.parent().unwrap().join("PRESS_RELEASE.md");

        if !pr_path.exists() {
            continue;
        }

        let content = match std::fs::read_to_string(&pr_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if let Some(pattern) = structural::first_unfilled_placeholder_pattern(&content) {
            problems.push(
                Problem::error(
                    pr_path,
                    format!(
                        "PRESS_RELEASE.md has unresolved scaffold/default text (pattern: {})",
                        pattern
                    ),
                )
                .with_check_id(CheckId::EpicPressReleaseIncomplete)
                .with_scope(epic.id()),
            );
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use std::fs;

    #[test]
    fn check_epic_press_release_reports_unresolved_scaffold_as_error() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .build();
        let pr_path = temp.path().join("epics/epic-1/PRESS_RELEASE.md");
        fs::write(&pr_path, "# PRESS RELEASE\n\nTODO: finalize release copy").unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_epic_press_release(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::EpicPressReleaseIncomplete);
        assert!(problems[0].message.contains("pattern: TODO:"));
    }

    #[test]
    fn check_epic_press_release_allows_missing_press_release() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .build();

        let pr_path = temp.path().join("epics/epic-1/PRESS_RELEASE.md");
        if pr_path.exists() {
            fs::remove_file(pr_path).unwrap();
        }

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_epic_press_release(&board);
        assert!(problems.is_empty());
    }

    #[test]
    fn check_epic_status_drift_reports_incoherent_derived_state() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .voyage(TestVoyage::new("v1", "epic-1").status("draft"))
            .build();
        let mut board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        // Simulate stale derived state drift to exercise coherence mapping.
        board
            .epics
            .get_mut("epic-1")
            .unwrap()
            .set_status(crate::domain::model::EpicState::Active);

        let problems = check_epic_status_drift(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::EpicStatusDrift);
    }

    #[test]
    fn goal_lineage_errors_are_actionable() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .build();
        let prd_path = temp.path().join("epics/epic-1/PRD.md");
        fs::write(
            &prd_path,
            r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Keep planning coherent | linked requirements | 100% |

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Use a valid goal link. | GOAL-99 | must | validation |
| FR-02 | Demonstrate missing links. |  | should | validation |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_epic_goal_lineage_coherence(&board);

        assert!(
            problems.iter().any(|problem| {
                problem.path == prd_path
                    && problem.message.contains("FR-01")
                    && problem.message.contains("GOAL-99")
                    && problem.message.contains("unknown goal")
            }),
            "expected unknown-goal problem with requirement id, goal id, and PRD path: {problems:#?}"
        );
        assert!(
            problems.iter().any(|problem| {
                problem.path == prd_path
                    && problem.message.contains("FR-02")
                    && problem.message.contains("missing Goals")
            }),
            "expected missing-goals problem with requirement id and PRD path: {problems:#?}"
        );
    }

    #[test]
    fn doctor_reports_goal_lineage_gaps() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .build();
        let prd_path = temp.path().join("epics/epic-1/PRD.md");
        fs::write(
            &prd_path,
            r#"# PRD

## Goals & Objectives
| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Keep planning coherent | linked requirements | 100% |
| GOAL-02 | Remove hidden drift | actionable diagnostics | 100% |

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Reference an unknown goal. | GOAL-99 | must | validation |
| FR-02 | Omit goal links. |  | should | validation |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let report = crate::cli::commands::diagnostics::doctor::validate(temp.path()).unwrap();
        let goal_check = report
            .epic_checks
            .iter()
            .find(|check| check.name == "Goal lineage coherence")
            .expect("goal lineage check should be present");

        assert!(!goal_check.passed);
        assert!(goal_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::EpicGoalLineageCoherence
                && problem.path == prd_path
                && problem.message.contains("GOAL-99")
                && problem.message.contains("FR-01")
        }));
        assert!(goal_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::EpicGoalLineageCoherence
                && problem.path == prd_path
                && problem.message.contains("FR-02")
                && problem.message.contains("missing Goals")
        }));
        assert!(goal_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::EpicGoalLineageCoherence
                && problem.path == prd_path
                && problem.message.contains("GOAL-01")
                && problem.message.contains("no linked PRD requirements")
        }));
        assert!(goal_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::EpicGoalLineageCoherence
                && problem.path == prd_path
                && problem.message.contains("GOAL-02")
                && problem.message.contains("no linked PRD requirements")
        }));
    }
}
