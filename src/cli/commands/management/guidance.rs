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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_next_step_only() {
        let guidance = CanonicalGuidance::next("keel story start S1");
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(json["next_step"]["command"], "keel story start S1");
        assert!(json.get("recovery_step").is_none());
    }

    #[test]
    fn serializes_recovery_step_only() {
        let guidance = CanonicalGuidance::recovery("keel story accept S1");
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(json["recovery_step"]["command"], "keel story accept S1");
        assert!(json.get("next_step").is_none());
    }
}
