//! Canonical role-context template registry for management and execution roles.

use crate::domain::model::taxonomy::RoleTaxonomy;
use crate::read_model::queue_policy::ActorQueueLane;

/// Immutable role-context template for a supported role family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoleContextTemplate {
    pub template_id: &'static str,
    pub role_family: &'static str,
    pub queue_lane: ActorQueueLane,
    pub persona: &'static str,
    pub priorities: &'static [&'static str],
    pub workflow_hints: &'static [&'static str],
}

/// Lookup failure for unsupported role families.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedRole {
    pub base_role: String,
}

const MANAGER_PRIORITIES: &[&str] = &[
    "Keep mission, epic, and voyage intent coherent before moving work.",
    "Clear acceptance and planning bottlenecks before starting new execution.",
    "Issue concrete next-step commands that keep the board moving.",
];

const MANAGER_WORKFLOW_HINTS: &[&str] = &[
    "Refresh mission and flow state before making management decisions.",
    "Use show surfaces and authored planning documents to validate scope and acceptance.",
    "Prefer planning, acceptance, and mission logging actions over direct implementation.",
];

const ENGINEER_PRIORITIES: &[&str] = &[
    "Pull the next ready execution slice and keep the scope tight.",
    "Write failing tests first, then implement only enough to pass.",
    "Record evidence, submit the story, and commit one atomic slice.",
];

const ENGINEER_WORKFLOW_HINTS: &[&str] = &[
    "Start from the story and voyage show surfaces before touching code.",
    "Keep edits confined to the active story and its directly coupled lifecycle work.",
    "Run quality, tests, doctests, and doctor before finalizing the slice.",
];

const CORE_ROLE_TEMPLATES: [RoleContextTemplate; 2] = [
    RoleContextTemplate {
        template_id: "manager-core",
        role_family: "manager/*",
        queue_lane: ActorQueueLane::Management,
        persona: "Mission steward for scope, approvals, and coordination.",
        priorities: MANAGER_PRIORITIES,
        workflow_hints: MANAGER_WORKFLOW_HINTS,
    },
    RoleContextTemplate {
        template_id: "engineer-core",
        role_family: "engineer/*",
        queue_lane: ActorQueueLane::Execution,
        persona: "Focused implementer for tested, evidence-backed delivery.",
        priorities: ENGINEER_PRIORITIES,
        workflow_hints: ENGINEER_WORKFLOW_HINTS,
    },
];

impl RoleContextTemplate {
    /// Return the canonical base role family key used for lookup.
    pub fn base_role(&self) -> &'static str {
        self.role_family
            .strip_suffix("/*")
            .expect("canonical role family must end with /*")
    }
}

impl UnsupportedRole {
    /// Return the supported canonical role families.
    pub fn supported_families() -> &'static [&'static str] {
        &["manager/*", "engineer/*"]
    }
}

impl std::fmt::Display for UnsupportedRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "unsupported role family `{}`. Supported families: {}.",
            self.base_role,
            Self::supported_families().join(", ")
        )
    }
}

impl std::error::Error for UnsupportedRole {}

/// Return the canonical core role templates.
pub fn core_role_templates() -> &'static [RoleContextTemplate] {
    &CORE_ROLE_TEMPLATES
}

/// Resolve the canonical role context for a parsed taxonomy.
pub fn resolve_role_context(role: &RoleTaxonomy) -> Result<RoleContextTemplate, UnsupportedRole> {
    core_role_templates()
        .iter()
        .copied()
        .find(|template| template.base_role() == role.role.as_str())
        .ok_or_else(|| UnsupportedRole {
            base_role: role.role.clone(),
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::taxonomy;
    use crate::read_model::queue_policy::ActorQueueLane;

    #[test]
    fn core_role_templates_cover_management_and_execution_families() {
        let templates = core_role_templates();

        assert_eq!(templates.len(), 2);

        let manager = templates
            .iter()
            .find(|template| template.role_family == "manager/*")
            .unwrap();
        assert_eq!(manager.template_id, "manager-core");
        assert_eq!(manager.queue_lane, ActorQueueLane::Management);
        assert!(!manager.persona.is_empty());
        assert!(!manager.priorities.is_empty());
        assert!(!manager.workflow_hints.is_empty());

        let engineer = templates
            .iter()
            .find(|template| template.role_family == "engineer/*")
            .unwrap();
        assert_eq!(engineer.template_id, "engineer-core");
        assert_eq!(engineer.queue_lane, ActorQueueLane::Execution);
        assert!(!engineer.persona.is_empty());
        assert!(!engineer.priorities.is_empty());
        assert!(!engineer.workflow_hints.is_empty());
    }

    #[test]
    fn resolve_role_context_is_deterministic_for_role_family_variants() {
        let manager_product = taxonomy::parse("manager/product").unwrap();
        let manager_delivery = taxonomy::parse("manager/delivery~decisive@L6").unwrap();
        let engineer_software = taxonomy::parse("engineer/software").unwrap();
        let engineer_platform = taxonomy::parse("engineer/platform:infra~steady#oncall").unwrap();

        assert_eq!(
            resolve_role_context(&manager_product).unwrap(),
            resolve_role_context(&manager_delivery).unwrap()
        );
        assert_eq!(
            resolve_role_context(&engineer_software).unwrap(),
            resolve_role_context(&engineer_platform).unwrap()
        );
    }
}
