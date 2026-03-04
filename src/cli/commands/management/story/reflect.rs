//! Reflect command - scaffold REFLECT.md for a story bundle

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::domain::model::StoryState;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::template_rendering;

use super::guidance::{StoryLifecycleAction, guidance_for_action, print_human};

/// Run the reflect command
pub fn run(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let story = board.require_story(id)?;

    if matches!(story.stage, StoryState::Backlog | StoryState::Icebox) {
        bail!(
            "Cannot create REFLECT.md for story {} in {} stage. Move it into active work first.",
            story.id(),
            story.stage
        );
    }

    let story_bundle_dir = story.path.parent().with_context(|| {
        format!(
            "Story {} has no parent bundle directory at {}",
            story.id(),
            story.path.display()
        )
    })?;
    let reflect_path = story_bundle_dir.join("REFLECT.md");

    if reflect_path.exists() {
        bail!("REFLECT.md already exists for story {}", story.id());
    }

    let content = template_rendering::render(
        crate::infrastructure::templates::story::REFLECT,
        &[("id", story.id()), ("title", story.title())],
    );
    fs::write(&reflect_path, content).with_context(|| {
        format!(
            "Failed to write reflection template: {}",
            reflect_path.display()
        )
    })?;

    println!("Created: stories/{}/REFLECT.md", story.id());
    let guidance = guidance_for_action(StoryLifecycleAction::Reflect, story.stage, story.id());
    print_human(guidance.as_ref());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn reflect_creates_template_for_in_progress_story() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("SREF01")
                    .title("Reflect Story")
                    .stage(StoryState::InProgress),
            )
            .build();
        let reflect_path = temp.path().join("stories/SREF01/REFLECT.md");
        fs::remove_file(&reflect_path).unwrap();

        run(temp.path(), "SREF01").unwrap();

        assert!(reflect_path.exists(), "REFLECT.md should be created");

        let content = fs::read_to_string(reflect_path).unwrap();
        assert!(content.contains("# Reflection - Reflect Story"));
        assert!(content.contains("### L001:"));
    }

    #[test]
    fn reflect_rejects_backlog_story() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("SREF02")
                    .title("Backlog Story")
                    .stage(StoryState::Backlog),
            )
            .build();

        let err = run(temp.path(), "SREF02").unwrap_err().to_string();
        assert!(err.contains("Cannot create REFLECT.md"));
        assert!(err.contains("backlog"));
    }

    #[test]
    fn reflect_rejects_when_reflect_file_already_exists() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("SREF03")
                    .title("Existing Reflect Story")
                    .stage(StoryState::InProgress),
            )
            .build();
        let reflect_path = temp.path().join("stories/SREF03/REFLECT.md");
        fs::write(&reflect_path, "# Existing").unwrap();

        let err = run(temp.path(), "SREF03").unwrap_err().to_string();
        assert!(err.contains("REFLECT.md already exists"));
    }
}
