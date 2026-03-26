//! Flow command - aggregate pull-system diagnostics

use anyhow::Result;
use chrono::{DateTime, Utc};
use txt_scene::{SceneFrame, SceneLine, ScenePalette};

use crate::cli::presentation::flow::display::render_annotated_flow;
use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::presentation::theme::Theme;
use keel::infrastructure::loader::load_board;
use keel::read_model::routine_materialization::existing_materializations;
use keel::read_model::scheduled_routines::{RoutineScheduleFilter, project_scheduled_routines};
use keel::read_model::{
    capacity::{ChargeState, WatchCapacityReport, project as project_capacity},
    flow_status, workflow_lane_flow, workflow_topology,
};

const FLOW_SCENE_WIDTH: usize = 69;
const FLOW_SCENE_FRAME_WIDTH: usize = 63;
const FLOW_SCENE_INDENT: &str = "    ";
const BATTERY_PACK_SLOTS: usize = 20;
const WATCH_DIAL_SLOTS: usize = 6;

#[derive(Debug, Clone, Copy)]
enum FlowSceneTone {
    Danger,
    WarningBold,
    WarningDim,
    Dim,
}

#[derive(Debug, Clone)]
struct SceneIndicator {
    bank: String,
    annotation: String,
}

#[derive(Debug, Clone)]
struct WatchSceneSummary {
    dial_states: Vec<Option<ChargeState>>,
    pressured_watch_count: usize,
    governed_story_count: usize,
}

/// Run the flow command
pub fn run(
    board_dir: &std::path::Path,
    no_color: bool,
    show_routines: bool,
    scene: bool,
) -> Result<()> {
    let board = load_board(board_dir)?;
    let topology = workflow_topology::load_for_board(board_dir)?;
    let lane_flow = workflow_lane_flow::project(&board, &topology);
    let metrics = flow_status::project(&board, chrono::Utc::now());
    let report = keel::read_model::diagnostics::validate_report(board_dir)?;
    let healthy = report.total_errors() == 0;

    // System is autonomous if no tasks are pending in manual_accept lanes
    let needs_human_input = lane_flow
        .lanes
        .iter()
        .any(|lane| lane.manual_accept && lane.total_count > 0);
    let autonomous = !needs_human_input;

    let in_progress = metrics.execution.in_progress_count;
    let recently_completed = metrics.execution.recently_completed_count;

    let (config, _) = keel::infrastructure::config::load_config();
    use chrono::Timelike;
    let current_hour = chrono::Local::now().hour() as u8;
    let within_working_hours = current_hour >= config.workflow.working_hours_start
        && current_hour < config.workflow.working_hours_end;
    let is_circuit_enabled = config.workflow.open_for_work && within_working_hours;
    let use_color = Theme::should_use_color(no_color);

    if scene {
        use owo_colors::OwoColorize;
        let capacity = project_capacity(&board);
        if !is_circuit_enabled {
            println!("\n{}", render_open_circuit_scene(use_color));
            return Err(anyhow::anyhow!(
                "Circuit is open (disabled or off the clock)"
            ));
        }

        if !healthy {
            println!("\n{}", render_unhealthy_scene(use_color));
            println!("Run `keel doctor` to repair the circuit.");
            return Err(anyhow::anyhow!("Short circuit: System is unhealthy"));
        }

        let energized = recently_completed > 0;
        if !energized {
            println!("\n{}", render_unplugged_scene(use_color));
            return Err(anyhow::anyhow!(
                "System is idle: Cord unplugged (poke to wake)"
            ));
        }

        let ready_backlog = metrics.execution.backlog_ready_count;
        println!(
            "\n{}",
            render_power_scene(
                ready_backlog,
                in_progress,
                &capacity.watches,
                autonomous,
                use_color,
            )
        );

        if !autonomous {
            let mut blocking_items = Vec::new();
            for lane in &lane_flow.lanes {
                if lane.manual_accept && lane.total_count > 0 {
                    for source in &lane.source_counts {
                        if source.count > 0 {
                            let items: Vec<_> = match source.source.as_str() {
                                "story.needs-human-verification" => board.stories.values()
                                    .filter(|s| s.status == keel::domain::model::StoryState::NeedsHumanVerification)
                                    .map(|s| format!("Story {}", s.id()))
                                    .collect(),
                                "mission.achieved" => board.missions.values()
                                    .filter(|m| m.status() == keel::domain::model::MissionStatus::Achieved)
                                    .map(|m| format!("Mission {}", m.id()))
                                    .collect(),
                                "voyage.draft" => board.voyages.values()
                                    .filter(|v| v.status() == keel::domain::state_machine::voyage::VoyageState::Draft)
                                    .map(|v| format!("Voyage {}", v.id()))
                                    .collect(),
                                "bearing.exploring" => board.bearings.values()
                                    .filter(|b| b.status() == keel::domain::model::BearingStatus::Exploring)
                                    .map(|b| format!("Bearing {}", b.id()))
                                    .collect(),
                                "bearing.evaluating" => board.bearings.values()
                                    .filter(|b| b.status() == keel::domain::model::BearingStatus::Evaluating)
                                    .map(|b| format!("Bearing {}", b.id()))
                                    .collect(),
                                "bearing.ready" => board.bearings.values()
                                    .filter(|b| b.status() == keel::domain::model::BearingStatus::Ready)
                                    .map(|b| format!("Bearing {}", b.id()))
                                    .collect(),
                                _ => vec![],
                            };
                            blocking_items.extend(items);
                        }
                    }
                }
            }

            if !blocking_items.is_empty() {
                println!("\nRun `keel workshop` to clear the bench:");
                for item in blocking_items.iter().take(3) {
                    let rendered = if use_color {
                        item.yellow().to_string()
                    } else {
                        item.to_string()
                    };
                    println!("  - {}", rendered);
                }
                if blocking_items.len() > 3 {
                    println!("  ... and {} more", blocking_items.len() - 3);
                }
            }
        }
        return Ok(());
    }

    let output = build_output(board_dir, no_color, show_routines)?;
    println!("{}", output);

    if !is_circuit_enabled {
        return Err(anyhow::anyhow!(
            "Circuit is open (disabled or off the clock)"
        ));
    }
    if !healthy {
        return Err(anyhow::anyhow!("Short circuit: System is unhealthy"));
    }
    if !autonomous {
        return Err(anyhow::anyhow!("System is idle: Human input required"));
    }
    if recently_completed == 0 {
        return Err(anyhow::anyhow!(
            "System is idle: Cord unplugged (poke to wake)"
        ));
    }

    Ok(())
}

