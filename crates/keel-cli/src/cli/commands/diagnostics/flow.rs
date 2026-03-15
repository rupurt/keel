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
    if scene {
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

        use owo_colors::OwoColorize;
        if autonomous {
            let mut circuit = String::new();
            circuit.push_str("\n    ┌───[BATTERY]───┐\n");
            circuit.push_str("    │               │\n");
            
            if !healthy {
                circuit.push_str("    │   [XX][XX]    │  <-- CAPACITORS BLOWN (SYSTEM UNHEALTHY)\n");
                circuit.push_str("    │    * SPARKS * │\n");
                circuit.push_str("    └───( X / X )───┘\n");
                println!("{}", circuit.red().bold());
                println!("Run `keel doctor` to repair the circuit.");
            } else {
                // Render capacitor bank if work volume is high
                if in_progress > 3 {
                    circuit.push_str("    │   [||][||]    │  <-- CAPACITOR BANK ACTIVE\n");
                } else if in_progress > 0 {
                    circuit.push_str("    │     [||]      │  <-- CAPACITOR CHARGING\n");
                } else {
                    circuit.push_str("    │               │\n");
                }
                
                circuit.push_str("    │               │\n");
                circuit.push_str("    └───( \\ / )─────┘\n");
                
                if in_progress > 0 {
                    circuit.push_str("         \\_/_/  <-- SYSTEM AUTONOMOUS (LIGHT ON)\n");
                    println!("{}", circuit.yellow().bold());
                } else if recently_completed > 0 {
                    circuit.push_str("         \\_/_/  <-- SYSTEM IDLE (LIGHT DIM)\n");
                    println!("{}", circuit.yellow().dimmed());
                } else {
                    circuit.push_str("         \\___/  <-- SYSTEM IDLE (LIGHT OFF)\n");
                    println!("{}", circuit.dimmed());
                }
            }
        } else {
            let circuit = r#"
    ┌───[BATTERY]───┐
    │               │
    │              / 
    │             /
    └───(     )───  <-- HUMAN INPUT REQUIRED (LIGHT OFF)
         \___/
"#;
            println!("{}", circuit.dimmed());
        }
        return Ok(());
    }

    let output = build_output(board_dir, no_color, show_routines)?;
    println!("{}", output);

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
