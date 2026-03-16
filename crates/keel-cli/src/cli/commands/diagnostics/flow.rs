//! Flow command - aggregate pull-system diagnostics

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::cli::presentation::flow::display::render_annotated_flow;
use crate::cli::presentation::terminal::get_terminal_width;
use keel::infrastructure::loader::load_board;
use keel::read_model::routine_materialization::existing_materializations;
use keel::read_model::scheduled_routines::{RoutineScheduleFilter, project_scheduled_routines};
use keel::read_model::{flow_status, workflow_lane_flow, workflow_topology};

/// Run the flow command
pub fn run(board_dir: &std::path::Path, no_color: bool, show_routines: bool, scene: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let topology = workflow_topology::load_for_board(board_dir)?;
    let lane_flow = workflow_lane_flow::project(&board, &topology);
    let metrics = flow_status::project(&board, chrono::Utc::now());
    let report = keel::read_model::diagnostics::validate_report(board_dir)?;
    let healthy = report.total_errors() == 0;
    
    // System is autonomous if no tasks are pending in manual_accept lanes
    let needs_human_input = lane_flow.lanes.iter().any(|lane| lane.manual_accept && lane.total_count > 0);
    let autonomous = !needs_human_input;

    let in_progress = metrics.execution.in_progress_count;
    let recently_completed = metrics.execution.recently_completed_count;

    let (config, _) = keel::infrastructure::config::load_config();
    use chrono::Timelike;
    let current_hour = chrono::Local::now().hour() as u8;
    let within_working_hours = current_hour >= config.workflow.working_hours_start && current_hour < config.workflow.working_hours_end;
    let is_circuit_enabled = config.workflow.open_for_work && within_working_hours;

    if scene {
        use owo_colors::OwoColorize;
        if !is_circuit_enabled {
            let mut circuit = String::new();
            circuit.push_str("\n    ┌───────────────────────────[         ]───────────────────────────┐\n");
            circuit.push_str("    │                                                                 │\n");
            circuit.push_str("    │                                /                                │\n");
            circuit.push_str("    │                               /                                 │\n");
            circuit.push_str("    │                              /                                  │\n");
            circuit.push_str("    └───────────────(               )─────────────────────────────────┘\n");
            circuit.push_str("                     \\             /   <-- CIRCUIT OPEN (OFF THE CLOCK / DISABLED)\n");
            circuit.push_str("                      \\___________/\n");
            println!("{}", circuit.dimmed());
            return Err(anyhow::anyhow!("Circuit is open (disabled or off the clock)"));
        } else if autonomous {
            let mut circuit = String::new();
            circuit.push_str("\n    ┌───────────────────────────[ BATTERY ]───────────────────────────┐\n");
            circuit.push_str("    │                                                                 │\n");
            
            let ready_backlog = metrics.execution.backlog_ready_count;
            let mut packs_visual = String::new();
            for _ in 0..ready_backlog.min(20) {
                packs_visual.push('█');
            }
            if !packs_visual.is_empty() {
                let label = format!("<-- {} BATTERY PACKS PLUGGED IN", ready_backlog);
                circuit.push_str(&format!("    │   [ {: <20} ]  {: <36}│\n", packs_visual, label));
            } else {
                circuit.push_str("    │                                                                 │\n");
            }

            if !healthy {
                circuit.push_str("    │                                                                 │\n");
                circuit.push_str("    │          [ XX ][ XX ]       [ XX ][ XX ]                        │\n");
                circuit.push_str("    │          <-- CAPACITORS BLOWN (SYSTEM UNHEALTHY)                │\n");
                circuit.push_str("    │               * SPARKS *                                        │\n");
                circuit.push_str("    └───────────────(   X       X   )─────────────────────────────────┘\n");
                println!("{}", circuit.red().bold());
                println!("Run `keel doctor` to repair the circuit.");
                return Err(anyhow::anyhow!("Short circuit: System is unhealthy"));
            } else {
                circuit.push_str("    │                                                                 │\n");
                // Render capacitor bank based on work volume
                if in_progress > 3 {
                    circuit.push_str("    │          [ || ][ || ]       [ || ][ || ]                        │\n");
                    circuit.push_str("    │          <-- CAPACITOR BANK ACTIVE (HIGH LOAD)                  │\n");
                } else if in_progress > 0 {
                    circuit.push_str("    │                     [ || ][ || ]                                │\n");
                    circuit.push_str("    │                 <-- CAPACITORS CHARGING                         │\n");
                } else {
                    circuit.push_str("    │                                                                 │\n");
                    circuit.push_str("    │                                                                 │\n");
                }
                
                circuit.push_str("    │                                                                 │\n");
                
                if recently_completed > 0 {
                    circuit.push_str("    └───────────────( \\             / )───────────────────────────────┘\n");
                    circuit.push_str("                     \\ \\           / /\n");
                    if in_progress > 0 || ready_backlog > 0 {
                        circuit.push_str("                      \\ \\_ _ _ _ _/ /  <-- SYSTEM AUTONOMOUS (LIGHT ON)\n");
                        circuit.push_str("                       \\___________/\n");
                        println!("{}", circuit.yellow().bold());
                    } else {
                        circuit.push_str("                      \\ \\_ _ _ _ _/ /  <-- SYSTEM IDLE (LIGHT DIM)\n");
                        circuit.push_str("                       \\___________/\n");
                        println!("{}", circuit.yellow().dimmed());
                    }
                } else {
                    circuit.push_str("    └───────────────( \\             / )   ───                         \n");
                    circuit.push_str("                     \\ \\           / /   <-- CORD UNPLUGGED           \n");
                    circuit.push_str("                      \\ \\_ _ _ _ _/ /        (POKE TO WAKE)           \n");
                    circuit.push_str("                       \\___________/                                  \n");
                    println!("{}", circuit.dimmed());
                    return Err(anyhow::anyhow!("System is idle: Battery is dead"));
                }
            }
        } else {
            let mut circuit = String::new();
            circuit.push_str("\n    ┌───────────────────────────[ BATTERY ]───────────────────────────┐\n");
            circuit.push_str("    │                                                                 │\n");
            circuit.push_str("    │                                /                                │\n");
            circuit.push_str("    │                               /                                 │\n");
            circuit.push_str("    │                              /                                  │\n");
            circuit.push_str("    └───────────────(               )─────────────────────────────────┘\n");
            circuit.push_str("                     \\             /   <-- HUMAN INPUT REQUIRED (LIGHT OFF)\n");
            circuit.push_str("                      \\___________/\n");

            let mut blocking_items = Vec::new();
            for lane in &lane_flow.lanes {
                if lane.manual_accept && lane.total_count > 0 {
                    for source in &lane.source_counts {
                        if source.count > 0 {
                            // Extract the specific entity types/states from the source string (e.g. "story.needs-human-verification")
                            let items: Vec<_> = match source.source.as_str() {
                                "story.needs-human-verification" => board
                                    .stories
                                    .values()
                                    .filter(|s| {
                                        s.status == keel::domain::model::StoryState::NeedsHumanVerification
                                    })
                                    .map(|s| format!("Story {}", s.id()))
                                    .collect(),
                                "mission.achieved" => board
                                    .missions
                                    .values()
                                    .filter(|m| {
                                        m.status() == keel::domain::model::MissionStatus::Achieved
                                    })
                                    .map(|m| format!("Mission {}", m.id()))
                                    .collect(),
                                "voyage.draft" => board
                                    .voyages
                                    .values()
                                    .filter(|v| {
                                        v.status() == keel::domain::state_machine::voyage::VoyageState::Draft
                                    })
                                    .map(|v| format!("Voyage {}", v.id()))
                                    .collect(),
                                "bearing.exploring" => board
                                    .bearings
                                    .values()
                                    .filter(|b| {
                                        b.status() == keel::domain::model::BearingStatus::Exploring
                                    })
                                    .map(|b| format!("Bearing {}", b.id()))
                                    .collect(),
                                "bearing.evaluating" => board
                                    .bearings
                                    .values()
                                    .filter(|b| {
                                        b.status() == keel::domain::model::BearingStatus::Evaluating
                                    })
                                    .map(|b| format!("Bearing {}", b.id()))
                                    .collect(),
                                "bearing.ready" => board
                                    .bearings
                                    .values()
                                    .filter(|b| {
                                        b.status() == keel::domain::model::BearingStatus::Ready
                                    })
                                    .map(|b| format!("Bearing {}", b.id()))
                                    .collect(),
                                _ => vec![],
                            };
                            blocking_items.extend(items);
                        }
                    }
                }
            }

            println!("{}", circuit.dimmed());
            if !blocking_items.is_empty() {
                println!("Items requiring human input:");
                for item in blocking_items.iter().take(5) {
                    println!("  - {}", item.yellow());
                }
                if blocking_items.len() > 5 {
                    println!("  ... and {} more", blocking_items.len() - 5);
                }
            }
            return Err(anyhow::anyhow!("System is idle: Human input required"));
        }
        return Ok(());
    }

    let output = build_output(board_dir, no_color, show_routines)?;
    println!("{}", output);

    if !is_circuit_enabled {
        return Err(anyhow::anyhow!("Circuit is open (disabled or off the clock)"));
    }
    if !healthy {
        return Err(anyhow::anyhow!("Short circuit: System is unhealthy"));
    }
    if !autonomous {
        return Err(anyhow::anyhow!("System is idle: Human input required"));
    }
    if recently_completed == 0 {
        return Err(anyhow::anyhow!("System is idle: Battery is dead"));
    }

    Ok(())
}

