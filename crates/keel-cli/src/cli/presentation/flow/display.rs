//! Flow visualization and terminal rendering

use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::fmt::Write;
use chrono::Utc;

use super::box_component::BoxComponent;
use super::format::render_epic_capacities;
use crate::cli::presentation::scheduled_routines::describe_scheduled_routine;
use crate::cli::presentation::theme::Theme;
use crate::cli::style;
use keel::domain::model::Board;
use keel::read_model::flow_metrics::FlowMetrics;
use keel::read_model::routine_materialization::projection_materialization_key;
use keel::read_model::scheduled_routines::{ScheduledRoutineProjection, ScheduledRoutineState};
use keel::read_model::workflow_lane_flow::{LaneFlowCard, LaneFlowProjection, LaneSourceCount};

/// Render an annotated pipeline flow diagram.
pub fn render_annotated_flow(
    board: &Board,
    metrics: &FlowMetrics,
    lane_flow: &LaneFlowProjection,
    scheduled: &[ScheduledRoutineProjection],
    materialized_by_key: &HashMap<String, String>,
    width: usize,
    no_color: bool,
    show_routines: bool,
) -> String {
    let mut output = String::new();
    let theme = Theme::for_color_mode(Theme::should_use_color(no_color));

    // 1. Command Directives (Admiral's Inbox)
    let directives = render_command_directives(
        board,
        scheduled,
        materialized_by_key,
        width,
        &theme,
        show_routines,
    );
    if !directives.is_empty() {
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output, "  Command Directives ⚓").unwrap();
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output).unwrap();
        write!(output, "{}", directives).unwrap();
    }

    // 2. Queue Handoff (Configured Workflow Lanes)
    ensure_section_spacing(&mut output);
    let lane_boxes = render_lane_boxes(lane_flow, width, &theme);
    write!(output, "{}", lane_boxes).unwrap();

    // 3. Execution Capacity (Strategic Throughput)
    let capacity = crate::cli::presentation::flow::capacity::calculate_system_capacity(board);
    let has_actionable_capacity = strategic_capacity_available(&capacity.epics);
    let cap_map = capacity
        .epics
        .iter()
        .cloned()
        .map(|report| (report.id.clone(), report))
        .collect::<std::collections::HashMap<_, _>>();

    let cap_render = render_epic_capacities(board, &cap_map, &theme);
    if !cap_render.is_empty() {
        ensure_section_spacing(&mut output);
        writeln!(
            output,
            "  Strategic Capacity ⚡ {}",
            "───────────────".dimmed()
        )
        .unwrap();
        writeln!(output, "{}", cap_render).unwrap();
        if !has_actionable_capacity {
            writeln!(output).unwrap();
            writeln!(output, "    {}", strategic_capacity_guidance(board, metrics)).unwrap();
        }
    }

    // 5. Bottleneck Dependencies (Only shown when blockage exists)
    let deps = keel::read_model::traceability::derive_implementation_dependencies(board);
    let scope_stories: Vec<_> = board
        .stories
        .values()
        .map(
            |story| crate::cli::presentation::flow::format::StoryScopeSummary {
                id: story.id(),
                title: story.title(),
                status: story.status,
                index: story.index(),
                scope: story.scope(),
            },
        )
        .collect();

    let verify_ids = board
        .stories
        .values()
        .filter(|s| s.status == keel::domain::model::StoryState::NeedsHumanVerification)
        .map(|s| s.id())
        .collect::<std::collections::HashSet<_>>();

    let summaries = crate::cli::presentation::flow::format::classify_stories(
        board,
        &scope_stories,
        &deps,
        &verify_ids,
    );
    let blocked_summaries: Vec<_> = summaries
        .iter()
        .filter(|(_, _, status, _)| {
            matches!(
                status,
                crate::cli::presentation::flow::format::DepStatus::Blocked
                    | crate::cli::presentation::flow::format::DepStatus::VerifyBlocked
            )
        })
        .cloned()
        .collect();

    if !blocked_summaries.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output, "  Bottleneck Dependencies (Active Blockages)").unwrap();
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output).unwrap();

        let mut max_id_width = 10;
        for story in board.stories.values() {
            if story.id().len() > max_id_width {
                max_id_width = story.id().len();
            }
        }
        max_id_width += 2;

        writeln!(output, "    {: <w$} TITLE", "ID", w = max_id_width).unwrap();
        writeln!(output, "  {}", "─".repeat(width - 2)).unwrap();

        // Use next_up_ids to show if any of these are the direct bottleneck
        let next_up_ids = crate::cli::presentation::flow::next_up::calculate_next_up(board)
            .human_items
            .into_iter()
            .map(|item| item.id)
            .collect::<std::collections::HashSet<_>>();

        writeln!(
            output,
            "{}",
            crate::cli::presentation::flow::format::render_dependency_chains(
                board,
                &blocked_summaries,
                &next_up_ids,
                &theme
            )
        )
        .unwrap();
    }

    output
}

