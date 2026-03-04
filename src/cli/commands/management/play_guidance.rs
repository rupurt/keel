//! Canonical guidance helpers for `keel play` outcomes.

use owo_colors::OwoColorize;

use crate::cli::commands::management::guidance::{
    CanonicalGuidance, CommandGuidance, render_command_guidance,
};

/// Deterministic guidance for a `keel play --suggest` outcome.
pub fn guidance_for_suggest(bearing_id: &str, prop_name: &str) -> Option<CanonicalGuidance> {
    render_command_guidance(Some(CommandGuidance::next(format!(
        "keel play {bearing_id} --prop {prop_name}"
    ))))
}

/// Exploratory play outcomes are intentionally non-prescriptive.
pub fn informational_for_exploration() -> Option<CanonicalGuidance> {
    None
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
    use serde::Serialize;
    use serde_json::json;

    #[derive(Serialize)]
    struct GuidanceEnvelope {
        #[serde(skip_serializing_if = "Option::is_none")]
        guidance: Option<CanonicalGuidance>,
    }

    fn assert_human_json_guidance_parity(guidance: Option<CanonicalGuidance>) {
        let rendered = render_human(guidance.as_ref());
        let json_value = serde_json::to_value(&guidance).unwrap();

        match guidance.as_ref() {
            Some(CanonicalGuidance {
                next_step: Some(step),
                recovery_step: None,
            }) => {
                assert!(rendered.contains("Next step:"));
                assert!(rendered.contains(step.command.as_str()));
                assert_eq!(json_value["next_step"]["command"], step.command);
                assert!(json_value["recovery_step"].is_null());
            }
            Some(CanonicalGuidance {
                next_step: None,
                recovery_step: Some(step),
            }) => {
                assert!(rendered.contains("Recovery step:"));
                assert!(rendered.contains(step.command.as_str()));
                assert_eq!(json_value["recovery_step"]["command"], step.command);
                assert!(json_value["next_step"].is_null());
            }
            Some(_) => panic!("guidance must not emit both next_step and recovery_step"),
            None => {
                assert!(rendered.is_empty());
                assert!(json_value.is_null());
            }
        }
    }

    #[test]
    fn suggest_outcome_maps_to_canonical_next_step() {
        let guidance = guidance_for_suggest("B1", "playwright").unwrap();
        let json_value = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json_value,
            json!({
                "next_step": { "command": "keel play B1 --prop playwright" }
            })
        );
    }

    #[test]
    fn exploratory_outcomes_emit_no_guidance() {
        let guidance = informational_for_exploration();
        assert!(guidance.is_none());
        assert!(render_human(guidance.as_ref()).is_empty());
    }

    #[test]
    fn exploratory_json_path_omits_guidance_field() {
        let json_value = serde_json::to_value(GuidanceEnvelope {
            guidance: informational_for_exploration(),
        })
        .unwrap();

        assert!(json_value.get("guidance").is_none());
    }

    #[test]
    fn play_outcomes_keep_human_and_json_guidance_in_sync() {
        let deterministic = guidance_for_suggest("B2", "improviser");
        let exploratory = informational_for_exploration();

        assert_human_json_guidance_parity(deterministic);
        assert_human_json_guidance_parity(exploratory);
    }
}
