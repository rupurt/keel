//! Show detailed knowledge unit command

use anyhow::{Result, anyhow};
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
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
        .row("ID:", k.id.to_string())
        .row("Category:", k.category.to_string())
        .row_optional("Story:", k.source_story_id.clone())
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
    document.push_header(metadata, None);
    let mut sections = vec![context, insight, action, applies_to];

    if !k.linked_ids.is_empty() {
        let mut linked = ShowSection::new("Linked Knowledge");
        linked.push_lines([k.linked_ids.join(", ")]);
        sections.push(linked);
    }

    if let (Some(similar_to), Some(score)) = (&k.similar_to, k.similarity_score) {
        let mut similar = ShowSection::new("Nearest Similarity");
        similar.push_lines([format!("{similar_to} ({score:.2})")]);
        sections.push(similar);
    }

    if k.is_applied() {
        let mut applied = ShowSection::new("Applied");
        applied.push_lines([k.applied.to_string()]);
        sections.push(applied);
    }
    document.push_spacer();
    document.push_sections_spaced(sections);

    document.print();

    Ok(())
}
