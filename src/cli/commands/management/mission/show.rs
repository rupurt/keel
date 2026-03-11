//! Show mission command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use keel::infrastructure::loader::load_board;
use keel::read_model::mission_show::{self, MissionShowProjection};

/// Show mission details
pub fn run(id: &str, json: bool) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let mission = board.require_mission(id)?;
    let projection = mission_show::build_projection(&board, mission)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&projection)?);
        return Ok(());
    }

    render_human_output(&projection);
    Ok(())
}

fn render_human_output(projection: &MissionShowProjection) {
    let width = crate::cli::presentation::terminal::get_terminal_width();

    let metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", projection.title.bold()))
        .row(
            "Status:",
            style::styled_mission_status(&projection.status.parse().unwrap()),
        )
        .row_optional(
            "Signal:",
            projection
                .operator_signal
                .as_ref()
                .map(|s| format!("{}", s.italic().bright_cyan())),
        );

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));

    let mut sections = Vec::new();

    // Goals Section
    let mut goals_section = ShowSection::new("Goals");
    if projection.goals.is_empty() {
        goals_section.push_text_block("  (no goals defined yet)");
    } else {
        for goal in &projection.goals {
            goals_section.push_lines([format!(
                "  {} - {} ({})",
                style::styled_goal_id(&goal.id),
                goal.description,
                goal.verification.raw().dimmed()
            )]);
        }
    }
    sections.push(goals_section);

    // Child Entities Section
    let mut children_section = ShowSection::new("Children");
    let epics_count = projection.child_entities.epics.len();
    let bearings_count = projection.child_entities.bearings.len();
    let adrs_count = projection.child_entities.adrs.len();

    if epics_count == 0 && bearings_count == 0 && adrs_count == 0 {
        children_section.push_text_block("  (no child entities yet)");
    } else {
        if epics_count > 0 {
            children_section.push_lines([format!("  Epics ({}):", epics_count)]);
            for epic in &projection.child_entities.epics {
                children_section.push_lines([format!(
                    "    {} - {} ({})",
                    style::styled_epic_id(&epic.id),
                    epic.title,
                    epic.status.dimmed()
                )]);
            }
        }
        if bearings_count > 0 {
            children_section.push_lines([format!("  Bearings ({}):", bearings_count)]);
            for bearing in &projection.child_entities.bearings {
                children_section.push_lines([format!(
                    "    {} - {} ({})",
                    style::styled_story_id(&bearing.id),
                    bearing.title,
                    bearing.status.dimmed()
                )]);
            }
        }
        if adrs_count > 0 {
            children_section.push_lines([format!("  ADRs ({}):", adrs_count)]);
            for adr in &projection.child_entities.adrs {
                children_section.push_lines([format!(
                    "    {} - {} ({})",
                    style::styled_story_id(&adr.id),
                    adr.title,
                    adr.status.dimmed()
                )]);
            }
        }
    }
    sections.push(children_section);

    // LOG Summary Section
    let mut log_section = ShowSection::new("Latest Log Entry");
    if let Some(log_summary) = &projection.log_summary {
        log_section.push_text_block(format!("  {}", log_summary));
    } else {
        log_section.push_text_block("  (no log entries found)");
    }
    sections.push(log_section);

    document.push_sections_spaced(sections);
    document.print();
}
