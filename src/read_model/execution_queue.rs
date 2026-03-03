//! Shared execution-queue classification for flow and capacity projections.
//!
//! This module keeps "ready vs blocked" semantics deterministic across
//! renderers and selectors by combining:
//! - canonical story workability invariants (scope/voyage readiness)
//! - derived implementation dependency readiness

use std::collections::HashMap;

use crate::domain::model::{Board, Story, StoryState};
use crate::read_model::traceability::derive_implementation_dependencies;

/// Backlog readiness state for an implementation story.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BacklogQueueState {
    Ready,
    Blocked,
}

/// Classify a backlog story as ready or blocked for agent implementation.
pub fn classify_backlog_story(
    board: &Board,
    story: &Story,
    dependencies: &HashMap<String, Vec<String>>,
) -> BacklogQueueState {
    if story.stage != StoryState::Backlog {
        return BacklogQueueState::Blocked;
    }

    let workable =
        crate::domain::state_machine::invariants::story_workable(story, board, &board.root);
    if !workable {
        return BacklogQueueState::Blocked;
    }

    let blocked_by_dependencies = dependencies.get(story.id()).is_some_and(|dep_ids| {
        dep_ids.iter().any(|dep_id| {
            board
                .stories
                .get(dep_id)
                .is_some_and(|dep_story| dep_story.stage != StoryState::Done)
        })
    });

    if blocked_by_dependencies {
        BacklogQueueState::Blocked
    } else {
        BacklogQueueState::Ready
    }
}

/// Count ready and blocked backlog stories using canonical queue semantics.
pub fn backlog_queue_counts(board: &Board) -> (usize, usize) {
    let dependencies = derive_implementation_dependencies(board);
    let mut ready_count = 0;
    let mut blocked_count = 0;

    for story in board
        .stories
        .values()
        .filter(|story| story.stage == StoryState::Backlog)
    {
        match classify_backlog_story(board, story, &dependencies) {
            BacklogQueueState::Ready => ready_count += 1,
            BacklogQueueState::Blocked => blocked_count += 1,
        }
    }

    (ready_count, blocked_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn backlog_queue_counts_treats_draft_voyage_backlog_as_blocked() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("draft")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1"))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let (ready, blocked) = backlog_queue_counts(&board);
        assert_eq!(ready, 0);
        assert_eq!(blocked, 1);
    }

    #[test]
    fn backlog_queue_counts_treats_planned_voyage_backlog_as_ready_when_unblocked() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .status("planned")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS"),
            )
            .story(TestStory::new("S1").scope("e1/v1"))
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let (ready, blocked) = backlog_queue_counts(&board);
        assert_eq!(ready, 1);
        assert_eq!(blocked, 0);
    }
}