fn render_open_circuit_scene(use_color: bool) -> String {
    render_static_scene(
        use_color,
        FlowSceneTone::Dim,
        &[
            "    ┌───────────────────────────[         ]───────────────────────────┐",
            "    │                                                                 │",
            "    │                                /                                │",
            "    │                               /                                 │",
            "    │                              /                                  │",
            "    └───────────────(               )─────────────────────────────────┘",
            "                     \\             /   <-- CIRCUIT OPEN (OFF THE CLOCK / DISABLED)",
            "                      \\___________/",
        ],
    )
}

fn render_unhealthy_scene(use_color: bool) -> String {
    render_static_scene(
        use_color,
        FlowSceneTone::Danger,
        &[
            "    ┌───────────────────────────[ BATTERY ]───────────────────────────┐",
            "    │                                                                 │",
            "    │                                                                 │",
            "    │          [ XX ][ XX ]       [ XX ][ XX ]                        │",
            "    │          <-- CAPACITORS BLOWN (SYSTEM UNHEALTHY)                │",
            "    │               * SPARKS *                                        │",
            "    └───────────────(   X       X   )─────────────────────────────────┘",
        ],
    )
}

fn render_unplugged_scene(use_color: bool) -> String {
    render_static_scene(
        use_color,
        FlowSceneTone::Dim,
        &[
            "    ┌───────────────────────────[ BATTERY ]───────────────────────────┐",
            "    │                                                                 │",
            "    │                                                                 │",
            "    └───────────────( \\             / )   ───                         ",
            "                     \\ \\           / /   <-- CORD UNPLUGGED           ",
            "                      \\ \\_ _ _ _ _/ /        (POKE TO WAKE)           ",
            "                       \\___________/                                  ",
        ],
    )
}

