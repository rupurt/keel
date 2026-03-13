//! Show mission command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::drift_surface::render_drift_show_section;
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
    build_document(
        projection,
        crate::cli::presentation::terminal::get_terminal_width(),
    )
    .print();
}

fn build_document(projection: &MissionShowProjection, width: usize) -> ShowDocument {
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

    sections.push(render_drift_show_section(&projection.drift));

    // LOG Summary Section
    let mut log_section = ShowSection::new("Latest Log Entry");
    if let Some(log_summary) = &projection.log_summary {
        log_section.push_text_block(format!("  {}", log_summary));
    } else {
        log_section.push_text_block("  (no log entries found)");
    }
    sections.push(log_section);

    document.push_sections_spaced(sections);
    document
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::read_model::knowledge_graph::{
        DriftFacetKind, DriftFacetSummary, DriftSurfaceSummary,
    };
    use keel::read_model::mission_show::{EntitySummary, MissionChildren};

    fn sample_projection() -> MissionShowProjection {
        MissionShowProjection {
            id: "M1".to_string(),
            title: "Mission One".to_string(),
            status: "active".to_string(),
            goals: Vec::new(),
            child_entities: MissionChildren {
                epics: vec![EntitySummary {
                    id: "E1".to_string(),
                    title: "Epic One".to_string(),
                    status: "active".to_string(),
                }],
                bearings: Vec::new(),
                adrs: Vec::new(),
            },
            drift: DriftSurfaceSummary {
                coefficient: 0.41,
                facets: vec![
                    DriftFacetSummary {
                        kind: DriftFacetKind::EntityArtifacts,
                        covered: 3,
                        total: 5,
                    },
                    DriftFacetSummary {
                        kind: DriftFacetKind::KnowledgeProvenance,
                        covered: 1,
                        total: 2,
                    },
                    DriftFacetSummary {
                        kind: DriftFacetKind::SourceAttachments,
                        covered: 4,
                        total: 7,
                    },
                    DriftFacetSummary {
                        kind: DriftFacetKind::ProjectDocs,
                        covered: 2,
                        total: 3,
                    },
                ],
            },
            log_summary: Some("2026-03-12T17:00:00 - Latest".to_string()),
            operator_signal: None,
        }
    }

    #[test]
    fn mission_show_renders_structural_drift_section() {
        let rendered = build_document(&sample_projection(), 100).render();

        assert!(rendered.contains("Structural Drift"));
        assert!(rendered.contains("0.41 (elevated)"));
        assert!(rendered.contains("Coverage:"));
    }
}
