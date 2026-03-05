//! List knowledge units command

use anyhow::Result;
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::table::Table;
use crate::read_model::knowledge::{Knowledge, KnowledgeSort, KnowledgeSourceType, scanner};

const UNKNOWN_SCOPE_SEGMENT: &str = "-";
const UNKNOWN_STORY_SORT_KEY: &str = "~";

/// List knowledge units with optional filters
pub fn run(
    board_dir: &Path,
    category: Option<&str>,
    pending: bool,
    sort: KnowledgeSort,
) -> Result<()> {
    let mut knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    if let Some(cat) = category {
        knowledge_list = scanner::filter_by_category(knowledge_list, cat);
    }

    if pending {
        knowledge_list = scanner::filter_unapplied(knowledge_list);
    }

    match sort {
        KnowledgeSort::Id => {
            knowledge_list.sort_by(|a, b| a.id.cmp(&b.id));
        }
        KnowledgeSort::Story => {
            knowledge_list.sort_by(|a, b| {
                story_sort_key(a)
                    .cmp(story_sort_key(b))
                    .then_with(|| a.id.cmp(&b.id))
            });
        }
    }

    if knowledge_list.is_empty() {
        println!("{}", "No knowledge units found.".yellow());
        return Ok(());
    }

    println!("{}", "Project Knowledge".bold().underline());
    println!();

    let mut table = Table::new(&["ID", "SCOPE", "CAT", "TITLE", "SOURCE", "STATUS"]);
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
        let scope_display = format_scope_triplet(k);

        table.row(&[
            &k.id.bold().to_string(),
            &scope_display,
            &k.category,
            &k.title,
            &source_display,
            &status,
        ]);
    }
    table.print();

    Ok(())
}

fn format_scope_triplet(k: &Knowledge) -> String {
    let (epic, voyage) = scope_pair(k);
    let story = normalized_story_id(k);
    if let Some(story_id) = story {
        format!(
            "{}/{}/{}",
            style_segment(epic, crate::cli::style::styled_epic_id),
            style_segment(voyage, crate::cli::style::styled_voyage_id),
            style_segment(story_id, crate::cli::style::styled_story_id)
        )
    } else {
        format!(
            "{}/{}",
            style_segment(epic, crate::cli::style::styled_epic_id),
            style_segment(voyage, crate::cli::style::styled_voyage_id)
        )
    }
}

fn scope_pair(k: &Knowledge) -> (&str, &str) {
    if let Some(scope) = k.scope.as_deref() {
        return split_scope(scope);
    }

    if k.source_type == KnowledgeSourceType::Voyage
        && let Some((epic, voyage)) = derive_voyage_scope_from_path(&k.source)
    {
        return (epic, voyage);
    }

    ("-", "-")
}

fn split_scope(scope: &str) -> (&str, &str) {
    let mut parts = scope.split('/');
    let epic = parts.next().unwrap_or(UNKNOWN_SCOPE_SEGMENT).trim();
    let voyage = parts.next().unwrap_or(UNKNOWN_SCOPE_SEGMENT).trim();
    let epic = if epic.is_empty() {
        UNKNOWN_SCOPE_SEGMENT
    } else {
        epic
    };
    let voyage = if voyage.is_empty() {
        UNKNOWN_SCOPE_SEGMENT
    } else {
        voyage
    };
    (epic, voyage)
}

fn derive_voyage_scope_from_path(path: &Path) -> Option<(&str, &str)> {
    let parts: Vec<&str> = path.iter().filter_map(|segment| segment.to_str()).collect();
    let epics_idx = parts.iter().position(|segment| *segment == "epics")?;
    let voyages_idx = parts[epics_idx + 1..]
        .iter()
        .position(|segment| *segment == "voyages")
        .map(|relative| epics_idx + 1 + relative)?;
    let epic = parts.get(epics_idx + 1).copied()?;
    let voyage = parts.get(voyages_idx + 1).copied()?;
    Some((epic, voyage))
}

fn story_sort_key(k: &Knowledge) -> &str {
    normalized_story_id(k).unwrap_or(UNKNOWN_STORY_SORT_KEY)
}

fn normalized_story_id(k: &Knowledge) -> Option<&str> {
    k.source_story_id
        .as_deref()
        .filter(|id| !id.trim().is_empty())
}

fn style_segment(value: &str, style_fn: fn(&str) -> String) -> String {
    if value == UNKNOWN_SCOPE_SEGMENT {
        value.dimmed().to_string()
    } else {
        style_fn(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn sample_knowledge() -> Knowledge {
        Knowledge {
            id: "1abcDEF23".to_string(),
            source: PathBuf::from(".keel/stories/1storyID9/REFLECT.md"),
            source_type: KnowledgeSourceType::Story,
            scope: Some("1epicID99/1voyage88".to_string()),
            source_story_id: Some("1storyID9".to_string()),
            title: "Sample".to_string(),
            category: "code".to_string(),
            context: String::new(),
            insight: "Insight".to_string(),
            suggested_action: String::new(),
            applies_to: String::new(),
            applied: String::new(),
            observed_at: None,
            score: 0.8,
            confidence: 0.9,
            linked_ids: Vec::new(),
            similar_to: None,
            similarity_score: None,
        }
    }

    #[test]
    fn format_scope_triplet_uses_scope_and_story_id() {
        let formatted = format_scope_triplet(&sample_knowledge());
        assert!(formatted.contains("1epicID99"));
        assert!(formatted.contains("1voyage88"));
        assert!(formatted.contains("1storyID9"));
    }

    #[test]
    fn format_scope_triplet_derives_voyage_scope_from_source_path() {
        let mut knowledge = sample_knowledge();
        knowledge.scope = None;
        knowledge.source_story_id = Some(String::new());
        knowledge.source_type = KnowledgeSourceType::Voyage;
        knowledge.source = PathBuf::from(".keel/epics/1epicID99/voyages/1voyage88/KNOWLEDGE.md");

        let formatted = format_scope_triplet(&knowledge);
        assert!(formatted.contains("1epicID99"));
        assert!(formatted.contains("1voyage88"));
        assert!(!formatted.ends_with('/'));
        assert_eq!(formatted.matches('/').count(), 1);
    }

    #[test]
    fn format_scope_triplet_falls_back_to_placeholders_when_unscoped() {
        let mut knowledge = sample_knowledge();
        knowledge.scope = None;
        knowledge.source_story_id = None;
        knowledge.source_type = KnowledgeSourceType::Adhoc;
        knowledge.source = PathBuf::from("knowledge/global.md");

        let formatted = format_scope_triplet(&knowledge);
        assert_eq!(formatted.matches('-').count(), 2);
        assert!(formatted.contains("/"));
    }
}
