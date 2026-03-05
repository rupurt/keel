//! Show bearing command.

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::domain::model::BearingStatus;
use crate::infrastructure::config::{find_board_dir, load_config};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::scoring::{calculate_score, load_assessment};

use super::guidance::{informational_for_show, print_human};
use super::{classify_fog, format_factor};

/// Show detailed bearing information.
pub fn run(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let (config, _source) = load_config();
    let weights = config.current_weights();
    let bearing = board.require_bearing(pattern)?;
    let width = crate::cli::presentation::terminal::get_terminal_width();

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", bearing.frontmatter.title.bold()))
        .row("ID:", bearing.id().to_string())
        .row("Status:", bearing.frontmatter.status.to_string());

    if matches!(
        bearing.frontmatter.status,
        BearingStatus::Exploring | BearingStatus::Parked
    ) {
        let fog = classify_fog(&board_dir, bearing);
        metadata.push_row("Fog:", fog.as_banner().to_string());
    }
    metadata.push_standard_timestamps(
        bearing.frontmatter.created_at.map(|d| d.to_string()),
        None,
        None,
        None,
    );

    let mut documents = ShowSection::new("Documents");
    documents.push_lines(["  README.md:     ✓ (frontmatter)".to_string()]);
    documents.push_lines(["  BRIEF.md:      ✓ (research)".to_string()]);
    documents.push_lines([format!(
        "  SURVEY.md:     {}",
        if bearing.has_survey {
            "✓"
        } else {
            "not created"
        }
    )]);
    documents.push_lines([format!(
        "  ASSESSMENT.md: {}",
        if bearing.has_assessment {
            "✓"
        } else {
            "not created"
        }
    )]);

    let mut sections = vec![documents];

    if bearing.has_assessment {
        let assessment_path = board_dir
            .join("bearings")
            .join(bearing.id())
            .join("ASSESSMENT.md");

        if let Ok(factors) = load_assessment(&assessment_path) {
            let mut scoring = ShowSection::new("EV Scoring");
            let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(11);
            fields.push_row("Mode:", config.mode().to_string());
            fields.push_row("Impact:", format_factor(factors.impact));
            fields.push_row("Confidence:", format_factor(factors.confidence));
            fields.push_row("Effort:", format_factor(factors.effort));
            fields.push_row("Risk:", format_factor(factors.risk));
            scoring.push_key_values(fields);

            if factors.is_complete() {
                if let Ok(score) = calculate_score(&factors, &weights) {
                    scoring.push_lines([format!("  EV Score: {:.2}", score.weighted_score)]);
                }
            } else {
                scoring.push_lines([format!(
                    "  Missing factors: {}",
                    factors.missing_factors().join(", ")
                )]);
            }

            sections.push(scoring);
        }
    }

    if let Some(reason) = &bearing.frontmatter.decline_reason {
        let mut decline = ShowSection::new("Decline Reason");
        decline.push_lines([reason.to_string()]);
        sections.push(decline);
    }

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    document.push_sections_spaced(sections);
    document.print();
    print_human(informational_for_show().as_ref());

    Ok(())
}
