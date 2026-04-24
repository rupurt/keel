//! Mission log command

use std::fs;
use std::path::Path;

use anyhow::Result;

use keel::domain::model::Board;
use keel::infrastructure::loader::load_board;
use keel::read_model::show_selector::{ShowEntityKind, resolve_show_selector};

/// Show the mission log for a mission with an explicit board directory.
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let content = read_log_with_dir(board_dir, id)?;
    print!("{content}");
    if !content.ends_with('\n') {
        println!();
    }
    Ok(())
}

/// Read the raw mission log content for a mission.
pub fn read_log_with_dir(board_dir: &Path, id: &str) -> Result<String> {
    let board = load_board(board_dir)?;
    let resolved_id = resolve_mission_id_with_board(board_dir, &board, id)?;
    let mission = board.require_mission(&resolved_id)?;
    let log_path = mission.path.parent().unwrap().join("LOG.md");
    Ok(fs::read_to_string(log_path)?)
}

/// Resolve a mission ID or HEAD selector for mission-log operations.
pub fn resolve_mission_id(board_dir: &Path, id: &str) -> Result<String> {
    let board = load_board(board_dir)?;
    resolve_mission_id_with_board(board_dir, &board, id)
}

fn resolve_mission_id_with_board(board_dir: &Path, board: &Board, id: &str) -> Result<String> {
    Ok(resolve_show_selector(
        board_dir,
        board,
        ShowEntityKind::Mission,
        id,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::application::mission_lifecycle::MissionLifecycleService;
    use keel::test_helpers::{TestBoardBuilder, TestMission};

    #[test]
    fn read_log_with_dir_returns_full_log_contents() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One"))
            .build();

        MissionLifecycleService::log(temp.path(), "M1", "First entry").unwrap();
        MissionLifecycleService::log(temp.path(), "M1", "Second entry").unwrap();

        let log = read_log_with_dir(temp.path(), "M1").unwrap();

        assert!(log.contains("# Mission One - Decision Log"));
        assert!(log.contains("First entry"));
        assert!(log.contains("Second entry"));
    }

    #[test]
    fn resolve_mission_id_accepts_head_selectors() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M2").title("Mission Two"))
            .mission(TestMission::new("M1").title("Mission One"))
            .build();

        assert_eq!(resolve_mission_id(temp.path(), "HEAD").unwrap(), "M1");
        assert_eq!(resolve_mission_id(temp.path(), "HEAD~").unwrap(), "M2");
    }
}