fn render_command_directives(
    board: &Board,
    scheduled: &[ScheduledRoutineProjection],
    materialized_by_key: &HashMap<String, String>,
    width: usize,
    theme: &Theme,
    show_routines: bool,
) -> String {
    let mut out = String::new();

    // 1. High-Priority Tasking (Routines)
    if show_routines {
        let tasking = render_high_priority_tasking(scheduled, materialized_by_key, theme);
        if !tasking.is_empty() {
            writeln!(out, "  {}", "High-Priority Tasking".bold().yellow()).unwrap();
            write!(out, "{}", tasking).unwrap();
            writeln!(out).unwrap();
        }
    }

    // 2. Mission Objectives
    let (mission_summary, _) = render_mission_summary(board, width, theme);
    if !mission_summary.is_empty() {
        writeln!(out, "  {}", "Mission Objectives".bold().cyan()).unwrap();
        write!(out, "{}", mission_summary).unwrap();
    }

    out
}

fn render_high_priority_tasking(
    scheduled: &[ScheduledRoutineProjection],
    materialized_by_key: &HashMap<String, String>,
    _theme: &Theme,
) -> String {
    let mut out = String::new();
    let mut high_priority: Vec<_> = scheduled
        .iter()
        .filter(|r| {
            matches!(
                r.state,
                ScheduledRoutineState::Due | ScheduledRoutineState::Invalid
            )
        })
        .collect();

    // Also include upcoming routines due in the next 24 hours
    let mut upcoming: Vec<_> = scheduled
        .iter()
        .filter(|r| {
            if let ScheduledRoutineState::Upcoming = r.state {
                if let Some(eligible) = r.next_eligible_at {
                    return eligible.signed_duration_since(Utc::now()).num_hours() < 24;
                }
            }
            false
        })
        .collect();

    high_priority.sort_by_key(|r| r.id.clone());
    upcoming.sort_by_key(|r| r.next_eligible_at);

    for routine in high_priority.into_iter().chain(upcoming) {
        let status = match routine.state {
            ScheduledRoutineState::Due => "DUE".bold().yellow().to_string(),
            ScheduledRoutineState::Invalid => "INVALID".bold().red().to_string(),
            ScheduledRoutineState::Upcoming => "UPCOMING".bold().yellow().to_string(),
        };

        writeln!(
            out,
            "    {} {} {} {}",
            "•".dimmed(),
            routine.id.bold().cyan(),
            status,
            format!("({})", routine.title).dimmed()
        )
        .unwrap();
        let line = render_scheduled_capacity_line(routine, materialized_by_key);
        writeln!(out, "        {}", line).unwrap();
    }

    out
}

fn ensure_section_spacing(output: &mut String) {
    if !output.is_empty() && !output.ends_with("\n\n") {
        writeln!(output).unwrap();
    }
}

fn render_mission_summary(board: &Board, _width: usize, theme: &Theme) -> (String, bool) {
    let (selected_missions, use_top_missions, truncated) = select_missions_for_flow_summary(board);
    if selected_missions.is_empty() {
        return (String::new(), false);
    }

    let mut out = String::new();
    let detail_label_width = ["Goals", "Child entities"]
        .into_iter()
        .map(str::len)
        .max()
        .unwrap_or(0);

    if truncated {
        writeln!(out, "  ...").unwrap();
    }

    for (idx, mission) in selected_missions.iter().enumerate() {
        let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
        let charter_content = std::fs::read_to_string(&charter_path).unwrap_or_default();
        let goals =
            keel::infrastructure::validation::charter::parse_mission_goals(&charter_content);

        let board_goals: Vec<_> = goals
            .iter()
            .filter(|g| {
                matches!(
                    g.verification,
                    keel::infrastructure::validation::charter::GoalVerification::Board(_)
                )
            })
            .collect();
        let board_met = board_goals
            .iter()
            .filter(|g| is_board_goal_met(board, g.verification.raw()))
            .count();

        let epics = board.epics_for_mission(mission.id());
        let epics_done = epics
            .iter()
            .filter(|e| e.status() == keel::domain::model::EpicState::Done)
            .count();
        let bearings = board.bearings_for_mission(mission.id());
        let bearings_terminal = bearings
            .iter()
            .filter(|b| {
                matches!(
                    b.frontmatter.status,
                    keel::domain::model::BearingStatus::Laid
                        | keel::domain::model::BearingStatus::Declined
                )
            })
            .count();

        writeln!(
            out,
            "  Mission: {} {} ({})",
            style::styled_story_id(mission.id()).bold(),
            mission.title().bold(),
            style::styled_mission_status(&mission.status())
        )
        .unwrap();
        writeln!(
            out,
            "    {:>label_width$}: {}",
            "Goals",
            style_mission_summary_value(
                format!("{}/{} board goals met", board_met, board_goals.len()),
                board_met,
                board_goals.len(),
                theme
            ),
            label_width = detail_label_width
        )
        .unwrap();
        writeln!(
            out,
            "    {:>label_width$}: {}, {}",
            "Child entities",
            style_mission_summary_value(
                format!("{}/{} epics done", epics_done, epics.len()),
                epics_done,
                epics.len(),
                theme
            ),
            style_mission_summary_value(
                format!("{}/{} bearings terminal", bearings_terminal, bearings.len()),
                bearings_terminal,
                bearings.len(),
                theme
            ),
            label_width = detail_label_width
        )
        .unwrap();
        if idx + 1 < selected_missions.len() {
            writeln!(out).unwrap();
        }
    }

    (out, use_top_missions)
}

