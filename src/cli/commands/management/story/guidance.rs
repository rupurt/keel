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
    Reject,
    Ice,
    Thaw,
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

/// Build canonical recovery guidance for a lifecycle action failure.
pub fn recovery_for_error(
    action: StoryLifecycleAction,
    story_id: &str,
    message: &str,
) -> Option<CanonicalGuidance> {
    render_command_guidance(
        recovery_command_for_error(action, story_id, message).map(CommandGuidance::recovery),
    )
}

/// Wrap an error message with deterministic recovery guidance when available.
pub fn error_with_recovery(
    action: StoryLifecycleAction,
    story_id: &str,
    error: anyhow::Error,
) -> anyhow::Error {
    let message = error.to_string();
    let guidance = recovery_for_error(action, story_id, &message);
    let rendered = render_human(guidance.as_ref());
    if rendered.is_empty() {
        anyhow::anyhow!(message)
    } else {
        anyhow::anyhow!("{message}{rendered}")
    }
}

fn recovery_command_for_error(
    action: StoryLifecycleAction,
    story_id: &str,
    message: &str,
) -> Option<String> {
    let lower = message.to_ascii_lowercase();

    if lower.contains("not found") {
        return Some("keel story list".to_string());
    }

    match action {
        StoryLifecycleAction::Start => {
            if lower.contains("board state has changed") {
                Some("keel status".to_string())
            } else if lower.contains("cannot start") {
                Some(format!("keel story show {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Submit => {
            if lower.contains("reflect.md missing in bundle") {
                Some(format!("keel story reflect {story_id}"))
            } else if lower.contains("unchecked acceptance criteria")
                || lower.contains("missing verification annotations")
                || lower.contains("missing srs refs")
                || lower.contains("missing evidence chain phase markers")
                || lower.contains("evidence directory missing in bundle")
                || lower.contains("verification failed")
                || lower.contains("unresolved scaffold/default text")
            {
                Some(format!("keel verify run {story_id}"))
            } else if lower.contains("cannot submit") {
                Some(format!("keel story show {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Accept => {
            if lower.contains("manual acceptance criteria") || lower.contains("use --human") {
                Some(format!("keel story accept {story_id} --human"))
            } else if lower.contains("reflect.md missing in bundle") {
                Some(format!("keel story reflect {story_id}"))
            } else if lower.contains("evidence directory missing in bundle")
                || lower.contains("unresolved scaffold/default text")
            {
                Some(format!("keel verify run {story_id}"))
            } else if lower.contains("cannot accept") {
                Some(format!("keel story show {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Reject => {
            if lower.contains("cannot reject") {
                Some(format!("keel story show {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Ice => {
            if lower.contains("cannot ice") {
                Some(format!("keel story show {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Thaw => {
            if lower.contains("cannot thaw") {
                Some(format!("keel story show {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Reflect => {
            if lower.contains("in backlog status") {
                Some(format!("keel story start {story_id}"))
            } else if lower.contains("in icebox status") {
                Some(format!("keel story thaw {story_id}"))
            } else if lower.contains("reflect.md already exists") {
                Some(format!("keel story submit {story_id}"))
            } else {
                None
            }
        }
        StoryLifecycleAction::Record => {
            if lower.contains("no acceptance criteria with verify annotations")
                || lower.contains("ac index")
                || lower.contains("no command specified")
            {
                Some(format!("keel story show {story_id}"))
            } else if lower.contains("command failed with exit code") {
                Some(format!("keel verify run {story_id}"))
            } else {
                None
            }
        }
    }
}

/// Render the human-readable next/recovery step block when guidance is present.
pub fn render_human(guidance: Option<&CanonicalGuidance>) -> String {
    if let Some(command) = guidance
        .and_then(|g| g.next_step.as_ref())
        .map(|step| step.command.as_str())
    {
        return format!("\nNext step:\n  {}\n", command.bold());
    }

    if let Some(command) = guidance
        .and_then(|g| g.recovery_step.as_ref())
        .map(|step| step.command.as_str())
    {
        return format!("\nRecovery step:\n  {}\n", command.bold());
    }

    String::new()
}

/// Print human-readable next-step guidance when available.
pub fn print_human(guidance: Option<&CanonicalGuidance>) {
    let rendered = render_human(guidance);
    if !rendered.is_empty() {
        print!("{rendered}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use serde_json::json;

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
        let rendered = render_human(Some(&guidance));
        assert!(rendered.contains("Next step:"));
        assert!(rendered.contains("keel story submit S1"));
    }

    #[test]
    fn guidance_serializes_with_canonical_next_step_shape() {
        let guidance =
            guidance_for_action(StoryLifecycleAction::Start, StoryState::InProgress, "S1").unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel story submit S1" }
            })
        );
    }

    #[test]
    fn recovery_not_found_maps_to_story_list() {
        let guidance = recovery_for_error(
            StoryLifecycleAction::Submit,
            "S404",
            "Story not found: S404",
        )
        .unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel story list" }
            })
        );
    }

    #[test]
    fn recovery_accept_manual_maps_to_human_flag() {
        let guidance = recovery_for_error(
            StoryLifecycleAction::Accept,
            "S6",
            "Story S6 has manual acceptance criteria. Please use --human to confirm manual verification.",
        )
        .unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel story accept S6 --human" }
            })
        );
    }

    #[test]
    fn recovery_submit_missing_reflect_maps_to_reflect_command() {
        let guidance = recovery_for_error(
            StoryLifecycleAction::Submit,
            "S7",
            "Cannot submit story S7:\n- REFLECT.md missing in bundle",
        )
        .unwrap();
        let rendered = render_human(Some(&guidance));
        assert!(rendered.contains("Recovery step:"));
        assert!(rendered.contains("keel story reflect S7"));
    }

    #[test]
    fn error_with_recovery_embeds_human_recovery_block() {
        let err = error_with_recovery(
            StoryLifecycleAction::Accept,
            "S8",
            anyhow!("Cannot accept story S8:\n- Story S8 has manual acceptance criteria"),
        )
        .to_string();

        assert!(err.contains("Cannot accept story S8"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story accept S8 --human"));
    }
}
