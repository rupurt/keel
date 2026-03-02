//! List knowledge units command

use anyhow::Result;
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::table::Table;
use crate::read_model::knowledge::scanner;

/// List knowledge units with optional filters
pub fn run(board_dir: &Path, category: Option<&str>, pending: bool) -> Result<()> {
    let mut knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    if let Some(cat) = category {
        knowledge_list = scanner::filter_by_category(knowledge_list, cat);
    }

    if pending {
        knowledge_list = scanner::filter_unapplied(knowledge_list);
    }

    if knowledge_list.is_empty() {
        println!("{}", "No knowledge units found.".yellow());
        return Ok(());
    }

    println!("{}", "Project Knowledge".bold().underline());
    println!();

    let mut table = Table::new(&["ID", "CAT", "TITLE", "SOURCE", "STATUS"]);
    for k in &knowledge_list {
        let k: &crate::read_model::knowledge::Knowledge = k;
        let status = if k.is_applied() {
            "applied".green().to_string()
        } else {
            "pending".yellow().to_string()
        };

        let source_display = format!(
            "{}:{}",
            k.source_type,
            k.source.file_name().unwrap_or_default().to_string_lossy()
        );

        table.row(&[
            &k.id.bold().to_string(),
            &k.category,
            &k.title,
            &source_display,
            &status,
        ]);
    }
    table.print();

    Ok(())
}
