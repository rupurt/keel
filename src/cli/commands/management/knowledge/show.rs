//! Show detailed knowledge unit command

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::read_model::knowledge::scanner;

/// Show detailed information for a single knowledge unit
pub fn run(board_dir: &Path, id: &str) -> Result<()> {
    let knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    let k = knowledge_list
        .iter()
        .find(|k| k.id == id)
        .ok_or_else(|| anyhow!("Knowledge unit not found: {}", id))?;

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(15)
        .with_bold_labels(true)
        .row("Title:", format!("{}", k.title.bold()))
        .row("ID:", k.id.to_string())
        .row("Category:", k.category.to_string())
        .row(
            "Source:",
            format!("{}:{}", k.source_type, k.source.display()),
        )
        .row_optional("Scope:", k.scope.clone());

    let status = if k.is_applied() {
        "applied".green().bold().to_string()
    } else {
        "pending".yellow().bold().to_string()
    };
    metadata.push_row("Status:", status);

    let mut context = ShowSection::new("Context");
    context.push_lines([k.context.to_string()]);

    let mut insight = ShowSection::new("Insight");
    insight.push_lines([k.insight.to_string()]);

    let mut action = ShowSection::new("Suggested Action");
    action.push_lines([k.suggested_action.to_string()]);

    let mut applies_to = ShowSection::new("Applies To");
    applies_to.push_lines([k.applies_to.to_string()]);

    let mut document = ShowDocument::new();
    document.push_key_values(metadata);
    document.push_spacer();
    document.push_section(context);
    document.push_spacer();
    document.push_section(insight);
    document.push_spacer();
    document.push_section(action);
    document.push_spacer();
    document.push_section(applies_to);

    if k.is_applied() {
        let mut applied = ShowSection::new("Applied");
        applied.push_lines([k.applied.to_string()]);
        document.push_spacer();
        document.push_section(applied);
    }

    document.print();

    Ok(())
}
