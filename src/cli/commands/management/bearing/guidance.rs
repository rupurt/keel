//! Canonical guidance helpers for bearing transition commands.

use owo_colors::OwoColorize;

use crate::cli::commands::management::capability_map::{
    ManagementCommand, render_guidance_for_command,
};
use crate::cli::commands::management::guidance::{CanonicalGuidance, CommandGuidance};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearingLifecycleAction {
    New,
    Research,
    Assess,
    Park,
    Decline,
    Lay,
}

/// Informational list command guidance: intentionally non-prescriptive.
pub fn informational_for_list() -> Option<CanonicalGuidance> {
    render_guidance_for_command(
        ManagementCommand::BearingList,
        Some(CommandGuidance::next("keel next --human")),
    )
}

/// Informational show command guidance: intentionally non-prescriptive.
pub fn informational_for_show() -> Option<CanonicalGuidance> {
    render_guidance_for_command(
        ManagementCommand::BearingShow,
        Some(CommandGuidance::next("keel next --human")),
    )
}

/// Build canonical next-step guidance for successful bearing transitions.
pub fn guidance_for_action(
    action: BearingLifecycleAction,
    bearing_id: &str,
) -> Option<CanonicalGuidance> {
    let (command_type, command) = match action {
        BearingLifecycleAction::New => (
            ManagementCommand::BearingNew,
            Some(format!("keel bearing research {bearing_id}")),
        ),
        BearingLifecycleAction::Research => (
            ManagementCommand::BearingResearch,
            Some(format!("keel bearing assess {bearing_id}")),
        ),
        BearingLifecycleAction::Assess => (
            ManagementCommand::BearingAssess,
            Some(format!("keel bearing lay {bearing_id}")),
        ),
        BearingLifecycleAction::Park => (
            ManagementCommand::BearingPark,
            Some("keel next --human".to_string()),
        ),
        BearingLifecycleAction::Decline => (
            ManagementCommand::BearingDecline,
            Some("keel next --human".to_string()),
        ),
        BearingLifecycleAction::Lay => (
            ManagementCommand::BearingLay,
            Some("keel next --human".to_string()),
        ),
    };

    render_guidance_for_command(command_type, command.map(CommandGuidance::next))
}

/// Build canonical recovery guidance for bearing transition failures.
pub fn recovery_for_error(
    action: BearingLifecycleAction,
    bearing_ref: &str,
    message: &str,
) -> Option<CanonicalGuidance> {
    render_guidance_for_command(
        command_type_for_action(action),
        recovery_command_for_error(action, bearing_ref, message).map(CommandGuidance::recovery),
    )
}

/// Wrap an error with deterministic recovery guidance when available.
pub fn error_with_recovery(
    action: BearingLifecycleAction,
    bearing_ref: &str,
    error: anyhow::Error,
) -> anyhow::Error {
    let message = error.to_string();
    let guidance = recovery_for_error(action, bearing_ref, &message);
    let rendered = render_human(guidance.as_ref());
    if rendered.is_empty() {
        anyhow::anyhow!(message)
    } else {
        anyhow::anyhow!("{message}{rendered}")
    }
}

fn recovery_command_for_error(
    action: BearingLifecycleAction,
    bearing_ref: &str,
    message: &str,
) -> Option<String> {
    let lower = message.to_ascii_lowercase();

    if lower.contains("not found") {
        return Some("keel bearing list".to_string());
    }

    if lower.contains("not decision-ready") {
        if lower.contains("assessment") || lower.contains("citation") || lower.contains("factor") {
            return Some(format!("keel bearing file {bearing_ref} ASSESSMENT"));
        }
        return Some(format!("keel bearing file {bearing_ref} EVIDENCE"));
    }

    if action == BearingLifecycleAction::Lay
        && lower.contains("epic already exists")
        && let Some(epic_id) = extract_epic_id(message)
    {
        return Some(format!("keel epic show {epic_id}"));
    }

    if lower.contains("unknown goal reference") {
        return Some(format!("keel bearing file {bearing_ref} BRIEF"));
    }

    if lower.contains("cannot ")
        || lower.contains("already exists for bearing")
        || lower.contains("already declined")
        || lower.contains("already been laid")
    {
        return Some(format!("keel bearing show {bearing_ref}"));
    }

    None
}

fn command_type_for_action(action: BearingLifecycleAction) -> ManagementCommand {
    match action {
        BearingLifecycleAction::New => ManagementCommand::BearingNew,
        BearingLifecycleAction::Research => ManagementCommand::BearingResearch,
        BearingLifecycleAction::Assess => ManagementCommand::BearingAssess,
        BearingLifecycleAction::Park => ManagementCommand::BearingPark,
        BearingLifecycleAction::Decline => ManagementCommand::BearingDecline,
        BearingLifecycleAction::Lay => ManagementCommand::BearingLay,
    }
}

fn extract_epic_id(message: &str) -> Option<String> {
    let marker = "Epic already exists:";
    let start = message.find(marker)?;
    let tail = message[start + marker.len()..].trim();
    let epic_id = tail
        .split_whitespace()
        .next()?
        .trim_end_matches('.')
        .to_string();
    if epic_id.is_empty() {
        None
    } else {
        Some(epic_id)
    }
}

