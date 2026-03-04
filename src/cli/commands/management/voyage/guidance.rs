//! Canonical next-step guidance helpers for voyage lifecycle commands.

use owo_colors::OwoColorize;

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoyageLifecycleAction {
    Plan,
    Start,
    Done,
}

pub fn guidance_for_action(
    action: VoyageLifecycleAction,
    voyage_id: &str,
) -> Option<CanonicalGuidance> {
    let command = match action {
        VoyageLifecycleAction::Plan => Some(format!("keel voyage start {voyage_id}")),
        VoyageLifecycleAction::Start => Some(format!("keel voyage done {voyage_id}")),
        VoyageLifecycleAction::Done => None,
    };

    render_command_guidance(command.map(CommandGuidance::next))
}

pub fn render_human(guidance: Option<&CanonicalGuidance>) -> Option<String> {
    let command = guidance
        .and_then(|g| g.next_step.as_ref())
        .map(|step| step.command.as_str())?;
    Some(format!("\nNext step:\n  {}\n", command.bold()))
}

pub fn print_human(guidance: Option<&CanonicalGuidance>) {
    if let Some(rendered) = render_human(guidance) {
        print!("{rendered}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn plan_action_suggests_start_transition() {
        let guidance = guidance_for_action(VoyageLifecycleAction::Plan, "V1").unwrap();
        assert_eq!(guidance.next_step.unwrap().command, "keel voyage start V1");
    }

    #[test]
    fn start_action_suggests_done_transition() {
        let guidance = guidance_for_action(VoyageLifecycleAction::Start, "V2").unwrap();
        assert_eq!(guidance.next_step.unwrap().command, "keel voyage done V2");
    }

    #[test]
    fn done_action_omits_next_step_guidance() {
        assert!(guidance_for_action(VoyageLifecycleAction::Done, "V3").is_none());
    }

    #[test]
    fn serializes_plan_guidance_with_canonical_next_step_shape() {
        let guidance = guidance_for_action(VoyageLifecycleAction::Plan, "V1").unwrap();
        let json_value = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json_value,
            json!({
                "next_step": { "command": "keel voyage start V1" }
            })
        );
    }

    #[test]
    fn render_human_formats_next_step_block() {
        let guidance = guidance_for_action(VoyageLifecycleAction::Start, "V9").unwrap();
        let rendered = render_human(Some(&guidance)).unwrap();
        assert!(rendered.contains("Next step:"));
        assert!(rendered.contains("keel voyage done V9"));
    }
}
