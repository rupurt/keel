//! Health checks for voyages

use anyhow::Result;
use std::path::Path;

use super::super::types::{CheckId, Fix, GapCategory, Problem, Severity};
use crate::domain::model::{Board, VoyageState};
use crate::domain::state_machine::invariants;
use crate::infrastructure::validation::structural;

pub struct VoyageScanResult {
    pub problems: Vec<Problem>,
    pub file_count: usize,
}

pub fn scan_voyage_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    structural::scan_voyage_files(board_dir)
}

pub fn check_voyage_status_drift(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        let story_states: Vec<_> = board
            .stories_for_voyage(voyage)
            .into_iter()
            .map(|story| story.stage)
            .collect();

        let violations = invariants::validate_voyage_story_coherence(
            voyage.id(),
            voyage.status(),
            &story_states,
        );

        for violation in violations {
            let fix = match violation.suggested_fix() {
                Some(invariants::SuggestedFix::UpdateVoyageStatus { new_status }) => {
                    Some(Fix::UpdateVoyageStatus {
                        path: voyage.path.clone(),
                        new_status: new_status.to_string(),
                    })
                }
                _ => None,
            };

            problems.push(Problem {
                severity: Severity::Error,
                path: voyage.path.clone(),
                message: violation.message(),
                fix,
                scope: Some(voyage.scope_path()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::VoyageStatusDrift,
            });
        }
    }

    problems
}

pub fn check_prd_lineage_coherence(board: &Board) -> Vec<Problem> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|a, b| a.index().cmp(&b.index()).then_with(|| a.id().cmp(b.id())));

    let mut problems = Vec::new();
    for voyage in voyages {
        if voyage.status() == VoyageState::Done {
            continue;
        }
        problems.extend(invariants::prd_srs_lineage_problems(
            voyage,
            board,
            CheckId::VoyagePrdLineageCoherence,
        ));
    }

    problems
}

pub fn check_scope_lineage_coherence(board: &Board) -> Vec<Problem> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|a, b| a.index().cmp(&b.index()).then_with(|| a.id().cmp(b.id())));

    let mut problems = Vec::new();
    for voyage in voyages {
        if voyage.status() == VoyageState::Done {
            continue;
        }
        problems.extend(invariants::voyage_scope_lineage_problems(
            voyage,
            board,
            CheckId::VoyageScopeLineageCoherence,
        ));
    }

    problems
}

