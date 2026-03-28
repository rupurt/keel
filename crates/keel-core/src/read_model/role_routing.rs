//! Canonical role and routing inspection projections.

use crate::domain::model::taxonomy;
use crate::read_model::queue_policy::ActorQueueLane;
use crate::read_model::role_context;
use crate::read_model::workflow_topology::{ResolvedWorkflowTopology, UnknownRoleFamily};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleRoutingExplanation {
    pub role: String,
    pub lane: String,
    pub lane_description: String,
    pub queue_lane: ActorQueueLane,
    pub parallel: bool,
    pub manual_accept: bool,
    pub operational_contract: String,
    pub persona: &'static str,
    pub priorities: &'static [&'static str],
    pub workflow_hints: &'static [&'static str],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleSurfaceEntry {
    pub role_family: String,
    pub routing: RoleRoutingExplanation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoleOverrideEntry {
    pub taxonomy: String,
    pub operational_contract: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RolesProjection {
    pub management_role_example: String,
    pub delivery_role_example: String,
    pub roles: Vec<RoleSurfaceEntry>,
    pub overrides: Vec<RoleOverrideEntry>,
}

pub fn explain_role(
    topology: &ResolvedWorkflowTopology,
    role: &crate::domain::model::taxonomy::RoleTaxonomy,
) -> Result<RoleRoutingExplanation, UnknownRoleFamily> {
    let actor = topology.resolve_actor_context(role)?;
    let lane = topology.resolve_actor_lane(role)?;
    let contract = role_context::resolve_role_context(topology, role)?;

    Ok(RoleRoutingExplanation {
        role: role.to_string(),
        lane: actor.lane_name,
        lane_description: lane.description.clone(),
        queue_lane: actor.queue_lane,
        parallel: actor.parallel,
        manual_accept: actor.manual_accept,
        operational_contract: actor.operational_contract,
        persona: contract.persona,
        priorities: contract.priorities,
        workflow_hints: contract.workflow_hints,
    })
}

pub fn project_roles(topology: &ResolvedWorkflowTopology) -> RolesProjection {
    let mut roles: Vec<_> = topology
        .roles
        .keys()
        .map(|role_family| {
            let taxonomy = taxonomy::parse(role_family)
                .expect("workflow topology role family names must parse as taxonomy");
            RoleSurfaceEntry {
                role_family: role_family.clone(),
                routing: explain_role(topology, &taxonomy)
                    .expect("topology role families must resolve to routing explanations"),
            }
        })
        .collect();
    roles.sort_by(|left, right| left.role_family.cmp(&right.role_family));

    let mut overrides: Vec<_> = topology
        .role_overrides
        .iter()
        .map(|(taxonomy, override_)| RoleOverrideEntry {
            taxonomy: taxonomy.clone(),
            operational_contract: override_.operational_contract.clone(),
        })
        .collect();
    overrides.sort_by(|left, right| left.taxonomy.cmp(&right.taxonomy));

    RolesProjection {
        management_role_example: topology.management_role_example().to_string(),
        delivery_role_example: topology.delivery_role_example().to_string(),
        roles,
        overrides,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::taxonomy;
    use crate::infrastructure::config::Config;
    use crate::read_model::workflow_topology;

    #[test]
    fn roles_projection_is_deterministic_for_default_topology() {
        let topology = workflow_topology::resolve(&Config::default()).unwrap();

        let first = project_roles(&topology);
        let second = project_roles(&topology);

        assert_eq!(first, second);
        assert_eq!(first.management_role_example, "manager");
        assert_eq!(first.delivery_role_example, "operator");
        assert_eq!(
            first
                .roles
                .iter()
                .map(|role| role.role_family.as_str())
                .collect::<Vec<_>>(),
            vec!["manager", "operator"]
        );
    }

    #[test]
    fn role_explanation_comes_from_topology_and_role_context() {
        let topology = workflow_topology::resolve(&Config::default()).unwrap();
        let role = taxonomy::parse("operator/software").unwrap();
        let explanation = explain_role(&topology, &role).unwrap();

        assert_eq!(explanation.role, "operator/software");
        assert_eq!(explanation.lane, "delivery");
        assert_eq!(explanation.queue_lane, ActorQueueLane::Execution);
        assert!(explanation.parallel);
        assert!(!explanation.manual_accept);
        assert_eq!(explanation.operational_contract, "operator-core");
        assert_eq!(
            explanation.persona,
            "Focused operator for evidence-backed delivery."
        );
    }
}