fn render_power_scene(
    ready_backlog: usize,
    in_progress: usize,
    watch_reports: &[WatchCapacityReport],
    autonomous: bool,
    use_color: bool,
) -> String {
    let tone = if autonomous && (in_progress > 0 || ready_backlog > 0) {
        FlowSceneTone::WarningBold
    } else {
        FlowSceneTone::WarningDim
    };
    let palette = ScenePalette::new(use_color);
    let frame = SceneFrame::new(FLOW_SCENE_INDENT, "│", "│", FLOW_SCENE_FRAME_WIDTH);
    let (bank_left, label_left, label_right, state_left, state_right, bank_right) =
        capacitor_bank_layout(in_progress);
    let watch_scene = summarize_watch_scene(watch_reports);

    let mut lines = Vec::new();
    lines.push(style_flow_scene(
        &palette,
        tone,
        "    .───────────────────────────[ POWER ]───────────────────────────.",
    ));
    lines.push(style_flow_scene(&palette, tone, frame.empty_row()));
    lines.push(render_power_meter_row(&palette, tone, ready_backlog));
    lines.push(render_watch_meter_row(&palette, tone, &watch_scene));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.push(" `───────────────────────────────────────────────────────────' ");
    }));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.pad_to(10).push("|");
        line.pad_to(52).push("|");
    }));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.pad_to(6).push(".───┴───.");
        line.pad_to(48).push(".───┴───.");
    }));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.pad_to(6).push(bank_left);
        line.pad_to(15).push(label_left);
        line.pad_to(38).push(label_right);
        line.pad_to(49).push(bank_right);
    }));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.pad_to(6).push(bank_left);
        line.pad_to(16).push(state_left);
        line.pad_to(39).push(state_right);
        line.pad_to(49).push(bank_right);
    }));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.pad_to(6).push("'───────'");
        line.pad_to(48).push("'───────'");
    }));
    lines.push(render_frame_row(&frame, &palette, tone, |line| {
        line.pad_to(10).push("|");
        line.pad_to(52).push("|");
    }));
    lines.push(render_flow_line(&palette, tone, |line| {
        line.push("    └──────────┴────( \\                       / )────┴──────────┘");
    }));
    lines.push(render_flow_line(&palette, tone, |line| {
        line.pad_to(21).push("\\ \\");
        line.pad_to(44).push("/ /");
    }));

    if autonomous {
        if in_progress > 0 || ready_backlog > 0 {
            lines.push(render_flow_line(&palette, tone, |line| {
                line.pad_to(22)
                    .push("\\ \\___________________/ / <-- SYSTEM AUTONOMOUS");
            }));
            lines.push(render_flow_line(&palette, tone, |line| {
                line.pad_to(23)
                    .push("\\_____________________/        (LIGHT ON)");
            }));
        } else {
            lines.push(render_flow_line(&palette, tone, |line| {
                line.pad_to(22)
                    .push("\\ \\___________________/ / <-- SYSTEM IDLE");
            }));
            lines.push(render_flow_line(&palette, tone, |line| {
                line.pad_to(23)
                    .push("\\_____________________/        (LIGHT DIM)");
            }));
        }
    } else {
        lines.push(render_flow_line(&palette, tone, |line| {
            line.pad_to(22)
                .push("\\ \\___________________/ / <-- WORKSHOP BUSY");
        }));
        lines.push(render_flow_line(&palette, tone, |line| {
            line.pad_to(23)
                .push("\\_____________________/        (LIGHT ON)");
        }));
    }

    lines.join("\n")
}

fn render_power_meter_row(
    palette: &ScenePalette,
    tone: FlowSceneTone,
    ready_backlog: usize,
) -> String {
    render_indicator_row(
        palette,
        tone,
        SceneIndicator {
            bank: render_battery_packs(palette, ready_backlog),
            annotation: format!(
                "{} {} PLUGGED IN",
                ready_backlog,
                count_noun(ready_backlog, "BATTERY PACK", "BATTERY PACKS")
            ),
        },
    )
}

fn render_battery_packs(palette: &ScenePalette, ready_backlog: usize) -> String {
    let mut packs = String::new();
    for i in 0..BATTERY_PACK_SLOTS {
        if i < ready_backlog {
            packs.push_str(&palette.green("█"));
        } else {
            packs.push_str(&palette.dim("░"));
        }
    }
    packs
}