fn select_missions_for_flow_summary(
    board: &Board,
) -> (Vec<&keel::domain::model::Mission>, bool, bool) {
    let mut active_missions: Vec<_> = board
        .missions
        .values()
        .filter(|mission| mission.status() == keel::domain::model::MissionStatus::Active)
        .collect();
    if !active_missions.is_empty() {
        active_missions.sort_by_key(|mission| mission.id());
        return (active_missions, false, false);
    }

    let mut fallback_missions: Vec<_> = board
        .missions
        .values()
        .filter(|mission| mission.status() == keel::domain::model::MissionStatus::Defining)
        .collect();
    if fallback_missions.is_empty() {
        fallback_missions = board
            .missions
            .values()
            .filter(|mission| !mission.status().is_terminal())
            .collect();
    }
    let has_more_than_three = fallback_missions.len() > 3;
    fallback_missions.sort_by(|left, right| {
        let (left_open, left_total) = mission_strategic_summary(board, left);
        let (right_open, right_total) = mission_strategic_summary(board, right);
        right_open
            .cmp(&left_open)
            .then_with(|| right_total.cmp(&left_total))
            .then_with(|| left.id().cmp(right.id()))
    });
    fallback_missions.truncate(3);

    (fallback_missions, true, has_more_than_three)
}

fn mission_strategic_summary(
    board: &Board,
    mission: &keel::domain::model::Mission,
) -> (usize, usize) {
    let epics = board.epics_for_mission(mission.id());
    let bearings = board.bearings_for_mission(mission.id());
    let adrs = board.adrs_for_mission(mission.id());
    let total = epics.len() + bearings.len() + adrs.len();
    let open = epics
        .iter()
        .filter(|epic| epic.status() != keel::domain::model::EpicState::Done)
        .count()
        + bearings
            .iter()
            .filter(|bearing| !bearing.is_complete())
            .count()
        + adrs
            .iter()
            .filter(|adr| !adr.status().is_terminal())
            .count();
    (open, total)
}

fn style_mission_summary_value(
    value: String,
    completed: usize,
    total: usize,
    theme: &Theme,
) -> String {
    if theme.reset.is_empty() {
        return value;
    }

    if total == 0 {
        return value.dimmed().to_string();
    }

    if completed == total {
        return value.green().bold().to_string();
    }

    value.yellow().bold().to_string()
}

fn is_board_goal_met(board: &Board, target: &str) -> bool {
    let target = target.trim();
    if target.is_empty() || target == "..." {
        return false;
    }

    if let Some(epic) = board.epics.get(target) {
        return epic.status() == keel::domain::model::EpicState::Done;
    }
    if let Some(voyage) = board.voyages.get(target) {
        return voyage.status() == keel::domain::state_machine::voyage::VoyageState::Done;
    }
    if let Some(story) = board.stories.get(target) {
        return story.status == keel::domain::model::StoryState::Done;
    }
    false
}

fn render_scheduled_capacity(
    scheduled: &[ScheduledRoutineProjection],
    materialized_by_key: &HashMap<String, String>,
    _width: usize,
    _theme: &Theme,
) -> String {
    if scheduled.is_empty() {
        return String::new();
    }

    let mut out = String::new();
    for routine in scheduled {
        writeln!(
            out,
            "  - {} {} [{}] {}",
            style::styled_id(&routine.id),
            routine.title,
            style::styled_scope(Some(&routine.target_scope)),
            render_scheduled_capacity_line(routine, materialized_by_key)
        )
        .unwrap();
    }

    out.trim_end().to_string()
}

