//! Knowledge impact and drift analysis command

use anyhow::Result;
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::table::Table;
use crate::read_model::knowledge::scanner;

/// Run the impact command to track knowledge institutionalization
pub fn run(board_dir: &Path) -> Result<()> {
    let knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    let (applied, pending): (Vec<_>, Vec<_>) = knowledge_list
        .into_iter()
        .partition(|k: &crate::read_model::knowledge::Knowledge| k.is_applied());

    println!("{}", "Knowledge Impact & Drift Analysis".bold().underline());
    println!();

    // 1. High-level Summary
    println!("{}", "Summary:".bold());
    println!(
        "  Institutionalized:  {:>3} (Applied)",
        applied.len().green()
    );
    println!(
        "  Drift Risk:         {:>3} (Pending)",
        pending.len().yellow()
    );
    println!();

    // 2. Pending Knowledge (Drift Risk)
    if !pending.is_empty() {
        println!("{}", "Unapplied Insights (Drift Risk)".bold().yellow());
        println!(
            "These insights have been discovered but not yet institutionalized into CLAUDE.md or ADRs."
        );
        println!();

        let mut table = Table::new(&["ID", "CAT", "TITLE", "APPLIES TO"]);
        for k in &pending {
            table.row(&[
                &k.id.bold().to_string(),
                &k.category,
                &k.title,
                &k.applies_to,
            ]);
        }
        table.print();
        println!();
    }

    // 3. Recently Applied Knowledge
    if !applied.is_empty() {
        println!("{}", "Recently Institutionalized".bold().green());
        println!();

        let mut table = Table::new(&["ID", "TITLE", "RECORD OF APPLICATION"]);
        // Only show last 5 for brevity
        for k in applied.iter().take(5) {
            table.row(&[&k.id.bold().to_string(), &k.title, &k.applied]);
        }
        table.print();
        if applied.len() > 5 {
            println!("  ... and {} more", applied.len() - 5);
        }
    }

    Ok(())
}
