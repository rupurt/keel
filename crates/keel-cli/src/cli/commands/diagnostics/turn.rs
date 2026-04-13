//! Turn command - inspect the canonical operating loop.

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct TurnPayload {
    phases: Vec<TurnPhasePayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mission_stack: Option<TurnMissionStackPayload>,
}

#[derive(Debug, Serialize)]
struct TurnPhasePayload {
    phase: String,
    title: String,
    purpose: String,
    commands: Vec<TurnCommandPayload>,
}

#[derive(Debug, Serialize)]
struct TurnCommandPayload {
    path: String,
    example: String,
    docs_slug: String,
}

#[derive(Debug, Serialize)]
struct TurnMissionStackPayload {
    id: String,
    branch: String,
    current_branch: Option<String>,
    local_repo: String,
    local_role: keel::read_model::mission_stack::MissionStackMemberRole,
    local_state: String,
    local_mission: Option<String>,
    mode: keel::read_model::mission_stack::MissionStackModeProjection,
    checkpoint: Option<keel::read_model::mission_stack::MissionStackCheckpointProjection>,
    execution_gate: keel::read_model::mission_stack::MissionStackExecutionGateProjection,
    foreign_execution_required: bool,
    foreign_execution_state: keel::read_model::mission_stack::MissionStackForeignExecutionState,
}

pub fn run(json: bool) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let projection = keel::read_model::turn_loop::project_for_board(&board_dir)?;

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload_for(&projection))?
        );
    } else {
        print!("{}", render_text(&projection));
    }

    Ok(())
}

fn payload_for(projection: &keel::read_model::turn_loop::TurnLoopProjection) -> TurnPayload {
    TurnPayload {
        phases: projection
            .phases
            .iter()
            .map(|phase| TurnPhasePayload {
                phase: phase.title.to_lowercase(),
                title: phase.title.to_string(),
                purpose: phase.purpose.to_string(),
                commands: phase
                    .commands
                    .iter()
                    .map(|command| {
                        let descriptor = command.descriptor();
                        TurnCommandPayload {
                            path: descriptor.full_path().to_string(),
                            example: command.example.to_string(),
                            docs_slug: descriptor.docs_slug.to_string(),
                        }
                    })
                    .collect(),
            })
            .collect(),
        mission_stack: projection
            .mission_stack
            .as_ref()
            .map(turn_mission_stack_payload),
    }
}

fn turn_mission_stack_payload(
    stack: &keel::read_model::mission_stack::MissionStackProjection,
) -> TurnMissionStackPayload {
    TurnMissionStackPayload {
        id: stack.id.clone(),
        branch: stack.branch.clone(),
        current_branch: stack.current_branch.clone(),
        local_repo: stack.local_repo.clone(),
        local_role: stack.local_member.role,
        local_state: stack.local_member.state.clone(),
        local_mission: stack.local_member.mission.clone(),
        mode: stack.mode.clone(),
        checkpoint: stack.checkpoint.clone(),
        execution_gate: stack.local_execution_gate(),
        foreign_execution_required: stack.checkout.foreign_execution_required,
        foreign_execution_state: stack.checkout.foreign_execution_state,
    }
}

fn render_text(projection: &keel::read_model::turn_loop::TurnLoopProjection) -> String {
    let mut output = String::from("The Turn Loop\n\n");

    if let Some(stack) = projection.mission_stack.as_ref() {
        output.push_str(&render_mission_stack_text(stack));
        output.push('\n');
    }

    for (index, phase) in projection.phases.iter().enumerate() {
        output.push_str(&format!(
            "{}. {}  {}\n",
            index + 1,
            phase.title,
            phase.purpose
        ));
        for command in &phase.commands {
            output.push_str(&format!("   - {}\n", command.example));
        }
        output.push('\n');
    }

    output
}

fn render_mission_stack_text(
    stack: &keel::read_model::mission_stack::MissionStackProjection,
) -> String {
    let gate = stack.local_execution_gate();
    let mut output = String::from("Mission Stack\n");
    output.push_str(&format!("  Stack: {} on {}\n", stack.id, stack.branch));
    output.push_str(&format!(
        "  Local member: {} ({:?})\n",
        stack.local_repo, stack.local_member.role
    ));
    output.push_str(&format!("  Mode: {}\n", stack.mode_label()));
    if let Some(checkpoint) = &stack.checkpoint {
        output.push_str(&format!("  Checkpoint: {}\n", checkpoint.name));
    }
    output.push_str(&format!(
        "  Foreign execution: {:?}\n",
        stack.checkout.foreign_execution_state
    ));
    output.push_str(&format!("  Execution gate: {:?}\n", gate.status));
    output
}

#[cfg(test)]
mod tests {
    use super::{payload_for, render_text};
    use chrono::Utc;
    use keel::test_helpers::{TestBoardBuilder, git, init_git_repo, write_stack_manifest};
    use std::fs;

    #[test]
    fn turn_text_surface_contains_documented_examples() {
        let projection = keel::read_model::turn_loop::project();
        let rendered = render_text(&projection);

        assert!(rendered.contains("1. Orient  Read the board before you move it."));
        assert!(rendered.contains("keel mission next --status"));
        assert!(rendered.contains("keel story accept --role manager STORY-ID"));
    }

