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
use crate::infrastructure::scoring::{load_assessment, load_bearing_score};
use crate::read_model::bearing_show::{
    self, BearingAssessmentSummary, BearingBriefSummary, BearingEvidenceSourceSummary,
    BearingEvidenceSummary,
};

use super::guidance::{informational_for_show, print_human};
use super::{classify_fog, format_factor};

const BRIEF_PLACEHOLDER: &str = "(not authored in BRIEF.md yet)";
const EVIDENCE_PLACEHOLDER: &str = "(not authored in EVIDENCE.md yet)";
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
    let evidence_path = bearing_dir.join("EVIDENCE.md");
    let assessment_path = bearing_dir.join("ASSESSMENT.md");
    let brief_content = fs::read_to_string(&brief_path).unwrap_or_default();
    let evidence_content = fs::read_to_string(&evidence_path).ok();
    let assessment_content = fs::read_to_string(&assessment_path).ok();

    let projection = bearing_show::build_bearing_show_projection(
        &readme_content,
        &brief_content,
        evidence_content.as_deref(),
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
    push_document_path_rows(&mut metadata, &brief_path, &evidence_path, &assessment_path);

    let mut sections = vec![
        render_documents_section(bearing),
        render_brief_section(&projection.brief),
        render_evidence_section(bearing.id(), projection.evidence.as_ref()),
        render_assessment_section(
            bearing.id(),
            &evidence_path,
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
    let mut unauthored_documents: Vec<&str> = Vec::new();

    if bearing.has_evidence {
        // documented in inspect commands
    } else {
        unauthored_documents.push("EVIDENCE.md");
    }

    if bearing.has_assessment {
        // documented in inspect commands
    } else {
        unauthored_documents.push("ASSESSMENT.md");
    }

    if !unauthored_documents.is_empty() {
        section.push_lines([format!(
            "  unauthored: {}",
            unauthored_documents.join(", ")
        )]);
    }
    section.push_labeled_bullets(
        "Inspect:",
        bearing_file_commands(bearing.id()),
        None::<String>,
    );
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

    let success_criteria_placeholder = if summary.total_success_criteria == 0 {
        format!("{}", BRIEF_PLACEHOLDER.dimmed())
    } else if summary.unchecked_success_criteria.is_empty() {
        format!("{}", "all checked".green())
    } else {
        format!("{}", NONE_PLACEHOLDER.dimmed())
    };
    section.push_labeled_bullets(
        success_criteria_label(
            summary.checked_success_criteria,
            summary.total_success_criteria,
        ),
        summary.unchecked_success_criteria.iter().cloned(),
        Some(success_criteria_placeholder),
    );
    push_counted_bullets(
        &mut section,
        "Open Questions",
        summary.open_questions.iter().cloned(),
        dimmed_none_placeholder(),
    );

    section
}

fn render_evidence_section(
    bearing_id: &str,
    summary: Option<&BearingEvidenceSummary>,
) -> ShowSection {
    let mut section = ShowSection::new("Evidence");
    let Some(summary) = summary else {
        section.push_lines([format!("  {}", EVIDENCE_PLACEHOLDER.dimmed())]);
        return section;
    };

    section.push_labeled_excerpt(
        "Feasibility:",
        summary.feasibility.as_ref(),
        SECTION_EXCERPT_LIMITS,
        Some(EVIDENCE_PLACEHOLDER),
    );

    push_counted_bullets(
        &mut section,
        "Key Findings",
        summary.key_findings.iter().cloned(),
        dimmed_none_placeholder(),
    );
    push_counted_bullets(
        &mut section,
        "Unknowns",
        summary.unknowns.iter().cloned(),
        dimmed_none_placeholder(),
    );
    push_counted_bullets(
        &mut section,
        "Sources",
        summary.sources.iter().map(format_source_summary),
        dimmed_none_placeholder(),
    );
    section.push_labeled_bullets(
        "Drill Down:",
        [format!("keel bearing file {bearing_id} EVIDENCE")],
        None::<String>,
    );

    section
}

fn render_assessment_section(
    bearing_id: &str,
    evidence_path: &Path,
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

    if let Ok(analysis) = load_assessment(assessment_path) {
        fields.push_row("Impact:", format_factor(analysis.factors.impact));
        fields.push_row("Confidence:", format_factor(analysis.factors.confidence));
        fields.push_row("Effort:", format_factor(analysis.factors.effort));
        fields.push_row("Risk:", format_factor(analysis.factors.risk));
        if analysis.factors.is_complete() {
            match load_bearing_score(assessment_path, evidence_path, weights) {
                Ok(score) => {
                    fields.push_row("EV Score:", score.display());
                    fields.push_row(
                        "Evidence Multiplier:",
                        format!("{:.2}", score.evidence_multiplier),
                    );
                }
                Err(err) => fields.push_row("Evidence Support:", err.to_string()),
            }
        } else {
            fields.push_row(
                "Missing Factors:",
                analysis.factors.missing_factors().join(", "),
            );
        }
    }

    fields.push_optional_row(
        "Recommendation:",
        summary.recommendation.as_deref().map(ToString::to_string),
    );

    section.push_key_values(fields);

    push_counted_bullets(
        &mut section,
        "Findings",
        summary.findings.iter().cloned(),
        dimmed_none_placeholder(),
    );
    section.push_labeled_excerpt(
        "Opportunity Cost:",
        summary.opportunity_cost.as_ref(),
        SECTION_EXCERPT_LIMITS,
        Some(NONE_PLACEHOLDER),
    );

    push_counted_bullets(
        &mut section,
        "Dependencies",
        summary.dependencies.iter().cloned(),
        dimmed_none_placeholder(),
    );
    push_counted_bullets(
        &mut section,
        "Alternatives",
        summary.alternatives.iter().cloned(),
        dimmed_none_placeholder(),
    );
    section.push_labeled_bullets(
        "Drill Down:",
        [format!("keel bearing file {bearing_id} ASSESSMENT")],
        None::<String>,
    );

    section
}

fn bearing_file_commands(bearing_id: &str) -> Vec<String> {
    let mut commands = vec![
        format!("keel bearing file {bearing_id} README"),
        format!("keel bearing file {bearing_id} BRIEF"),
        format!("keel bearing file {bearing_id} EVIDENCE"),
        format!("keel bearing file {bearing_id} ASSESSMENT"),
    ];

    commands.dedup();
    commands
}

fn format_source_summary(source: &BearingEvidenceSourceSummary) -> String {
    format!(
        "{} {} via {} | authority {} | freshness {}",
        source.id, source.class, source.provenance, source.authority, source.freshness
    )
}

fn push_document_path_rows(
    metadata: &mut ShowKeyValues,
    brief_path: &Path,
    evidence_path: &Path,
    assessment_path: &Path,
) {
    metadata.push_row("BRIEF.md:", format!("{}", brief_path.display().dimmed()));
    metadata.push_row(
        "EVIDENCE.md:",
        format!("{}", evidence_path.display().dimmed()),
    );
    metadata.push_row(
        "ASSESSMENT.md:",
        format!("{}", assessment_path.display().dimmed()),
    );
}

fn counted_label(label: &str, count: usize) -> String {
    format!("{label}({count}):")
}

fn dimmed_none_placeholder() -> String {
    format!("{}", NONE_PLACEHOLDER.dimmed())
}

fn push_counted_bullets<I, S>(
    section: &mut ShowSection,
    label: &str,
    items: I,
    empty_placeholder: String,
) where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let items: Vec<String> = items
        .into_iter()
        .map(|item| item.as_ref().to_string())
        .collect();
    section.push_labeled_bullets(
        counted_label(label, items.len()),
        items,
        Some(empty_placeholder),
    );
}

fn success_criteria_label(checked: usize, total: usize) -> String {
    format!("Success Criteria({checked}/{total}):")
}

#[cfg(test)]
mod tests {
    use super::{
        BearingAssessmentSummary, BearingBriefSummary, BearingEvidenceSourceSummary,
        BearingEvidenceSummary, push_document_path_rows, render_assessment_section,
        render_brief_section, render_documents_section, render_evidence_section,
    };
    use crate::cli::presentation::show::{ShowDocument, ShowKeyValues};
    use crate::infrastructure::config::ModeWeights;
    use crate::infrastructure::markdown_sections::SectionExcerpt;
    use crate::test_helpers::BearingFactory;
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

        assert!(lines.contains(&"  Success Criteria(1/2):".to_string()));
        assert!(lines.contains(&"    - Criterion two".to_string()));
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with("  Success Criteria:"))
        );
        assert!(
            lines
                .iter()
                .all(|line| !line.starts_with("  Unchecked Criteria:"))
        );
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
    fn bearing_evidence_renders_inline_list_counts() {
        let section = render_evidence_section(
            "BRG-01",
            Some(&BearingEvidenceSummary {
                feasibility: Some(SectionExcerpt {
                    paragraphs: vec!["Looks feasible".to_string()],
                    list_items: Vec::new(),
                }),
                key_findings: vec!["Finding one".to_string()],
                unknowns: vec!["Unknown one".to_string(), "Unknown two".to_string()],
                sources: Vec::new(),
            }),
        );

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
    fn bearing_documents_section_shows_unauthed_row_only_when_needed() {
        let bearing_with_all_documents = BearingFactory::new("BRG-01")
            .has_evidence(true)
            .has_assessment(true)
            .build();
        let all_lines = render_lines(render_documents_section(&bearing_with_all_documents));

        assert!(
            all_lines.iter().all(|line| !line.contains("unauthored")),
            "should not show unauthored when all docs are present: {all_lines:?}"
        );

        let bearing_with_missing_documents = BearingFactory::new("BRG-01")
            .has_evidence(false)
            .has_assessment(false)
            .build();
        let missing_lines = render_lines(render_documents_section(&bearing_with_missing_documents));

        assert!(missing_lines.contains(&"  unauthored: EVIDENCE.md, ASSESSMENT.md".to_string()));
    }

    #[test]
    fn bearing_assessment_renders_inline_list_counts() {
        let section = render_assessment_section(
            "BRG-01",
            Path::new("/tmp/EVIDENCE.md"),
            Path::new("/tmp/ASSESSMENT.md"),
            Some(&BearingAssessmentSummary {
                recommendation: Some("Proceed".to_string()),
                findings: vec!["Finding one [SRC-01]".to_string()],
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

        assert!(lines.contains(&"  Findings(1):".to_string()));
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
            Path::new("/tmp/EVIDENCE.md"),
            Path::new("/tmp/ASSESSMENT.md"),
        );

        let mut document = ShowDocument::new();
        document.push_header(metadata, None);
        let rendered = document.render();

        let path_idx = rendered.find("Path:").unwrap();
        let brief_idx = rendered.find("BRIEF.md:").unwrap();
        let evidence_idx = rendered.find("EVIDENCE.md:").unwrap();
        let assessment_idx = rendered.find("ASSESSMENT.md:").unwrap();

        assert!(path_idx < brief_idx);
        assert!(brief_idx < evidence_idx);
        assert!(evidence_idx < assessment_idx);
        assert!(rendered.contains("/tmp/BRIEF.md"));
        assert!(rendered.contains("/tmp/EVIDENCE.md"));
        assert!(rendered.contains("/tmp/ASSESSMENT.md"));
    }

    #[test]
    fn bearing_section_excerpt_renders_list_items_below_paragraph() {
        let section = render_evidence_section(
            "BRG-01",
            Some(&BearingEvidenceSummary {
                feasibility: Some(SectionExcerpt {
                    paragraphs: vec!["Looks feasible".to_string()],
                    list_items: vec![
                        "First proof point".to_string(),
                        "Second proof point".to_string(),
                    ],
                }),
                key_findings: Vec::new(),
                unknowns: Vec::new(),
                sources: Vec::new(),
            }),
        );

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

    #[test]
    fn bearing_brief_renders_all_checked_success_criteria_without_none_placeholder() {
        let section = render_brief_section(&BearingBriefSummary {
            hypothesis: None,
            problem_space: None,
            checked_success_criteria: 2,
            total_success_criteria: 2,
            unchecked_success_criteria: Vec::new(),
            open_questions: Vec::new(),
        });

        let lines = render_lines(section);

        let success_criteria_line = lines
            .iter()
            .find(|line| line.contains("Success Criteria(2/2):"))
            .expect("success criteria line should be rendered");
        assert!(success_criteria_line.contains("all checked"));
        assert!(
            !success_criteria_line.contains("(none)"),
            "all checked state should not render the none placeholder: {lines:?}"
        );
    }

    #[test]
    fn bearing_show_renders_compact_evidence_provenance() {
        let bearing = BearingFactory::new("BRG-01")
            .has_evidence(true)
            .has_assessment(true)
            .build();
        let mut document = ShowDocument::new();
        document.push_section(render_documents_section(&bearing));
        document.push_spacer();
        document.push_section(render_evidence_section(
            "BRG-01",
            Some(&BearingEvidenceSummary {
                feasibility: None,
                key_findings: vec!["Finding one".to_string()],
                unknowns: vec!["Unknown one".to_string()],
                sources: vec![
                    BearingEvidenceSourceSummary {
                        id: "SRC-01".to_string(),
                        class: "academic".to_string(),
                        provenance: "manual:prior-art-review".to_string(),
                        authority: "high".to_string(),
                        freshness: "medium".to_string(),
                    },
                    BearingEvidenceSourceSummary {
                        id: "SRC-02".to_string(),
                        class: "web".to_string(),
                        provenance: "manual:official-doc".to_string(),
                        authority: "high".to_string(),
                        freshness: "high".to_string(),
                    },
                    BearingEvidenceSourceSummary {
                        id: "SRC-03".to_string(),
                        class: "social".to_string(),
                        provenance: "manual:community-signal".to_string(),
                        authority: "low".to_string(),
                        freshness: "high".to_string(),
                    },
                    BearingEvidenceSourceSummary {
                        id: "SRC-04".to_string(),
                        class: "manual".to_string(),
                        provenance: "manual:internal-notes".to_string(),
                        authority: "medium".to_string(),
                        freshness: "medium".to_string(),
                    },
                ],
            }),
        ));
        document.push_spacer();
        document.push_section(render_assessment_section(
            "BRG-01",
            Path::new("/tmp/EVIDENCE.md"),
            Path::new("/tmp/ASSESSMENT.md"),
            Some(&BearingAssessmentSummary {
                recommendation: Some("Proceed → convert to epic".to_string()),
                findings: vec!["Finding one [SRC-01]".to_string()],
                opportunity_cost: None,
                dependencies: vec!["Dependency one [SRC-02]".to_string()],
                alternatives: vec!["Alternative one [SRC-03]".to_string()],
            }),
            "balanced",
            &ModeWeights::default(),
        ));

        let rendered = document.render();

        assert!(rendered.contains("Sources(4):"));
        assert!(
            rendered.contains(
                "academic via manual:prior-art-review | authority high | freshness medium"
            )
        );
        assert!(rendered.contains("web via manual:official-doc | authority high | freshness high"));
        assert!(
            rendered.matches("SRC-01").count() >= 2,
            "expected highlighted source ids to remain visible in both evidence and findings: {rendered}"
        );
        assert!(rendered.contains("SRC-02"));
        assert!(rendered.contains("SRC-03"));
        assert!(rendered.contains("SRC-04"));
        assert!(!rendered.contains("... 1 more"));
        assert!(rendered.contains("Recommendation:"));
        assert!(rendered.contains("Proceed → convert to epic"));
        assert!(!rendered.contains("Proceed → convert to epic [SRC-01][SRC-02]"));
        assert!(!rendered.contains("Cited Sources:"));
        assert!(rendered.contains("keel bearing file BRG-01 README"));
        assert!(rendered.contains("keel bearing file BRG-01 BRIEF"));
        assert!(rendered.contains("keel bearing file BRG-01 EVIDENCE"));
        assert!(rendered.contains("keel bearing file BRG-01 ASSESSMENT"));
    }
}
