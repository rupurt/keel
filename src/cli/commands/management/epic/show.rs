//! Show epic command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::duration::render_completed_with_length;
use crate::cli::presentation::planning_lineage;
use crate::cli::presentation::progress::render_count_bar;
use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::domain::model::{Board, Epic};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::utils::cmp_optional_index_then_id;
use crate::read_model::planning_show::{self, EpicShowProjection};

use std::fs;
use std::path::Path;

const PROBLEM_PLACEHOLDER: &str = "(not authored in PRD.md yet)";
const GOALS_PLACEHOLDER: &str = "(not authored in PRD.md yet)";
const SCOPE_DRIFT_PLACEHOLDER: &str = "(no scope drift detected)";
const REQUIREMENTS_PLACEHOLDER: &str = "(no authored requirements found in PRD.md)";
const PROBLEM_MAX_PARAGRAPHS: usize = 1;
const VERIFICATION_STRATEGY_PLACEHOLDER: &str =
    "(no authored verification strategy found in PRD.md)";
const ETA_PLACEHOLDER: &str = "insufficient throughput data (4-week window)";
const VERIFICATION_PLACEHOLDER: &str = "no verification annotations found yet";
const PRESS_RELEASE_GUIDANCE: &str = "optional; use for large user-facing value shifts";

/// Show epic details
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Show epic details with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(id)?;
    let report = build_epic_show_report(&board, epic)?;
    let width = crate::cli::presentation::terminal::get_terminal_width();

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", epic.frontmatter.title.bold()))
        .row("Status:", style::styled_epic_stage(&epic.status()));
    metadata.push_standard_timestamps(
        epic.frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
        report
            .started_at
            .map(|started_at| format!("{}", started_at.dimmed())),
        report
            .updated_at
            .map(|updated_at| format!("{}", updated_at.dimmed())),
        report
            .completed_at
            .map(|completed_at| render_completed_with_length(report.started_at, completed_at)),
    );
    let prd_path = epic.path.parent().unwrap_or(&epic.path).join("PRD.md");
    metadata.push_optional_row("Desc:", epic.frontmatter.description.clone());
    metadata.push_row("Path:", format!("{}", epic.path.display().dimmed()));
    metadata.push_row("PRD:", format!("{}", prd_path.display().dimmed()));

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    let mut sections = render_planning_sections(&report);
    sections.extend([
        render_requirement_coverage(&report),
        render_press_release(epic),
        render_progress(&report),
        render_verification_strategy(&report),
        render_verification_readiness(&report),
    ]);
    if let Some(voyages) = render_voyages(&board, epic) {
        sections.push(voyages);
    }
    document.push_sections_spaced(sections);
    document.print();

    Ok(())
}

fn build_epic_show_report(board: &Board, epic: &Epic) -> Result<EpicShowProjection> {
    planning_show::build_epic_show_projection(board, epic)
}

fn render_planning_sections(report: &EpicShowProjection) -> Vec<ShowSection> {
    vec![
        render_problem_section(report),
        render_goals_section(report),
        render_scope_drift_section(report),
    ]
}

fn render_problem_section(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Problem");
    section.push_text_block_limited(
        report
            .doc
            .problem_statement
            .as_deref()
            .unwrap_or(PROBLEM_PLACEHOLDER),
        PROBLEM_MAX_PARAGRAPHS,
    );
    section
}

fn render_goals_section(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Goals");
    section.push_bullets(
        report
            .doc
            .goals
            .iter()
            .map(|goal| style::styled_inline_markdown(goal)),
        Some(format!("{}", GOALS_PLACEHOLDER.dimmed())),
    );
    section
}

fn render_scope_drift_section(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Scope Drift");
    section.push_bullets(
        report
            .scope_drift
            .iter()
            .map(planning_lineage::format_scope_drift_row),
        Some(format!("{}", SCOPE_DRIFT_PLACEHOLDER.dimmed())),
    );
    section
}

