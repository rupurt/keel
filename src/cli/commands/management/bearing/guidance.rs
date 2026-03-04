//! Canonical guidance helpers for bearing transition commands.

use owo_colors::OwoColorize;

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearingLifecycleAction {
    Survey,
    Assess,
    Park,
    Decline,
    Lay,
}

/// Informational list command guidance: intentionally non-prescriptive.
pub fn informational_for_list() -> Option<CanonicalGuidance> {
    None
}

/// Informational show command guidance: intentionally non-prescriptive.
pub fn informational_for_show() -> Option<CanonicalGuidance> {
    None
}

/// Build canonical next-step guidance for successful bearing transitions.
pub fn guidance_for_action(
    action: BearingLifecycleAction,
    bearing_id: &str,
) -> Option<CanonicalGuidance> {
    let command = match action {
        BearingLifecycleAction::Survey => Some(format!("keel bearing assess {bearing_id}")),
        BearingLifecycleAction::Assess => Some(format!("keel bearing lay {bearing_id}")),
        BearingLifecycleAction::Park
        | BearingLifecycleAction::Decline
        | BearingLifecycleAction::Lay => Some("keel next --human".to_string()),
    };
    render_command_guidance(command.map(CommandGuidance::next))
}

/// Build canonical recovery guidance for bearing transition failures.
pub fn recovery_for_error(
    action: BearingLifecycleAction,
    bearing_ref: &str,
    message: &str,
) -> Option<CanonicalGuidance> {
    render_command_guidance(
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

    if action == BearingLifecycleAction::Lay
        && lower.contains("epic already exists")
        && let Some(epic_id) = extract_epic_id(message)
    {
        return Some(format!("keel epic show {epic_id}"));
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
        let guidance = guidance_for_action(BearingLifecycleAction::Survey, "B1").unwrap();
        let json = serde_json::to_value(guidance).unwrap();
        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel bearing assess B1" }
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
            BearingLifecycleAction::Survey,
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
