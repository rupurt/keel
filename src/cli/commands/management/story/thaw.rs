use std::path::Path;
use keel::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use keel::application::process_manager::{DomainProcessManager, LiveProcessActionExecutor};
/// Thaw command - move story from icebox to backlog

use std::sync::Arc;

use anyhow::Result;
use keel::infrastructure::storage::filesystem::FileSystemAdapter;

use super::guidance::{StoryLifecycleAction, error_with_recovery};
use keel::application::story_lifecycle::StoryLifecycleService;

/// Run the thaw command
pub fn run(board_dir: &Path, id: &str) -> Result<()> {
    let adapter = Arc::new(FileSystemAdapter::new(board_dir));
    let voyage_service = Arc::new(VoyageEpicLifecycleService::new(
        board_dir.to_path_buf(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
    ));
    let executor = LiveProcessActionExecutor::new(voyage_service.clone());
    let process_manager = Arc::new(DomainProcessManager::new(executor));
    
    let service = StoryLifecycleService::new(
        board_dir.to_path_buf(),
        adapter.clone(),
        adapter,
        process_manager,
    );

    

    

    service.thaw( id)
        .map_err(|err| error_with_recovery(StoryLifecycleAction::Thaw, id, err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::StoryState;
    use keel::test_helpers::{TestBoardBuilder, TestStory};
    use keel::infrastructure::storage::filesystem::FileSystemAdapter;
    use std::sync::Arc;
    use std::fs;

    #[test]
    fn thaw_moves_story_to_backlog() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Frozen Story")
                    .status(StoryState::Icebox),
            )
            .build();

        run(temp.path(), "0001").unwrap();

        // Story bundle should exist
        let story_path = temp.path().join("stories/0001/README.md");
        assert!(story_path.exists());

        // Status should be updated to backlog
        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: backlog"));
    }

    #[test]
    fn thaw_updates_frontmatter() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Frozen Story")
                    .status(StoryState::Icebox),
            )
            .build();

        run(temp.path(), "0001").unwrap();

        let content = fs::read_to_string(temp.path().join("stories/0001/README.md")).unwrap();

        assert!(content.contains("status: backlog"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn thaw_errors_on_not_in_icebox() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0002")
                    .title("Pending Story")
                    .status(StoryState::Backlog),
            )
            .build();

        let result = run(temp.path(), "0002");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Cannot thaw"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story show 0002"));
    }

    #[test]
    fn thaw_errors_on_not_found() {
        let temp = TestBoardBuilder::new().build();

        let result = run(temp.path(), "NONEXISTENT");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    fn thaw_flat_updates_frontmatter_without_moving() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsAAA")
                    .title("Frozen Story")
                    .status(StoryState::Icebox),
            )
            .build();

        run(temp.path(), "1vkqtsAAA").unwrap();

        // Frontmatter should be updated in the bundle README
        let story_path = temp.path().join("stories/1vkqtsAAA/README.md");
        assert!(story_path.exists(), "Story bundle README should exist");

        let content = fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: backlog"));
        assert!(content.contains("updated_at:"));
    }

    #[test]
    fn thaw_flat_errors_on_not_in_icebox() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("1vkqtsBBB")
                    .title("Pending Story")
                    .status(StoryState::Backlog),
            )
            .build();

        let result = run(temp.path(), "1vkqtsBBB");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot thaw"));
    }

    #[test]
    fn thaw_blocks_scoped_story_missing_srs_traceability() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("TSCOPE01")
                    .title("Scoped Frozen Story")
                    .scope("test-epic/01-draft")
                    .status(StoryState::Icebox)
                    .body("\n## Acceptance Criteria\n\n- [ ] Missing traceability"),
            )
            .build();

        let result = run(temp.path(), "TSCOPE01");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("missing SRS refs"), "Error was: {}", err);
    }

    #[test]
    fn thaw_allows_scoped_story_with_srs_traceability() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("TSCOPE02")
                    .title("Scoped Frozen Story")
                    .scope("test-epic/01-draft")
                    .status(StoryState::Icebox)
                    .body("\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Valid traceability"),
            )
            .build();

        run(temp.path(), "TSCOPE02").unwrap();

        let content = fs::read_to_string(temp.path().join("stories/TSCOPE02/README.md")).unwrap();
        assert!(content.contains("status: backlog"));
    }
}
