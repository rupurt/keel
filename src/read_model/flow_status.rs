//! Canonical flow/status projection service.
//!
//! This read model centralizes board operational metrics so `flow`, `status`,
//! and `next` consume one deterministic projection source.

use crate::cli::presentation::flow::metrics::{FlowMetrics, calculate_metrics};
use crate::domain::model::Board;

/// Combined operational projection for flow and status consumers.
#[derive(Debug)]
pub struct FlowStatusProjection {
    pub flow: FlowMetrics,
    pub status: StatusMetrics,
}

/// Status-oriented projection used by the status adapter.
#[derive(Debug)]
pub struct StatusMetrics {
    pub execution: StatusExecutionMetrics,
    pub planning: StatusPlanningMetrics,
    pub research: StatusResearchMetrics,
    pub verification: StatusVerificationMetrics,
}

#[derive(Debug)]
pub struct StatusExecutionMetrics {
    pub backlog_count: usize,
    pub in_progress_count: usize,
    pub total_count: usize,
}

#[derive(Debug)]
pub struct StatusPlanningMetrics {
    pub draft_count: usize,
    pub planned_count: usize,
}

#[derive(Debug)]
pub struct StatusResearchMetrics {
    pub exploring_count: usize,
    pub evaluating_count: usize,
    pub parked_count: usize,
}

#[derive(Debug)]
pub struct StatusVerificationMetrics {
    pub count: usize,
    pub average_age_days: f64,
}

/// Build the canonical flow/status projection from a board snapshot.
pub fn project(board: &Board) -> FlowStatusProjection {
    let flow = calculate_metrics(board);

    let status = StatusMetrics {
        execution: StatusExecutionMetrics {
            backlog_count: flow.execution.backlog_count,
            in_progress_count: flow.execution.in_progress_count,
            total_count: board.stories.len(),
        },
        planning: StatusPlanningMetrics {
            draft_count: flow.planning.draft_count,
            planned_count: flow.planning.planned_count,
        },
        research: StatusResearchMetrics {
            exploring_count: flow.research.exploring_count,
            evaluating_count: flow.research.surveying_count + flow.research.assessing_count,
            parked_count: flow.research.parked_count,
        },
        verification: StatusVerificationMetrics {
            count: flow.verification.count,
            average_age_days: flow.verification.avg_age_days,
        },
    };

    FlowStatusProjection { flow, status }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn project_exposes_flow_and_status_views() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").stage(StoryState::Backlog))
            .story(TestStory::new("S2").stage(StoryState::InProgress))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = project(&board);
        assert_eq!(projection.flow.execution.backlog_count, 1);
        assert_eq!(projection.flow.execution.in_progress_count, 1);
        assert_eq!(projection.status.execution.total_count, 2);
    }
}
