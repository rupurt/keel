//! Roles command - inspect configured workflow roles and lanes.

use anyhow::Result;
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Serialize)]
struct RolesPayload {
    defaults: RoleDefaultsPayload,
    roles: Vec<RolePayload>,
    overrides: Vec<RoleOverridePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selected_role: Option<String>,
}

#[derive(Debug, Serialize)]
struct RoleDefaultsPayload {
    management_role: String,
    delivery_role: String,
}

#[derive(Debug, Serialize)]
struct RolePayload {
    role: String,
    lane: String,
    lane_description: String,
    queue_type: String,
    parallel: bool,
    manual_accept: bool,
    operational_contract: String,
    persona: String,
    priorities: Vec<String>,
    workflow: Vec<String>,
}

#[derive(Debug, Serialize)]
struct RoleOverridePayload {
    taxonomy: String,
    operational_contract: String,
}

pub fn run(board_dir: &Path, role_filter: Option<&str>, json: bool) -> Result<()> {
    let topology = keel::read_model::workflow_topology::load_for_board(board_dir)?;
    let projection = keel::read_model::role_routing::project_roles(&topology);

    let (roles, selected_role) = if let Some(role_filter) = role_filter {
        let role = keel::domain::model::taxonomy::parse(role_filter)?;
        (
            vec![keel::read_model::role_routing::explain_role(
                &topology, &role,
            )?],
            Some(role.to_string()),
        )
    } else {
        (
            projection
                .roles
                .iter()
                .map(|entry| entry.routing.clone())
                .collect(),
            None,
        )
    };

    let payload = RolesPayload {
        defaults: RoleDefaultsPayload {
            management_role: projection.management_role_example,
            delivery_role: projection.delivery_role_example,
        },
        roles: roles.into_iter().map(role_payload).collect(),
        overrides: projection
            .overrides
            .into_iter()
            .map(|entry| RoleOverridePayload {
                taxonomy: entry.taxonomy,
                operational_contract: entry.operational_contract,
            })
            .collect(),
        selected_role,
    };

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        print!("{}", render_text(&payload));
    }

    Ok(())
}

fn role_payload(role: keel::read_model::role_routing::RoleRoutingExplanation) -> RolePayload {
    RolePayload {
        role: role.role,
        lane: role.lane,
        lane_description: role.lane_description,
        queue_type: queue_type_label(role.queue_lane).to_string(),
        parallel: role.parallel,
        manual_accept: role.manual_accept,
        operational_contract: role.operational_contract,
        persona: role.persona.to_string(),
        priorities: role
            .priorities
            .iter()
            .map(|priority| (*priority).to_string())
            .collect(),
        workflow: role
            .workflow_hints
            .iter()
            .map(|hint| (*hint).to_string())
            .collect(),
    }
}

fn render_text(payload: &RolesPayload) -> String {
    let mut output = String::from("Workflow Roles\n\nDefaults:\n");
    output.push_str(&format!(
        "  Management: {}\n  Delivery: {}\n",
        payload.defaults.management_role, payload.defaults.delivery_role
    ));

    output.push_str("\nRoles:\n");
    for role in &payload.roles {
        output.push_str(&format!(
            "  - {} -> {} [{}]\n",
            role.role, role.lane, role.queue_type
        ));
        output.push_str(&format!(
            "    Contract: {}\n    Lane behavior: {}{}\n    Persona: {}\n",
            role.operational_contract,
            if role.parallel { "parallel" } else { "serial" },
            if role.manual_accept {
                ", manual accept"
            } else {
                ""
            },
            role.persona
        ));
        output.push_str(&format!("    Lane: {}\n", role.lane_description));
    }

    if !payload.overrides.is_empty() {
        output.push_str("\nOverrides:\n");
        for override_ in &payload.overrides {
            output.push_str(&format!(
                "  - {} -> {}\n",
                override_.taxonomy, override_.operational_contract
            ));
        }
    }

    output
}

fn queue_type_label(queue_lane: keel::read_model::queue_policy::ActorQueueLane) -> &'static str {
    match queue_lane {
        keel::read_model::queue_policy::ActorQueueLane::Management => "management",
        keel::read_model::queue_policy::ActorQueueLane::Execution => "execution",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RoleDefaultsPayload, RoleOverridePayload, RolesPayload, queue_type_label, render_text,
        role_payload,
    };

    #[test]
    fn roles_text_surface_is_compact_and_legible() {
        let payload = RolesPayload {
            defaults: RoleDefaultsPayload {
                management_role: "manager".to_string(),
                delivery_role: "operator".to_string(),
            },
            roles: vec![role_payload(
                keel::read_model::role_routing::RoleRoutingExplanation {
                    role: "manager".to_string(),
                    lane: "management".to_string(),
                    lane_description: "Planning, triage, approvals".to_string(),
                    queue_lane: keel::read_model::queue_policy::ActorQueueLane::Management,
                    parallel: false,
                    manual_accept: true,
                    operational_contract: "manager-core".to_string(),
                    persona: "Mission steward.",
                    priorities: &[],
                    workflow_hints: &[],
                },
            )],
            overrides: vec![RoleOverridePayload {
                taxonomy: "operator/software".to_string(),
                operational_contract: "software-operator-core".to_string(),
            }],
            selected_role: None,
        };

        let rendered = render_text(&payload);
        assert!(rendered.contains("Workflow Roles"));
        assert!(rendered.contains("manager -> management [management]"));
        assert!(rendered.contains("Overrides:"));
    }

    #[test]
    fn queue_type_labels_match_json_contract() {
        assert_eq!(
            queue_type_label(keel::read_model::queue_policy::ActorQueueLane::Management),
            "management"
        );
        assert_eq!(
            queue_type_label(keel::read_model::queue_policy::ActorQueueLane::Execution),
            "execution"
        );
    }
}