fn render_progress(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Progress");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(21);
    fields.push_row(
        "Voyages:",
        render_count_bar(report.done_voyages, report.total_voyages, 15, None),
    );
    fields.push_row(
        "Stories:",
        render_count_bar(report.done_stories, report.total_stories, 15, None),
    );

    let eta = match report.eta.eta_weeks {
        Some(weeks) => format!(
            "~{weeks:.1} weeks ({:.1} stories/week, 4w)",
            report.eta.throughput_stories_per_week
        ),
        None if report.eta.remaining_stories == 0 => "complete".to_string(),
        None => ETA_PLACEHOLDER.to_string(),
    };
    fields.push_row("ETA:", eta);
    section.push_key_values(fields);
    section
}

fn render_verification_strategy(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Verification Strategy");
    section.push_labeled_bullets(
        "Techniques:",
        report.doc.verification_strategy.iter().cloned(),
        Some(format!("{}", VERIFICATION_STRATEGY_PLACEHOLDER.dimmed())),
    );
    section
}

fn render_requirement_coverage(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Requirement Coverage");

    if report.requirement_coverage.is_empty() {
        section.push_lines([format!("  {}", REQUIREMENTS_PLACEHOLDER.dimmed())]);
        return section;
    }

    for row in &report.requirement_coverage {
        let summary = if row.is_covered() {
            String::new()
        } else {
            " (uncovered)".to_string()
        };

        section.push_lines([format!(
            "  {} - {}{}",
            style::styled_requirement_id(&row.id),
            style::styled_inline_markdown(&row.description),
            summary
        )]);

        push_requirement_linkage(
            &mut section,
            "Linked goals",
            row.linked_goals.len(),
            row.linked_goals
                .iter()
                .map(|goal| style::styled_goal_id(goal)),
        );
        push_requirement_linkage(
            &mut section,
            "Linked children",
            row.linked_child_count(),
            row.linked_children.iter().map(|child| {
                format!(
                    "{}/{}",
                    style::styled_voyage_id(&child.voyage_id),
                    style::styled_requirement_id(&child.requirement_id)
                )
            }),
        );
    }

    section
}

fn push_requirement_linkage<I>(section: &mut ShowSection, label: &str, count: usize, values: I)
where
    I: IntoIterator<Item = String>,
{
    let values: Vec<String> = values.into_iter().collect();
    if values.is_empty() {
        return;
    }

    section.push_lines([format!("    {label} ({count}): {}", values.join(", "))]);
}

fn render_press_release(epic: &Epic) -> ShowSection {
    let mut section = ShowSection::new("Press Release");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(11);
    let press_release_path = epic
        .path
        .parent()
        .unwrap_or(&epic.path)
        .join("PRESS_RELEASE.md");

    if !press_release_path.exists() {
        fields.push_row("Status:", format!("{}", "not authored".dimmed()));
        fields.push_row("Guidance:", PRESS_RELEASE_GUIDANCE);
        section.push_key_values(fields);
        return section;
    }

    fields.push_row("Status:", format!("{}", "authored".green()));
    fields.push_row(
        "Path:",
        format!("{}", press_release_path.display().dimmed()),
    );
    if let Ok(content) = fs::read_to_string(&press_release_path)
        && let Some(headline) = extract_press_release_headline(&content)
    {
        fields.push_row("Headline:", headline);
    }
    section.push_key_values(fields);
    section
}

fn extract_press_release_headline(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("<!--")
            || trimmed == "---"
        {
            continue;
        }

        let normalized = trimmed
            .trim_start_matches("**")
            .trim_end_matches("**")
            .trim();
        if !normalized.is_empty() {
            return Some(normalized.to_string());
        }
    }

    None
}

