//! Link command - link story to voyage

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::infrastructure::config::find_board_dir;
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;

/// Run the link command
pub fn run(story_id: &str, voyage_id: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, story_id, voyage_id)
}

/// Run the link command with an explicit board directory
pub fn run_with_dir(board_dir: &Path, story_id: &str, voyage_id: &str) -> Result<()> {
    let board = load_board(board_dir)?;

    // Find the story
    let story = board.require_story(story_id)?;

    // Find the voyage
    let voyage = board.require_voyage(voyage_id)?;

    // Calculate new scope
    let new_scope = voyage.scope_path();

    // Check if already linked
    if story.frontmatter.scope.as_deref() == Some(&new_scope) {
        return Err(anyhow!(
            "Story {} is already linked to {}",
            story.id(),
            new_scope
        ));
    }

    // Read and update content
    let content = fs::read_to_string(&story.path)
        .with_context(|| format!("Failed to read story: {}", story.path.display()))?;

    let updated_content = update_scope(&content, &new_scope)?;

    // Write back
    fs::write(&story.path, updated_content)
        .with_context(|| format!("Failed to write story: {}", story.path.display()))?;

    println!("Linked: {} -> {}", story.id(), new_scope);

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

/// Update or add scope field in frontmatter
fn update_scope(content: &str, new_scope: &str) -> Result<String> {
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    Ok(apply(
        content,
        &[
            Mutation::set("scope", new_scope),
            Mutation::set("updated_at", now),
        ],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn link_adds_scope() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("planned"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Test Story")
                    .stage(crate::domain::model::StoryState::Backlog),
            )
            .build();

        run_with_dir(temp.path(), "FEAT0001", "01-first").unwrap();

        let content = fs::read_to_string(temp.path().join("stories/FEAT0001/README.md")).unwrap();

        assert!(content.contains("scope: test-epic/01-first"));
    }

    #[test]
    fn link_errors_on_missing_story() {
        let temp = TestBoardBuilder::new().build();

        let result = run_with_dir(temp.path(), "NONEXISTENT", "01-first");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Story not found"));
    }

    #[test]
    fn link_errors_on_missing_voyage() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001"))
            .build();

        let result = run_with_dir(temp.path(), "FEAT0001", "nonexistent");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Voyage not found"));
    }
}
