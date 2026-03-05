//! Generate command - regenerate all README files

use std::fs;
use std::path::Path;

use anyhow::Result;

use crate::domain::model::VoyageState;
use crate::infrastructure::loader::load_board;

/// Run the generate command
pub fn run(board_dir: &Path) -> Result<()> {
    let backfill_stats = crate::application::started_at_backfill::backfill(board_dir)?;
    let board = load_board(board_dir)?;

    // 1. Generate board-level README.md
    crate::infrastructure::generate::board_readme::generate(board_dir, &board)?;

    // 2. Generate epic-level README.md files
    for epic in board.epics.values() {
        crate::infrastructure::generate::epic_readme::generate(board_dir, &board, epic)?;
    }

    // 3. Generate voyage-level README.md and done-only report artifacts
    for voyage in board.voyages.values() {
        crate::infrastructure::generate::voyage_readme::generate(board_dir, &board, voyage)?;

        if voyage.status() == VoyageState::Done {
            crate::infrastructure::generate::knowledge_synthesis::synthesize_voyage_knowledge(
                &board, voyage,
            )?;
            crate::infrastructure::generate::voyage_report::generate(board_dir, &board, voyage)?;
            crate::infrastructure::generate::compliance_report::generate(
                board_dir, &board, voyage,
            )?;
        } else {
            remove_report_artifacts(voyage)?;
        }
    }

    // 4. Regenerate persistent weekly throughput history for diagnostics graphs.
    let history = crate::read_model::throughput_history::project_default(&board);
    crate::infrastructure::throughput_history_store::save_if_changed(board_dir, &history)?;

    // 5. Rebuild centralized knowledge manifest.
    crate::read_model::knowledge::sync_knowledge_manifest(board_dir)?;

    if backfill_stats.stories_updated > 0 || backfill_stats.voyages_updated > 0 {
        println!(
            "Backfilled started_at timestamps (stories: {}, voyages: {})",
            backfill_stats.stories_updated, backfill_stats.voyages_updated
        );
    }
    println!("Board updated");

    Ok(())
}

fn remove_report_artifacts(voyage: &crate::domain::model::Voyage) -> Result<()> {
    let Some(voyage_dir) = voyage.path.parent() else {
        return Ok(());
    };

    for artifact in ["VOYAGE_REPORT.md", "COMPLIANCE_REPORT.md"] {
        let artifact_path = voyage_dir.join(artifact);
        if artifact_path.exists() {
            fs::remove_file(artifact_path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use std::path::Path;

    fn inject_documents_markers(voyage_readme_path: &Path) {
        let original = fs::read_to_string(voyage_readme_path).unwrap();
        let with_documents = original.replace(
            "## Stories",
            "## Documents\n\n<!-- BEGIN DOCUMENTS -->\n| placeholder | placeholder |\n<!-- END DOCUMENTS -->\n\n## Stories",
        );
        fs::write(voyage_readme_path, with_documents).unwrap();
    }

    #[test]
    fn generate_creates_board_readme() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Active Story")
                    .stage(crate::domain::model::StoryState::InProgress)
                    .scope("test-epic/01-first"),
            )
            .build();

        run(temp.path()).unwrap();

        let readme = fs::read_to_string(temp.path().join("README.md")).unwrap();
        assert!(readme.contains("# Planning Board"));
        assert!(readme.contains("FEAT0001"));
    }

    #[test]
    fn generate_writes_flow_history_snapshot() {
        let temp = TestBoardBuilder::new().build();
        run(temp.path()).unwrap();

        let history = fs::read_to_string(temp.path().join("flow_history.json")).unwrap();
        assert!(history.contains("\"schema_version\": 1"));
    }

    #[test]
    fn generate_updates_epic_readme() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .build();

        run(temp.path()).unwrap();

        let readme = fs::read_to_string(temp.path().join("epics/test-epic/README.md")).unwrap();
        assert!(readme.contains("# test-epic Epic"));
        assert!(readme.contains("## Voyages"));
        assert!(readme.contains("01-first"));
    }

    #[test]
    fn generate_updates_voyage_readme() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Active Story")
                    .stage(crate::domain::model::StoryState::InProgress)
                    .scope("test-epic/01-first"),
            )
            .build();

        run(temp.path()).unwrap();

        let readme = fs::read_to_string(
            temp.path()
                .join("epics/test-epic/voyages/01-first/README.md"),
        )
        .unwrap();
        assert!(readme.contains("# 01-first Voyage"));
        assert!(readme.contains("## Stories"));
        assert!(readme.contains("FEAT0001"));
    }

    #[test]
    fn generate_hides_reports_for_non_done_voyages() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .build();

        let voyage_dir = temp.path().join("epics/test-epic/voyages/01-first");
        let readme_path = voyage_dir.join("README.md");
        inject_documents_markers(&readme_path);

        fs::write(voyage_dir.join("VOYAGE_REPORT.md"), "stale voyage report").unwrap();
        fs::write(
            voyage_dir.join("COMPLIANCE_REPORT.md"),
            "stale compliance report",
        )
        .unwrap();

        run(temp.path()).unwrap();

        assert!(!voyage_dir.join("VOYAGE_REPORT.md").exists());
        assert!(!voyage_dir.join("COMPLIANCE_REPORT.md").exists());

        let readme = fs::read_to_string(readme_path).unwrap();
        assert!(readme.contains("[SRS.md](SRS.md)"));
        assert!(readme.contains("[SDD.md](SDD.md)"));
        assert!(!readme.contains("VOYAGE_REPORT.md"));
        assert!(!readme.contains("COMPLIANCE_REPORT.md"));
    }

    #[test]
    fn generate_includes_reports_for_done_voyages() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-done", "test-epic").status("done"))
            .build();

        let voyage_dir = temp.path().join("epics/test-epic/voyages/01-done");
        let readme_path = voyage_dir.join("README.md");
        inject_documents_markers(&readme_path);

        run(temp.path()).unwrap();

        assert!(voyage_dir.join("VOYAGE_REPORT.md").exists());
        assert!(voyage_dir.join("COMPLIANCE_REPORT.md").exists());

        let readme = fs::read_to_string(readme_path).unwrap();
        assert!(readme.contains("VOYAGE_REPORT.md"));
        assert!(readme.contains("COMPLIANCE_REPORT.md"));
    }
}