fn render_scheduled_capacity_line(
    routine: &ScheduledRoutineProjection,
    materialized_by_key: &HashMap<String, String>,
) -> String {
    let mut line = describe_scheduled_routine(routine);
    match routine.state {
        ScheduledRoutineState::Due => {
            let guidance = projection_materialization_key(routine)
                .and_then(|key| materialized_by_key.get(&key).cloned())
                .map(|story_id| format!("already materialized this window as {story_id}"))
                .unwrap_or_else(|| "run `keel pulse` to materialize".to_string());
            line.push_str("; ");
            line.push_str(&guidance);
        }
        ScheduledRoutineState::Upcoming => {
            line.push_str("; no pulse action yet");
        }
        ScheduledRoutineState::Invalid => {
            line.push_str("; repair routine cadence");
        }
    }
    line
}

/// Render configured workflow lanes as queue cards.
pub fn render_lane_boxes(lane_flow: &LaneFlowProjection, width: usize, theme: &Theme) -> String {
    if lane_flow.lanes.is_empty() {
        return String::new();
    }

    let max_source_lines = lane_flow
        .lanes
        .iter()
        .map(|l| {
            let count = l.source_counts.iter().filter(|s| s.count > 0).count();
            if count == 0 { 1 } else { count }
        })
        .max()
        .unwrap_or(0);

    if width >= 80 {
        render_lane_boxes_grid(lane_flow, width, theme, max_source_lines)
    } else {
        render_lane_boxes_stacked(lane_flow, width, theme, max_source_lines)
    }
}

fn render_lane_boxes_grid(
    lane_flow: &LaneFlowProjection,
    width: usize,
    theme: &Theme,
    max_source_lines: usize,
) -> String {
    let mut output = String::new();
    let col_width = (width - 2) / 2;

    let boxes: Vec<_> = lane_flow
        .lanes
        .iter()
        .map(|lane| build_lane_box(lane, col_width, theme, max_source_lines))
        .collect();

    let max_height = boxes.iter().map(|b| b.lines.len() + 2).max().unwrap_or(0);

    for (index, chunk) in boxes.chunks(2).enumerate() {
        let rendered: Vec<_> = chunk
            .iter()
            .map(|b| b.render_with_height(max_height))
            .collect();

        for row in 0..max_height {
            let left = rendered
                .first()
                .and_then(|lines| lines.get(row))
                .cloned()
                .unwrap_or_else(|| " ".repeat(col_width));

            if let Some(right_lines) = rendered.get(1) {
                let right = right_lines
                    .get(row)
                    .cloned()
                    .unwrap_or_else(|| " ".repeat(col_width));
                writeln!(output, "{}  {}", left, right).unwrap();
            } else {
                writeln!(output, "{}", left).unwrap();
            }
        }

        if index + 1 < boxes.len().div_ceil(2) {
            writeln!(output).unwrap();
        }
    }

    output.trim_end().to_string()
}

fn render_lane_boxes_stacked(
    lane_flow: &LaneFlowProjection,
    width: usize,
    theme: &Theme,
    max_source_lines: usize,
) -> String {
    let mut output = String::new();

    let boxes: Vec<_> = lane_flow
        .lanes
        .iter()
        .map(|lane| build_lane_box(lane, width, theme, max_source_lines))
        .collect();

    let max_height = boxes.iter().map(|b| b.lines.len() + 2).max().unwrap_or(0);

    for (index, b) in boxes.iter().enumerate() {
        for line in b.render_with_height(max_height) {
            writeln!(output, "{}", line).unwrap();
        }

        if index + 1 < boxes.len() {
            writeln!(output).unwrap();
        }
    }

    output.trim_end().to_string()
}

fn build_lane_box(
    lane: &LaneFlowCard,
    width: usize,
    theme: &Theme,
    max_source_lines: usize,
) -> BoxComponent {
    let mut lane_box = BoxComponent::new(
        &format!("{} ({}) [p{}]", lane.name, lane.total_count, lane.priority),
        width,
    );

    let non_zero_sources: Vec<_> = lane
        .source_counts
        .iter()
        .filter(|source| source.count > 0)
        .collect();
    let label_width = lane_source_label_width(&non_zero_sources);

    let mut lines_pushed = 0;
    if non_zero_sources.is_empty() {
        lane_box.push_line(format!("  {}", "No items in lane".dimmed()));
        lines_pushed += 1;
    } else {
        for source in non_zero_sources {
            lane_box.push_line(render_lane_source_line(
                source,
                label_width,
                width - 2,
                theme,
            ));
            lines_pushed += 1;
        }
    }

    // Pad source section to match global max
    while lines_pushed < max_source_lines {
        lane_box.push_line(" ".to_string());
        lines_pushed += 1;
    }

    lane_box.push_rule();
    lane_box.push_line(render_lane_capabilities_line(lane, width - 2));

    lane_box
}

