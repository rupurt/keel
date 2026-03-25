use keel::application::domain_events::DomainEvent;
use keel::application::process_manager::{DomainProcessManager, LiveProcessActionExecutor};
use keel::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use keel::infrastructure::loader::load_board;
use keel::infrastructure::storage::filesystem::FileSystemAdapter;
/// Done voyage command
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

/// Run the complete voyage command
pub fn run(
    board_dir: &Path,
    id: &str,
    well: Option<String>,
    hard: Option<String>,
    different: Option<String>,
) -> Result<()> {
    let adapter = Arc::new(FileSystemAdapter::new(board_dir));
    let board = load_board(board_dir)?;
    let voyage = board.require_voyage(id)?;
    let epic_id = voyage.epic_id.clone();

    let service = Arc::new(VoyageEpicLifecycleService::new(
        board_dir.to_path_buf(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
        adapter.clone(),
    ));
    let process_manager =
        DomainProcessManager::new(LiveProcessActionExecutor::new(service.clone()));

    service.complete_voyage(id, well, hard, different)?;
    process_manager.handle(
        board_dir,
        DomainEvent::VoyageCompleted {
            voyage_id: id.to_string(),
            epic_id,
        },
    )?;

    crate::cli::presentation::audio::play(crate::cli::presentation::audio::SoundEvent::VoyageDone);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};

    #[test]
    fn test_complete_voyage_updates_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("in-progress"))
            .build();

        run(temp.path(), "v1", None, None, None).unwrap();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        assert_eq!(voyage.status(), keel::domain::model::VoyageState::Done);
    }
}
