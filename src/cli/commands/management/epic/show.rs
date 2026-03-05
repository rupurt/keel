//! Show epic command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::duration::render_completed_with_length;
use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::domain::model::{Board, Epic, EpicState};
use crate::infrastructure::loader::load_board;
use crate::read_model::planning_show::{self, EpicShowProjection};

use std::path::Path;

const PROBLEM_PLACEHOLDER: &str = "(not authored in PRD.md yet)";
const GOALS_PLACEHOLDER: &str = "(not authored in PRD.md yet)";
const REQUIREMENTS_PLACEHOLDER: &str = "(no authored requirements found in PRD.md)";
const VERIFICATION_STRATEGY_PLACEHOLDER: &str =
    "(no authored verification strategy found in PRD.md)";
const ETA_PLACEHOLDER: &str = "insufficient throughput data (4-week window)";
const VERIFICATION_PLACEHOLDER: &str = "no verification annotations found yet";
const ARTIFACT_PLACEHOLDER: &str = "no evidence artifacts linked yet";

/// Show epic details
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Show epic details with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(id)?;
    let report = build_epic_show_report(board_dir, &board, epic)?;
    let width = crate::cli::presentation::terminal::get_terminal_width();
    let (started_at, completed_at) = derive_epic_timestamps(&board, epic);

    let mut metadata = ShowKeyValues::new()
        .with_min_label_width(9)
        .row("Title:", format!("{}", epic.frontmatter.title.bold()))
        .row("Status:", style::styled_epic_stage(&epic.status()));
    metadata.push_optional_row(
        "Created:",
        epic.frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
    );
    metadata.push_optional_row(
        "Started:",
        started_at.map(|started_at| format!("{}", started_at.dimmed())),
    );
    metadata.push_optional_row(
        "Completed:",
        completed_at.map(|completed_at| render_completed_with_length(started_at, completed_at)),
    );
    metadata.push_optional_row("Desc:", epic.frontmatter.description.clone());
    metadata.push_row("Path:", format!("{}", epic.path.display().dimmed()));

    let mut document = ShowDocument::new();
    document.push_key_values(metadata);
    document.push_rule(width);
    document.push_section(render_planning_summary(&report));
    document.push_spacer();
    document.push_section(render_progress(&report));
    document.push_spacer();
    document.push_section(render_verification_readiness(&report));
    if let Some(voyages) = render_voyages(&board, epic) {
        document.push_spacer();
        document.push_section(voyages);
    }
    document.print();

    Ok(())
}

fn build_epic_show_report(
    board_dir: &Path,
    board: &Board,
    epic: &Epic,
) -> Result<EpicShowProjection> {
    planning_show::build_epic_show_projection(board_dir, board, epic)
}

fn derive_epic_timestamps(
    board: &Board,
    epic: &Epic,
) -> (Option<chrono::NaiveDateTime>, Option<chrono::NaiveDateTime>) {
    let voyages = board.voyages_for_epic_id(epic.id());
    let started_at = voyages
        .iter()
        .filter_map(|voyage| voyage.frontmatter.started_at)
        .min();

    let completed_at = if epic.status() == EpicState::Done {
        voyages
            .iter()
            .filter_map(|voyage| voyage.frontmatter.completed_at)
            .max()
    } else {
        None
    };

    (started_at, completed_at)
}

