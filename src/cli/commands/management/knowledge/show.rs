//! Show detailed knowledge unit command

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::style;
use crate::read_model::knowledge::scanner;

/// Show detailed information for a single knowledge unit
pub fn run(board_dir: &Path, id: &str) -> Result<()> {
    let knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    let k = knowledge_list
        .iter()
        .find(|k| k.id == id)
        .ok_or_else(|| anyhow!("Knowledge unit not found: {}", id))?;

    let width = crate::cli::presentation::terminal::get_terminal_width();
    println!("{}", style::heavy_rule(width, None));
    println!("{}", style::header(&k.id, &k.title, style::styled_story_id));
    println!("{}", style::heavy_rule(width, None));
    println!();

    println!("{:<15} {}", "Category:".bold(), k.category);
    println!(
        "{:<15} {}:{}",
        "Source:".bold(),
        k.source_type,
        k.source.display()
    );

    if let Some(scope) = &k.scope {
        println!("{:<15} {}", "Scope:".bold(), scope);
    }

    let status = if k.is_applied() {
        "applied".green().bold().to_string()
    } else {
        "pending".yellow().bold().to_string()
    };
    println!("{:<15} {}", "Status:".bold(), status);
    println!();

    println!("{}", "Context:".bold());
    println!("{}", k.context);
    println!();

    println!("{}", "Insight:".bold().cyan());
    println!("{}", k.insight);
    println!();

    println!("{}", "Suggested Action:".bold().green());
    println!("{}", k.suggested_action);
    println!();

    println!("{}", "Applies To:".bold());
    println!("{}", k.applies_to);
    println!();

    if k.is_applied() {
        println!("{}", "Applied:".bold());
        println!("{}", k.applied);
    }

    Ok(())
}
