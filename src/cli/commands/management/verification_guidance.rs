//! Canonical guidance helpers shared by `verify` and `story audit` commands.

use owo_colors::OwoColorize;

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};
use crate::cli::commands::management::story::guidance::next_command_for_state;
use crate::domain::model::StoryState;
use crate::infrastructure::verification::VerificationReport;

/// Build canonical guidance for a single-story verify outcome.
pub fn guidance_for_verify_story(
    story_id: &str,
    story_state: StoryState,
    report: &VerificationReport,
) -> Option<CanonicalGuidance> {
    let guidance = if report.passed() {
        next_command_for_state(story_state, story_id).map(CommandGuidance::next)
    } else {
        Some(CommandGuidance::recovery(format!(
            "keel story audit {story_id}"
        )))
    };

    render_command_guidance(guidance)
}

/// Build canonical guidance for a successful story audit outcome.
pub fn guidance_for_audit_story(
    story_id: &str,
    story_state: StoryState,
) -> Option<CanonicalGuidance> {
    render_command_guidance(
        next_command_for_state(story_state, story_id).map(CommandGuidance::next),
    )
}

/// Wrap verify command errors with deterministic recovery guidance when available.
pub fn verify_error_with_recovery(story_ref: Option<&str>, error: anyhow::Error) -> anyhow::Error {
    let message = error.to_string();
    let guidance = recovery_for_verify_error(story_ref, &message);
    error_with_recovery(message, guidance)
}

/// Wrap audit command errors with deterministic recovery guidance when available.
pub fn audit_error_with_recovery(entity_ref: Option<&str>, error: anyhow::Error) -> anyhow::Error {
    let message = error.to_string();
    let guidance = recovery_for_audit_error(entity_ref, &message);
    error_with_recovery(message, guidance)
}

fn recovery_for_verify_error(story_ref: Option<&str>, message: &str) -> Option<CanonicalGuidance> {
    let lower = message.to_ascii_lowercase();
    let command = if lower.contains("not found") {
        Some("keel story list".to_string())
    } else if let Some(story_id) = story_ref {
        Some(format!("keel story show {story_id}"))
    } else {
        Some("keel doctor".to_string())
    };

    render_command_guidance(command.map(CommandGuidance::recovery))
}

fn recovery_for_audit_error(entity_ref: Option<&str>, message: &str) -> Option<CanonicalGuidance> {
    let lower = message.to_ascii_lowercase();
    let command = if lower.contains("not found") {
        Some("keel story list".to_string())
    } else if entity_ref.is_some() {
        Some("keel status".to_string())
    } else {
        Some("keel doctor".to_string())
    };

    render_command_guidance(command.map(CommandGuidance::recovery))
}

fn error_with_recovery(message: String, guidance: Option<CanonicalGuidance>) -> anyhow::Error {
    let rendered = render_human(guidance.as_ref());
    if rendered.is_empty() {
        anyhow::anyhow!(message)
    } else {
        anyhow::anyhow!("{message}{rendered}")
    }
}

fn render_human(guidance: Option<&CanonicalGuidance>) -> String {
    if let Some(step) = guidance.and_then(|g| g.next_step.as_ref()) {
        return format!("\nNext step:\n  {}\n", step.command.bold());
    }

    if let Some(step) = guidance.and_then(|g| g.recovery_step.as_ref()) {
        return format!("\nRecovery step:\n  {}\n", step.command.bold());
    }

    String::new()
}

/// Print human-readable next/recovery guidance when available.
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

    use crate::infrastructure::verification::VerificationResult;

    fn report_for(story_id: &str, passed: bool) -> VerificationReport {
        VerificationReport {
            story_id: story_id.to_string(),
            results: vec![VerificationResult {
                criterion: "AC-01".to_string(),
                passed,
                expected: "ok".to_string(),
                actual: if passed {
                    "ok".to_string()
                } else {
                    "exit code 1".to_string()
                },
                requires_human_review: false,
            }],
        }
    }

    fn assert_human_json_guidance_parity(guidance: Option<CanonicalGuidance>) {
        let rendered = render_human(guidance.as_ref());
        let json = serde_json::to_value(&guidance).unwrap();

        match guidance.as_ref() {
            Some(CanonicalGuidance {
                next_step: Some(step),
                recovery_step: None,
            }) => {
                assert!(rendered.contains("Next step:"));
                assert!(rendered.contains(step.command.as_str()));
                assert_eq!(json["next_step"]["command"], step.command);
                assert!(json["recovery_step"].is_null());
            }
            Some(CanonicalGuidance {
                next_step: None,
                recovery_step: Some(step),
            }) => {
                assert!(rendered.contains("Recovery step:"));
                assert!(rendered.contains(step.command.as_str()));
                assert_eq!(json["recovery_step"]["command"], step.command);
                assert!(json["next_step"].is_null());
            }
            Some(_) => panic!("guidance must not emit both next_step and recovery_step"),
            None => {
                assert!(rendered.is_empty());
                assert!(json.is_null());
            }
        }
    }

    #[test]
    fn verify_success_guidance_maps_to_story_lifecycle_next_step() {
        let guidance =
            guidance_for_verify_story("S1", StoryState::InProgress, &report_for("S1", true))
                .unwrap();
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel story submit S1" }
            })
        );
    }

    #[test]
    fn verify_failed_report_maps_to_story_audit_recovery() {
        let guidance =
            guidance_for_verify_story("S2", StoryState::InProgress, &report_for("S2", false))
                .unwrap();
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel story audit S2" }
            })
        );
    }

    #[test]
    fn audit_story_success_guidance_maps_to_story_lifecycle_next_step() {
        let guidance = guidance_for_audit_story("S3", StoryState::Backlog).unwrap();
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel story start S3" }
            })
        );
    }

    #[test]
    fn verify_and_audit_guidance_preserve_human_json_parity() {
        let verify_success = guidance_for_verify_story(
            "S4",
            StoryState::NeedsHumanVerification,
            &report_for("S4", true),
        );
        let verify_failed = guidance_for_verify_story(
            "S4",
            StoryState::NeedsHumanVerification,
            &report_for("S4", false),
        );
        let audit_success = guidance_for_audit_story("S4", StoryState::NeedsHumanVerification);

        assert_human_json_guidance_parity(verify_success);
        assert_human_json_guidance_parity(verify_failed);
        assert_human_json_guidance_parity(audit_success);
    }

    #[test]
    fn verify_not_found_recovery_maps_to_story_list() {
        let err = verify_error_with_recovery(Some("MISSING"), anyhow!("Story not found: MISSING"))
            .to_string();

        assert!(err.contains("Story not found: MISSING"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story list"));
    }

    #[test]
    fn audit_not_found_recovery_maps_to_story_list() {
        let err = audit_error_with_recovery(Some("MISSING"), anyhow!("Entity not found: MISSING"))
            .to_string();

        assert!(err.contains("Entity not found: MISSING"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story list"));
    }
}