fn lane_source_label_width(sources: &[&LaneSourceCount]) -> usize {
    sources
        .iter()
        .map(|source| keel::infrastructure::utils::visible_width(&source.source))
        .max()
        .unwrap_or(0)
}

fn render_lane_source_line(
    source: &LaneSourceCount,
    label_width: usize,
    width: usize,
    theme: &Theme,
) -> String {
    let count = if source.source.starts_with("story.") {
        format!(
            "{}{}{}{}",
            theme.bold, theme.agent, source.count, theme.reset
        )
    } else {
        format!(
            "{}{}{}{}",
            theme.bold, theme.human, source.count, theme.reset
        )
    };
    let line = format!("  {:<label_width$}  {}", source.source, count);
    crate::cli::presentation::flow::format::pad_to_width(&line, width)
}

fn render_lane_capabilities_line(lane: &LaneFlowCard, width: usize) -> String {
    let mode = if lane.parallel { "parallel" } else { "serial" };
    let accept = if lane.manual_accept {
        "manual-accept"
    } else {
        "no-manual-accept"
    };
    let line = format!("  mode: {mode}, {accept}");
    crate::cli::presentation::flow::format::pad_to_width(&line, width)
}

fn strategic_capacity_available(
    reports: &[crate::cli::presentation::flow::format::EpicCapacityReport],
) -> bool {
    reports.iter().any(|report| {
        report.capacity.ready + report.capacity.in_flight + report.capacity.blocked > 0
    })
}

