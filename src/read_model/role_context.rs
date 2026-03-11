//! Topology-driven role-context guidance profiles.
//!
//! Note: These "templates" are guidance profiles (persona, priorities, hints)
//! used for CLI guidance and should not be confused with the file-based
//! markdown templates defined in `src/infrastructure/templates.rs`.

use crate::domain::model::taxonomy::RoleTaxonomy;
use crate::read_model::queue_policy::ActorQueueLane;
use crate::read_model::workflow_topology::{ResolvedWorkflowTopology, UnknownRoleFamily};

/// Resolved role-context template guidance for a configured role taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleContextTemplate {
    pub template_id: String,
    pub lane: String,
    pub persona: &'static str,
    pub priorities: &'static [&'static str],
    pub workflow_hints: &'static [&'static str],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RoleContextProfile {
    persona: &'static str,
    priorities: &'static [&'static str],
    workflow_hints: &'static [&'static str],
}

const MANAGEMENT_PRIORITIES: &[&str] = &[
    "Keep mission, epic, and voyage intent coherent before moving work.",
    "Clear acceptance and planning bottlenecks before starting new delivery.",
    "Issue concrete next-step commands that keep the board moving.",
];

const MANAGEMENT_WORKFLOW_HINTS: &[&str] = &[
    "Refresh mission and flow state before making management decisions.",
    "Use show surfaces and authored planning documents to validate scope and acceptance.",
    "Prefer planning, acceptance, and mission logging actions over direct delivery work.",
];

const DELIVERY_PRIORITIES: &[&str] = &[
    "Pull the next ready delivery slice and keep the scope tight.",
    "Use the active verification method to prove the slice before moving on.",
    "Record evidence, submit the story, and commit one atomic slice.",
];

const DELIVERY_WORKFLOW_HINTS: &[&str] = &[
    "Start from the story and voyage show surfaces before touching the work.",
    "Keep changes confined to the active story and its directly coupled lifecycle work.",
    "Run project quality checks, tests, doctests, and doctor before finalizing the slice.",
];

const MANAGEMENT_PROFILE: RoleContextProfile = RoleContextProfile {
    persona: "Mission steward for scope, approvals, and coordination.",
    priorities: MANAGEMENT_PRIORITIES,
    workflow_hints: MANAGEMENT_WORKFLOW_HINTS,
};

const DELIVERY_PROFILE: RoleContextProfile = RoleContextProfile {
    persona: "Focused operator for evidence-backed delivery.",
    priorities: DELIVERY_PRIORITIES,
    workflow_hints: DELIVERY_WORKFLOW_HINTS,
};

/// Resolve role-context guidance from the shared workflow topology.
pub fn resolve_role_context(
    topology: &ResolvedWorkflowTopology,
    role: &RoleTaxonomy,
) -> Result<RoleContextTemplate, UnknownRoleFamily> {
    let lane = topology.resolve_actor_lane(role)?;
    let template_id = topology.resolve_template(role)?.to_string();
    let profile = profile_for_lane(topology.queue_lane_for_actor(role)?);

    Ok(RoleContextTemplate {
        template_id,
        lane: lane.name.clone(),
        persona: profile.persona,
        priorities: profile.priorities,
        workflow_hints: profile.workflow_hints,
    })
}

fn profile_for_lane(queue_lane: ActorQueueLane) -> RoleContextProfile {
    match queue_lane {
        ActorQueueLane::Management => MANAGEMENT_PROFILE,
        ActorQueueLane::Execution => DELIVERY_PROFILE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::taxonomy;
    use crate::infrastructure::config::{Config, LaneConfig, RoleFamilyConfig, RoleOverrideConfig};
    use crate::read_model::workflow_topology;

    #[test]
    fn role_context_topology_resolves_configured_base_role_template_and_lane() {
        let mut config = Config::default();
        config.workflow.defaults.management_role = "director".to_string();
        config.workflow.defaults.management_lane = "review".to_string();
        config.workflow.defaults.delivery_role = "maker".to_string();
        config.workflow.defaults.delivery_lane = "delivery".to_string();
        config.roles = std::collections::BTreeMap::from([
            (
                "director".to_string(),
                RoleFamilyConfig {
                    default_lane: "review".to_string(),
                    template: "director-core".to_string(),
                },
            ),
            (
                "maker".to_string(),
                RoleFamilyConfig {
                    default_lane: "delivery".to_string(),
                    template: "maker-core".to_string(),
                },
            ),
        ]);
        config.lanes = std::collections::BTreeMap::from([
            (
                "review".to_string(),
                LaneConfig {
                    description: "Review".to_string(),
                    include: vec!["story.needs-human-verification".to_string()],
                    exclude: Vec::new(),
                    parallel: false,
                    manual_accept: true,
                    priority: 100,
                },
            ),
            (
                "delivery".to_string(),
                LaneConfig {
                    description: "Delivery".to_string(),
                    include: vec!["story.backlog".to_string(), "story.in-progress".to_string()],
                    exclude: Vec::new(),
                    parallel: true,
                    manual_accept: false,
                    priority: 50,
                },
            ),
        ]);

        let topology = workflow_topology::resolve(&config).unwrap();
        let maker = taxonomy::parse("maker").unwrap();
        let template = resolve_role_context(&topology, &maker).unwrap();

        assert_eq!(template.template_id, "maker-core");
        assert_eq!(template.lane, "delivery");
        assert_eq!(
            template.persona,
            "Focused operator for evidence-backed delivery."
        );
        assert_eq!(template.priorities, DELIVERY_PRIORITIES);
        assert_eq!(template.workflow_hints, DELIVERY_WORKFLOW_HINTS);
    }

    #[test]
    fn role_context_topology_exact_override_takes_precedence_for_full_taxonomy() {
        let mut config = Config::default();
        config.role_overrides.insert(
            "operator/software".to_string(),
            RoleOverrideConfig {
                template: "software-operator-core".to_string(),
            },
        );

        let topology = workflow_topology::resolve(&config).unwrap();
        let base = taxonomy::parse("operator").unwrap();
        let software = taxonomy::parse("operator/software").unwrap();

        assert_eq!(
            resolve_role_context(&topology, &base).unwrap().template_id,
            "operator-core"
        );
        assert_eq!(
            resolve_role_context(&topology, &software)
                .unwrap()
                .template_id,
            "software-operator-core"
        );
    }

    #[test]
    fn role_context_topology_resolution_is_deterministic() {
        let topology = workflow_topology::resolve(&Config::default()).unwrap();
        let operator_software = taxonomy::parse("operator/software").unwrap();
        let operator_platform = taxonomy::parse("operator/platform:infra~steady#oncall").unwrap();

        assert_eq!(
            resolve_role_context(&topology, &operator_software).unwrap(),
            resolve_role_context(&topology, &operator_platform).unwrap()
        );
    }
}