/// Check voyage title case
pub fn check_voyage_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        let title = &voyage.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(Problem {
                severity: Severity::Warning,
                path: voyage.path.clone(),
                message: format!("title '{}' should use Title Case", title),
                fix: Some(Fix::UpdateTitle {
                    path: voyage.path.clone(),
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

/// Check voyage date field consistency
pub fn check_voyage_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        problems.extend(structural::check_date_consistency(
            &voyage.path,
            CheckId::VoyageDateConsistency,
        ));
    }

    problems
}

/// Voyage work products should not include a PRESS_RELEASE.md.
/// Press releases are epic-level artifacts only.
pub fn check_voyage_press_release_artifacts(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        let voyage_dir = voyage.path.parent().unwrap_or(&voyage.path);
        let press_release_path = voyage_dir.join("PRESS_RELEASE.md");
        if press_release_path.exists() {
            problems.push(
                Problem::error(
                    press_release_path.clone(),
                    "voyage contains PRESS_RELEASE.md; press releases are epic-only artifacts",
                )
                .with_check_id(CheckId::VoyageUnexpectedPressRelease)
                .with_scope(voyage.scope_path())
                .with_fix(Fix::RemoveFile {
                    path: press_release_path,
                }),
            );
        }
    }

    problems
}

/// Check for duplicate voyage IDs across all epics
pub fn check_voyage_duplicates(board_dir: &Path) -> Vec<Problem> {
    crate::infrastructure::duplicate_ids::duplicate_id_problems(
        board_dir,
        crate::infrastructure::duplicate_ids::DuplicateEntity::Voyage,
    )
}

/// Check voyage ID-directory consistency
pub fn check_voyage_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        // Extract directory name from path: epics/{epic-id}/voyages/{dir-name}/README.md
        let dir_name = voyage
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        let frontmatter_id = voyage.id();

        if let Some(dir) = dir_name
            && dir != frontmatter_id
        {
            let old_path = voyage.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(Problem {
                severity: Severity::Error,
                path: voyage.path.clone(),
                message: format!(
                    "directory name '{}' differs from frontmatter id '{}'",
                    dir, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: Some(voyage.scope_path()),
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn status_drift_reports_backlog_story_in_draft_voyage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("draft")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1").stage(StoryState::Backlog))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_status_drift(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::VoyageStatusDrift);
        assert!(problems[0].message.contains("voyage is 'draft'"));
    }

    #[test]
    fn status_drift_suggests_fix_when_all_stories_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("in-progress")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1").stage(StoryState::Done))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_status_drift(&board);

        assert_eq!(problems.len(), 1);
        assert!(problems[0].message.contains("all 1 stories done"));
        assert!(matches!(
            problems[0].fix,
            Some(Fix::UpdateVoyageStatus {
                ref new_status, ..
            }) if new_status == "done"
        ));
    }

    #[test]
    fn doctor_and_gate_share_prd_lineage_rules() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("draft").srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Missing source. |  | cargo test |
| SRS-02 | Alias source. | PRD-01 | cargo test |
| SRS-03 | Unknown parent. | FR-99 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] valid planning story"),
            )
            .build();
        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Canonical parent. | must | coverage |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();

        let gate_messages: Vec<_> = crate::domain::state_machine::evaluate_voyage_transition(
            &board,
            voyage,
            crate::domain::state_machine::VoyageTransition::Plan,
            true,
        )
        .into_iter()
        .filter(|problem| problem.check_id == CheckId::VoyagePrdLineageCoherence)
        .map(|problem| problem.message)
        .collect();

        let doctor_messages: Vec<_> = check_prd_lineage_coherence(&board)
            .into_iter()
            .map(|problem| problem.message)
            .collect();

        assert_eq!(doctor_messages, gate_messages);
    }

    #[test]
    fn doctor_prd_lineage_skips_done_voyages() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("done").srs_content(
                r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Legacy completed voyage. | PRD-01 | cargo test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
            ))
            .build();
        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Canonical parent. | must | coverage |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        assert!(check_prd_lineage_coherence(&board).is_empty());
    }

    #[test]
    fn doctor_reports_scope_drift_and_contradictions() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned").srs_content(
                r#"# SRS

## Scope

In scope:
- [SCOPE-03] Illegally pull an out-of-scope item into this voyage.
- [SCOPE-99] Reference a missing parent scope item.

Out of scope:
- [SCOPE-02] Defer a valid in-scope item for a later voyage.
"#,
            ))
            .build();
        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Scope

### In Scope
- [SCOPE-01] Parse canonical scope IDs.
- [SCOPE-02] Render scope drift findings.

### Out of Scope
- [SCOPE-03] Story-level runtime enforcement.
"#,
        )
        .unwrap();

        let report = crate::cli::commands::diagnostics::doctor::validate(temp.path()).unwrap();
        let scope_check = report
            .voyage_checks
            .iter()
            .find(|check| check.name == "Scope lineage coherence")
            .expect("scope lineage check should be present");

        assert!(!scope_check.passed);
        assert!(scope_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::VoyageScopeLineageCoherence
                && problem.message.contains("SCOPE-01")
                && problem.message.contains("missing a voyage scope mapping")
        }));
        assert!(scope_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::VoyageScopeLineageCoherence
                && problem.message.contains("SCOPE-99")
                && problem.message.contains("unknown parent scope ID")
        }));
        assert!(scope_check.problems.iter().any(|problem| {
            problem.check_id == CheckId::VoyageScopeLineageCoherence
                && problem.message.contains("SCOPE-03")
                && problem
                    .message
                    .contains("contradicts the PRD by marking an out-of-scope item as in scope")
        }));
    }

    #[test]
    fn scope_drift_errors_are_actionable() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned").srs_content(
                r#"# SRS

