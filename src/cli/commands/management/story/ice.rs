//! Ice command - move story to icebox

use std::path::Path;

use anyhow::Result;

use crate::application::story_lifecycle::StoryLifecycleService;

/// Run the ice command
pub fn run(board_dir: &Path, id: &str) -> Result<()> {
    StoryLifecycleService::ice(board_dir, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use std::fs;

    #[test]
    fn ice_moves_story_to_icebox() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Test Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "0001").unwrap();

        // Story bundle should exist
        let story_path = temp.path().join("stories/0001/README.md");
        assert!(story_path.exists());

        // Status should be updated to icebox
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: icebox"));
    }

    #[test]
    fn ice_updates_frontmatter() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Test Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "0001").unwrap();

        let content = fs::read_to_string(temp.path().join("stories/0001/README.md")).unwrap();

        assert!(content.contains("status: icebox"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn ice_errors_on_already_in_icebox() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0002")
                    .title("Frozen Story")
                    .stage(StoryState::Icebox),
            )
            .build();

        let result = run(temp.path(), "0002");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot ice"));
    }

    #[test]
    fn ice_errors_on_done_story() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0003")
                    .title("Done Story")
                    .stage(StoryState::Done),
            )
            .build();

        let result = run(temp.path(), "0003");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot ice"));
    }

    #[test]
    fn ice_errors_on_not_found() {
        let temp = TestBoardBuilder::new().build();

        let result = run(temp.path(), "NONEXISTENT");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn ice_flat_updates_frontmatter_without_moving() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsAAA")
                    .title("Test Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        run(temp.path(), "1vkqtsAAA").unwrap();

        // Frontmatter should be updated in the bundle README
        let story_path = temp.path().join("stories/1vkqtsAAA/README.md");
        assert!(story_path.exists(), "Story bundle README should exist");

        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: icebox"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn ice_flat_errors_on_already_in_icebox() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsBBB")
                    .title("Frozen Story")
                    .stage(StoryState::Icebox),
            )
            .build();

        let result = run(temp.path(), "1vkqtsBBB");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot ice"));
    }
}
