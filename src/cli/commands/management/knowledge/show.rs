//! Show detailed knowledge unit command

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::read_model::knowledge::scanner;

/// Show detailed information for a single knowledge unit
pub fn run(board_dir: &Path, id: &str) -> Result<()> {
    if !crate::read_model::knowledge::is_canonical_knowledge_id(id) {
        return Err(anyhow!(
            "Knowledge show expects a canonical global ID (9-character base62): {}",
            id
        ));
    }

    let knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    let k = knowledge_list
        .iter()
        .find(|k| k.id == id)
        .ok_or_else(|| anyhow!("Knowledge unit not found: {}", id))?;

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(15)
        .with_bold_labels(true)
        .row("Title:", format!("{}", k.title.bold()))
        .row("Category:", k.category.to_string())
        .row_optional("Story:", k.source_story_id.clone())
        .row_optional("Scope:", k.scope.clone())
        .row_optional(
            "Created:",
            k.created_at
                .map(|created_at| format!("{}", created_at.dimmed())),
        );

    let status = if k.is_applied() {
        "applied".green().bold().to_string()
    } else {
        "pending".yellow().bold().to_string()
    };
    metadata.push_row("Status:", status);
    metadata.push_row(
        "Source:",
        format!("{}:{}", k.source_type, k.source.display()),
    );

    let mut context = ShowSection::new("Context");
    context.push_lines(styled_content_lines(&k.context));

    let mut insight = ShowSection::new("Insight");
    insight.push_lines(styled_content_lines(&k.insight));

    let mut action = ShowSection::new("Suggested Action");
    action.push_lines(styled_content_lines(&k.suggested_action));

    let mut applies_to = ShowSection::new("Applies To");
    applies_to.push_lines(styled_content_lines(&k.applies_to));

    let width = crate::cli::presentation::terminal::get_terminal_width();
    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    let mut sections = vec![context, insight, action, applies_to];

    if !k.linked_ids.is_empty() {
        let mut linked = ShowSection::new("Linked Knowledge");
        linked.push_lines([style::styled_inline_markdown(&k.linked_ids.join(", "))]);
        sections.push(linked);
    }

    if let (Some(similar_to), Some(score)) = (&k.similar_to, k.similarity_score) {
        let mut similar = ShowSection::new("Nearest Similarity");
        similar.push_lines([style::styled_inline_markdown(&format!(
            "{similar_to} ({score:.2})"
        ))]);
        sections.push(similar);
    }

    if k.is_applied() {
        let mut applied = ShowSection::new("Applied");
        applied.push_lines(styled_content_lines(&k.applied));
        sections.push(applied);
    }
    document.push_sections_spaced(sections);

    document.print();

    Ok(())
}

fn styled_content_lines(value: &str) -> Vec<String> {
    value.lines().map(style::styled_inline_markdown).collect()
}