## Scope

In scope:
- [SCOPE-03] Pull a forbidden item into scope.
- [SCOPE-99] Reference a missing parent.

Out of scope:
- [SCOPE-02] Defer a valid in-scope item.
"#,
            ))
            .build();
        let prd_path = temp.path().join("epics/e1/PRD.md");
        std::fs::write(
            &prd_path,
            r#"# PRD

## Scope

### In Scope
- [SCOPE-01] Parse canonical scope IDs.
- [SCOPE-02] Render scope drift findings.

### Out of Scope
- [SCOPE-03] Story-level runtime enforcement.
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_scope_lineage_coherence(&board);
        let srs_path = temp.path().join("epics/e1/voyages/v1/SRS.md");

        assert!(problems.iter().any(|problem| {
            problem.path == srs_path
                && problem.message.contains("SCOPE-01")
                && problem.message.contains("missing a voyage scope mapping")
        }));
        assert!(problems.iter().any(|problem| {
            problem.path == srs_path
                && problem.message.contains("SCOPE-99")
                && problem.message.contains("unknown parent scope ID")
        }));
        assert!(problems.iter().any(|problem| {
            problem.path == srs_path
                && problem.message.contains("SCOPE-03")
                && problem.message.contains("out-of-scope item as in scope")
        }));
    }

    #[test]
    fn scope_lineage_rejects_legacy_untagged_paths() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned").srs_content(
                r#"# SRS

## Scope

In scope:
- Parse canonical scope IDs without a parent tag.

Out of scope:
- [SCOPE-02] Leave a valid item out of this slice.
"#,
            ))
            .build();
        std::fs::write(
            temp.path().join("epics/e1/PRD.md"),
            r#"# PRD

## Scope

### In Scope
- Parse canonical scope IDs without a parent tag.
- [SCOPE-02] Render scope drift findings.
"#,
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_scope_lineage_coherence(&board);

        assert!(problems.iter().any(|problem| {
            problem.check_id == CheckId::VoyageScopeLineageCoherence
                && problem
                    .message
                    .contains("uses a legacy untagged scope bullet")
        }));
    }

    #[test]
    fn status_drift_accepts_planned_voyage_with_backlog_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("planned")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1").stage(StoryState::Backlog))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_status_drift(&board);
        assert!(problems.is_empty());
    }

    #[test]
    fn status_drift_accepts_planned_voyage_with_done_and_backlog_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("planned")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1").stage(StoryState::Done))
            .story(TestStory::new("S2").scope("e1/v1").stage(StoryState::Backlog))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_status_drift(&board);
        assert!(
            problems.is_empty(),
            "planned voyage with no active stories is coherent"
        );
    }

    #[test]
    fn status_drift_suggests_done_when_planned_voyage_has_only_done_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("planned")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1").stage(StoryState::Done))
            .story(TestStory::new("S2").scope("e1/v1").stage(StoryState::Done))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_status_drift(&board);

        assert_eq!(problems.len(), 1);
        assert!(problems[0].message.contains("all 2 stories done"));
        assert!(matches!(
            problems[0].fix,
            Some(Fix::UpdateVoyageStatus {
                ref new_status, ..
            }) if new_status == "done"
        ));
    }

    #[test]
    fn voyage_press_release_artifact_reports_error() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1"))
            .build();

        let press_release = temp.path().join("epics/e1/voyages/v1/PRESS_RELEASE.md");
        std::fs::write(&press_release, "# Voyage press release").unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_press_release_artifacts(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].check_id, CheckId::VoyageUnexpectedPressRelease);
        assert_eq!(problems[0].severity, Severity::Error);
        assert!(matches!(
            problems[0].fix,
            Some(Fix::RemoveFile { ref path }) if path == &press_release
        ));
    }

    #[test]
    fn voyage_press_release_artifact_allows_voyage_without_press_release() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1"))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_voyage_press_release_artifacts(&board);
        assert!(problems.is_empty());
    }
}