fn build_output(board_dir: &std::path::Path, no_color: bool, show_routines: bool) -> Result<String> {
    build_output_at(board_dir, no_color, show_routines, Utc::now())
}

fn build_output_at(
    board_dir: &std::path::Path,
    no_color: bool,
    show_routines: bool,
    reference_time: DateTime<Utc>,
) -> Result<String> {
    let board = load_board(board_dir)?;
    let width = get_terminal_width();

    let metrics = flow_status::project(&board, chrono::Utc::now());
    let topology = workflow_topology::load_for_board(board_dir)?;
    let lane_flow = workflow_lane_flow::project(&board, &topology);
    let scheduled =
        project_scheduled_routines(&board, reference_time, RoutineScheduleFilter::default());
    let materialized_by_key = existing_materializations(&board)?;

    Ok(render_annotated_flow(
        &board,
        &metrics,
        &lane_flow,
        &scheduled,
        &materialized_by_key,
        width,
        no_color,
        show_routines,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use keel::domain::model::StoryState;
    use keel::read_model::routine_materialization::materialization_marker;
    use keel::test_helpers::{TestBearing, TestBoardBuilder, TestMission, TestStory};
    use std::fs;
    use std::path::Path;

    fn write_routine(root: &Path, id: &str, title: &str, target_scope: &str, cadence_block: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: {title}
cadence:
{cadence_block}
target-scope: {target_scope}
created_at: 2026-01-01T00:00:00
updated_at: 2026-01-01T00:00:00
---

# Blueprint

- Review recurring work.
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn test_flow_run() {
        let temp = TestBoardBuilder::new().build();
        let result = build_output(temp.path(), true, true);
        assert!(result.is_ok());
    }

    #[test]
    fn build_output_renders_configured_lanes_in_priority_order() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::NeedsHumanVerification))
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .story(TestStory::new("S3").status(StoryState::Done))
            .bearing(TestBearing::new("B1").status("exploring"))
            .build();
        fs::write(
            temp.path().join("keel.toml"),
            r#"[workflow.defaults]
management_role = "reviewer"
delivery_role = "maker"
management_lane = "review"
delivery_lane = "delivery"

[roles.reviewer]
default_lane = "review"
operational_contract = "reviewer-core"

[roles.maker]
default_lane = "delivery"
operational_contract = "maker-core"

[roles.researcher]
default_lane = "research"
operational_contract = "researcher-core"

[lanes.review]
description = "Manual review work"
include = ["story.needs-human-verification"]
exclude = []
parallel = false
manual_accept = true
priority = 300

[lanes.delivery]
description = "Delivery work"
include = ["story.*"]
exclude = ["story.done", "story.icebox", "story.needs-human-verification", "story.rejected"]
parallel = true
manual_accept = false
priority = 200

[lanes.research]
description = "Research work"
include = ["bearing.exploring"]
exclude = []
parallel = false
manual_accept = false
priority = 100
"#,
        )
        .unwrap();

        let output = build_output(temp.path(), true, true).unwrap();
        let review = output.find("review (1) [p300]").unwrap();
        let delivery = output.find("delivery (1) [p200]").unwrap();
        let research = output.find("research (1) [p100]").unwrap();

        assert!(review < delivery);
        assert!(delivery < research);
        assert!(output.contains("story.needs-human-verification"));
        assert!(output.contains("story.backlog"));
        assert!(output.contains("bearing.exploring"));
        assert!(!output.contains("story.done"));
    }

    #[test]
    fn build_output_surfaces_scheduled_capacity_from_routine_schedule_state() {
        let temp = TestBoardBuilder::new().build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-upcoming",
            "Friday Review",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
        );

        let output = build_output_at(
            temp.path(),
            true,
            false,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        assert!(output.contains("Scheduled Capacity"));
        assert!(output.contains("routine-due"));
        assert!(output.contains("due now"));
        assert!(output.contains("run `keel pulse`"));
        assert!(output.contains("routine-upcoming"));
        assert!(output.contains("next run in 1h"));
        let management = output.find("management (").unwrap();
        let scheduled_capacity = output.find("  Scheduled Capacity").unwrap();
        assert!(management < scheduled_capacity);
    }

    #[test]
    fn build_output_marks_due_routine_as_already_materialized_after_pulse_window() {
        let temp = TestBoardBuilder::new().build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );
        fs::create_dir_all(temp.path().join("stories").join("S1")).unwrap();
        fs::write(
            temp.path().join("stories").join("S1").join("README.md"),
            format!(
                r#"---
id: S1
title: Weekly Review
type: feat
status: backlog
created_at: 2026-01-05T18:00:00
updated_at: 2026-01-05T18:00:00
---

{}

# Materialized story
"#,
                materialization_marker("routine-due@2026-01-12T17:00:00Z")
            ),
        )
        .unwrap();

        let output = build_output_at(
            temp.path(),
            true,
            false,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        assert!(output.contains("already materialized this window as S1"));
    }

    #[test]
    fn build_output_omits_flow_assessment_section() {
        let temp = TestBoardBuilder::new().build();

        let output = build_output(temp.path(), true, true).unwrap();
        assert!(!output.contains("  Flow Assessment:"));
        assert!(!output.contains("  Suggested:"));
    }

    #[test]
    fn build_output_omits_pipeline_status_block() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .build();

        let output = build_output(temp.path(), true, true).unwrap();
        assert!(!output.contains("Governance"));
        assert!(!output.contains("Research"));
        assert!(!output.contains("Planning"));
        assert!(!output.contains("Execution"));
        assert!(!output.contains("Verification"));
        assert!(!output.contains("Done"));
    }
}
