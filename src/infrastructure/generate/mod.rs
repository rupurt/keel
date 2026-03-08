//! README generation for the board
//!
//! Generates markdown files for the board, epics, and voyages.

use anyhow::{Result, anyhow};

use crate::domain::model::Board;

pub mod artifact_io;
pub mod board_readme;
pub mod compliance_report;
pub mod epic_readme;
pub mod knowledge_synthesis;
pub mod sections;
pub mod voyage_artifacts;
pub mod voyage_readme;
pub mod voyage_report;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BoardArtifactSyncOptions {
    pub synthesize_knowledge_for_voyage: Option<String>,
}

/// Regenerate all persisted board artifacts from the current board snapshot.
pub fn sync_board_artifacts(board: &Board, options: &BoardArtifactSyncOptions) -> Result<()> {
    board_readme::generate(board)?;

    for epic in board.epics.values() {
        epic_readme::generate(board, epic)?;
    }

    let mut issues = Vec::new();
    for voyage in board.voyages.values() {
        let outcome = voyage_artifacts::sync(
            board,
            voyage,
            voyage_artifacts::SyncOptions {
                synthesize_knowledge: options.synthesize_knowledge_for_voyage.as_deref()
                    == Some(voyage.id()),
            },
        );
        issues.extend(outcome.issues.into_iter().map(|issue| {
            format!(
                "voyage {} {}: {}",
                voyage.id(),
                issue.artifact,
                issue.message
            )
        }));
    }

    if !issues.is_empty() {
        return Err(anyhow!(
            "failed to sync generated voyage artifacts:\n{}",
            issues.join("\n")
        ));
    }

    let history = crate::read_model::throughput_history::project_default(board);
    crate::infrastructure::throughput_history_store::save_if_changed(&board.root, &history)?;

    Ok(())
}
