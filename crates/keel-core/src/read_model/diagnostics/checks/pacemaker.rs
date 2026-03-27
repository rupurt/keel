//! Pacemaker integrity checks (heartbeat)

use crate::domain::model::Board;
use crate::infrastructure::validation::{CheckId, Problem, Severity};

/// Check whether an existing heartbeat is committed.
pub fn check_pacemaker_stability(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    if let Some(hb) = &board.heartbeat
        && hb.is_dirty
    {
        problems.push(Problem {
            severity: Severity::Warning,
            path: hb.path.clone(),
            message:
                "Pacemaker has uncommitted energy. Commit .keel/heartbeat to stabilize the board."
                    .to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::check_pacemaker_stability;
    use crate::domain::model::{Board, Heartbeat};
    use crate::infrastructure::validation::Severity;
    use std::path::PathBuf;
    use std::time::SystemTime;

    #[test]
    fn missing_heartbeat_is_not_a_problem() {
        let board = Board::new(PathBuf::from(".keel"));

        let problems = check_pacemaker_stability(&board);

        assert!(problems.is_empty());
    }

    #[test]
    fn dirty_heartbeat_remains_a_warning() {
        let mut board = Board::new(PathBuf::from(".keel"));
        board.heartbeat = Some(Heartbeat::new(
            PathBuf::from(".keel/heartbeat"),
            SystemTime::now(),
            true,
        ));

        let problems = check_pacemaker_stability(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Warning);
        assert!(problems[0].message.contains("uncommitted energy"));
    }
}
