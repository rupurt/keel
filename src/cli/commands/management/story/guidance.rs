//! Canonical next-step guidance helpers for story lifecycle commands.

use owo_colors::OwoColorize;

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};
use crate::domain::model::StoryState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoryLifecycleAction {
    Start,
    Reflect,
    Record,
    Submit,
    Accept,
}

/// Build canonical guidance for a lifecycle action after it succeeds.
pub fn guidance_for_action(
    action: StoryLifecycleAction,
    resulting_state: StoryState,
    story_id: &str,
) -> Option<CanonicalGuidance> {
    if action == StoryLifecycleAction::Accept {
        // `done` is terminal in the story state machine, so no next transition exists.
        return None;
    }

    render_command_guidance(
        next_command_for_state(resulting_state, story_id).map(CommandGuidance::next),
    )
}

/// Return the canonical next lifecycle command for the given story state.
pub fn next_command_for_state(state: StoryState, story_id: &str) -> Option<String> {
    match state {
        StoryState::Backlog | StoryState::Rejected => Some(format!("keel story start {story_id}")),
        StoryState::InProgress => Some(format!("keel story submit {story_id}")),
        StoryState::NeedsHumanVerification => Some(format!("keel story accept {story_id}")),
        StoryState::Icebox => Some(format!("keel story thaw {story_id}")),
        StoryState::Done => None,
    }
}

/// Render the human-readable next-step block if guidance contains a next-step.
pub fn render_human(guidance: Option<&CanonicalGuidance>) -> Option<String> {
    let command = guidance
        .and_then(|g| g.next_step.as_ref())
        .map(|step| step.command.as_str())?;

    Some(format!("\nNext step:\n  {}\n", command.bold()))
}

/// Print human-readable next-step guidance when available.
pub fn print_human(guidance: Option<&CanonicalGuidance>) {
    if let Some(rendered) = render_human(guidance) {
        print!("{rendered}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_action_suggests_submit_for_in_progress() {
        let guidance =
            guidance_for_action(StoryLifecycleAction::Start, StoryState::InProgress, "S1").unwrap();
        assert_eq!(
            guidance.next_step.unwrap().command,
            "keel story submit S1".to_string()
        );
    }

    #[test]
    fn reflect_action_suggests_accept_for_needs_human_verification() {
        let guidance = guidance_for_action(
            StoryLifecycleAction::Reflect,
            StoryState::NeedsHumanVerification,
            "S2",
        )
        .unwrap();
        assert_eq!(
            guidance.next_step.unwrap().command,
            "keel story accept S2".to_string()
        );
    }

    #[test]
    fn record_action_suggests_start_for_rejected_story() {
        let guidance =
            guidance_for_action(StoryLifecycleAction::Record, StoryState::Rejected, "S3").unwrap();
        assert_eq!(
            guidance.next_step.unwrap().command,
            "keel story start S3".to_string()
        );
    }

    #[test]
    fn submit_action_omits_guidance_for_done_story() {
        let guidance = guidance_for_action(StoryLifecycleAction::Submit, StoryState::Done, "S4");
        assert!(guidance.is_none());
    }

    #[test]
    fn accept_action_omits_guidance() {
        let guidance = guidance_for_action(StoryLifecycleAction::Accept, StoryState::Done, "S5");
        assert!(guidance.is_none());
    }

    #[test]
    fn render_human_formats_next_step_block() {
        let guidance = CanonicalGuidance::next("keel story submit S1");
        let rendered = render_human(Some(&guidance)).unwrap();
        assert!(rendered.contains("Next step:"));
        assert!(rendered.contains("keel story submit S1"));
    }

    #[test]
    fn guidance_serializes_with_canonical_next_step_shape() {
        let guidance =
            guidance_for_action(StoryLifecycleAction::Start, StoryState::InProgress, "S1").unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(json["next_step"]["command"], "keel story submit S1");
        assert!(json.get("recovery_step").is_none());
    }
}