    #[test]
    fn turn_json_surface_is_stable() {
        let projection = keel::read_model::turn_loop::project();
        let payload = payload_for(&projection);
        let json = serde_json::to_value(payload).expect("turn payload should serialize");

        assert_eq!(json["phases"][0]["phase"], "orient");
        assert_eq!(json["phases"][1]["phase"], "inspect");
        assert!(
            json["phases"][1]["commands"]
                .as_array()
                .expect("inspect commands")
                .iter()
                .any(|command| command["path"] == "mission next")
        );
        assert!(
            json["phases"][3]["commands"]
                .as_array()
                .expect("ship commands")
                .iter()
                .any(|command| command["example"] == "keel story submit STORY-ID")
        );
    }

    #[test]
    fn turn_surfaces_mission_stack_context_in_text_and_json() {
        let temp = TestBoardBuilder::new().build();
        init_git_repo(temp.path());
        git(temp.path(), &["checkout", "-b", "stack/demo-stack"]);
        write_stack_manifest(
            temp.path(),
            "demo-stack",
            r#"
id: demo-stack
steward_repo: keel
local_repo: keel
mode:
  kind: shared
  active_repos:
    - keel
members:
  - repo: keel
    role: steward
    state: active
    mission: M1
checkpoint:
  name: integration
  required_members:
    - keel
foreign_execution:
  required: true
  managed_path: managed/paddles
"#,
        );

        let projection = keel::read_model::turn_loop::project_for_board(temp.path()).unwrap();
        let rendered = render_text(&projection);
        let json = serde_json::to_value(payload_for(&projection)).unwrap();

        assert!(rendered.contains("Mission Stack"));
        assert!(rendered.contains("demo-stack"));
        assert!(rendered.contains("stack/demo-stack"));
        assert!(rendered.contains("Mode: shared"));
        assert!(rendered.contains("Checkpoint: integration"));
        assert!(rendered.contains("Foreign execution"));

        assert_eq!(json["mission_stack"]["id"], "demo-stack");
        assert_eq!(json["mission_stack"]["branch"], "stack/demo-stack");
        assert_eq!(json["mission_stack"]["local_role"], "steward");
        assert_eq!(json["mission_stack"]["mode"]["kind"], "shared");
        assert_eq!(json["mission_stack"]["checkpoint"]["name"], "integration");
        assert_eq!(
            json["mission_stack"]["foreign_execution_state"],
            "missing_managed_checkout"
        );
    }

    #[test]
    fn mission_stack_surfaces_expose_deterministic_json() {
        let temp = TestBoardBuilder::new().build();
        init_git_repo(temp.path());
        git(temp.path(), &["checkout", "-b", "stack/demo-stack"]);
        write_stack_manifest(
            temp.path(),
            "demo-stack",
            r#"
id: demo-stack
steward_repo: keel
local_repo: keel
mode:
  kind: exclusive
  active_repo: paddles
members:
  - repo: keel
    role: steward
    state: waiting
    mission: M1
  - repo: paddles
    role: member
    state: active
    mission: M2
"#,
        );

        let projection = keel::read_model::turn_loop::project_for_board(temp.path()).unwrap();
        let json = serde_json::to_value(payload_for(&projection)).unwrap();

        assert_eq!(json["mission_stack"]["id"], "demo-stack");
        assert_eq!(json["mission_stack"]["mode"]["kind"], "exclusive");
        assert_eq!(json["mission_stack"]["execution_gate"]["status"], "yield");
        assert_eq!(
            json["mission_stack"]["execution_gate"]["reason"],
            "exclusive_lease_held_elsewhere"
        );
        assert_eq!(
            json["mission_stack"]["execution_gate"]["active_repos"][0],
            "paddles"
        );
    }

    #[test]
    fn mission_stack_surfaces_preserve_heartbeat_semantics() {
        let temp = tempfile::TempDir::new().unwrap();
        let board_dir = temp.path().join(".keel");
        fs::create_dir_all(&board_dir).unwrap();
        fs::write(board_dir.join("README.md"), "# Board\n").unwrap();
        init_git_repo(temp.path());
        git(temp.path(), &["checkout", "-b", "stack/demo-stack"]);
        write_stack_manifest(
            &board_dir,
            "demo-stack",
            r#"
id: demo-stack
steward_repo: keel
local_repo: keel
mode:
  kind: shared
  active_repos:
    - keel
members:
  - repo: keel
    role: steward
    state: active
"#,
        );
        git(temp.path(), &["add", ".keel/stacks"]);
        git(temp.path(), &["commit", "-m", "add stack"]);

        let before = keel::read_model::heartbeat::project(&board_dir, Utc::now());
        let _projection = keel::read_model::turn_loop::project_for_board(&board_dir).unwrap();
        let after = keel::read_model::heartbeat::project(&board_dir, Utc::now());

        assert_eq!(
            before.source,
            keel::read_model::heartbeat::HeartbeatSource::HeadCommit
        );
        assert_eq!(
            after.source,
            keel::read_model::heartbeat::HeartbeatSource::HeadCommit
        );
        assert_eq!(before.dirty, after.dirty);
    }
}