fn render_verification_readiness(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Verification Readiness");
    let mut fields = ShowKeyValues::new().with_indent(2).with_min_label_width(22);
    let total_criteria =
        report.verification.automated_criteria + report.verification.manual_criteria;
    if total_criteria == 0 {
        section.push_lines([format!("  {}", VERIFICATION_PLACEHOLDER.dimmed())]);
    } else {
        fields.push_row(
            "Criteria:",
            format!(
                "automated {} | manual {}",
                report.verification.automated_criteria, report.verification.manual_criteria
            ),
        );
        fields.push_row(
            "Requirement coverage:",
            format!(
                "automated {} | manual {}",
                report.verification.automated_requirements.len(),
                report.verification.manual_requirements.len()
            ),
        );
    }

    let (text_count, media_count, other_count) = report.verification.artifact_counts();
    fields.push_row(
        "Artifact inventory:",
        format!(
            "text {} | media {} | other {}",
            text_count, media_count, other_count
        ),
    );
    section.push_key_values(fields);

    if report.verification.missing_linked_proofs > 0 {
        section.push_lines([format!(
            "  Warning: {} linked proof file(s) are missing from EVIDENCE/",
            report.verification.missing_linked_proofs
        )]);
    }

    section
}

fn render_voyages(board: &Board, epic: &Epic) -> Option<ShowSection> {
    let mut voyages = board.voyages_for_epic_id(epic.id());
    voyages.sort_by(|a, b| cmp_optional_index_then_id(a.index(), a.id(), b.index(), b.id()));

    if voyages.is_empty() {
        return None;
    }

    let mut section = ShowSection::new("Voyages");
    for voyage in voyages {
        section.push_lines([format!(
            "  {} - {} ({})",
            style::styled_voyage_id(voyage.id()),
            style::styled_inline_markdown(&voyage.frontmatter.title),
            style::styled_voyage_stage(&voyage.status())
        )]);
    }

    Some(section)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::domain::state_machine::invariants::{ScopeLineageIssue, ScopeLineageIssueKind};
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::{Duration, Local, NaiveDate};
    use std::fs;
    use std::path::Path;

    fn replace_frontmatter_timestamp(path: &Path, key: &str, value: &str) {
        let content = fs::read_to_string(path).unwrap();
        let mut replaced_or_inserted = false;
        let mut frontmatter_fence_count = 0;
        let mut lines = Vec::new();
        for line in content.lines() {
            if line.trim() == "---" {
                frontmatter_fence_count += 1;
                if !replaced_or_inserted && frontmatter_fence_count == 2 {
                    lines.push(format!("{key}: {value}"));
                    replaced_or_inserted = true;
                }
                lines.push(line.to_string());
                continue;
            }

            if !replaced_or_inserted && line.starts_with(&format!("{key}:")) {
                lines.push(format!("{key}: {value}"));
                replaced_or_inserted = true;
            } else {
                lines.push(line.to_string());
            }
        }
        assert!(
            replaced_or_inserted,
            "expected to find frontmatter block for key '{key}'"
        );
        fs::write(path, format!("{}\n", lines.join("\n"))).unwrap();
    }

    #[test]
    fn epic_duration_rendering_formats_elapsed_time() {
        let started = NaiveDate::from_ymd_opt(2026, 3, 4)
            .unwrap()
            .and_hms_opt(9, 0, 0)
            .unwrap();
        let completed = NaiveDate::from_ymd_opt(2026, 3, 5)
            .unwrap()
            .and_hms_opt(11, 30, 0)
            .unwrap();

        let value = render_completed_with_length(Some(started), completed);

        assert!(value.contains("1d 2h 30m"));
    }

    #[test]
    fn epic_show_derives_updated_from_latest_story_or_voyage_activity() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1").status("in-progress"))
            .story(TestStory::new("S1").scope("epic1/v1"))
            .build();

        let story_path = temp.path().join("stories/S1/README.md");
        replace_frontmatter_timestamp(&story_path, "updated_at", "2026-03-05T12:34:56");

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(&board, epic).unwrap();
        let expected = NaiveDate::from_ymd_opt(2026, 3, 5)
            .unwrap()
            .and_hms_opt(12, 34, 56)
            .unwrap();
        assert_eq!(report.updated_at, Some(expected));
    }

    #[test]
    fn test_show_epic() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1").status("planned"))
            .build();

        let result = run_with_dir(temp.path(), "epic1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_epic_not_found() {
        let temp = TestBoardBuilder::new().build();

        let result = run_with_dir(temp.path(), "NONEXISTENT");
        assert!(result.is_err());
    }

    #[test]
    fn epic_show_planning_summary_renders_problem_and_goals() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .build();

        let prd_path = temp.path().join("epics/epic1/PRD.md");
        fs::write(
            &prd_path,
            r#"# Epic 1 PRD

## Problem Statement

Teams cannot quickly understand planning intent and verification readiness.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Improve planning readability | epic show usefulness | 95% |

## Requirements

### Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-10 | Render actionable planning summaries. | must | Human acceptance speed |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(&board, epic).unwrap();

        assert_eq!(
            report.doc.problem_statement.as_deref(),
            Some("Teams cannot quickly understand planning intent and verification readiness.")
        );
        assert!(
            report
                .doc
                .goals
                .iter()
                .any(|goal| goal.contains("GOAL-01: Improve planning readability"))
        );
    }

    #[test]
    fn press_release_headline_extracts_first_authored_line() {
        let content = r#"# PRESS RELEASE: Sample

## FOR IMMEDIATE RELEASE

**Keel introduces Sample Epic: Better outcomes**

More details.
"#;
        assert_eq!(
            extract_press_release_headline(content).as_deref(),
            Some("Keel introduces Sample Epic: Better outcomes")
        );
    }

    #[test]
    fn render_press_release_reports_optional_when_missing() {
        let temp = TestBoardBuilder::new().epic(TestEpic::new("epic1")).build();

        let press_release = temp.path().join("epics/epic1/PRESS_RELEASE.md");
        if press_release.exists() {
            fs::remove_file(&press_release).unwrap();
        }

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let section = render_press_release(epic);
        let mut doc = ShowDocument::new();
        doc.push_sections_spaced([section]);
        let rendered = doc.render();
        assert!(rendered.contains("Status:"));
        assert!(rendered.contains("not authored"));
        assert!(rendered.contains(PRESS_RELEASE_GUIDANCE));
    }

    #[test]
    fn epic_show_eta_projection_4w_uses_recent_window_and_fallback() {
        let old_completion = (Local::now() - Duration::weeks(6))
            .format("%Y-%m-%dT%H:%M:%S")
            .to_string();

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .story(
                TestStory::new("DONE1")
                    .status(StoryState::Done)
                    .scope("epic1/v1")
                    .body("- [x] done"),
            )
            .story(
                TestStory::new("TODO1")
                    .status(StoryState::Backlog)
                    .scope("epic1/v1")
                    .body("- [ ] todo"),
            )
            .build();

        let done_story_path = temp.path().join("stories/DONE1/README.md");
        let done_story = fs::read_to_string(&done_story_path).unwrap();
        let done_story = done_story.replace(
            "status: done\n",
            &format!("status: done\ncompleted_at: {old_completion}\n"),
        );
        fs::write(done_story_path, done_story).unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(&board, epic).unwrap();

        assert_eq!(report.eta.remaining_stories, 1);
        assert_eq!(report.eta.eta_weeks, None);
    }

    #[test]
    fn epic_show_verification_surface_rolls_up_annotations_and_artifacts() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .story(TestStory::new("S1").scope("epic1/v1").body(
                r#"
## Acceptance Criteria
- [x] Automated check <!-- verify: cargo test, SRS-01:start, proof: ac-1.log -->
- [x] Manual check <!-- verify: manual, SRS-02:start:end, proof: ac-2.gif -->
"#,
            ))
            .build();

        let evidence_dir = temp.path().join("stories/S1/EVIDENCE");
        fs::write(evidence_dir.join("ac-1.log"), "ok").unwrap();
        fs::write(evidence_dir.join("ac-2.gif"), "gif").unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(&board, epic).unwrap();

        assert_eq!(report.verification.automated_criteria, 1);
        assert_eq!(report.verification.manual_criteria, 1);
        assert_eq!(report.verification.automated_requirements.len(), 1);
        assert_eq!(report.verification.manual_requirements.len(), 1);

        let (text_count, media_count, _) = report.verification.artifact_counts();
        assert_eq!(text_count, 1);
        assert_eq!(media_count, 1);
        assert!(
            report
                .verification
                .linked_artifacts
                .iter()
                .any(|artifact| artifact.ends_with("ac-1.log"))
        );
        assert!(
            report
                .verification
                .linked_artifacts
                .iter()
                .any(|artifact| artifact.ends_with("ac-2.gif"))
        );
    }

    #[test]
    fn epic_show_missing_data_placeholders_render_explicit_warnings() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .build();

        let prd_path = temp.path().join("epics/epic1/PRD.md");
        fs::write(
            prd_path,
            r#"# PRD

## Problem Statement
<!-- What user problem does this solve? -->

## Goals & Objectives
<!-- TODO: Add goals -->

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
<!-- END FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(&board, epic).unwrap();

        assert!(report.doc.problem_statement.is_none());
        assert!(report.doc.goals.is_empty());
        assert!(report.verification.linked_artifacts.is_empty());

        // Render placeholder paths explicitly to guarantee command output behavior.
        render_planning_sections(&report);
        render_verification_strategy(&report);
        render_verification_readiness(&report);
    }

    #[test]
    fn render_problem_section_places_value_on_its_own_line() {
        let report = EpicShowProjection {
            doc: planning_show::PlanningDocSummary {
                problem_statement: Some(
                    "Inline `problem` values are hard to scan in show output.".to_string(),
                ),
                ..planning_show::PlanningDocSummary::default()
            },
            scope_drift: Vec::new(),
            requirement_coverage: Vec::new(),
            total_voyages: 0,
            done_voyages: 0,
            total_stories: 0,
            done_stories: 0,
            started_at: None,
            completed_at: None,
            updated_at: None,
            eta: planning_show::EtaSummary {
                throughput_stories_per_week: 0.0,
                remaining_stories: 0,
                eta_weeks: None,
            },
            verification: planning_show::VerificationRollup::default(),
        };

        let section = render_problem_section(&report);
        let mut document = ShowDocument::new();
        document.push_sections_spaced([section]);
        let rendered = document.render();

        assert!(rendered.contains("Problem"));
        assert!(rendered.contains("    Inline "));
        assert!(rendered.contains("problem"));
        assert!(!rendered.contains("Planning Summary"));
    }

    #[test]
    fn render_problem_section_truncates_long_problem_with_ellipsis_line() {
        let report = EpicShowProjection {
            doc: planning_show::PlanningDocSummary {
                problem_statement: Some(
                    "Paragraph one.\nStill paragraph one.\n\nParagraph two.\n\nParagraph three."
                        .to_string(),
                ),
                ..planning_show::PlanningDocSummary::default()
            },
            scope_drift: Vec::new(),
            requirement_coverage: Vec::new(),
            total_voyages: 0,
            done_voyages: 0,
            total_stories: 0,
            done_stories: 0,
            started_at: None,
            completed_at: None,
            updated_at: None,
            eta: planning_show::EtaSummary {
                throughput_stories_per_week: 0.0,
                remaining_stories: 0,
                eta_weeks: None,
            },
            verification: planning_show::VerificationRollup::default(),
        };

        let section = render_problem_section(&report);
        let mut document = ShowDocument::new();
        document.push_sections_spaced([section]);
        let rendered = document.render();

        assert!(rendered.contains("    Paragraph one."));
        assert!(rendered.contains("    Still paragraph one."));
        assert!(rendered.contains("    ..."));
        assert!(!rendered.contains("    Paragraph two."));
    }

    #[test]
    fn planning_sections_render_as_top_level_headings() {
        let report = EpicShowProjection {
            doc: planning_show::PlanningDocSummary {
                problem_statement: Some("Make epic show easier to scan.".to_string()),
                goals: vec!["GOAL-01: Reduce duplication (review load -> lower)".to_string()],
                verification_strategy: vec!["FR-01: cargo test".to_string()],
            },
            scope_drift: Vec::new(),
            requirement_coverage: vec![planning_show::EpicRequirementCoverageRow {
                id: "FR-01".to_string(),
                description: "Keep requirement details in the dedicated coverage section."
                    .to_string(),
                linked_goals: vec!["GOAL-01".to_string()],
                linked_children: Vec::new(),
            }],
            total_voyages: 0,
            done_voyages: 0,
            total_stories: 0,
            done_stories: 0,
            started_at: None,
            completed_at: None,
            updated_at: None,
            eta: planning_show::EtaSummary {
                throughput_stories_per_week: 0.0,
                remaining_stories: 0,
                eta_weeks: None,
            },
            verification: planning_show::VerificationRollup::default(),
        };

        let mut document = ShowDocument::new();
        let mut sections = render_planning_sections(&report);
        sections.push(render_requirement_coverage(&report));
        document.push_sections_spaced(sections);
        let rendered = document.render();

        assert!(rendered.contains("Problem"));
        assert!(rendered.contains("Goals"));
        assert!(rendered.contains("Scope Drift"));
        assert!(!rendered.contains("Planning Summary"));
        assert!(!rendered.contains("Key Requirements:"));
        assert!(!rendered.contains("Verification Strategy:"));
        assert!(rendered.contains("Requirement Coverage"));
        assert!(rendered.contains("FR-01"));
    }

    #[test]
    fn verification_strategy_renders_as_own_section_before_readiness() {
        let report = EpicShowProjection {
            doc: planning_show::PlanningDocSummary {
                verification_strategy: vec!["FR-01: cargo test -p keel epic_show".to_string()],
                ..planning_show::PlanningDocSummary::default()
            },
            scope_drift: Vec::new(),
            requirement_coverage: Vec::new(),
            total_voyages: 0,
            done_voyages: 0,
            total_stories: 0,
            done_stories: 0,
            started_at: None,
            completed_at: None,
            updated_at: None,
            eta: planning_show::EtaSummary {
                throughput_stories_per_week: 0.0,
                remaining_stories: 0,
                eta_weeks: None,
            },
            verification: planning_show::VerificationRollup::default(),
        };

        let mut document = ShowDocument::new();
        document.push_sections_spaced([
            render_progress(&report),
            render_verification_strategy(&report),
            render_verification_readiness(&report),
        ]);
        let rendered = document.render();

        let strategy_idx = rendered.find("Verification Strategy").unwrap();
        let readiness_idx = rendered.find("Verification Readiness").unwrap();
        assert!(strategy_idx < readiness_idx);
        assert!(rendered.contains("Techniques:"));
        assert!(rendered.contains("cargo test -p keel epic_show"));
    }

    #[test]
    fn requirement_coverage_section_renders_counts_and_uncovered_rows() {
        let report = EpicShowProjection {
            doc: planning_show::PlanningDocSummary::default(),
            scope_drift: Vec::new(),
            requirement_coverage: vec![
                planning_show::EpicRequirementCoverageRow {
                    id: "FR-01".to_string(),
                    description: "Shared parent.".to_string(),
                    linked_goals: vec!["GOAL-01".to_string(), "GOAL-02".to_string()],
                    linked_children: vec![
                        planning_show::EpicRequirementCoverageChild {
                            voyage_id: "v1".to_string(),
                            requirement_id: "SRS-01".to_string(),
                        },
                        planning_show::EpicRequirementCoverageChild {
                            voyage_id: "v2".to_string(),
                            requirement_id: "SRS-02".to_string(),
                        },
                    ],
                },
                planning_show::EpicRequirementCoverageRow {
                    id: "FR-02".to_string(),
                    description: "Uncovered parent.".to_string(),
                    linked_goals: vec!["GOAL-01".to_string()],
                    linked_children: Vec::new(),
                },
            ],
            total_voyages: 0,
            done_voyages: 0,
            total_stories: 0,
            done_stories: 0,
            started_at: None,
            completed_at: None,
            updated_at: None,
            eta: planning_show::EtaSummary {
                throughput_stories_per_week: 0.0,
                remaining_stories: 0,
                eta_weeks: None,
            },
            verification: planning_show::VerificationRollup::default(),
        };

        let section = render_requirement_coverage(&report);
        let mut document = ShowDocument::new();
        document.push_sections_spaced([section]);
        let rendered = document.render();

        assert!(!rendered.contains("linked SRS row(s) across"));
        assert!(rendered.contains("Linked goals (2):"));
        assert!(rendered.contains("Linked goals (1):"));
        assert!(rendered.contains("GOAL-01"));
        assert!(rendered.contains("GOAL-02"));
        assert!(rendered.contains("Linked children (2):"));
        assert!(rendered.contains("Uncovered parent. (uncovered)"));
    }

    #[test]
    fn epic_show_keeps_goals_and_moves_goal_linkage_into_requirement_coverage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .build();

        let prd_path = temp.path().join("epics/epic1/PRD.md");
        fs::write(
            &prd_path,
            r#"# Epic 1 PRD

## Problem Statement

Teams cannot see whether goals are actually covered by PRD requirements.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-02 | Reduce review ambiguity | review time | -50% |
| GOAL-01 | Improve traceability | linked requirements | 100% |

## Requirements

### Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-02 | Surface uncovered goals. | GOAL-01 | should | Review coverage gaps |
| FR-01 | Render objective coverage. | GOAL-02 GOAL-01 | must | Planning clarity |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(&board, epic).unwrap();

        let requirements = render_requirement_coverage(&report);
        let mut document = ShowDocument::new();
        let mut sections = render_planning_sections(&report);
        sections.push(requirements);
        document.push_sections_spaced(sections);
        let rendered = document.render();

        assert!(rendered.contains("Goals"));
        assert!(!rendered.contains("Goal Coverage:"));
        assert!(rendered.contains("GOAL-01"));
        assert!(rendered.contains("GOAL-02"));
        assert!(rendered.contains("Linked goals (2):"));
        assert!(rendered.contains("Linked goals (1):"));
        assert!(rendered.contains("FR-01"));
        assert!(rendered.contains("FR-02"));
    }

    #[test]
    fn epic_show_renders_only_scope_drift_section() {
        let report = EpicShowProjection {
            doc: planning_show::PlanningDocSummary::default(),
            scope_drift: vec![planning_show::ScopeDriftRow {
                voyage_id: Some("v1".to_string()),
                issue: ScopeLineageIssue {
                    artifact_path: std::path::PathBuf::from("SRS.md"),
                    scope_id: Some("SCOPE-02".to_string()),
                    line: None,
                    kind: ScopeLineageIssueKind::UnknownScopeRef,
                },
            }],
            requirement_coverage: Vec::new(),
            total_voyages: 0,
            done_voyages: 0,
            total_stories: 0,
            done_stories: 0,
            started_at: None,
            completed_at: None,
            updated_at: None,
            eta: planning_show::EtaSummary {
                throughput_stories_per_week: 0.0,
                remaining_stories: 0,
                eta_weeks: None,
            },
            verification: planning_show::VerificationRollup::default(),
        };

        let section = render_scope_drift_section(&report);
        let mut document = ShowDocument::new();
        document.push_sections_spaced([section]);
        let rendered = document.render();

        assert!(!rendered.contains("Scope Coverage:"));
        assert!(rendered.contains("Scope Drift"));
        assert!(rendered.contains("SCOPE-02"));
    }
}
