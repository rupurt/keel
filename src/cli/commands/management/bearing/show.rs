//! Show bearing command.

use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::domain::model::BearingStatus;
use crate::infrastructure::config::{find_board_dir, load_config};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::scoring::{calculate_score, load_assessment};
use crate::read_model::bearing_show::{
    self, BearingAssessmentSummary, BearingBriefSummary, BearingSurveySummary,
};

use super::guidance::{informational_for_show, print_human};
use super::{classify_fog, format_factor};

const BRIEF_PLACEHOLDER: &str = "(not authored in BRIEF.md yet)";
const SURVEY_PLACEHOLDER: &str = "(not authored in SURVEY.md yet)";
const ASSESSMENT_PLACEHOLDER: &str = "(not authored in ASSESSMENT.md yet)";
const NONE_PLACEHOLDER: &str = "(none)";

/// Show detailed bearing information.
pub fn run(pattern: &str) -> Result<()> {
    let board_dir = find_board_dir()?;
    let board = load_board(&board_dir)?;
    let (config, _source) = load_config();
    let weights = config.current_weights();
    let bearing = board.require_bearing(pattern)?;
    let bearing_dir = bearing.path.parent().unwrap_or(&bearing.path);

    let readme_content = fs::read_to_string(&bearing.path).unwrap_or_default();
    let brief_path = bearing_dir.join("BRIEF.md");
    let survey_path = bearing_dir.join("SURVEY.md");
    let assessment_path = bearing_dir.join("ASSESSMENT.md");
    let brief_content = fs::read_to_string(&brief_path).unwrap_or_default();
    let survey_content = fs::read_to_string(&survey_path).ok();
    let assessment_content = fs::read_to_string(&assessment_path).ok();

    let projection = bearing_show::build_bearing_show_projection(
        &readme_content,
        &brief_content,
        survey_content.as_deref(),
        assessment_content.as_deref(),
    );

    let width = crate::cli::presentation::terminal::get_terminal_width();

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", bearing.frontmatter.title.bold()))
        .row("Status:", bearing.frontmatter.status.to_string());

    if matches!(
        bearing.frontmatter.status,
        BearingStatus::Exploring | BearingStatus::Parked
    ) {
        let fog = classify_fog(&board_dir, bearing);
        metadata.push_row("Fog:", fog.as_banner().to_string());
    }

    metadata.push_optional_row(
        "Created:",
        bearing
            .frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
    );
    metadata.push_optional_row(
        "Laid:",
        bearing
            .frontmatter
            .laid_at
            .map(|laid_at| format!("{}", laid_at.dimmed())),
    );

    let mut skipped_date_keys = BTreeSet::from(["created_at".to_string()]);
    if bearing.frontmatter.laid_at.is_some() {
        skipped_date_keys.insert("laid_at".to_string());
    }

    for field in &projection.frontmatter_datetimes {
        if skipped_date_keys.contains(&field.key) {
            continue;
        }
        metadata.push_row(
            format!("{}:", field.label),
            format!("{}", field.value.dimmed()),
        );
    }

    metadata.push_row("Path:", format!("{}", bearing.path.display().dimmed()));

    let mut sections = vec![
        render_documents_section(bearing),
        render_brief_section(&projection.brief),
        render_survey_section(projection.survey.as_ref()),
        render_assessment_section(
            &assessment_path,
            projection.assessment.as_ref(),
            config.mode(),
            &weights,
        ),
    ];

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

fn render_documents_section(bearing: &crate::domain::model::Bearing) -> ShowSection {
    let mut section = ShowSection::new("Documents");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(15);
    fields.push_row("README.md:", format!("{}", "authored".green()));
    fields.push_row("BRIEF.md:", format!("{}", "authored".green()));
    fields.push_row(
        "SURVEY.md:",
        if bearing.has_survey {
            format!("{}", "authored".green())
        } else {
            format!("{}", "not created".dimmed())
        },
    );
    fields.push_row(
        "ASSESSMENT.md:",
        if bearing.has_assessment {
            format!("{}", "authored".green())
        } else {
            format!("{}", "not created".dimmed())
        },
    );
    section.push_key_values(fields);
    section
}

fn render_brief_section(summary: &BearingBriefSummary) -> ShowSection {
    let mut section = ShowSection::new("Brief");

    push_labeled_text_block(
        &mut section,
        "Hypothesis:",
        summary
            .hypothesis
            .as_deref()
            .unwrap_or(BRIEF_PLACEHOLDER)
            .to_string(),
    );
    push_labeled_text_block(
        &mut section,
        "Problem Space:",
        summary
            .problem_space
            .as_deref()
            .unwrap_or(BRIEF_PLACEHOLDER)
            .to_string(),
    );

    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(18);
    if summary.total_success_criteria == 0 {
        fields.push_row(
            "Success Criteria:",
            format!("{}", BRIEF_PLACEHOLDER.dimmed()),
        );
    } else {
        fields.push_row(
            "Success Criteria:",
            format!(
                "{}/{} checked",
                summary.checked_success_criteria, summary.total_success_criteria
            ),
        );
    }
    fields.push_row("Open Questions:", summary.open_questions.len().to_string());
    section.push_key_values(fields);

    section.push_labeled_bullets(
        "Open Questions:",
        styled_inline_items(&summary.open_questions),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Unchecked Criteria:",
        styled_inline_items(&summary.unchecked_success_criteria),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );

    section
}

fn render_survey_section(summary: Option<&BearingSurveySummary>) -> ShowSection {
    let mut section = ShowSection::new("Survey");
    let Some(summary) = summary else {
        section.push_lines([format!("  {}", SURVEY_PLACEHOLDER.dimmed())]);
        return section;
    };

    push_labeled_text_block(
        &mut section,
        "Feasibility:",
        summary
            .feasibility
            .as_deref()
            .unwrap_or(SURVEY_PLACEHOLDER)
            .to_string(),
    );

    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(13);
    fields.push_row("Key Findings:", summary.key_findings.len().to_string());
    fields.push_row("Unknowns:", summary.unknowns.len().to_string());
    section.push_key_values(fields);

    section.push_labeled_bullets(
        "Key Findings:",
        styled_inline_items(&summary.key_findings),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Unknowns:",
        styled_inline_items(&summary.unknowns),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );

    section
}

fn render_assessment_section(
    assessment_path: &Path,
    summary: Option<&BearingAssessmentSummary>,
    mode: &str,
    weights: &crate::infrastructure::config::ModeWeights,
) -> ShowSection {
    let mut section = ShowSection::new("Assessment");
    let Some(summary) = summary else {
        section.push_lines([format!("  {}", ASSESSMENT_PLACEHOLDER.dimmed())]);
        return section;
    };

    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(17);
    fields.push_row("Status:", format!("{}", "authored".green()));
    fields.push_row("Mode:", mode.to_string());

    if let Ok(factors) = load_assessment(assessment_path) {
        fields.push_row("Impact:", format_factor(factors.impact));
        fields.push_row("Confidence:", format_factor(factors.confidence));
        fields.push_row("Effort:", format_factor(factors.effort));
        fields.push_row("Risk:", format_factor(factors.risk));
        if factors.is_complete() {
            if let Ok(score) = calculate_score(&factors, weights) {
                fields.push_row("EV Score:", format!("{:.2}", score.weighted_score));
            }
        } else {
            fields.push_row("Missing Factors:", factors.missing_factors().join(", "));
        }
    }

    fields.push_optional_row(
        "Recommendation:",
        summary
            .recommendation
            .as_deref()
            .map(style::styled_inline_emphasis),
    );

    section.push_key_values(fields);

    push_labeled_text_block(
        &mut section,
        "Opportunity Cost:",
        summary
            .opportunity_cost
            .as_deref()
            .unwrap_or(NONE_PLACEHOLDER)
            .to_string(),
    );

    let mut counts = ShowKeyValues::new().with_indent(2).with_min_label_width(17);
    counts.push_row("Dependencies:", summary.dependencies.len().to_string());
    counts.push_row("Alternatives:", summary.alternatives.len().to_string());
    section.push_key_values(counts);

    section.push_labeled_bullets(
        "Dependencies:",
        styled_inline_items(&summary.dependencies),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Alternatives:",
        styled_inline_items(&summary.alternatives),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );

    section
}

fn push_labeled_text_block(section: &mut ShowSection, label: &str, value: String) {
    section.push_lines([format!("  {label}")]);

    let mut value_lines: Vec<String> = value
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| format!("    {}", style::styled_inline_emphasis(line)))
        .collect();

    if value_lines.is_empty() {
        value_lines.push(format!("    {}", NONE_PLACEHOLDER.dimmed()));
    }

    section.push_lines(value_lines);
}

fn styled_inline_items(items: &[String]) -> Vec<String> {
    items
        .iter()
        .map(|item| style::styled_inline_emphasis(item))
        .collect()
}
