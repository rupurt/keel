//! Canonical flow projection service.
//!
//! This read model centralizes board operational metrics so `flow` and `next`
//! consume one deterministic projection source.

use crate::domain::model::Board;
use crate::read_model::flow_metrics::{FlowMetrics, calculate_metrics};

/// Build the canonical flow projection from a board snapshot.
pub fn project(board: &Board) -> FlowMetrics {
    calculate_metrics(board)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn project_exposes_flow_metrics() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .story(TestStory::new("S2").status(StoryState::InProgress))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = project(&board);
        assert_eq!(projection.execution.backlog_count, 1);
        assert_eq!(projection.execution.in_progress_count, 1);
    }
}
