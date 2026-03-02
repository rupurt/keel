//! Knowledge synthesis for voyages
//!
//! Aggregates story reflections into a voyage-level KNOWLEDGE.md file.

use crate::domain::model::{Board, Voyage};
use anyhow::{Context, Result};
use std::fs;

/// Synthesize reflections for a voyage into a KNOWLEDGE.md file
pub fn synthesize_voyage_knowledge(board: &Board, voyage: &Voyage) -> Result<()> {
    let stories = board.stories_for_voyage(voyage);
    let mut synthesis = format!(
        "# Knowledge - {}

> Automated synthesis of story reflections.

",
        voyage.id()
    );

    let mut found_reflections = false;
    for story in stories {
        let story_dir = story.path.parent().unwrap();
        let reflect_path = story_dir.join("REFLECT.md");

        if reflect_path.exists() {
            let content = fs::read_to_string(&reflect_path)
                .with_context(|| format!("Failed to read reflection for story {}", story.id()))?;

            synthesis.push_str(&format!(
                "## Story: {} ({})

",
                story.title(),
                story.id()
            ));
            synthesis.push_str(&content);
            synthesis.push_str(
                "

---

",
            );
            found_reflections = true;
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
