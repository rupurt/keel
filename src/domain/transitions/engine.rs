#![allow(dead_code)]
//! Transition engine - apply state changes to entities

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::domain::model::StoryState;
use crate::domain::transitions::{TransitionSpec, update_frontmatter};
use crate::infrastructure::loader::load_board;

/// Result of a transition execution
#[derive(Debug)]
pub struct TransitionResult {
    pub story: crate::domain::model::Story,
    pub from: StoryState,
    pub to: StoryState,
}

/// Execute a transition on a story
pub fn execute(
    board_dir: &Path,
    story_id: &str,
    spec: &TransitionSpec,
) -> Result<TransitionResult> {
    let board = load_board(board_dir)?;
    let story = board.require_story(story_id)?;

    let from_stage = story.status;
    let to_stage = spec.to;

    // Validate transition is allowed
    if !spec.from.contains(&from_stage) {
        return Err(anyhow!(
            "Cannot {} story {} — must be in one of: {:?}",
            spec.name,
            story.id(),
            spec.from
        ));
    }

    // Check Preconditions (e.g. voyage state)
    // We reuse the state machine logic via the Preconditions trait
    use crate::domain::state_machine::story::StoryTransition;
    use crate::domain::state_machine::{Preconditions, StoryStateMachine, TransitionContext};

    let sm = StoryStateMachine::from_state(from_stage);
    let transition = match spec.name {
        "start" => Some(StoryTransition::Start),
        "submit" => Some(StoryTransition::Submit),
        "accept" => Some(StoryTransition::Accept),
        "reject" => Some(StoryTransition::Reject),
        "ice" => Some(StoryTransition::Ice),
        "thaw" => Some(StoryTransition::Thaw),
        _ => None,
    };

    if let Some(t) = transition {
        let mut ctx = TransitionContext::default();
        if let Some(voyage_id) = story.voyage()
            && let Some(voyage) = board.voyages.get(voyage_id)
        {
            ctx.voyage_state = Some(voyage.status());
        }

        sm.check_preconditions(&t, &ctx)
            .map_err(|e| anyhow!("{}", e))?;
    }

    // Read story content
    let content = fs::read_to_string(&story.path)
        .with_context(|| format!("Failed to read story: {}", story.path.display()))?;

    // Update frontmatter
    let updated_content = update_frontmatter(&content, to_stage, &spec.timestamps)?;

    // Write back to same path (Story Bundles don't move between directories)
    fs::write(&story.path, updated_content)
        .with_context(|| format!("Failed to write story: {}", story.path.display()))?;

    // Reload story to return updated version
    let updated_board = load_board(board_dir)?;
    let updated_story = updated_board.require_story(story.id())?.clone();

    Ok(TransitionResult {
        story: updated_story,
        from: from_stage,
        to: to_stage,
    })
}

/// Execute a transition with a custom validation function
pub fn execute_with_validate<F>(
    board_dir: &Path,
    story_id: &str,
    spec: &TransitionSpec,
    validate: F,
) -> Result<TransitionResult>
where
    F: FnOnce(&crate::domain::model::Story, &str) -> Result<()>,
{
    let board = load_board(board_dir)?;
    let story = board.require_story(story_id)?;

    // Validate transition is allowed
    if !spec.from.contains(&story.status) {
        return Err(anyhow!(
            "Cannot {} story {} — must be in one of: {:?}",
            spec.name,
            story.id(),
            spec.from
        ));
    }

    let content = fs::read_to_string(&story.path)
        .with_context(|| format!("Failed to read story: {}", story.path.display()))?;

    // Run custom validation
    validate(story, &content)?;

    execute(board_dir, story_id, spec)
}

