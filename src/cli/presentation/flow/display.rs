//! Flow visualization and terminal rendering

use owo_colors::OwoColorize;
use std::fmt::Write;

use super::bottleneck::{ActorQueue, TwoActorHealth};
use super::box_component::BoxComponent;
use super::format::{
    QueueItemDisplay, classify_stories, render_dependency_chains, render_epic_capacities,
};
use crate::cli::presentation::flow::layout::LayoutConfig;
use crate::cli::presentation::theme::Theme;
use crate::cli::style;
use crate::domain::model::Board;
use crate::read_model::flow_metrics::FlowMetrics;

/// Render an annotated pipeline flow diagram.
pub fn render_annotated_flow(
    board: &Board,
    metrics: &FlowMetrics,
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

    writeln!(output, "{}", style::heavy_rule(width, Some(&theme))).unwrap();
    writeln!(output).unwrap();

    // 2. Queue Handoff (Pull System Health)
    let two_actor = crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(metrics);
    let queue_boxes = render_queue_boxes(&two_actor, width, &theme);
    writeln!(output, "{}", queue_boxes).unwrap();

    // 2b. Bottleneck Analysis
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

    // 3. Execution Capacity (Strategic Throughput)
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

/// Render side-by-side or stacked queue boxes for human/agent handoff.
pub fn render_queue_boxes(health: &TwoActorHealth, width: usize, theme: &Theme) -> String {
    if width >= 80 {
        render_side_by_side_queue_boxes(health, width, theme)
    } else {
        render_stacked_queue_boxes(health, width, theme)
    }
}

fn render_side_by_side_queue_boxes(health: &TwoActorHealth, width: usize, theme: &Theme) -> String {
    let mut output = String::new();
    let col_width = (width - 4) / 2;

    let mut human_box = BoxComponent::new("HUMAN QUEUE (To Start/Accept)", col_width);
    let mut agent_box = BoxComponent::new("AGENT QUEUE (To Implement)", col_width);

    // Populate Human box
    render_queue_into_box(&mut human_box, &health.human_queue, theme);

    // Populate Agent box
    render_queue_into_box(&mut agent_box, &health.agent_queue, theme);

    // Ensure both boxes have the same height
    let height = human_box.lines.len().max(agent_box.lines.len()) + 2;
    let human_lines = human_box.render_with_height(height);
    let agent_lines = agent_box.render_with_height(height);

    for i in 0..height {
        let left = human_lines[i].clone();
        let right = agent_lines[i].clone();
        writeln!(output, "{}  {}", left, right).unwrap();
    }

    output
}

fn render_queue_into_box(box_comp: &mut BoxComponent, queue: &ActorQueue, theme: &Theme) {
    if queue.items.is_empty() {
        box_comp.push_line(format!("  {}", "No items in queue".dimmed()));
    } else {
        for item in &queue.items {
            let display_item = QueueItemDisplay::from_item(item.clone());
            box_comp.push_line(display_item.render_to_string(box_comp.width() - 4, theme));
        }
    }

    if queue.is_starved {
        box_comp.push_rule();
        if let Some(ref msg) = queue.starvation_message {
            box_comp.push_line(format!("  {}", msg));
        }
    }
}

fn render_stacked_queue_boxes(health: &TwoActorHealth, width: usize, theme: &Theme) -> String {
    let mut output = String::new();

    let mut human_box = BoxComponent::new("HUMAN QUEUE (To Start/Accept)", width);
    render_queue_into_box(&mut human_box, &health.human_queue, theme);
    for line in human_box.render() {
        writeln!(output, "{}", line).unwrap();
    }

    writeln!(output).unwrap();

    let mut agent_box = BoxComponent::new("AGENT QUEUE (To Implement)", width);
    render_queue_into_box(&mut agent_box, &health.agent_queue, theme);
    for line in agent_box.render() {
        writeln!(output, "{}", line).unwrap();
    }

    output
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
    use crate::cli::presentation::flow::bottleneck::TwoActorHealth;
    use crate::read_model::flow_metrics::{
        ExecutionMetrics, GovernanceMetrics, PlanningMetrics, ResearchMetrics, VerificationMetrics,
    };

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

    fn make_test_two_actor_health() -> TwoActorHealth {
        crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(&make_test_metrics())
    }

    #[test]
    fn render_queue_boxes_contains_human_header() {
        let health = make_test_two_actor_health();
        let theme = Theme::default();
        let rendered = render_queue_boxes(&health, 100, &theme);
        assert!(rendered.contains("HUMAN QUEUE"));
    }

    #[test]
    fn render_queue_boxes_contains_agent_header() {
        let health = make_test_two_actor_health();
        let theme = Theme::default();
        let rendered = render_queue_boxes(&health, 100, &theme);
        assert!(rendered.contains("AGENT QUEUE"));
    }

    #[test]
    fn test_render_annotated_flow() {
        let board = Board::default();
        let metrics = make_test_metrics();
        let rendered = render_annotated_flow(&board, &metrics, 100, false);
        assert!(rendered.contains("Governance"));
        assert!(rendered.contains("Research"));
        assert!(rendered.contains("Planning"));
        assert!(rendered.contains("Execution"));
        assert!(rendered.contains("Verification"));
        assert!(rendered.contains("HUMAN QUEUE"));
        assert!(rendered.contains("AGENT QUEUE"));
        assert!(rendered.contains("No executable epic capacity"));
    }
}
