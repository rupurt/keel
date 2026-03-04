//! Canonical guidance output contract for actionable command responses.

use serde::Serialize;

/// Canonical next/recovery guidance payload.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CanonicalGuidance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<GuidanceStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_step: Option<GuidanceStep>,
}

/// Single actionable guidance step.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GuidanceStep {
    pub command: String,
}

/// High-level guidance decision emitted by command mappers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandGuidance {
    Next(String),
    Recovery(String),
}

impl CanonicalGuidance {
    /// Construct a next-step guidance payload.
    pub fn next(command: impl Into<String>) -> Self {
        Self {
            next_step: Some(GuidanceStep {
                command: command.into(),
            }),
            recovery_step: None,
        }
    }

    /// Construct a recovery-step guidance payload.
    pub fn recovery(command: impl Into<String>) -> Self {
        Self {
            next_step: None,
            recovery_step: Some(GuidanceStep {
                command: command.into(),
            }),
        }
    }
}

impl CommandGuidance {
    pub fn next(command: impl Into<String>) -> Self {
        Self::Next(command.into())
    }

    pub fn recovery(command: impl Into<String>) -> Self {
        Self::Recovery(command.into())
    }

    fn into_payload(self) -> CanonicalGuidance {
        match self {
            Self::Next(command) => CanonicalGuidance::next(command),
            Self::Recovery(command) => CanonicalGuidance::recovery(command),
        }
    }
}

/// Render a command-level guidance decision into the canonical payload shape.
pub fn render_command_guidance(guidance: Option<CommandGuidance>) -> Option<CanonicalGuidance> {
    guidance.map(CommandGuidance::into_payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn serializes_next_step_only() {
        let guidance = CanonicalGuidance::next("keel story start S1");
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json,
            json!({
                "next_step": { "command": "keel story start S1" }
            })
        );
    }

    #[test]
    fn serializes_recovery_step_only() {
        let guidance = CanonicalGuidance::recovery("keel story accept S1");
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json,
            json!({
                "recovery_step": { "command": "keel story accept S1" }
            })
        );
    }

    #[test]
    fn serializes_omitted_guidance_as_empty_object() {
        let guidance = CanonicalGuidance {
            next_step: None,
            recovery_step: None,
        };
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(json, json!({}));
    }

    #[test]
    fn render_command_guidance_next_preserves_command_string() {
        let guidance =
            render_command_guidance(Some(CommandGuidance::next("keel story start S1"))).unwrap();
        assert_eq!(guidance.next_step.unwrap().command, "keel story start S1");
    }

    #[test]
    fn render_command_guidance_recovery_preserves_command_string() {
        let guidance =
            render_command_guidance(Some(CommandGuidance::recovery("keel story accept S1")))
                .unwrap();
        assert_eq!(
            guidance.recovery_step.unwrap().command,
            "keel story accept S1"
        );
    }

    #[test]
    fn render_command_guidance_none_returns_none() {
        assert!(render_command_guidance(None).is_none());
    }

    #[test]
    fn render_command_guidance_never_emits_conflicting_fields() {
        let next =
            render_command_guidance(Some(CommandGuidance::next("keel story start S1"))).unwrap();
        assert!(next.next_step.is_some());
        assert!(next.recovery_step.is_none());

        let recovery =
            render_command_guidance(Some(CommandGuidance::recovery("keel story accept S1")))
                .unwrap();
        assert!(recovery.next_step.is_none());
        assert!(recovery.recovery_step.is_some());
    }
}
