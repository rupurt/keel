//! Show epic command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::style;
use crate::domain::model::{Board, Epic};
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
const RECOMMENDATION_PLACEHOLDER: &str = "no high-signal automated verification additions detected";

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
    println!("{}", style::heavy_rule(width, None));
    println!(
        "{}",
        style::header(epic.id(), &epic.frontmatter.title, style::styled_epic_id)
    );
    println!("{}", style::heavy_rule(width, None));
    println!();

    println!("Status:   {}", style::styled_epic_stage(&epic.status()));
    if let Some(desc) = &epic.frontmatter.description {
        println!("Desc:     {}", desc);
    }
    render_planning_summary(&report);
    render_progress(&report);
    render_verification_readiness(&report);
    println!();
    println!("Path: {}", epic.path.display().dimmed());
    render_voyages(&board, epic);

    Ok(())
}

fn build_epic_show_report(
    board_dir: &Path,
    board: &Board,
    epic: &Epic,
) -> Result<EpicShowProjection> {
    planning_show::build_epic_show_projection(board_dir, board, epic)
}

fn render_planning_summary(report: &EpicShowProjection) {
    println!();
    println!("{}", "Planning Summary".bold());
    println!(
        "  Problem: {}",
        report
            .doc
            .problem_statement
            .as_deref()
            .unwrap_or(PROBLEM_PLACEHOLDER)
    );

    if report.doc.goals.is_empty() {
        println!("  Goals:   {}", GOALS_PLACEHOLDER.dimmed());
    } else {
        println!("  Goals:");
        for goal in &report.doc.goals {
            println!("    - {}", goal);
        }
    }

    if report.doc.key_requirements.is_empty() {
        println!("  Key Requirements: {}", REQUIREMENTS_PLACEHOLDER.dimmed());
    } else {
        println!("  Key Requirements:");
        for req in report.doc.key_requirements.iter().take(5) {
            println!("    - {}", req);
        }
        if report.doc.key_requirements.len() > 5 {
            println!(
                "    - ... {} more",
                report.doc.key_requirements.len().saturating_sub(5)
            );
        }
    }

    if report.doc.verification_strategy.is_empty() {
        println!(
            "  Verification Strategy: {}",
            VERIFICATION_STRATEGY_PLACEHOLDER.dimmed()
        );
    } else {
        println!("  Verification Strategy:");
        for item in report.doc.verification_strategy.iter().take(5) {
            println!("    - {}", item);
        }
        if report.doc.verification_strategy.len() > 5 {
            println!(
                "    - ... {} more",
                report.doc.verification_strategy.len().saturating_sub(5)
            );
        }
    }
}

fn render_progress(report: &EpicShowProjection) {
    println!();
    println!("{}", "Progress".bold());
    if report.total_voyages > 0 {
        println!(
            "  Voyages: {}/{} {}",
            report.done_voyages,
            report.total_voyages,
            style::progress_bar(report.done_voyages, report.total_voyages, 15, None)
        );
    } else {
        println!("  Voyages: 0/0");
    }

    if report.total_stories > 0 {
        println!(
            "  Stories: {}/{} {}",
            report.done_stories,
            report.total_stories,
            style::progress_bar(report.done_stories, report.total_stories, 15, None)
        );
    } else {
        println!("  Stories: 0/0");
    }

    let eta = match report.eta.eta_weeks {
        Some(weeks) => format!(
            "~{weeks:.1} weeks ({:.1} stories/week, 4w)",
            report.eta.throughput_stories_per_week
        ),
        None if report.eta.remaining_stories == 0 => "complete".to_string(),
        None => ETA_PLACEHOLDER.to_string(),
    };
    println!("  ETA:     {eta}");
}

fn render_verification_readiness(report: &EpicShowProjection) {
    println!();
    println!("{}", "Verification Readiness".bold());
    let total_criteria =
        report.verification.automated_criteria + report.verification.manual_criteria;
    if total_criteria == 0 {
        println!("  {}", VERIFICATION_PLACEHOLDER.dimmed());
    } else {
        println!(
            "  Criteria: automated {} | manual {}",
            report.verification.automated_criteria, report.verification.manual_criteria
        );
        println!(
            "  Requirement coverage: automated {} | manual {}",
            report.verification.automated_requirements.len(),
            report.verification.manual_requirements.len()
        );
    }

    if report.project_signals.is_empty() {
        println!("  Project signals: none detected");
    } else {
        println!("  Project signals: {}", report.project_signals.join(", "));
    }

    let (text_count, media_count, other_count) = report.verification.artifact_counts();
    println!(
        "  Artifact inventory: text {} | media {} | other {}",
        text_count, media_count, other_count
    );

    if report.verification.linked_artifacts.is_empty() {
        println!("  {}", ARTIFACT_PLACEHOLDER.dimmed());
    } else {
        println!("  Linked artifacts:");
        for artifact in report.verification.linked_artifacts.iter().take(6) {
            println!("    - {}", artifact);
        }
        if report.verification.linked_artifacts.len() > 6 {
            println!(
                "    - ... {} more",
                report.verification.linked_artifacts.len().saturating_sub(6)
            );
        }
    }

    if report.verification.missing_linked_proofs > 0 {
        println!(
            "  Warning: {} linked proof file(s) are missing from EVIDENCE/",
            report.verification.missing_linked_proofs
        );
    }

    for line in recommendation_lines(report) {
        println!("{}", line);
    }
}

