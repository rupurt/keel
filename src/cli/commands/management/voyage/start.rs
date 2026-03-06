//! Start voyage command

use std::path::Path;

use anyhow::Result;

use crate::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use crate::infrastructure::config::find_board_dir;
#[cfg(test)]
use crate::infrastructure::loader::load_board;

use super::guidance::{VoyageLifecycleAction, guidance_for_action, print_human};

/// Run the start-voyage command
pub fn run(id: &str, force: bool, expect_version: Option<u64>) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_options(&board_dir, id, force, expect_version)
}

/// Run the start-voyage command with an explicit board directory (legacy, no force)
#[allow(dead_code)] // Used by tests
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    run_with_options(board_dir, id, false, None)
}

/// Run the start-voyage command with all options
pub fn run_with_options(
    board_dir: &Path,
    id: &str,
    force: bool,
    expect_version: Option<u64>,
) -> Result<()> {
    VoyageEpicLifecycleService::start_voyage(board_dir, id, force, expect_version)?;
    let guidance = guidance_for_action(VoyageLifecycleAction::Start, id);
    print_human(guidance.as_ref());
    Ok(())
}

/// Check that all stories in the voyage are in valid states for starting.
///
/// Voyage start allows stories that are still queued (`backlog`, `icebox`) and
/// stories already completed before the voyage was formally started (`done`).
/// Stories that are actively underway still block the transition.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use std::fs;

    fn write_prd(temp: &tempfile::TempDir, epic_id: &str, content: &str) {
        fs::write(temp.path().join(format!("epics/{epic_id}/PRD.md")), content).unwrap();
    }

    #[test]
    fn start_voyage_updates_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-planning", "test-epic").status("planned"))
            .build();

        run_with_dir(temp.path(), "01-planning").unwrap();

        let content = fs::read_to_string(
            temp.path()
                .join("epics/test-epic/voyages/01-planning/README.md"),
        )
        .unwrap();

        assert!(content.contains("status: in-progress"));
        assert!(content.contains("started_at:"));
    }

    #[test]
    fn start_voyage_errors_on_already_in_progress() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("02-progress", "test-epic").status("in-progress"))
            .build();

        let result = run_with_dir(temp.path(), "02-progress");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Cannot start voyage"), "Error was: {}", err);
    }

    #[test]
    fn start_voyage_errors_on_not_found() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .build();

        let result = run_with_dir(temp.path(), "nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn start_voyage_blocks_on_uncovered_requirements() {
        use crate::test_helpers::TestStory;

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First requirement | FR-01 | test |
| SRS-02 | Second requirement | FR-02 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        // Story only covers SRS-01, not SRS-02
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-draft")
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->",
                    ),
            )
            .build();
        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | First requirement | must | test |
| FR-02 | Second requirement | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

        let result = run_with_options(temp.path(), "01-draft", false, None);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("SRS-02"),
            "Error should mention uncovered requirement: {}",
            err
        );
        assert!(
            err.contains("uncovered") || err.contains("not covered"),
            "Error should mention coverage: {}",
            err
        );
    }

    #[test]
    fn start_voyage_allows_when_all_requirements_covered() {
        use crate::test_helpers::TestStory;

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First requirement | FR-01 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-draft")
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->",
                    ),
            )
            .build();
        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | First requirement | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

        let result = run_with_options(temp.path(), "01-draft", false, None);

        assert!(
            result.is_ok(),
            "Should succeed when all requirements covered: {:?}",
            result
        );
    }

    #[test]
    fn start_voyage_force_bypasses_coverage_check() {
        use crate::test_helpers::TestStory;

        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First requirement | FR-01 | test |
| SRS-02 | Second requirement | FR-02 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        // Story only covers SRS-01, not SRS-02
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-draft")
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->",
                    ),
            )
            .build();
        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | First requirement | must | test |
