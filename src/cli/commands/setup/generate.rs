//! Generate command - regenerate all README files

use std::path::Path;

use anyhow::Result;

/// Run the generate command
pub fn run(board_dir: &Path) -> Result<()> {
    let board = crate::infrastructure::loader::load_board(board_dir)?;
    crate::infrastructure::generate::sync_board_artifacts(
        &board,
        &crate::infrastructure::generate::BoardArtifactSyncOptions::default(),
    )?;

    println!("Board updated");

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
                    .status(crate::domain::model::StoryState::InProgress)
                    .scope("test-epic/01-first"),
            )
            .build();

        run(temp.path()).unwrap();

        let readme = fs::read_to_string(temp.path().join("README.md")).unwrap();
        assert!(readme.contains("# Planning Board"));
        assert!(readme.contains("## Epics"));
        assert!(readme.contains("01-first"));
        assert!(!readme.contains("FEAT0001"));
    }

    #[test]
    fn generate_writes_flow_history_snapshot() {
        let temp = TestBoardBuilder::new().build();
        run(temp.path()).unwrap();

        let history = fs::read_to_string(temp.path().join("flow_history.json")).unwrap();
        assert!(history.contains("\"schema_version\": 3"));
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
                    .status(crate::domain::model::StoryState::InProgress)
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

    #[test]
    fn generate_does_not_synthesize_voyage_knowledge() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-done", "test-epic").status("done"))
            .build();

        let voyage_dir = temp.path().join("epics/test-epic/voyages/01-done");
        let knowledge_path = voyage_dir.join("KNOWLEDGE.md");
        assert!(!knowledge_path.exists());

        run(temp.path()).unwrap();

        assert!(!knowledge_path.exists());
    }
}
