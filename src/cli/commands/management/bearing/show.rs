//! Show bearing command.

use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use crate::cli::presentation::show::{ShowDocument, ShowExcerptLimits, ShowKeyValues, ShowSection};
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
const SECTION_EXCERPT_LIMITS: ShowExcerptLimits = ShowExcerptLimits {
    max_paragraphs: 1,
    max_list_items: 3,
};

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
    push_document_path_rows(&mut metadata, &brief_path, &survey_path, &assessment_path);

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
        decline.push_lines([style::styled_inline_markdown(reason)]);
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

    section.push_labeled_excerpt(
        "Hypothesis:",
        summary.hypothesis.as_ref(),
        SECTION_EXCERPT_LIMITS,
        Some(BRIEF_PLACEHOLDER),
    );
    section.push_labeled_excerpt(
        "Problem Space:",
        summary.problem_space.as_ref(),
        SECTION_EXCERPT_LIMITS,
        Some(BRIEF_PLACEHOLDER),
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
    section.push_key_values(fields);

    section.push_labeled_bullets(
        counted_label("Open Questions", summary.open_questions.len()),
        summary.open_questions.iter().cloned(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        "Unchecked Criteria:",
        summary.unchecked_success_criteria.iter().cloned(),
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

    section.push_labeled_excerpt(
        "Feasibility:",
        summary.feasibility.as_ref(),
        SECTION_EXCERPT_LIMITS,
        Some(SURVEY_PLACEHOLDER),
    );

    section.push_labeled_bullets(
        counted_label("Key Findings", summary.key_findings.len()),
        summary.key_findings.iter().cloned(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        counted_label("Unknowns", summary.unknowns.len()),
        summary.unknowns.iter().cloned(),
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
        summary.recommendation.as_deref().map(ToString::to_string),
    );

    section.push_key_values(fields);

    section.push_labeled_excerpt(
        "Opportunity Cost:",
        summary.opportunity_cost.as_ref(),
        SECTION_EXCERPT_LIMITS,
        Some(NONE_PLACEHOLDER),
    );

    section.push_labeled_bullets(
        counted_label("Dependencies", summary.dependencies.len()),
        summary.dependencies.iter().cloned(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets(
        counted_label("Alternatives", summary.alternatives.len()),
        summary.alternatives.iter().cloned(),
        Some(format!("{}", NONE_PLACEHOLDER.dimmed())),
    );

    section
}

fn push_document_path_rows(
    metadata: &mut ShowKeyValues,
    brief_path: &Path,
    survey_path: &Path,
    assessment_path: &Path,
) {
    metadata.push_row("BRIEF.md:", format!("{}", brief_path.display().dimmed()));
    metadata.push_row("SURVEY.md:", format!("{}", survey_path.display().dimmed()));
    metadata.push_row(
        "ASSESSMENT.md:",
        format!("{}", assessment_path.display().dimmed()),
    );
}

fn counted_label(label: &str, count: usize) -> String {
    format!("{label}({count}):")
}

#[cfg(test)]
mod tests {
    use super::{
        BearingAssessmentSummary, BearingBriefSummary, BearingSurveySummary,
        push_document_path_rows, render_assessment_section, render_brief_section,
        render_survey_section,
    };
    use crate::cli::presentation::show::{ShowDocument, ShowKeyValues};
    use crate::infrastructure::config::ModeWeights;
    use crate::infrastructure::markdown_sections::SectionExcerpt;
    use std::path::Path;

    fn render_lines(section: crate::cli::presentation::show::ShowSection) -> Vec<String> {
        let mut document = ShowDocument::new();
        document.push_section(section);
        document.render().lines().map(ToString::to_string).collect()
    }

    #[test]
    fn bearing_brief_renders_inline_open_question_count() {
        let section = render_brief_section(&BearingBriefSummary {
            hypothesis: Some(SectionExcerpt {
                paragraphs: vec!["Test hypothesis".to_string()],
                list_items: Vec::new(),
            }),
            problem_space: Some(SectionExcerpt {
                paragraphs: vec!["Test problem".to_string()],
                list_items: Vec::new(),
            }),
            checked_success_criteria: 1,
            total_success_criteria: 2,
            unchecked_success_criteria: vec!["Criterion two".to_string()],
            open_questions: vec!["Question one".to_string(), "Question two".to_string()],
        });

        let lines = render_lines(section);

        assert!(lines.contains(&"  Open Questions(2):".to_string()));
        assert!(lines.contains(&"    - Question one".to_string()));
        assert!(lines.contains(&"    - Question two".to_string()));
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with("  Open Questions:"))
        );
    }

    #[test]
    fn bearing_survey_renders_inline_list_counts() {
        let section = render_survey_section(Some(&BearingSurveySummary {
            feasibility: Some(SectionExcerpt {
                paragraphs: vec!["Looks feasible".to_string()],
                list_items: Vec::new(),
            }),
            key_findings: vec!["Finding one".to_string()],
            unknowns: vec!["Unknown one".to_string(), "Unknown two".to_string()],
        }));

        let lines = render_lines(section);

        assert!(lines.contains(&"  Key Findings(1):".to_string()));
        assert!(lines.contains(&"  Unknowns(2):".to_string()));
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with("  Key Findings:"))
        );
        assert!(lines.iter().all(|line| !line.starts_with("  Unknowns:")));
    }

    #[test]
    fn bearing_assessment_renders_inline_list_counts() {
        let section = render_assessment_section(
            Path::new("/tmp/ASSESSMENT.md"),
            Some(&BearingAssessmentSummary {
                recommendation: Some("Proceed".to_string()),
                opportunity_cost: Some(SectionExcerpt {
                    paragraphs: vec!["A roadmap delay".to_string()],
                    list_items: Vec::new(),
                }),
                dependencies: vec!["Dependency one".to_string()],
                alternatives: vec!["Alternative one".to_string(), "Alternative two".to_string()],
            }),
            "balanced",
            &ModeWeights::default(),
        );

        let lines = render_lines(section);

        assert!(lines.contains(&"  Dependencies(1):".to_string()));
        assert!(lines.contains(&"  Alternatives(2):".to_string()));
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with("  Dependencies:"))
        );
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with("  Alternatives:"))
        );
    }

    #[test]
    fn bearing_metadata_includes_document_paths_below_readme_path() {
        let mut metadata = ShowKeyValues::new().with_min_label_width(9);
        metadata.push_row(
            "Path:",
            format!("{}", Path::new("/tmp/README.md").display()),
        );
        push_document_path_rows(
            &mut metadata,
            Path::new("/tmp/BRIEF.md"),
            Path::new("/tmp/SURVEY.md"),
            Path::new("/tmp/ASSESSMENT.md"),
        );

        let mut document = ShowDocument::new();
        document.push_header(metadata, None);
        let rendered = document.render();

        let path_idx = rendered.find("Path:").unwrap();
        let brief_idx = rendered.find("BRIEF.md:").unwrap();
        let survey_idx = rendered.find("SURVEY.md:").unwrap();
        let assessment_idx = rendered.find("ASSESSMENT.md:").unwrap();

        assert!(path_idx < brief_idx);
        assert!(brief_idx < survey_idx);
        assert!(survey_idx < assessment_idx);
        assert!(rendered.contains("/tmp/BRIEF.md"));
        assert!(rendered.contains("/tmp/SURVEY.md"));
        assert!(rendered.contains("/tmp/ASSESSMENT.md"));
    }

    #[test]
    fn bearing_section_excerpt_renders_list_items_below_paragraph() {
        let section = render_survey_section(Some(&BearingSurveySummary {
            feasibility: Some(SectionExcerpt {
                paragraphs: vec!["Looks feasible".to_string()],
                list_items: vec![
                    "First proof point".to_string(),
                    "Second proof point".to_string(),
                ],
            }),
            key_findings: Vec::new(),
            unknowns: Vec::new(),
        }));

        let lines = render_lines(section);

        assert!(lines.contains(&"  Feasibility:".to_string()));
        assert!(lines.contains(&"    Looks feasible".to_string()));
        assert!(lines.contains(&"    - First proof point".to_string()));
        assert!(lines.contains(&"    - Second proof point".to_string()));
    }

    #[test]
    fn bearing_section_excerpt_adds_ellipsis_when_more_content_exists() {
        let section = render_brief_section(&BearingBriefSummary {
            hypothesis: Some(SectionExcerpt {
                paragraphs: vec!["Paragraph one".to_string(), "Paragraph two".to_string()],
                list_items: vec![
                    "Item one".to_string(),
                    "Item two".to_string(),
                    "Item three".to_string(),
                    "Item four".to_string(),
                ],
            }),
            problem_space: None,
            checked_success_criteria: 0,
            total_success_criteria: 0,
            unchecked_success_criteria: Vec::new(),
            open_questions: Vec::new(),
        });

        let lines = render_lines(section);

        assert!(lines.contains(&"    Paragraph one".to_string()));
        assert!(lines.contains(&"    - Item one".to_string()));
        assert!(lines.contains(&"    - Item three".to_string()));
        assert!(lines.contains(&"    ...".to_string()));
        assert!(!lines.contains(&"    Paragraph two".to_string()));
        assert!(!lines.contains(&"    - Item four".to_string()));
    }
}
