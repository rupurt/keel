//! Flow visualization and terminal rendering

use owo_colors::OwoColorize;
use std::fmt::Write;

use super::box_component::BoxComponent;
use super::format::render_epic_capacities;
use crate::cli::presentation::flow::layout::LayoutConfig;
use crate::cli::presentation::theme::Theme;
use crate::cli::style;
use crate::domain::model::Board;
use crate::read_model::flow_metrics::FlowMetrics;
use crate::read_model::workflow_lane_flow::{LaneFlowCard, LaneFlowProjection, LaneSourceCount};

/// Render an annotated pipeline flow diagram.
pub fn render_annotated_flow(
    board: &Board,
    metrics: &FlowMetrics,
    lane_flow: &LaneFlowProjection,
    width: usize,
    no_color: bool,
) -> String {
    let mut output = String::new();
    let config = LayoutConfig::from_terminal_width(width);
    let use_color = Theme::should_use_color(no_color);
    let theme = Theme::for_color_mode(use_color);

    // 1. Pipeline Stages (Strategic & Tactical Flow)
    writeln!(output, "{}", style::heavy_rule(width, Some(&theme))).unwrap();
    writeln!(output).unwrap();

    // Stage labels
    let labels = config.render_stage_labels(theme.human, theme.agent, theme.reset);
    writeln!(output, "{}", labels).unwrap();

    // Visual flow diagram (ASCII art)
    let flow = config.render_flow_diagram();
    writeln!(output, "{}", flow).unwrap();

    // Item counts per stage
    let human_counts = [
        metrics.research.exploring_count,
        metrics.research.surveying_count,
        metrics.research.assessing_count,
        metrics.planning.draft_count,
        metrics.planning.planned_count,
        metrics.execution.active_voyages_count,
        metrics.verification.count,
    ];
    let agent_counts = [
        metrics.execution.backlog_count,
        metrics.execution.in_progress_count,
    ];
    let counts = config.render_stage_counts(
        metrics.governance.proposed_count,
        &human_counts,
        &agent_counts,
        metrics.done_count,
    );
    writeln!(output, "{}", counts).unwrap();
    writeln!(output).unwrap();

    // 1b. Mission Summary (Long-running Objectives)
    let mission_summary = render_mission_summary(board, width, &theme);
    if !mission_summary.is_empty() {
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output, "  Active Missions").unwrap();
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "{}", mission_summary).unwrap();
    }

    // 2. Flow Assessment (Bottleneck Analysis)
    let throughput = crate::cli::presentation::flow::throughput::calculate_throughput(board, 4);
    let health = crate::cli::presentation::flow::bottleneck::analyze_health(metrics, &throughput);

    writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
    let constraint_text = if use_color {
        health.constraint_reason.yellow().to_string()
    } else {
        health.constraint_reason.clone()
    };
    writeln!(output, "  Flow Assessment: {}", constraint_text).unwrap();
    if !health.suggested_actions.is_empty() {
        let suggested_text = if use_color {
            health.suggested_actions[0].bold().to_string()
        } else {
            health.suggested_actions[0].clone()
        };
        writeln!(output, "  Suggested: {}", suggested_text).unwrap();
    }
    writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
    writeln!(output).unwrap();

    // 3. Queue Handoff (Configured Workflow Lanes)
    let lane_boxes = render_lane_boxes(lane_flow, width, &theme);
    writeln!(output, "{}", lane_boxes).unwrap();

    // 4. Execution Capacity (Strategic Throughput)
    let capacity = crate::cli::presentation::flow::capacity::calculate_system_capacity(board);
    let has_actionable_capacity = strategic_capacity_available(&capacity.epics);
    let cap_map = capacity
        .epics
        .iter()
        .cloned()
        .map(|report| (report.id.clone(), report))
        .collect::<std::collections::HashMap<_, _>>();

    let cap_render = render_epic_capacities(&cap_map, &theme);
    if !cap_render.is_empty() {
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output, "  Strategic Capacity").unwrap();
        writeln!(output, "{}", style::rule(width, Some(&theme))).unwrap();
        writeln!(output).unwrap();
        writeln!(output, "{}", cap_render).unwrap();
        if !has_actionable_capacity {
            writeln!(output).unwrap();
            writeln!(output, "  {}", strategic_capacity_guidance(board, metrics)).unwrap();
        }
    }

    // 4. Bottleneck Dependencies (Only shown when blockage exists)
    let deps = crate::read_model::traceability::derive_implementation_dependencies(board);
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
        .filter(|s| s.status == crate::domain::model::StoryState::NeedsHumanVerification)
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