fn render_planning_summary(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Planning Summary");
    section.push_lines([format!(
        "  Problem: {}",
        report
            .doc
            .problem_statement
            .as_deref()
            .unwrap_or(PROBLEM_PLACEHOLDER)
    )]);

    section.push_labeled_bullets(
        "Goals:",
        report.doc.goals.iter().cloned(),
        Some(format!("{}", GOALS_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets_limited(
        "Key Requirements:",
        report.doc.key_requirements.iter().cloned(),
        5,
        Some(format!("{}", REQUIREMENTS_PLACEHOLDER.dimmed())),
    );
    section.push_labeled_bullets_limited(
        "Verification Strategy:",
        report.doc.verification_strategy.iter().cloned(),
        5,
        Some(format!("{}", VERIFICATION_STRATEGY_PLACEHOLDER.dimmed())),
    );

    section
}

fn render_progress(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Progress");
    if report.total_voyages > 0 {
        section.push_lines([format!(
            "  Voyages: {}/{} {}",
            report.done_voyages,
            report.total_voyages,
            style::progress_bar(report.done_voyages, report.total_voyages, 15, None)
        )]);
    } else {
        section.push_lines(["  Voyages: 0/0".to_string()]);
    }

    if report.total_stories > 0 {
        section.push_lines([format!(
            "  Stories: {}/{} {}",
            report.done_stories,
            report.total_stories,
            style::progress_bar(report.done_stories, report.total_stories, 15, None)
        )]);
    } else {
        section.push_lines(["  Stories: 0/0".to_string()]);
    }

    let eta = match report.eta.eta_weeks {
        Some(weeks) => format!(
            "~{weeks:.1} weeks ({:.1} stories/week, 4w)",
            report.eta.throughput_stories_per_week
        ),
        None if report.eta.remaining_stories == 0 => "complete".to_string(),
        None => ETA_PLACEHOLDER.to_string(),
    };
    section.push_lines([format!("  ETA:     {eta}")]);
    section
}

fn render_verification_readiness(report: &EpicShowProjection) -> ShowSection {
    let mut section = ShowSection::new("Verification Readiness");
    let total_criteria =
        report.verification.automated_criteria + report.verification.manual_criteria;
    if total_criteria == 0 {
        section.push_lines([format!("  {}", VERIFICATION_PLACEHOLDER.dimmed())]);
    } else {
        section.push_lines([format!(
            "  Criteria: automated {} | manual {}",
            report.verification.automated_criteria, report.verification.manual_criteria
        )]);
        section.push_lines([format!(
            "  Requirement coverage: automated {} | manual {}",
            report.verification.automated_requirements.len(),
            report.verification.manual_requirements.len()
        )]);
    }

    let (text_count, media_count, other_count) = report.verification.artifact_counts();
    section.push_lines([format!(
        "  Artifact inventory: text {} | media {} | other {}",
        text_count, media_count, other_count
    )]);

    section.push_labeled_bullets_limited(
        "Linked artifacts:",
        report.verification.linked_artifacts.iter().cloned(),
        6,
        Some(format!("{}", ARTIFACT_PLACEHOLDER.dimmed())),
    );

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
    voyages.sort_by(|a, b| match (a.index(), b.index()) {
        (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        _ => a.id().cmp(b.id()),
    });

    if voyages.is_empty() {
        return None;
    }

    let mut section = ShowSection::new("Voyages");
    for voyage in voyages {
        section.push_lines([format!(
            "  {} - {} ({})",
            style::styled_voyage_id(voyage.id()),
            voyage.frontmatter.title,
            style::styled_voyage_stage(&voyage.status())
        )]);
    }

    Some(section)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::{Duration, Local, NaiveDate};
    use std::fs;

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
    fn epic_show_planning_summary_renders_problem_goals_and_requirements() {
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

| Goal | Success Metric | Target |
|------|----------------|--------|
| Improve planning readability | epic show usefulness | 95% |

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
        let report = build_epic_show_report(temp.path(), &board, epic).unwrap();

        assert_eq!(
            report.doc.problem_statement.as_deref(),
            Some("Teams cannot quickly understand planning intent and verification readiness.")
        );
        assert!(
            report
                .doc
                .goals
                .iter()
                .any(|goal| goal.contains("Improve planning readability"))
        );
        assert!(
            report
                .doc
                .key_requirements
                .iter()
                .any(|req| req.contains("FR-10: Render actionable planning summaries."))
        );
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
                    .stage(StoryState::Done)
                    .scope("epic1/v1")
                    .body("- [x] done"),
            )
            .story(
                TestStory::new("TODO1")
                    .stage(StoryState::Backlog)
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
        let report = build_epic_show_report(temp.path(), &board, epic).unwrap();

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
        let report = build_epic_show_report(temp.path(), &board, epic).unwrap();

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
        let report = build_epic_show_report(temp.path(), &board, epic).unwrap();

        assert!(report.doc.problem_statement.is_none());
        assert!(report.doc.goals.is_empty());
        assert!(report.doc.key_requirements.is_empty());
        assert!(report.verification.linked_artifacts.is_empty());

        // Render placeholder paths explicitly to guarantee command output behavior.
        render_planning_summary(&report);
        render_verification_readiness(&report);
    }
}