/// Execute a transition and transform the story body
pub fn execute_with_body_transform<F>(
    board_dir: &Path,
    story_id: &str,
    spec: &TransitionSpec,
    transform: F,
) -> Result<TransitionResult>
where
    F: FnOnce(&str) -> Result<String>,
{
    let board = load_board(board_dir)?;
    let story = board.require_story(story_id)?;

    // Validate transition is allowed
    if !spec.from.contains(&story.status) {
        return Err(anyhow!(
            "Cannot {} story {} — must be in one of: {:?}",
            spec.name,
            story.id(),
            spec.from
        ));
    }

    let content = fs::read_to_string(&story.path)
        .with_context(|| format!("Failed to read story: {}", story.path.display()))?;

    // Update body
    let updated_body = transform(&content)?;

    // Update frontmatter
    let updated_content = update_frontmatter(&updated_body, spec.to, &spec.timestamps)?;

    // Write back
    fs::write(&story.path, updated_content)
        .with_context(|| format!("Failed to write story: {}", story.path.display()))?;

    // Reload story
    let updated_board = load_board(board_dir)?;
    let updated_story = updated_board.require_story(story.id())?.clone();

    Ok(TransitionResult {
        story: updated_story,
        from: story.status,
        to: spec.to,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transitions::transitions;
    use crate::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn start_transition_moves_to_in_progress() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001").status(StoryState::Backlog))
            .build();

        let result = execute(temp.path(), "FEAT0001", &transitions::START).unwrap();

        assert_eq!(result.from, StoryState::Backlog);
        assert_eq!(result.to, StoryState::InProgress);

        // Story bundle should exist
        let story_path = temp.path().join("stories/FEAT0001/README.md");
        assert!(story_path.exists());

        // Frontmatter should be updated
        let content = std::fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: in-progress"));
    }

    #[test]
    fn submit_transition_moves_to_ready_for_acceptance() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0002").status(StoryState::InProgress))
            .build();

        let result = execute(temp.path(), "FEAT0002", &transitions::SUBMIT).unwrap();

        assert_eq!(result.from, StoryState::InProgress);
        assert_eq!(result.to, StoryState::NeedsHumanVerification);
    }

    #[test]
    fn accept_transition_moves_to_done() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0003").status(StoryState::NeedsHumanVerification))
            .build();

        let result = execute(temp.path(), "FEAT0003", &transitions::ACCEPT).unwrap();

        assert_eq!(result.from, StoryState::NeedsHumanVerification);
        assert_eq!(result.to, StoryState::Done);
    }

    #[test]
    fn reject_transition_moves_to_rejected() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0003").status(StoryState::NeedsHumanVerification))
            .build();

        let result = execute(temp.path(), "FEAT0003", &transitions::REJECT).unwrap();

        assert_eq!(result.from, StoryState::NeedsHumanVerification);
        assert_eq!(result.to, StoryState::Rejected);
    }

    #[test]
    fn ice_transition_moves_to_icebox() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001").status(StoryState::Backlog))
            .build();

        let result = execute(temp.path(), "FEAT0001", &transitions::ICE).unwrap();

        assert_eq!(result.from, StoryState::Backlog);
        assert_eq!(result.to, StoryState::Icebox);
    }

    #[test]
    fn thaw_transition_moves_to_backlog() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0004").status(StoryState::Icebox))
            .build();

        let result = execute(temp.path(), "FEAT0004", &transitions::THAW).unwrap();

        assert_eq!(result.from, StoryState::Icebox);
        assert_eq!(result.to, StoryState::Backlog);
    }

    #[test]
    fn story_not_found_returns_error() {
        let temp = TestBoardBuilder::new().build();

        let result = execute(temp.path(), "NONEXISTENT", &transitions::START);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Story not found"));
    }

    #[test]
    fn invalid_transition_returns_error() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001").status(StoryState::Backlog))
            .build();

        // Cannot submit from backlog
        let result = execute(temp.path(), "FEAT0001", &transitions::SUBMIT);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Cannot submit"), "Error was: {}", err);
    }

    #[test]
    fn flat_structure_updates_in_place() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("1vkqtsAAA").status(StoryState::Backlog))
            .build();

        execute(temp.path(), "1vkqtsAAA", &transitions::START).unwrap();

        // Story bundle should still exist
        let story_path = temp.path().join("stories/1vkqtsAAA/README.md");
        assert!(story_path.exists(), "Story bundle README should exist");

        // Frontmatter should be updated
        let content = std::fs::read_to_string(&story_path).unwrap();
        assert!(content.contains("status: in-progress"));
    }

    #[test]
    fn test_execute_with_validate() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::InProgress))
            .build();

        // Should pass
        let result = execute_with_validate(temp.path(), "S1", &transitions::SUBMIT, |_, _| Ok(()));
        assert!(result.is_ok());

        // Should fail validation
        let result = execute_with_validate(temp.path(), "S1", &transitions::SUBMIT, |_, _| {
            Err(anyhow!("failed"))
        });
        assert!(result.is_err());

        // Should fail if transition not allowed
        let result = execute_with_validate(temp.path(), "S1", &transitions::START, |_, _| Ok(()));
        assert!(result.is_err());
    }

    #[test]
    fn test_execute_with_body_transform() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::InProgress))
            .build();

        let result = execute_with_body_transform(temp.path(), "S1", &transitions::SUBMIT, |body| {
            Ok(format!("{}TRANSFORMED", body))
        })
        .unwrap();

        assert!(result.story.id() == "S1");
        // Reload and check body
        let content = std::fs::read_to_string(&result.story.path).unwrap();
        assert!(content.contains("TRANSFORMED"));
    }
}
