//! Canonical guidance output contract for actionable command responses.

use crate::domain::model::taxonomy::RoleTaxonomy;
use crate::read_model::role_context::RoleContextTemplate;
use serde::Serialize;

/// Canonical next/recovery guidance payload.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CanonicalGuidance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_step: Option<GuidanceStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_step: Option<GuidanceStep>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_context: Option<RoleContextGuidance>,
}

/// Single actionable guidance step.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct GuidanceStep {
    pub command: String,
}

/// Role-aware context attached to `keel next` actionable guidance.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RoleContextGuidance {
    pub role: String,
    pub template_id: String,
    pub lane: String,
    pub persona: String,
    pub priorities: Vec<String>,
    pub workflow: Vec<String>,
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
            role_context: None,
        }
    }

    /// Construct a recovery-step guidance payload.
    pub fn recovery(command: impl Into<String>) -> Self {
        Self {
            next_step: None,
            recovery_step: Some(GuidanceStep {
                command: command.into(),
            }),
            role_context: None,
        }
    }

    /// Attach resolved role context to the canonical guidance envelope.
    pub fn with_role_context(mut self, role_context: RoleContextGuidance) -> Self {
        self.role_context = Some(role_context);
        self
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

impl RoleContextGuidance {
    /// Materialize the serialized guidance payload from a role taxonomy and template.
    pub fn from_template(role: &RoleTaxonomy, template: RoleContextTemplate) -> Self {
        Self {
            role: role.to_string(),
            template_id: template.template_id,
            lane: template.lane,
            persona: template.persona.to_string(),
            priorities: template
                .priorities
                .iter()
                .map(|priority| (*priority).to_string())
                .collect(),
            workflow: template
                .workflow_hints
                .iter()
                .map(|hint| (*hint).to_string())
                .collect(),
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
    use crate::infrastructure::config::Config;
    use crate::read_model::{role_context, workflow_topology};
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
            role_context: None,
        };
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(json, json!({}));
    }

    #[test]
    fn serializes_role_context_when_present() {
        let role =
            crate::domain::model::taxonomy::parse("operator/software:infra~steady#oncall").unwrap();
        let topology = workflow_topology::resolve(&Config::default()).unwrap();
        let template = role_context::resolve_role_context(&topology, &role).unwrap();
        let guidance = CanonicalGuidance::next("keel story start S1")
            .with_role_context(RoleContextGuidance::from_template(&role, template));
        let json = serde_json::to_value(guidance).unwrap();

        assert_eq!(
            json["role_context"],
            json!({
                "role": "operator/software:infra~steady#oncall",
                "template_id": "operator-core",
                "lane": "delivery",
                "persona": "Focused operator for evidence-backed delivery.",
                "priorities": [
                    "Pull the next ready delivery slice and keep the scope tight.",
                    "Use the active verification method to prove the slice before moving on.",
                    "Record evidence, submit the story, and commit one atomic slice."
                ],
                "workflow": [
                    "Start from the story and voyage show surfaces before touching the work.",
                    "Keep changes confined to the active story and its directly coupled lifecycle work.",
                    "Run project quality checks, tests, doctests, and doctor before finalizing the slice."
                ]
            })
        );
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
