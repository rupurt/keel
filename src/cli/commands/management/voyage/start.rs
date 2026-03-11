use keel::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use keel::infrastructure::storage::filesystem::FileSystemAdapter;
/// Start voyage command
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

/// Run the start voyage command
pub fn run(board_dir: &Path, id: &str, force: bool, expect_version: Option<u64>) -> Result<()> {
    let adapter = Arc::new(FileSystemAdapter::new(board_dir));
    let service = VoyageEpicLifecycleService::new(
        board_dir.to_path_buf(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
    );

    service.start_voyage(id, force, expect_version)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};

    #[test]
    fn test_start_voyage_updates_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned"))
            .build();

        run(temp.path(), "v1", false, None).unwrap();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        assert_eq!(
            voyage.status(),
            keel::domain::model::VoyageState::InProgress
        );
    }
}
