//! One-time migration to canonical global knowledge IDs.

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

/// Run the migration command.
pub fn run(board_dir: &Path) -> Result<()> {
    let report = crate::read_model::knowledge::migrate_legacy_knowledge_ids(board_dir)?;

    println!("{}", "Knowledge ID migration complete".green().bold());
    println!("  Files updated: {}", report.files_updated);
    println!("  IDs regenerated: {}", report.ids_regenerated);
    println!("  Knowledge entries: {}", report.knowledge_entries);
    Ok(())
}
