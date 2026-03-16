//! Pacemaker integrity checks (heartbeat)

use crate::domain::model::Board;
use crate::infrastructure::validation::{CheckId, Problem, Severity};

/// Check if the system heartbeat is stable (committed)
pub fn check_pacemaker_stability(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Some(hb) = &board.heartbeat {
        if hb.is_dirty {
            problems.push(Problem {
                severity: Severity::Warning,
                path: hb.path.clone(),
                message: "Pacemaker has uncommitted energy. Commit .keel/heartbeat to stabilize the board.".to_string(),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    } else {
        problems.push(Problem {
            severity: Severity::Error,
            path: board.root.join("heartbeat"),
            message: "Pacemaker missing. System cannot be energized without .keel/heartbeat.".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}