fn strategic_capacity_guidance(board: &Board, metrics: &FlowMetrics) -> &'static str {
    if board.epics.is_empty() {
        return "No executable epic capacity. Next step: create an epic with `keel epic new` or lay an assessed bearing.";
    }

    if metrics.planning.draft_count > 0 {
        return "No executable epic capacity. Next step: plan a draft voyage with `keel voyage plan <id>` to thaw scoped work.";
    }

    if metrics.planning.epics_needing_voyages > 0 {
        return "No executable epic capacity. Next step: add a voyage with `keel voyage new` under the next draft epic.";
    }

    "No executable epic capacity. Next step: add or thaw scoped stories inside a planned voyage."
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use keel::infrastructure::config::Config;
    use keel::infrastructure::loader;
    use keel::read_model::flow_metrics::{
        ExecutionMetrics, GovernanceMetrics, PlanningMetrics, ResearchMetrics, VerificationMetrics,
    };
    use keel::read_model::routine_materialization::materialization_key;
    use keel::read_model::scheduled_routines::{
        ScheduledRoutineGatingReason, ScheduledRoutineProjection, ScheduledRoutineState,
    };
    use keel::read_model::{workflow_lane_flow, workflow_topology};
    use keel::test_helpers::{
        TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };
    use owo_colors::OwoColorize;
    use std::fs;

    fn write_test_mission_charter(root: &std::path::Path, mission_id: &str, goals_table: &str) {
        fs::write(
            root.join("missions").join(mission_id).join("CHARTER.md"),
            format!("# Mission Charter\n\n## Goals\n{goals_table}\n"),
        )
        .unwrap();
    }

    fn make_test_metrics() -> FlowMetrics {
        FlowMetrics {
            execution: ExecutionMetrics {
                backlog_count: 5,
                backlog_ready_count: 5,
                backlog_blocked_count: 0,
                in_progress_count: 2,
                active_voyages_count: 2,
                recently_completed_count: 0,
            },
            planning: PlanningMetrics {
                draft_count: 1,
                planned_count: 2,
                epics_needing_voyages: 1,
            },
            research: ResearchMetrics {
                surveying_count: 1,
                assessing_count: 1,
                laid_count: 0,
                exploring_count: 1,
                parked_count: 0,
            },
            verification: VerificationMetrics {
                count: 1,
                avg_age_days: 1.5,
                max_age_days: 2,
                items: vec![],
            },
            governance: GovernanceMetrics {
                proposed_count: 1,
                accepted_count: 1,
            },
            done_count: 10,
        }
    }

    fn make_test_lane_flow() -> LaneFlowProjection {
        let topology = workflow_topology::resolve(&Config::default()).unwrap();
        workflow_lane_flow::project(&Board::default(), &topology)
    }

    fn make_scheduled_projection(
        id: &str,
        title: &str,
        target_scope: &str,
        state: ScheduledRoutineState,
        next_eligible_at: Option<chrono::DateTime<chrono::Utc>>,
        countdown: Option<&str>,
        error: Option<&str>,
    ) -> ScheduledRoutineProjection {
        ScheduledRoutineProjection {
            id: id.to_string(),
            title: title.to_string(),
            target_scope: target_scope.to_string(),
            state,
            actionable: matches!(state, ScheduledRoutineState::Due),
            gating_reason: match state {
                ScheduledRoutineState::Due => ScheduledRoutineGatingReason::DueNow,
                ScheduledRoutineState::Upcoming => {
                    ScheduledRoutineGatingReason::NotDueUntilNextEligible
                }
                ScheduledRoutineState::Invalid => ScheduledRoutineGatingReason::InvalidCadence,
            },
            next_eligible_at,
            countdown: countdown.map(str::to_string),
            error: error.map(str::to_string),
        }
    }

    #[test]
    fn render_lane_boxes_contains_management_header() {
        let lane_flow = make_test_lane_flow();
        let theme = Theme::default();
        let rendered = render_lane_boxes(&lane_flow, 100, &theme);
        assert!(rendered.contains("management (0) [p100]"));
    }

    #[test]
    fn render_lane_boxes_contains_delivery_header() {
        let lane_flow = make_test_lane_flow();
        let theme = Theme::default();
        let rendered = render_lane_boxes(&lane_flow, 100, &theme);
        assert!(rendered.contains("delivery (0) [p50]"));
    }

    #[test]
    fn render_lane_source_line_bolds_colored_counts() {
        let theme = Theme::default();
        let management_line = render_lane_source_line(
            &LaneSourceCount {
                source: "bearing.laid".to_string(),
                count: 7,
            },
            "bearing.laid".len(),
            40,
            &theme,
        );
        let delivery_line = render_lane_source_line(
            &LaneSourceCount {
                source: "story.backlog".to_string(),
                count: 3,
            },
            "story.backlog".len(),
            40,
            &theme,
        );

        assert!(
            management_line.contains(&format!("{}{}7{}", theme.bold, theme.human, theme.reset))
        );
        assert!(delivery_line.contains(&format!("{}{}3{}", theme.bold, theme.agent, theme.reset)));
    }

    #[test]
    fn render_lane_source_line_keeps_counts_close_to_labels() {
        let line = render_lane_source_line(
            &LaneSourceCount {
                source: "bearing.evaluating".to_string(),
                count: 12,
            },
            "bearing.evaluating".len(),
            40,
            &Theme::no_color(),
        );

        assert!(line.contains("bearing.evaluating  12"));
        assert!(!line.contains("bearing.evaluating             12"));
    }

    #[test]
    fn test_render_annotated_flow() {
        let board = Board::default();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();
        let rendered = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &[],
            &HashMap::new(),
            100,
            false,
            true,
        );
        assert!(rendered.contains("management (0) [p100]"));
        assert!(rendered.contains("delivery (0) [p50]"));
        assert!(rendered.contains("No executable epic capacity"));
        assert!(!rendered.contains("Governance"));
        assert!(!rendered.contains("Research"));
        assert!(!rendered.contains("Planning"));
        assert!(!rendered.contains("Execution"));
        assert!(!rendered.contains("Verification"));
        assert!(!rendered.contains("Done"));
    }

    #[test]
    fn render_annotated_flow_shows_scheduled_capacity_guidance() {
        let board = Board::default();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();
        let due_next = Utc.with_ymd_and_hms(2026, 1, 12, 17, 0, 0).unwrap();
        let scheduled = vec![
            make_scheduled_projection(
                "routine-due",
                "Weekly Review",
                "E1/V1",
                ScheduledRoutineState::Due,
                Some(due_next),
                Some("in 6d 23h"),
                None,
            ),
            make_scheduled_projection(
                "routine-upcoming",
                "Friday Review",
                "E1/V1",
                ScheduledRoutineState::Upcoming,
                Some(Utc.with_ymd_and_hms(2026, 1, 5, 19, 0, 0).unwrap()),
                Some("in 1h"),
                None,
            ),
            make_scheduled_projection(
                "routine-invalid",
                "Broken Review",
                "E1/V1",
                ScheduledRoutineState::Invalid,
                None,
                None,
                Some("missing cadence.cron"),
            ),
        ];
        let materialized_by_key = HashMap::from([(
            materialization_key("routine-due", due_next),
            "S1".to_string(),
        )]);

        let rendered = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &scheduled,
            &materialized_by_key,
            100,
            true,
            true,
        );

        assert!(rendered.contains("Scheduled Capacity"));
        assert!(rendered.contains("routine-due"));
        assert!(rendered.contains("due now; next run in 6d 23h (2026-01-12T17:00:00Z)"));
        assert!(rendered.contains("already materialized this window as S1"));
        assert!(rendered.contains("routine-upcoming"));
        assert!(rendered.contains("next run in 1h (2026-01-05T19:00:00Z); no pulse action yet"));
        assert!(rendered.contains("routine-invalid"));
        assert!(rendered.contains("invalid cadence: missing cadence.cron; repair routine cadence"));
    }

    #[test]
    fn render_annotated_flow_keeps_scheduled_output_stable_across_widths() {
        let board = Board::default();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();
        let scheduled = vec![
            make_scheduled_projection(
                "routine-due",
                "Weekly Review",
                "E1/V1",
                ScheduledRoutineState::Due,
                Some(Utc.with_ymd_and_hms(2026, 1, 12, 17, 0, 0).unwrap()),
                Some("in 6d 23h"),
                None,
            ),
            make_scheduled_projection(
                "routine-upcoming",
                "Friday Review",
                "E1/V1",
                ScheduledRoutineState::Upcoming,
                Some(Utc.with_ymd_and_hms(2026, 1, 5, 19, 0, 0).unwrap()),
                Some("in 1h"),
                None,
            ),
        ];

        let wide = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &scheduled,
            &HashMap::new(),
            100,
            true,
            true,
        );
        let narrow = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &scheduled,
            &HashMap::new(),
            72,
            true,
            true,
        );

        for rendered in [wide, narrow] {
            assert!(rendered.contains("Scheduled Capacity"));
            assert!(rendered.contains(
                "due now; next run in 6d 23h (2026-01-12T17:00:00Z); run `keel pulse` to materialize"
            ));
            assert!(
                rendered.contains("next run in 1h (2026-01-05T19:00:00Z); no pulse action yet")
            );
        }
    }

    #[test]
    fn render_mission_summary_is_compact_around_items() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .build();
        let board = loader::load_board(temp.path()).unwrap();

        let (rendered, _) = render_mission_summary(&board, 100, &Theme::default());

        assert!(rendered.starts_with("  Mission:"));
        assert!(!rendered.ends_with("\n\n"));
    }

    #[test]
    fn render_mission_summary_uses_top_defining_missions_when_no_active_missions() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Mission One")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M2")
                    .title("Mission Two")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M3")
                    .title("Mission Three")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M4")
                    .title("Mission Four")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").title("Mission 1 Epic 1").mission("M1"))
            .epic(TestEpic::new("E3").title("Mission 2 Epic 1").mission("M2"))
            .epic(TestEpic::new("E4").title("Mission 2 Epic 2").mission("M2"))
            .epic(TestEpic::new("E5").title("Mission 2 Epic 3").mission("M2"))
            .epic(TestEpic::new("E8").title("Mission 4 Epic 1").mission("M4"))
            .epic(TestEpic::new("E9").title("Mission 4 Epic 2").mission("M4"))
            .epic(TestEpic::new("E10").title("Mission 4 Epic 3").mission("M4"))
            .build();
        let board = loader::load_board(temp.path()).unwrap();

        let (rendered, use_top_missions) = render_mission_summary(&board, 100, &Theme::default());
        let lines = rendered
            .lines()
            .filter(|line| line.contains("Mission:"))
            .collect::<Vec<_>>();

        assert!(use_top_missions);
        assert_eq!(lines.len(), 3);
        assert!(rendered.starts_with("  ..."));
        assert!(lines[0].contains("M2"));
        assert!(lines[1].contains("M4"));
        assert!(lines[2].contains("M1"));
    }

    #[test]
    fn select_missions_for_flow_summary_prefers_defining_missions() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Defining Mission")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M2")
                    .title("Achieved Mission")
                    .status("achieved"),
            )
            .mission(
                TestMission::new("M3")
                    .title("Defining Mission Two")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M4")
                    .title("Paused Mission")
                    .status("paused"),
            )
            .mission(
                TestMission::new("M5")
                    .title("Defining Mission Three")
                    .status("defining"),
            )
            .build();
        let board = loader::load_board(temp.path()).unwrap();

        let (missions, use_top_missions, has_more_than_three) =
            select_missions_for_flow_summary(&board);

        assert!(use_top_missions);
        assert!(!has_more_than_three);
        let mission_ids: Vec<_> = missions.iter().map(|mission| mission.id()).collect();
        assert_eq!(mission_ids, vec!["M1", "M3", "M5"]);
    }

    #[test]
    fn render_mission_summary_aligns_mission_detail_values() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .build();
        let board = loader::load_board(temp.path()).unwrap();

        let (rendered, _) = render_mission_summary(&board, 100, &Theme::default());
        let goals_line = rendered
            .lines()
            .find(|line| line.contains("Goals"))
            .unwrap();
        let child_entities_line = rendered
            .lines()
            .find(|line| line.contains("Child entities"))
            .unwrap();

        assert_eq!(goals_line.find(':'), child_entities_line.find(':'));
    }

    #[test]
    fn render_mission_summary_colors_goal_value_by_completion_state() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .story(
                TestStory::new("S1")
                    .scope("E1")
                    .status(keel::domain::model::StoryState::Done),
            )
            .story(
                TestStory::new("S2")
                    .scope("E1")
                    .status(keel::domain::model::StoryState::Backlog),
            )
            .build();
        write_test_mission_charter(
            temp.path(),
            "M1",
            "| ID | Description | Verification |\n|----|-------------|--------------|\n| MG-01 | First goal | board: S1 |\n| MG-02 | Second goal | board: S2 |",
        );
        let board = loader::load_board(temp.path()).unwrap();

        let (rendered, _) = render_mission_summary(&board, 100, &Theme::default());

        assert!(rendered.contains(&format!("{}", "1/2 board goals met".yellow().bold())));
    }

    #[test]
    fn render_mission_summary_colors_child_entity_segments_independently() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .bearing(
                TestBearing::new("B1")
                    .title("Exploring Bearing")
                    .mission("M1")
                    .status("exploring"),
            )
            .build();
        let board = loader::load_board(temp.path()).unwrap();

        let (rendered, _) = render_mission_summary(&board, 100, &Theme::default());

        assert!(rendered.contains(&format!("{}", "1/1 epics done".green().bold())));
        assert!(rendered.contains(&format!("{}", "0/1 bearings terminal".red().bold())));
    }

    #[test]
    fn render_annotated_flow_shows_top_missions_when_no_active_missions() {
        let temp = TestBoardBuilder::new()
            .mission(
                TestMission::new("M1")
                    .title("Defining Mission One")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M2")
                    .title("Defining Mission Two")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M3")
                    .title("Defining Mission Three")
                    .status("defining"),
            )
            .mission(
                TestMission::new("M4")
                    .title("Defining Mission Four")
                    .status("defining"),
            )
            .epic(TestEpic::new("E1").mission("M1"))
            .epic(TestEpic::new("E2").mission("M1"))
            .epic(TestEpic::new("E3").mission("M2"))
            .epic(TestEpic::new("E4").mission("M2"))
            .epic(TestEpic::new("E5").mission("M2"))
            .epic(TestEpic::new("E6").mission("M3"))
            .build();
        let board = loader::load_board(temp.path()).unwrap();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();

        let rendered = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &[],
            &HashMap::new(),
            100,
            true,
            true,
        );

        assert!(rendered.contains("  Top Missions"));
        assert!(rendered.contains("  ..."));
    }

    #[test]
    fn render_annotated_flow_does_not_prefix_active_missions_with_heavy_rule() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .build();
        let board = loader::load_board(temp.path()).unwrap();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();

        let rendered = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &[],
            &HashMap::new(),
            100,
            true,
            true,
        );

        let first_non_empty = rendered
            .lines()
            .find(|line| !line.trim().is_empty())
            .unwrap();

        assert!(first_non_empty.starts_with('─'));
        assert!(!first_non_empty.starts_with('═'));
        assert!(rendered.contains("  Active Missions"));
    }

    #[test]
    fn render_annotated_flow_places_active_mission_spacer_under_header_not_before_lanes() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").title("Mission One").status("active"))
            .build();
        let board = loader::load_board(temp.path()).unwrap();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();

        let rendered = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &[],
            &HashMap::new(),
            100,
            true,
            true,
        );

        let lines = rendered.lines().collect::<Vec<_>>();
        let header_index = lines
            .iter()
            .position(|line| *line == "  Active Missions")
            .unwrap();
        let mission_index = lines
            .iter()
            .position(|line| line.contains("Mission:"))
            .unwrap();
        let child_entities_index = lines
            .iter()
            .position(|line| line.contains("Child entities"))
            .unwrap();
        let spacer_index = child_entities_index + 1;
        let lane_index = lines
            .iter()
            .position(|line| line.contains("management (0) [p100]"))
            .unwrap();

        assert!(lines[header_index + 1].starts_with('─'));
        assert!(lines[header_index + 2].is_empty());
        assert_eq!(mission_index, header_index + 3);
        assert!(lines[spacer_index].is_empty());
        assert_eq!(lane_index, spacer_index + 1);
    }

    #[test]
    fn render_annotated_flow_places_next_section_immediately_after_lane_boxes() {
        let board = Board::default();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();

        let rendered = render_annotated_flow(
            &board,
            &metrics,
            &lane_flow,
            &[],
            &HashMap::new(),
            100,
            true,
            true,
        );

        let lines = rendered.lines().collect::<Vec<_>>();
        let lane_bottom_index = lines.iter().position(|line| line.starts_with('└')).unwrap();

        assert!(lines[lane_bottom_index + 1].starts_with('─'));
        assert_eq!(lines[lane_bottom_index + 2], "  Strategic Capacity");
    }
}
