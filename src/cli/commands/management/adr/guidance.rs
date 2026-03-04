//! Canonical guidance helpers for ADR transition commands.

use owo_colors::OwoColorize;

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};

fn success_command() -> CommandGuidance {
    CommandGuidance::next("keel next --human")
}

/// Informational list command guidance: intentionally non-prescriptive.
pub fn informational_for_list() -> Option<CanonicalGuidance> {
    None
}

/// Informational show command guidance: intentionally non-prescriptive.
pub fn informational_for_show() -> Option<CanonicalGuidance> {
    None
}

pub fn success_for_accept() -> Option<CanonicalGuidance> {
    render_command_guidance(Some(success_command()))
}

pub fn success_for_reject() -> Option<CanonicalGuidance> {
    render_command_guidance(Some(success_command()))
}

pub fn success_for_deprecate() -> Option<CanonicalGuidance> {
    render_command_guidance(Some(success_command()))
}

pub fn success_for_supersede() -> Option<CanonicalGuidance> {
    render_command_guidance(Some(success_command()))
}

pub fn recovery_for_status_mismatch(adr_id: &str) -> CanonicalGuidance {
    render_command_guidance(Some(CommandGuidance::recovery(format!(
        "keel adr show {adr_id}"
    ))))
    .expect("recovery guidance should be present")
}

pub fn recovery_for_missing() -> CanonicalGuidance {
    render_command_guidance(Some(CommandGuidance::recovery("keel adr list")))
        .expect("recovery guidance should be present")
}

pub fn error_with_recovery(
    message: impl Into<String>,
    guidance: CanonicalGuidance,
) -> anyhow::Error {
    let rendered = render_human(Some(&guidance));
    anyhow::anyhow!("{}\n{}", message.into(), rendered)
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
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct GuidanceEnvelope {
        #[serde(skip_serializing_if = "Option::is_none")]
        guidance: Option<CanonicalGuidance>,
    }

    #[test]
    fn success_guidance_for_transitions_is_canonical_next_step() {
        let accept_json = serde_json::to_value(success_for_accept().unwrap()).unwrap();
        let reject_json = serde_json::to_value(success_for_reject().unwrap()).unwrap();
        let deprecate_json = serde_json::to_value(success_for_deprecate().unwrap()).unwrap();
        let supersede_json = serde_json::to_value(success_for_supersede().unwrap()).unwrap();

        let expected = json!({
            "next_step": { "command": "keel next --human" }
        });

        assert_eq!(accept_json, expected);
        assert_eq!(reject_json, expected);
        assert_eq!(deprecate_json, expected);
        assert_eq!(supersede_json, expected);
    }

    #[test]
    fn recovery_status_mismatch_is_canonical_recovery_step() {
        let json = serde_json::to_value(recovery_for_status_mismatch("ADR1")).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel adr show ADR1" }
            })
        );
    }

    #[test]
    fn recovery_missing_is_canonical_recovery_step() {
        let json = serde_json::to_value(recovery_for_missing()).unwrap();
        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel adr list" }
            })
        );
    }

    #[test]
    fn render_human_supports_next_and_recovery_blocks() {
        let next = render_human(success_for_accept().as_ref());
        assert!(next.contains("Next step:"));
        assert!(next.contains("keel next --human"));

        let recovery = render_human(Some(&recovery_for_status_mismatch("ADR1")));
        assert!(recovery.contains("Recovery step:"));
        assert!(recovery.contains("keel adr show ADR1"));
    }

    #[test]
    fn error_with_recovery_embeds_recovery_command_block() {
        let err = error_with_recovery(
            "Cannot accept ADR ADR1 - status is 'accepted', expected 'proposed'",
            recovery_for_status_mismatch("ADR1"),
        )
        .to_string();

        assert!(err.contains("Cannot accept ADR ADR1"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel adr show ADR1"));
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
}
