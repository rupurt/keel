//! Knowledge synthesis for voyages
//!
//! Aggregates story reflections into a voyage-level KNOWLEDGE.md file.

use crate::domain::model::{Board, Voyage};
use anyhow::{Context, Result};
use chrono::NaiveDateTime;
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

fn synthesized_knowledge_id(voyage_id: &str, story_id: &str, source_knowledge_id: &str) -> String {
    let mut hasher = DefaultHasher::new();
    voyage_id.hash(&mut hasher);
    story_id.hash(&mut hasher);
    source_knowledge_id.hash(&mut hasher);

    let raw = hasher.finish() % 13_537_086_546_263_552_u64; // 62^9
    crate::infrastructure::story_id::encode_base62(raw, 9)
}

fn voyage_knowledge_created_at(board: &Board, voyage: &Voyage) -> Option<NaiveDateTime> {
    board
        .stories_for_voyage(voyage)
        .into_iter()
        .filter_map(|story| story.frontmatter.submitted_at)
        .max()
        .or(voyage.frontmatter.completed_at)
        .or(voyage.frontmatter.started_at)
        .or(voyage.frontmatter.created_at)
}

fn render_frontmatter_datetime(value: NaiveDateTime) -> String {
    value.format("%Y-%m-%dT%H:%M:%S").to_string()
}

fn push_knowledge_table(
    output: &mut String,
    id: &str,
    title: &str,
    knowledge: &crate::read_model::knowledge::Knowledge,
    linked_knowledge_ids: Option<&str>,
    include_metrics: bool,
) {
    output.push_str(&format!("### {id}: {title}\n\n"));
    output.push_str("| Field | Value |\n");
    output.push_str("|-------|-------|\n");
    output.push_str(&format!("| **Category** | {} |\n", knowledge.category));
    output.push_str(&format!("| **Context** | {} |\n", knowledge.context));
    output.push_str(&format!("| **Insight** | {} |\n", knowledge.insight));
    output.push_str(&format!(
        "| **Suggested Action** | {} |\n",
        knowledge.suggested_action
    ));
    output.push_str(&format!("| **Applies To** | {} |\n", knowledge.applies_to));
    if let Some(linked_knowledge_ids) = linked_knowledge_ids {
        output.push_str(&format!(
            "| **Linked Knowledge IDs** | {} |\n",
            linked_knowledge_ids
        ));
    }
    if include_metrics {
        output.push_str(&format!("| **Score** | {:.2} |\n", knowledge.score));
        output.push_str(&format!(
            "| **Confidence** | {:.2} |\n",
            knowledge.confidence
        ));
    }
    output.push_str(&format!("| **Applied** | {} |\n\n", knowledge.applied));
}

/// Synthesize reflections for a voyage into a KNOWLEDGE.md file
pub fn synthesize_voyage_knowledge(board: &Board, voyage: &Voyage) -> Result<()> {
    let stories = board.stories_for_voyage(voyage);
    let created_at = voyage_knowledge_created_at(board, voyage).map(render_frontmatter_datetime);
    let mut synthesis = format!(
        "{}# Knowledge - {}

> Automated synthesis of story reflections.

",
        created_at
            .map(|value| format!("---\ncreated_at: {value}\n---\n\n"))
            .unwrap_or_default(),
        voyage.id()
    );

    let mut synthesized_units = Vec::new();
    let mut found_reflections = false;
    synthesis.push_str("## Story Knowledge\n\n");
    for story in stories {
        let story_dir = story.path.parent().unwrap();
        let reflect_path = story_dir.join("REFLECT.md");

        if reflect_path.exists() {
            let insights =
                crate::read_model::knowledge::load_reflection_knowledge(&board.root, &reflect_path)
                    .with_context(|| {
                        format!(
                            "Failed to read reflection knowledge for story {}",
                            story.id()
                        )
                    })?;
            if insights.is_empty() {
                continue;
            }

            synthesis.push_str(&format!(
                "## Story: {} ({})

",
                story.title(),
                story.id()
            ));
            for insight in insights {
                push_knowledge_table(
                    &mut synthesis,
                    &insight.id,
                    &insight.title,
                    &insight,
                    None,
                    false,
                );

                synthesized_units.push((story.id().to_string(), insight));
            }
            synthesis.push_str(
                "

---

",
            );
            found_reflections = true;
        }
    }

    if found_reflections {
        synthesis.push_str("## Synthesis\n\n");
        for (story_id, insight) in &synthesized_units {
            let synth_id = synthesized_knowledge_id(voyage.id(), story_id, &insight.id);
            push_knowledge_table(
                &mut synthesis,
                &synth_id,
                &insight.title,
                insight,
                Some(&insight.id),
                true,
            );
        }
    }

    if found_reflections {
        let voyage_dir = voyage.path.parent().unwrap();
        let knowledge_path = voyage_dir.join("KNOWLEDGE.md");
        fs::write(&knowledge_path, synthesis)
            .with_context(|| format!("Failed to write KNOWLEDGE.md for voyage {}", voyage.id()))?;
        println!("  ✓ Synthesized knowledge in {}", knowledge_path.display());
    }

    Ok(())
}