pub(crate) fn recommendation_lines(report: &EpicShowProjection) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("  Technique Recommendations:".to_string());
    if report.recommendations.is_empty() {
        lines.push(format!("    - {}", RECOMMENDATION_PLACEHOLDER.dimmed()));
    } else {
        for recommendation in report.recommendations.iter().take(5) {
            lines.push(format!(
                "    - {}: {}",
                recommendation.technique, recommendation.rationale
            ));
        }
        if report.recommendations.len() > 5 {
            lines.push(format!(
                "    - ... {} more",
                report.recommendations.len().saturating_sub(5)
            ));
        }
    }
    lines.push(
        "  Advisory only: recommended techniques are not executed by show commands.".to_string(),
    );
    lines
}

fn render_voyages(board: &Board, epic: &Epic) {
    let mut voyages = board.voyages_for_epic_id(epic.id());
    voyages.sort_by(|a, b| match (a.index(), b.index()) {
        (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        _ => a.id().cmp(b.id()),
    });

    if voyages.is_empty() {
        return;
    }

    println!();
    println!("Voyages:");
    for voyage in voyages {
        println!(
            "  {} - {} ({})",
            style::styled_voyage_id(voyage.id()),
            voyage.frontmatter.title,
            style::styled_voyage_stage(&voyage.status())
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::read_model::verification_techniques;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::{Duration, Local};
    use std::collections::BTreeSet;
    use std::fs;

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

    #[test]
    fn epic_show_verification_recommendations_follow_project_signals() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .story(TestStory::new("S1").scope("epic1/v1").body(
                r#"
## Acceptance Criteria
- [x] Needs manual review <!-- verify: manual, SRS-01:start -->
"#,
            ))
            .build();

        fs::write(
            temp.path().join("Cargo.toml"),
            r#"[package]
name = "signal-proj"
version = "0.1.0"
edition = "2021"
"#,
        )
        .unwrap();
        fs::write(
            temp.path().join("flake.nix"),
            r#"{ pkgs }: {
  devShell = pkgs.mkShell {
    buildInputs = [ pkgs.vhs pkgs.ffmpeg ];
  };
}"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("epic1").unwrap();
        let report = build_epic_show_report(temp.path(), &board, epic).unwrap();

        assert!(
            report
                .project_signals
                .iter()
                .any(|signal| signal.contains("Cargo.toml (Rust workspace)"))
        );
        assert!(
            report
                .project_signals
                .iter()
                .any(|signal| signal.contains("flake.nix includes vhs"))
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|recommendation| recommendation.technique.contains("Rust Coverage Gate"))
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|recommendation| recommendation.technique.contains("VHS CLI Recording"))
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|recommendation| { recommendation.technique.contains("LLM-Judge") })
        );
    }

    #[test]
    fn show_recommendation_sections() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1"))
            .story(TestStory::new("S1").scope("e1/v1").body(
                r#"
## Acceptance Criteria
- [x] [SRS-04/AC-01] show recommendations <!-- verify: cargo test --lib show_recommendation_sections, SRS-04:start -->
"#,
            ))
            .build();

        fs::write(temp.path().join("Cargo.toml"), "[package]\nname=\"demo\"\n").unwrap();

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();
        let epic_report = build_epic_show_report(temp.path(), &board, epic).unwrap();
        let epic_lines = recommendation_lines(&epic_report);

        let recommendation_report = verification_techniques::build_show_recommendation_report(
            temp.path(),
            &BTreeSet::new(),
        );
        let voyage_lines = crate::cli::commands::management::voyage::show::recommendation_lines(
            &recommendation_report,
        );
        let story_lines = crate::cli::commands::management::story::show::recommendation_lines(
            &recommendation_report,
        );

        assert!(
            epic_lines
                .iter()
                .any(|line| line.contains("Technique Recommendations"))
        );
        assert!(
            voyage_lines
                .iter()
                .any(|line| line.contains("Technique Recommendations"))
        );
        assert!(
            story_lines
                .iter()
                .any(|line| line.contains("Technique Recommendations"))
        );
    }

    #[test]
    fn show_recommendation_usage_status() {
        let temp = TestBoardBuilder::new().build();
        fs::write(temp.path().join("Cargo.toml"), "[package]\nname=\"demo\"\n").unwrap();
        fs::write(
            temp.path().join("flake.nix"),
            "buildInputs = [ pkgs.vhs pkgs.ffmpeg ];",
        )
        .unwrap();

        let mut used = BTreeSet::new();
        used.insert("llm-judge".to_string());

        let report = verification_techniques::build_show_recommendation_report(temp.path(), &used);
        let lines = crate::cli::commands::management::story::show::recommendation_lines(&report);

        assert!(lines.iter().any(|line| line.contains("configured-in-use")));
        assert!(lines.iter().any(|line| line.contains("configured-unused")));
        assert!(lines.iter().any(|line| line.contains("LLM-Judge")));
        assert!(lines.iter().any(|line| line.contains("VHS CLI Recording")));
    }

    #[test]
    fn show_recommendations_do_not_execute() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1"))
            .story(TestStory::new("S1").scope("e1/v1").body(
                r#"
## Acceptance Criteria
- [x] [SRS-04/AC-02] advisory recommendation surface <!-- verify: vhs demo.tape, SRS-NFR-02:start:end -->
"#,
            ))
            .build();

        let evidence_dir = temp.path().join("stories/S1/EVIDENCE");
        let before_count = fs::read_dir(&evidence_dir).unwrap().count();

        run_with_dir(temp.path(), "e1").unwrap();
        crate::cli::commands::management::voyage::show::run_with_dir(temp.path(), "v1").unwrap();
        crate::cli::commands::management::story::show::run_with_dir(temp.path(), "S1").unwrap();

        let after_count = fs::read_dir(&evidence_dir).unwrap().count();
        assert_eq!(before_count, after_count);

        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();
        let report = build_epic_show_report(temp.path(), &board, epic).unwrap();
        let lines = recommendation_lines(&report);
        assert!(lines.iter().any(|line| line.contains("Advisory only")));
    }
}