fn render_watch_meter_row(
    palette: &ScenePalette,
    tone: FlowSceneTone,
    watch_scene: &WatchSceneSummary,
) -> String {
    render_indicator_row(
        palette,
        tone,
        SceneIndicator {
            bank: render_watch_dials(palette, &watch_scene.dial_states),
            annotation: render_watch_annotation(watch_scene),
        },
    )
}

fn render_indicator_row(
    palette: &ScenePalette,
    tone: FlowSceneTone,
    indicator: SceneIndicator,
) -> String {
    let mut row = SceneLine::new(FLOW_SCENE_WIDTH);
    row.push(style_flow_scene(palette, tone, "    │"));
    row.push(style_flow_scene(palette, tone, "  [ "));
    row.push(indicator.bank);
    row.push(style_flow_scene(palette, tone, " ]  <-- "));
    row.push(style_flow_scene(palette, tone, indicator.annotation));
    row.pad_to(FLOW_SCENE_WIDTH - 1);
    row.push(style_flow_scene(palette, tone, "│"));
    row.finish()
}

fn summarize_watch_scene(watch_reports: &[WatchCapacityReport]) -> WatchSceneSummary {
    let mut dial_states = watch_reports
        .iter()
        .take(WATCH_DIAL_SLOTS)
        .map(|report| Some(report.charge_state))
        .collect::<Vec<_>>();
    dial_states.resize(WATCH_DIAL_SLOTS, None);

    WatchSceneSummary {
        dial_states,
        pressured_watch_count: watch_reports
            .iter()
            .filter(|report| actionable_watch_story_count(report) > 0)
            .count(),
        governed_story_count: watch_reports.iter().map(actionable_watch_story_count).sum(),
    }
}

fn actionable_watch_story_count(report: &WatchCapacityReport) -> usize {
    report.capacity.ready + report.capacity.in_flight + report.capacity.blocked
}

fn render_watch_dials(palette: &ScenePalette, dial_states: &[Option<ChargeState>]) -> String {
    let mut dials = String::new();
    for state in dial_states {
        let glyph = match state {
            None | Some(ChargeState::Discharged) => palette.dim("○"),
            Some(ChargeState::Trickle) => palette.green("◔"),
            Some(ChargeState::Charged) => palette.green("◑"),
            Some(ChargeState::Supercharged) => palette.yellow_bold("◕"),
            Some(ChargeState::Overloaded) => palette.red_bold("●"),
            Some(ChargeState::Blocked) => palette.red_bold("◴"),
        };
        dials.push_str(&glyph);
    }
    dials
}

fn render_watch_annotation(watch_scene: &WatchSceneSummary) -> String {
    if watch_scene.governed_story_count == 0 {
        return "WATCHES AT REST".to_string();
    }

    format!(
        "{} {} KEEPING {} {} ON CLOCK",
        watch_scene.pressured_watch_count,
        count_noun(watch_scene.pressured_watch_count, "WATCH", "WATCHES"),
        watch_scene.governed_story_count,
        count_noun(watch_scene.governed_story_count, "STORY", "STORIES"),
    )
}

fn count_noun(count: usize, singular: &'static str, plural: &'static str) -> &'static str {
    if count == 1 { singular } else { plural }
}

fn capacitor_bank_layout(
    in_progress: usize,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if in_progress > 3 {
        (
            "|[ || ]|",
            "[ HIGH LOAD ]",
            "[ ACTIVE ]",
            "( !!! )",
            "( !!! )",
            "|[ || ]|",
        )
    } else if in_progress > 0 {
        (
            "|[ || ]|",
            "[ CHARGING ]",
            "[ STABLE ]",
            "( ... )",
            "( ... )",
            "|[    ]|",
        )
    } else {
        (
            "|[    ]|",
            "[ IDLE ]",
            "[ READY ]",
            "( zzz )",
            "( ooo )",
            "|[    ]|",
        )
    }
}