fn render_mission_summary(board: &Board, _width: usize, _theme: &Theme) -> String {
    let mut out = String::new();
    let mut active_missions: Vec<_> = board
        .missions
        .values()
        .filter(|m| m.status() == crate::domain::model::MissionStatus::Active)
        .collect();
    active_missions.sort_by_key(|m| m.id());

    for mission in active_missions {
        let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
        let charter_content = std::fs::read_to_string(&charter_path).unwrap_or_default();
        let goals =
            crate::infrastructure::validation::charter::parse_mission_goals(&charter_content);

        let board_goals: Vec<_> = goals
            .iter()
            .filter(|g| {
                matches!(
                    g.verification,
                    crate::infrastructure::validation::charter::GoalVerification::Board(_)
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
            .filter(|e| e.status() == crate::domain::model::EpicState::Done)
            .count();
        let bearings = board.bearings_for_mission(mission.id());
        let bearings_terminal = bearings
            .iter()
            .filter(|b| {
                matches!(
                    b.frontmatter.status,
                    crate::domain::model::BearingStatus::Laid
                        | crate::domain::model::BearingStatus::Declined
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
            "    Goals: {}/{} board goals met",
            board_met,
            board_goals.len()
        )
        .unwrap();
        writeln!(
            out,
            "    Child entities: {}/{} epics done, {}/{} bearings terminal",
            epics_done,
            epics.len(),
            bearings_terminal,
            bearings.len()
        )
        .unwrap();
        writeln!(out).unwrap();
    }

    out
}

fn is_board_goal_met(board: &Board, target: &str) -> bool {
    let target = target.trim();
    if target.is_empty() || target == "..." {
        return false;
    }

    if let Some(epic) = board.epics.get(target) {
        return epic.status() == crate::domain::model::EpicState::Done;
    }
    if let Some(voyage) = board.voyages.get(target) {
        return voyage.status() == crate::domain::state_machine::voyage::VoyageState::Done;
    }
    if let Some(story) = board.stories.get(target) {
        return story.status == crate::domain::model::StoryState::Done;
    }
    false
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

    output
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

    let mut lines_pushed = 0;
    if non_zero_sources.is_empty() {
        lane_box.push_line(format!("  {}", "No items in lane".dimmed()));
        lines_pushed += 1;
    } else {
        for source in non_zero_sources {
            lane_box.push_line(render_lane_source_line(source, width - 2, theme));
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

fn render_lane_source_line(source: &LaneSourceCount, width: usize, theme: &Theme) -> String {
    let count = if source.source.starts_with("story.") {
        format!("{}{}{}", theme.agent, source.count, theme.reset)
    } else {
        format!("{}{}{}", theme.human, source.count, theme.reset)
    };
    let line = format!("  {:<28} {:>3}", source.source, count);
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
    use crate::infrastructure::config::Config;
    use crate::read_model::flow_metrics::{
        ExecutionMetrics, GovernanceMetrics, PlanningMetrics, ResearchMetrics, VerificationMetrics,
    };
    use crate::read_model::{workflow_lane_flow, workflow_topology};

    fn make_test_metrics() -> FlowMetrics {
        FlowMetrics {
            execution: ExecutionMetrics {
                backlog_count: 5,
                backlog_ready_count: 5,
                backlog_blocked_count: 0,
                in_progress_count: 2,
                active_voyages_count: 2,
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
    fn test_render_annotated_flow() {
        let board = Board::default();
        let metrics = make_test_metrics();
        let lane_flow = make_test_lane_flow();
        let rendered = render_annotated_flow(&board, &metrics, &lane_flow, 100, false);
        assert!(rendered.contains("Governance"));
        assert!(rendered.contains("Research"));
        assert!(rendered.contains("Planning"));
        assert!(rendered.contains("Execution"));
        assert!(rendered.contains("Verification"));
        assert!(rendered.contains("management (0) [p100]"));
        assert!(rendered.contains("delivery (0) [p50]"));
        assert!(rendered.contains("No executable epic capacity"));
    }
}
