//! Reopen epic command

use std::path::Path;

use crate::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use crate::infrastructure::config::find_board_dir;
use anyhow::Result;

/// Run the reopen-epic command
pub fn run(id: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Run the reopen-epic command with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    VoyageEpicLifecycleService::reopen_epic(board_dir, id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic};
    use std::fs;

    #[test]
    fn reopen_epic_updates_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("done-epic").status("done"))
            .build();

        let epic_path = temp.path().join("epics/done-epic/README.md");
        let original = fs::read_to_string(&epic_path).unwrap();
        let with_completed_at = original.replace(
            "status: done\n",
            "status: done\ncompleted_at: 2026-01-01T00:00:00\n",
        );
        fs::write(&epic_path, with_completed_at).unwrap();

        run_with_dir(temp.path(), "done-epic").unwrap();

        let content = fs::read_to_string(epic_path).unwrap();

        assert!(content.contains("status: strategic"));
        assert!(!content.contains("completed_at:"));
        assert!(!content.contains("\ncompleted:"));
    }

    #[test]
    fn reopen_epic_errors_on_not_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("active-epic").status("tactical"))
            .build();

        let result = run_with_dir(temp.path(), "active-epic");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("is not done"));
    }

    #[test]
    fn reopen_epic_errors_on_not_found() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .build();

        let result = run_with_dir(temp.path(), "nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