| FR-02 | Second requirement | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

        // With force=true, should succeed despite uncovered requirements
        let result = run_with_options(temp.path(), "01-draft", true, None);

        assert!(result.is_ok(), "Should succeed with --force: {:?}", result);
    }

    // ============ Story State Precondition Tests (SRS-03) ============

    #[test]
    fn start_voyage_fails_with_in_progress_story() {
        use crate::domain::model::StoryState;
        use crate::test_helpers::TestStory;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-test", "test-epic").status("planned"))
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-test")
                    .stage(StoryState::InProgress),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-test");

        assert!(result.is_err(), "Should fail with in-progress story");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("in-progress"),
            "Error should mention in-progress state: {}",
            err
        );
    }

    #[test]
    fn start_voyage_fails_with_needs_verification_story() {
        use crate::domain::model::StoryState;
        use crate::test_helpers::TestStory;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-test", "test-epic").status("planned"))
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-test")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-test");

        assert!(result.is_err(), "Should fail with needs-verification story");
        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("needs-human-verification"),
            "Error should mention needs-human-verification state: {}",
            err
        );
    }

    #[test]
    fn start_voyage_succeeds_with_done_story() {
        use crate::domain::model::StoryState;
        use crate::test_helpers::TestStory;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-test", "test-epic").status("planned"))
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-test")
                    .stage(StoryState::Done),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-test");

        assert!(
            result.is_ok(),
            "Should succeed with done story already completed before voyage start: {:?}",
            result
        );
    }

    #[test]
    fn start_voyage_succeeds_with_backlog_and_icebox_stories() {
        use crate::domain::model::StoryState;
        use crate::test_helpers::TestStory;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-test", "test-epic").status("planned"))
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-test")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("STORY02")
                    .scope("test-epic/01-test")
                    .stage(StoryState::Icebox),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-test");

        assert!(
            result.is_ok(),
            "Should succeed with backlog and icebox stories: {:?}",
            result
        );
    }

    #[test]
    fn start_voyage_error_lists_problematic_stories() {
        use crate::domain::model::StoryState;
        use crate::test_helpers::TestStory;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-test", "test-epic").status("planned"))
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-test")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("STORY02")
                    .scope("test-epic/01-test")
                    .stage(StoryState::InProgress),
            )
            .story(
                TestStory::new("STORY03")
                    .scope("test-epic/01-test")
                    .stage(StoryState::Done),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-test");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();

        // Should list only the problematic active story
        assert!(
            err.contains("STORY02"),
            "Error should list STORY02: {}",
            err
        );
        assert!(
            !err.contains("STORY03"),
            "Error should not list STORY03 because done stories are allowed: {}",
            err
        );
        // Should not list the valid story
        assert!(
            !err.contains("STORY01"),
            "Error should not list STORY01: {}",
            err
        );
    }

    #[test]
    fn start_voyage_succeeds_with_no_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-test", "test-epic").status("planned"))
            .build();

        let result = run_with_dir(temp.path(), "01-test");

        assert!(
            result.is_ok(),
            "Should succeed with no stories: {:?}",
            result
        );
    }

    // ============ Optimistic Locking Tests (SRS-05) ============

    #[test]
    fn start_voyage_succeeds_with_matching_version() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-ver", "test-epic").status("planned"))
            .build();

        // Get current board version
        let board = load_board(temp.path()).unwrap();
        let version = board.snapshot_version();

        // Start with matching version should succeed
        let result = run_with_options(temp.path(), "01-ver", false, Some(version));
        assert!(result.is_ok(), "Should succeed with matching version");
    }

    #[test]
    fn start_voyage_fails_with_mismatched_version() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-ver2", "test-epic").status("planned"))
            .build();

        // Use an arbitrary wrong version
        let wrong_version = 12345u64;

        let result = run_with_options(temp.path(), "01-ver2", false, Some(wrong_version));
        assert!(result.is_err(), "Should fail with mismatched version");

        let err = result.unwrap_err().to_string();
        assert!(
            err.contains("version mismatch"),
            "Error should mention version mismatch: {}",
            err
        );
        assert!(
            err.contains("expected"),
            "Error should mention expected version: {}",
            err
        );
        assert!(
            err.contains("actual"),
            "Error should mention actual version: {}",
            err
        );
    }

    #[test]
    fn start_voyage_bypasses_check_when_version_omitted() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-ver3", "test-epic").status("planned"))
            .build();

        // Start without version should bypass check
        let result = run_with_options(temp.path(), "01-ver3", false, None);
        assert!(
            result.is_ok(),
            "Should succeed when version is omitted: {:?}",
            result
        );
    }
}
