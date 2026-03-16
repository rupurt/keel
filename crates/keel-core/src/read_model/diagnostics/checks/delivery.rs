//! Delivery subsystem health checks

use crate::domain::model::{Board, StoryState};
use crate::infrastructure::validation::{CheckId, Problem, Severity};

/// Check for delivery liquidity (ready stories)
pub fn check_delivery_liquidity(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();
    let ready_count = board
        .stories
        .values()
        .filter(|s| s.status == StoryState::Backlog)
        .count();

    if ready_count == 0 {
        problems.push(Problem {
            severity: Severity::Warning,
            path: board.root.join("stories"),
            message: "Zero delivery liquidity. No ready stories in the backlog to sustain the 9-5 window.".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}

/// Check for blocked delivery capacity
pub fn check_delivery_blockages(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();
    let blocked_count = board
        .stories
        .values()
        .filter(|s| s.status == StoryState::InProgress && !s.frontmatter.blocked_by.is_empty())
        .count();

    if blocked_count > 3 {
        problems.push(Problem {
            severity: Severity::Warning,
            path: board.root.join("stories"),
            message: format!(
                "High delivery friction: {} stories are currently blocked in-progress.",
                blocked_count
            ),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}
