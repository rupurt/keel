//! Capacity diagnostics adapter.
//!
//! Uses the canonical read-model capacity projection and formats it for CLI.

use anyhow::Result;

use crate::cli::presentation::flow::capacity as flow_capacity;

/// Run the capacity command.
pub fn run(board_dir: &std::path::Path) -> Result<()> {
    flow_capacity::run(board_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn blocked_stories_identified_from_unmet_deps() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .stage(StoryState::InProgress)
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .stage(StoryState::Backlog)
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let cap = flow_capacity::calculate_system_capacity(&board);
        let keel_cap = cap.epics.iter().find(|e| e.id == "keel").unwrap();

        assert_eq!(keel_cap.capacity.blocked, 1);
        assert_eq!(keel_cap.capacity.ready, 0);
    }

    #[test]
    fn stories_with_all_done_deps_not_blocked() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .stage(StoryState::Done)
                    .body("- [x] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .stage(StoryState::Backlog)
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let cap = flow_capacity::calculate_system_capacity(&board);
        let keel_cap = cap.epics.iter().find(|e| e.id == "keel").unwrap();

        assert_eq!(keel_cap.capacity.blocked, 0);
        assert_eq!(keel_cap.capacity.ready, 1);
    }
}
