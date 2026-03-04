//! Reject command - reject a story and move to rejected

use std::path::Path;

use anyhow::Result;

use super::guidance::{StoryLifecycleAction, error_with_recovery};
#[cfg(test)]
use crate::application::story_lifecycle;
use crate::application::story_lifecycle::StoryLifecycleService;

/// Run the reject command
pub fn run(board_dir: &Path, id: &str, reason: &str) -> Result<()> {
    StoryLifecycleService::reject(board_dir, id, reason)
        .map_err(|err| error_with_recovery(StoryLifecycleAction::Reject, id, err))
}

/// Append rejection reason to the story markdown
#[cfg(test)]
fn append_rejection(story_path: &Path, reason: &str) -> Result<()> {
    story_lifecycle::append_rejection(story_path, reason)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn reject_moves_story_to_rejected() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Story to Reject")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();

        run(temp.path(), "0001", "Missing tests").unwrap();

        // Status should be updated to rejected
        let story_path = temp.path().join("stories/0001/README.md");
        assert!(story_path.exists());

        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: rejected"));
    }

    #[test]
    fn reject_updates_frontmatter() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Story to Reject")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();

        run(temp.path(), "0001", "Missing tests").unwrap();

        let content = fs::read_to_string(temp.path().join("stories/0001/README.md")).unwrap();

        assert!(content.contains("status: rejected"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn reject_appends_rejection_section() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Story to Reject")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();

        run(temp.path(), "0001", "Missing tests").unwrap();

        let content = fs::read_to_string(temp.path().join("stories/0001/README.md")).unwrap();

        assert!(content.contains("## Rejections"));
        assert!(content.contains("Missing tests"));
    }

    #[test]
    fn reject_errors_on_already_rejected() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0002")
                    .title("Rejected Story")
                    .stage(StoryState::Rejected),
            )
            .build();

        let result = run(temp.path(), "0002", "Still missing tests");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Cannot reject"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story show 0002"));
    }

    #[test]
    fn reject_errors_on_done_story() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0003")
                    .title("Done Story")
                    .stage(StoryState::Done),
            )
            .build();

        let result = run(temp.path(), "0003", "Wait I changed my mind");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot reject"));
    }

    #[test]
    fn reject_errors_on_not_found() {
        let temp = TestBoardBuilder::new().build();

        let result = run(temp.path(), "NONEXISTENT", "Reason");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn reject_flat_updates_frontmatter_without_moving() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsAAA")
                    .title("Story to Reject")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();

        run(temp.path(), "1vkqtsAAA", "Missing tests").unwrap();

        // Story bundle README should still exist
        let story_path = temp.path().join("stories/1vkqtsAAA/README.md");
        assert!(story_path.exists(), "Story bundle README should exist");

        // Frontmatter should be updated
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: rejected"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn reject_flat_errors_on_already_rejected() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsBBB")
                    .title("Rejected Story")
                    .stage(StoryState::Rejected),
            )
            .build();

        let result = run(temp.path(), "1vkqtsBBB", "Reason");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot reject"));
    }

    #[test]
    fn append_rejection_creates_section_if_missing() {
        let temp = TempDir::new().unwrap();
        let story_path = temp.path().join("story.md");
        fs::write(&story_path, "# My Story\n").unwrap();

        append_rejection(&story_path, "Reason one").unwrap();

        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("## Rejections"));
        assert!(content.contains("Reason one"));
    }

    #[test]
    fn append_rejection_adds_to_existing_section() {
        let temp = TempDir::new().unwrap();
        let story_path = temp.path().join("story.md");
        fs::write(
            &story_path,
            "# My Story\n\n## Rejections\n\n### 2026-01-01\nOld reason\n",
        )
        .unwrap();

        append_rejection(&story_path, "New reason").unwrap();

        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("Old reason"));
        assert!(content.contains("New reason"));
        // Header should appear only once
        assert_eq!(content.matches("## Rejections").count(), 1);
    }
}