pub fn render_human(guidance: Option<&CanonicalGuidance>) -> String {
    if let Some(step) = guidance.and_then(|g| g.next_step.as_ref()) {
        return format!("\nNext step:\n  {}\n", step.command.bold());
    }

    if let Some(step) = guidance.and_then(|g| g.recovery_step.as_ref()) {
        return format!("\nRecovery step:\n  {}\n", step.command.bold());
    }

    String::new()
}

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
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct GuidanceEnvelope {
        #[serde(skip_serializing_if = "Option::is_none")]
        guidance: Option<CanonicalGuidance>,
    }

    #[test]
    fn survey_success_guidance_maps_to_assess() {
        let guidance = guidance_for_action(BearingLifecycleAction::Research, "B1").unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel bearing assess B1" }
            })
        );
    }

    #[test]
    fn new_success_guidance_maps_to_survey() {
        let guidance = guidance_for_action(BearingLifecycleAction::New, "B0").unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel bearing research B0" }
            })
        );
    }

    #[test]
    fn assess_success_guidance_maps_to_lay() {
        let guidance = guidance_for_action(BearingLifecycleAction::Assess, "B2").unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel bearing lay B2" }
            })
        );
    }

    #[test]
    fn bearing_research_guidance_and_docs_are_consistent() {
        let new_guidance = guidance_for_action(BearingLifecycleAction::New, "B0").unwrap();
        let research_guidance =
            guidance_for_action(BearingLifecycleAction::Research, "B0").unwrap();

        assert_eq!(
            serde_json::to_value(new_guidance).unwrap(),
            json!({
                "next_step": { "command": "keel bearing research B0" }
            })
        );
        assert_eq!(
            serde_json::to_value(research_guidance).unwrap(),
            json!({
                "next_step": { "command": "keel bearing assess B0" }
            })
        );

        let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let readme = std::fs::read_to_string(crate_root.join("README.md")).unwrap();
        let agents = std::fs::read_to_string(crate_root.join("AGENTS.md")).unwrap();
        assert!(readme.contains("bearing new/research/lay"));
        assert!(agents.contains("just keel bearing research <id>"));
        assert!(!readme.contains("bearing new/survey/lay"));
        assert!(!agents.contains("just keel bearing survey <id>"));
    }

    #[test]
    fn bearing_research_guidance_is_deterministic() {
        let left =
            serde_json::to_value(guidance_for_action(BearingLifecycleAction::New, "B7")).unwrap();
        let right =
            serde_json::to_value(guidance_for_action(BearingLifecycleAction::New, "B7")).unwrap();
        assert_eq!(left, right);

        let recovery_left = serde_json::to_value(
            recovery_for_error(
                BearingLifecycleAction::Research,
                "B7",
                "Cannot research bearing 'B7' from 'evaluating' state (must be exploring)",
            )
            .unwrap(),
        )
        .unwrap();
        let recovery_right = serde_json::to_value(
            recovery_for_error(
                BearingLifecycleAction::Research,
                "B7",
                "Cannot research bearing 'B7' from 'evaluating' state (must be exploring)",
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(recovery_left, recovery_right);
    }

    #[test]
    fn terminal_actions_map_to_next_human() {
        for action in [
            BearingLifecycleAction::Park,
            BearingLifecycleAction::Decline,
            BearingLifecycleAction::Lay,
        ] {
            let guidance = guidance_for_action(action, "B3").unwrap();
            let json = serde_json::to_value(guidance).unwrap();
            assert_eq!(
                json,
                json!({
                    "next_step": { "command": "keel next --human" }
                })
            );
        }
    }

    #[test]
    fn informational_commands_emit_no_guidance() {
        assert!(informational_for_list().is_none());
        assert!(informational_for_show().is_none());
        assert!(render_human(informational_for_list().as_ref()).is_empty());
        assert!(render_human(informational_for_show().as_ref()).is_empty());
    }

    #[test]
    fn informational_json_path_omits_guidance_field() {
        let list_json = serde_json::to_value(GuidanceEnvelope {
            guidance: informational_for_list(),
        })
        .unwrap();
        let show_json = serde_json::to_value(GuidanceEnvelope {
            guidance: informational_for_show(),
        })
        .unwrap();

        assert!(list_json.get("guidance").is_none());
        assert!(show_json.get("guidance").is_none());
    }

    #[test]
    fn not_found_recovery_maps_to_bearing_list() {
        let guidance = recovery_for_error(
            BearingLifecycleAction::New,
            "missing",
            "Bearing not found: missing",
        )
        .unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel bearing list" }
            })
        );
    }

    #[test]
    fn lay_epic_exists_recovery_maps_to_epic_show() {
        let guidance = recovery_for_error(
            BearingLifecycleAction::Lay,
            "B4",
            "Epic already exists: 1vxABC. Choose a different bearing name.",
        )
        .unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel epic show 1vxABC" }
            })
        );
    }

    #[test]
    fn render_human_supports_recovery_step_block() {
        let guidance = recovery_for_error(
            BearingLifecycleAction::Assess,
            "B5",
            "Cannot assess bearing 'B5' from 'exploring' state (must be evaluating)",
        )
        .unwrap();
        let rendered = render_human(Some(&guidance));
        assert!(rendered.contains("Recovery step:"));
        assert!(rendered.contains("keel bearing show B5"));
    }

    #[test]
    fn lay_unknown_goal_recovery_maps_to_brief() {
        let guidance = recovery_for_error(
            BearingLifecycleAction::Lay,
            "B8",
            "Unknown goal reference(s) in BRIEF.md Success Criteria: GOAL-99",
        )
        .unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel bearing file B8 BRIEF" }
            })
        );
    }

    #[test]
    fn error_with_recovery_embeds_human_recovery_block() {
        let err = error_with_recovery(
            BearingLifecycleAction::Decline,
            "B6",
            anyhow!("Bearing B6 is already declined"),
        )
        .to_string();

        assert!(err.contains("already declined"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel bearing show B6"));
    }
}
