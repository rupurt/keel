//! Show ADR command.

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::infrastructure::config::find_board_dir;
use crate::infrastructure::loader::load_board;

use super::guidance;

/// Show details for a specific ADR.
pub fn run(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let adr = board.require_adr(pattern)?;

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", adr.frontmatter.title.bold()))
        .row("ID:", adr.id().to_string())
        .row("Status:", adr.frontmatter.status.to_string())
        .row_optional("Context:", adr.frontmatter.context.clone());
    if !adr.frontmatter.applies_to.is_empty() {
        metadata.push_row("Applies:", adr.frontmatter.applies_to.join(", "));
    }
    metadata.push_optional_row(
        "Date:",
        adr.frontmatter
            .decided_at
            .map(|date| date.format("%Y-%m-%d").to_string()),
    );

    let mut sections = Vec::new();
    if !adr.frontmatter.supersedes.is_empty() || adr.frontmatter.superseded_by.is_some() {
        let mut relationships = ShowSection::new("Relationships");
        if !adr.frontmatter.supersedes.is_empty() {
            relationships.push_lines([format!(
                "  Supersedes: {}",
                adr.frontmatter.supersedes.join(", ")
            )]);
        }
        if let Some(by) = &adr.frontmatter.superseded_by {
            relationships.push_lines([format!("  Superseded by: {}", by)]);
        }
        sections.push(relationships);
    }

    let mut location = ShowSection::new("Location");
    location.push_key_values(
        ShowKeyValues::new()
            .with_indent(2)
            .with_min_label_width(9)
            .row("File:", adr.path.display().to_string()),
    );
    sections.push(location);

    let mut document = ShowDocument::new();
    document.push_header(metadata, None);
    if !sections.is_empty() {
        document.push_spacer();
        document.push_sections_spaced(sections);
    }
    document.print();
    guidance::print_human(guidance::informational_for_show().as_ref());

    Ok(())
}