fn render_static_scene(use_color: bool, tone: FlowSceneTone, lines: &[&str]) -> String {
    let palette = ScenePalette::new(use_color);
    lines
        .iter()
        .map(|line| style_flow_scene(&palette, tone, *line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn render_frame_row<F>(
    frame: &SceneFrame<'_>,
    palette: &ScenePalette,
    tone: FlowSceneTone,
    build: F,
) -> String
where
    F: FnOnce(&mut SceneLine),
{
    style_flow_scene(palette, tone, frame.row(build))
}

fn render_flow_line<F>(palette: &ScenePalette, tone: FlowSceneTone, build: F) -> String
where
    F: FnOnce(&mut SceneLine),
{
    let mut line = SceneLine::new(FLOW_SCENE_WIDTH);
    build(&mut line);
    style_flow_scene(palette, tone, line.finish())
}

fn style_flow_scene(
    palette: &ScenePalette,
    tone: FlowSceneTone,
    text: impl Into<String>,
) -> String {
    match tone {
        FlowSceneTone::Danger => palette.red_bold(text),
        FlowSceneTone::WarningBold => palette.yellow_bold(text),
        FlowSceneTone::WarningDim => palette.yellow_dim(text),
        FlowSceneTone::Dim => palette.dim(text),
    }
}

fn build_output(
    board_dir: &std::path::Path,
    no_color: bool,
    show_routines: bool,
) -> Result<String> {
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
    use keel::read_model::capacity::EpicCapacity;
    use keel::read_model::routine_materialization::materialization_marker;
    use keel::test_helpers::{TestBearing, TestBoardBuilder, TestMission, TestStory};
    use std::fs;
    use std::path::Path;
    use txt_scene::visible_width;

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
            true,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        let stripped_output = ansi_escape_sequences::strip_ansi(&output);

        assert!(stripped_output.contains("High-Priority Tasking"));
        assert!(stripped_output.contains("routine-due"));
        assert!(stripped_output.contains("due now"));
        assert!(stripped_output.contains("run `keel pulse`"));
        assert!(stripped_output.contains("routine-upcoming"));
        assert!(stripped_output.contains("next run in 1h"));
        let management = stripped_output.find("management (").unwrap();
        let scheduled_capacity = stripped_output.find("High-Priority Tasking").unwrap();
        assert!(scheduled_capacity < management);
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
            true,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        let stripped_output = ansi_escape_sequences::strip_ansi(&output);
        assert!(stripped_output.contains("already materialized this window as S1"));
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

    #[test]
    fn render_power_scene_has_stable_width_without_color() {
        let scene = render_power_scene(5, 0, &[], true, false);
        let lines: Vec<_> = scene.lines().collect();

        assert_eq!(lines.len(), 15);
        assert!(
            lines
                .iter()
                .all(|line| visible_width(line) == FLOW_SCENE_WIDTH)
        );
        assert!(!scene.contains("\x1b["));
        assert_eq!(
            lines[3],
            "    │  [ ○○○○○○ ]  <-- WATCHES AT REST                              │"
        );
        assert_eq!(
            lines[7],
            "    │      |[    ]| [ IDLE ]               [ READY ]  |[    ]|      │"
        );
        assert_eq!(
            lines[8],
            "    │      |[    ]|  ( zzz )                ( ooo )   |[    ]|      │"
        );
    }

    #[test]
    fn render_power_scene_has_stable_width_with_color() {
        let scene = render_power_scene(
            5,
            0,
            &[watch_report("W1", ChargeState::Charged, 3, 1, 1)],
            true,
            true,
        );
        assert!(scene.contains("\x1b["));
        assert!(
            scene
                .lines()
                .all(|line| visible_width(line) == FLOW_SCENE_WIDTH)
        );
    }

    #[test]
    fn render_power_scene_surfaces_watch_pressure_without_color() {
        let scene = render_power_scene(
            5,
            0,
            &[watch_report("W1", ChargeState::Charged, 3, 1, 1)],
            true,
            false,
        );

        assert!(scene.contains("◑"));
        assert!(scene.contains("1 WATCH KEEPING 5 STORIES ON CLOCK"));
    }

    fn watch_report(
        id: &str,
        charge_state: ChargeState,
        ready: usize,
        in_flight: usize,
        blocked: usize,
    ) -> WatchCapacityReport {
        WatchCapacityReport {
            id: id.to_string(),
            title: format!("Watch {id}"),
            charge_state,
            capacity: EpicCapacity {
                ready,
                in_flight,
                blocked,
                inactive: 0,
                done: 0,
            },
        }
    }
}
