//! Prune duplicate knowledge and rewrite canonical catalog files.

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

/// Run the prune command.
pub fn run(board_dir: &Path) -> Result<()> {
    let catalog = crate::read_model::knowledge::prune_knowledge_catalog(board_dir)?;

    println!("{}", "Knowledge prune complete".green().bold());
    println!("  Active knowledge entries: {}", catalog.units.len());
    println!(
        "  Catalog directory: {}",
        board_dir.join("knowledge").display()
    );

    Ok(())
}
