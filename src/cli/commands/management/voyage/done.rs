//! Done voyage command

use std::path::Path;

use crate::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use crate::infrastructure::config::find_board_dir;
use anyhow::Result;

use super::guidance::{VoyageLifecycleAction, guidance_for_action, print_human};

/// Run the done-voyage command
pub fn run(
    id: &str,
    well: Option<String>,
    hard: Option<String>,
    different: Option<String>,
) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, id, well, hard, different)
}

/// Run the done-voyage command with an explicit board directory
pub fn run_with_dir(
    board_dir: &Path,
    id: &str,
    well: Option<String>,
    hard: Option<String>,
    different: Option<String>,
) -> Result<()> {
    VoyageEpicLifecycleService::complete_voyage(board_dir, id, well, hard, different)?;
    let guidance = guidance_for_action(VoyageLifecycleAction::Done, id);
    print_human(guidance.as_ref());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    #[test]
    fn done_voyage_updates_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-progress", "test-epic").status("in-progress"))
            .voyage(TestVoyage::new("02-done", "test-epic").status("done"))
            .build();

        run_with_dir(temp.path(), "01-progress", None, None, None).unwrap();

        let content = fs::read_to_string(
            temp.path()
                .join("epics/test-epic/voyages/01-progress/README.md"),
        )
        .unwrap();

        assert!(content.contains("status: done"));
        assert!(content.contains("completed_at:"));
    }

    #[test]
    fn done_voyage_adds_retrospective() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-progress", "test-epic").status("in-progress"))
            .build();

        run_with_dir(
            temp.path(),
            "01-progress",
            Some("Clean design".to_string()),
            Some("Edge cases".to_string()),
            None,
        )
        .unwrap();

        let content = fs::read_to_string(
            temp.path()
                .join("epics/test-epic/voyages/01-progress/README.md"),
        )
        .unwrap();

        assert!(content.contains("## Retrospective"));
        assert!(content.contains("Clean design"));
        assert!(content.contains("Edge cases"));
    }

    #[test]
    fn done_voyage_rejects_incomplete_evidence_chain() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("evidence-epic"))
            .voyage(
                TestVoyage::new("01-evidence", "evidence-epic")
                    .status("in-progress")
                    .srs_content(
                        r#"
## Functional Requirements
BEGIN FUNCTIONAL_REQUIREMENTS
| SRS-01 | First requirement |
END FUNCTIONAL_REQUIREMENTS
"#,
                    ),
            )
            .story(
                TestStory::new("EVID1")
                    .title("Evidence Story")
                    .stage(crate::domain::model::StoryState::InProgress)
                    .scope("evidence-epic/01-evidence")
                    .body(
                        "- [x] [SRS-01/AC-01] Validate end-only evidence <!-- verify: echo evidence-ready SRS-01:end -->\n",
                    ),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-evidence", None, None, None);

        assert!(result.is_err());
        let msg = result.unwrap_err().to_string();
        assert!(msg.contains("Cannot complete voyage 01-evidence"));
        assert!(msg.contains("is not complete"));
    }

    #[test]
    fn done_voyage_errors_on_already_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("02-done", "test-epic").status("done"))
            .build();

        let result = run_with_dir(temp.path(), "02-done", None, None, None);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Cannot complete voyage"));
        assert!(err.contains("must be in-progress"));
    }

    #[test]
    fn done_voyage_errors_on_not_found() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .build();

        let result = run_with_dir(temp.path(), "nonexistent", None, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn done_voyage_auto_completes_epic_when_all_voyages_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-done", "test-epic").status("done"))
            .voyage(TestVoyage::new("02-in-progress", "test-epic").status("in-progress"))
            .build();

        run_with_dir(temp.path(), "02-in-progress", None, None, None).unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let epic = board.require_epic("test-epic").unwrap();
        assert_eq!(epic.status(), crate::domain::model::EpicState::Done);

        let content = fs::read_to_string(temp.path().join("epics/test-epic/README.md")).unwrap();
        assert!(!content.contains("\nstatus:"));
        assert!(!content.contains("\ncompleted_at:"));
    }

    #[test]
    fn done_voyage_keeps_epic_active_if_voyages_incomplete() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-done", "test-epic").status("done"))
            .voyage(TestVoyage::new("02-in-progress", "test-epic").status("in-progress"))
            .voyage(TestVoyage::new("03-in-progress", "test-epic").status("in-progress"))
            .build();

        run_with_dir(temp.path(), "02-in-progress", None, None, None).unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let epic = board.require_epic("test-epic").unwrap();
        assert_eq!(epic.status(), crate::domain::model::EpicState::Active);
    }

    #[test]
    fn done_voyage_generates_press_release() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("release-epic"))
            .voyage(TestVoyage::new("01-release", "release-epic").status("in-progress"))
            .story(
                TestStory::new("S1")
                    .title("Story 1")
                    .scope("release-epic/01-release")
                    .stage(crate::domain::model::StoryState::Done)
                    .body("## Summary\nThis is story 1 summary.\n"),
            )
            .build();

        // Add reflection manually
        let reflect_path = temp.path().join("stories/S1/REFLECT.md");
        fs::write(
            reflect_path,
            "## Knowledge\n\n### L001: Insight 1\n\n| Field | Value |\n|-------|-------|\n| **Insight** | Keep story summary and evidence synchronized |\n| **Suggested Action** | Require synchronized updates in done flow |\n",
        )
        .unwrap();

        // Add evidence manually
        let evidence_dir = temp.path().join("stories/S1/EVIDENCE");
        fs::create_dir_all(&evidence_dir).unwrap();
        fs::write(evidence_dir.join("proof.txt"), "proof content").unwrap();

        run_with_dir(temp.path(), "01-release", None, None, None).unwrap();

        let release_path = temp
            .path()
            .join("epics/release-epic/voyages/01-release/PRESS_RELEASE.md");
        assert!(release_path.exists());

        let release_content = fs::read_to_string(release_path).unwrap();
        assert!(release_content.contains("# PRESS RELEASE: 01-release Voyage"));
        assert!(release_content.contains("## Narrative Summary"));
        assert!(release_content.contains("### Story 1"));
        assert!(release_content.contains("This is story 1 summary."));
        assert!(release_content.contains("## Key Insights"));
        assert!(release_content.contains("L001: Insight 1"));
        assert!(release_content.contains("Suggested Action"));
        assert!(release_content.contains("## Verification Proof"));
        assert!(release_content.contains("proof.txt"));

        let voyage_report_path = temp
            .path()
            .join("epics/release-epic/voyages/01-release/VOYAGE_REPORT.md");
        let compliance_report_path = temp
            .path()
            .join("epics/release-epic/voyages/01-release/COMPLIANCE_REPORT.md");
        assert!(voyage_report_path.exists());
        assert!(compliance_report_path.exists());
    }

    #[test]
    fn done_voyage_refreshes_existing_report_artifacts() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("report-epic"))
            .voyage(TestVoyage::new("01-report", "report-epic").status("in-progress"))
            .story(
                TestStory::new("R1")
                    .title("Report Story")
                    .scope("report-epic/01-report")
                    .stage(crate::domain::model::StoryState::Done),
            )
            .build();

        let voyage_dir = temp.path().join("epics/report-epic/voyages/01-report");
        fs::write(voyage_dir.join("VOYAGE_REPORT.md"), "stale-voyage-report").unwrap();
        fs::write(
            voyage_dir.join("COMPLIANCE_REPORT.md"),
            "stale-compliance-report",
        )
        .unwrap();

        run_with_dir(temp.path(), "01-report", None, None, None).unwrap();

        let voyage_report = fs::read_to_string(voyage_dir.join("VOYAGE_REPORT.md")).unwrap();
        assert!(voyage_report.contains("# VOYAGE REPORT: 01-report Voyage"));
        assert!(!voyage_report.contains("stale-voyage-report"));

        let compliance_report =
            fs::read_to_string(voyage_dir.join("COMPLIANCE_REPORT.md")).unwrap();
        assert!(compliance_report.contains("# COMPLIANCE REPORT: 01-report Voyage"));
        assert!(!compliance_report.contains("stale-compliance-report"));
    }
}
