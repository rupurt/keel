//! Start story command - pull story from backlog into execution

use std::path::Path;

use anyhow::Result;

#[cfg(test)]
use crate::application::story_lifecycle;
use crate::application::story_lifecycle::StoryLifecycleService;

/// Run the start story command
pub fn run(board_dir: &Path, id: &str, version: Option<u64>) -> Result<()> {
    StoryLifecycleService::start(board_dir, id, version)
}

/// Check if knowledge is relevant to the given epic and voyage scope
#[cfg(test)]
fn is_relevant_knowledge(
    knowledge: &crate::read_model::knowledge::Knowledge,
    epic_id: Option<&str>,
    scope: Option<&str>,
) -> bool {
    story_lifecycle::is_relevant_knowledge(knowledge, epic_id, scope)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use std::fs;

    #[test]
    fn start_moves_story_to_in_progress() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("BACKLOG1")
                    .title("Backlog Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "BACKLOG1", None).unwrap();

        // Status should be updated to in-progress
        let story_path = temp.path().join("stories/BACKLOG1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: in-progress"));
    }

    #[test]
    fn start_updates_frontmatter() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("UPDATE1")
                    .title("Update Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "UPDATE1", None).unwrap();

        let content = fs::read_to_string(temp.path().join("stories/UPDATE1/README.md")).unwrap();

        assert!(content.contains("status: in-progress"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn start_fuzzy_matches_by_partial_id() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("FUZZY1")
                    .title("Fuzzy Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "UZZY1", None).unwrap();

        let content = fs::read_to_string(temp.path().join("stories/FUZZY1/README.md")).unwrap();
        assert!(content.contains("status: in-progress"));
    }

    #[test]
    fn start_works_from_rejected_state() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("REJECTED1")
                    .title("Rejected Story")
                    .stage(StoryState::Rejected)
                    .body("## Rejections\n\n### 2026-01-25\n\nMissing tests coverage."),
            )
            .build();

        run(temp.path(), "REJECTED1", None).unwrap();

        // Content should preserve rejection history and update status
        let story_path = temp.path().join("stories/REJECTED1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("## Rejections"));
        assert!(content.contains("Missing tests coverage"));
        assert!(content.contains("status: in-progress"));
    }

    #[test]
    fn start_errors_on_ready_for_acceptance() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("READY1")
                    .title("Ready Story")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();

        let result = run(temp.path(), "READY1", None);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot start"));
    }

    #[test]
    fn is_relevant_knowledge_matches_by_scope() {
        let knowledge = crate::read_model::knowledge::Knowledge {
            id: "L001".to_string(),
            source: std::path::PathBuf::from("/test.md"),
            source_type: crate::read_model::knowledge::KnowledgeSourceType::Story,
            scope: Some("board-cli/01-core".to_string()),
            title: "Test".to_string(),
            category: "code".to_string(),
            context: String::new(),
            insight: "Insight".to_string(),
            suggested_action: String::new(),
            applies_to: String::new(),
            applied: String::new(),
            observed_at: None,
            score: 0.5,
            confidence: 0.8,
        };

        // Same epic is relevant
        assert!(is_relevant_knowledge(
            &knowledge,
            Some("board-cli"),
            Some("board-cli/07-learning")
        ));

        // Different epic is not relevant
        assert!(!is_relevant_knowledge(
            &knowledge,
            Some("other-epic"),
            Some("other-epic/01-test")
        ));
    }

    // Flat structure tests - stories stay in stories/ directory
    #[test]
    fn start_flat_updates_frontmatter_without_moving() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("FLAT1")
                    .title("Flat Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "FLAT1", None).unwrap();

        // Frontmatter should be updated
        let story_path = temp.path().join("stories/FLAT1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: in-progress"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn start_errors_on_version_conflict() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").stage(StoryState::Backlog))
            .build();

        // Pass an old version (0 instead of current)
        let result = run(temp.path(), "S1", Some(9999));
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Board state has changed")
        );
    }

    #[test]
    fn start_first_story_auto_starts_voyage() {
        use crate::test_helpers::TestVoyage;
        let temp = TestBoardBuilder::new()
            .epic(crate::test_helpers::TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned"))
            .story(
                TestStory::new("S1")
                    .stage(StoryState::Backlog)
                    .scope("e1/v1"),
            )
            .build();

        run(temp.path(), "S1", None).unwrap();

        // Voyage should now be in-progress
        let voyage_path = temp.path().join("epics/e1/voyages/v1/README.md");
        let content = fs::read_to_string(&voyage_path).unwrap();
        assert!(content.contains("status: in-progress"));
    }
}
