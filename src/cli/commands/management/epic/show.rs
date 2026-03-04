//! Show epic command

use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::BTreeSet;
use std::fs;

use crate::cli::style;
use crate::domain::model::{Board, Epic};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification::parser::{Comparison, parse_verify_annotations};

use std::path::Path;

const PROBLEM_PLACEHOLDER: &str = "(not authored in PRD.md yet)";
const GOALS_PLACEHOLDER: &str = "(not authored in PRD.md yet)";
const REQUIREMENTS_PLACEHOLDER: &str = "(no authored requirements found in PRD.md)";
const ETA_PLACEHOLDER: &str = "insufficient throughput data (4-week window)";
const VERIFICATION_PLACEHOLDER: &str = "no verification annotations found yet";
const ARTIFACT_PLACEHOLDER: &str = "no evidence artifacts linked yet";
const RECOMMENDATION_PLACEHOLDER: &str = "no high-signal automated verification additions detected";

#[derive(Debug, Default)]
struct PrdSummary {
    problem_statement: Option<String>,
    goals: Vec<String>,
    key_requirements: Vec<String>,
}

#[derive(Debug, Default)]
struct VerificationRollup {
    automated_requirements: BTreeSet<String>,
    manual_requirements: BTreeSet<String>,
    automated_criteria: usize,
    manual_criteria: usize,
    used_techniques: BTreeSet<String>,
    linked_artifacts: BTreeSet<String>,
    all_artifacts: BTreeSet<String>,
    missing_linked_proofs: usize,
}

impl VerificationRollup {
    fn artifact_counts(&self) -> (usize, usize, usize) {
        let mut text = 0;
        let mut media = 0;
        let mut other = 0;

        for artifact in &self.all_artifacts {
            if is_media_artifact(artifact) {
                media += 1;
            } else if is_text_artifact(artifact) {
                text += 1;
            } else {
                other += 1;
            }
        }

        (text, media, other)
    }
}

#[derive(Debug)]
struct EtaSummary {
    throughput_stories_per_week: f64,
    remaining_stories: usize,
    eta_weeks: Option<f64>,
}

#[derive(Debug)]
struct EpicShowReport {
    prd: PrdSummary,
    total_voyages: usize,
    done_voyages: usize,
    total_stories: usize,
    done_stories: usize,
    eta: EtaSummary,
    verification: VerificationRollup,
    project_signals: Vec<String>,
    recommendations: Vec<VerificationRecommendation>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct VerificationRecommendation {
    technique: String,
    rationale: String,
}

#[derive(Debug, Default)]
struct ProjectSignals {
    rust_workspace: bool,
    node_workspace: bool,
    playwright_project: bool,
    vhs_available: bool,
    ffmpeg_available: bool,
}

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

fn build_epic_show_report(board_dir: &Path, board: &Board, epic: &Epic) -> Result<EpicShowReport> {
    let prd_path = epic.path.parent().unwrap().join("PRD.md");
    let prd_content = fs::read_to_string(&prd_path).unwrap_or_default();
    let prd = parse_prd_summary(&prd_content);

    let voyages = board.voyages_for_epic_id(epic.id());
    let done_voyages = voyages.iter().filter(|v| v.status().to_string() == "done").count();

    let stories: Vec<_> = board
        .stories
        .values()
        .filter(|story| story.epic() == Some(epic.id()))
        .collect();
    let done_stories = stories
        .iter()
        .filter(|story| story.stage.to_string() == "done")
        .count();
    let total_stories = stories.len();
    let remaining_stories = total_stories.saturating_sub(done_stories);

    let throughput = crate::cli::presentation::flow::throughput::calculate_throughput(board, 4);
    let throughput_stories_per_week = throughput.avg_stories_per_week;
    let eta_weeks = if remaining_stories > 0 && throughput_stories_per_week > 0.0 {
        Some(remaining_stories as f64 / throughput_stories_per_week)
    } else {
        None
    };

    let mut verification = VerificationRollup::default();
    for story in stories {
        let story_content = fs::read_to_string(&story.path).unwrap_or_default();
        for ann in parse_verify_annotations(&story_content) {
            if let Some(command) = ann.command.as_deref() {
                let command = command.trim();
                if command.starts_with("vhs ") || command == "vhs" {
                    verification.used_techniques.insert("vhs".to_string());
                }
                if command.starts_with("llm-judge") || command == "llm-judge" {
                    verification.used_techniques.insert("llm-judge".to_string());
                }
                if command.contains("playwright") {
                    verification.used_techniques.insert("playwright".to_string());
                }
            }

            if ann.comparison == Comparison::Manual {
                verification.manual_criteria += 1;
                if let Some(req) = ann.requirement {
                    verification.manual_requirements.insert(req.id);
                }
            } else {
                verification.automated_criteria += 1;
                if let Some(req) = ann.requirement {
                    verification.automated_requirements.insert(req.id);
                }
            }

            if let Some(proof) = ann.proof {
                let proof_path = story.path.parent().unwrap().join("EVIDENCE").join(&proof);
                let rel = format!("stories/{}/EVIDENCE/{}", story.id(), proof);
                if proof_path.exists() {
                    verification.linked_artifacts.insert(rel);
                } else {
                    verification.missing_linked_proofs += 1;
                }
            }
        }

        let evidence_dir = story.path.parent().unwrap().join("EVIDENCE");
        if evidence_dir.exists()
            && let Ok(entries) = fs::read_dir(evidence_dir)
        {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    verification
                        .all_artifacts
                        .insert(format!("stories/{}/EVIDENCE/{}", story.id(), name));
                }
            }
        }
    }

    let signals = detect_project_signals(board_dir);
    let project_signals = render_project_signals(&signals);
    let recommendations = build_verification_recommendations(&signals, &verification);

    Ok(EpicShowReport {
        prd,
        total_voyages: voyages.len(),
        done_voyages,
        total_stories,
        done_stories,
        eta: EtaSummary {
            throughput_stories_per_week,
            remaining_stories,
            eta_weeks,
        },
        verification,
        project_signals,
        recommendations,
    })
}

fn parse_prd_summary(content: &str) -> PrdSummary {
    let problem_statement = extract_section(content, "## Problem Statement")
        .and_then(|section| first_authored_text(&section));
    let goals = extract_section(content, "## Goals & Objectives")
        .map(|section| parse_goals(&section))
        .unwrap_or_default();

    let mut key_requirements = parse_requirements_block(
        content,
        "BEGIN FUNCTIONAL_REQUIREMENTS",
        "END FUNCTIONAL_REQUIREMENTS",
    );
    key_requirements.extend(parse_requirements_block(
        content,
        "BEGIN NON_FUNCTIONAL_REQUIREMENTS",
        "END NON_FUNCTIONAL_REQUIREMENTS",
    ));

    PrdSummary {
        problem_statement,
        goals,
        key_requirements,
    }
}

fn extract_section(content: &str, heading: &str) -> Option<String> {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|c| *c == '#').count();

    for line in content.lines() {
        if line.starts_with(heading) {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with('#') {
                let level = line.chars().take_while(|c| *c == '#').count();
                if level <= heading_level {
                    break;
                }
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    if result.trim().is_empty() {
        None
    } else {
        Some(result)
    }
}

fn first_authored_text(section: &str) -> Option<String> {
    section
        .lines()
        .map(str::trim)
        .find(|line| {
            !line.is_empty()
                && !line.starts_with("<!--")
                && !line.starts_with('|')
                && !line.starts_with('-')
                && !line.contains("What user problem")
                && !line.contains("TODO:")
        })
        .map(ToOwned::to_owned)
}

fn parse_goals(section: &str) -> Vec<String> {
    let mut goals = Vec::new();
    for line in section.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('|') {
            let cols: Vec<&str> = trimmed
                .split('|')
                .map(str::trim)
                .filter(|col| !col.is_empty())
                .collect();
            if cols.len() >= 3
                && !cols[0].eq_ignore_ascii_case("Goal")
                && !cols[0].starts_with("---")
            {
                goals.push(format!("{} ({} -> {})", cols[0], cols[1], cols[2]));
            }
            continue;
        }

        if trimmed.starts_with("- ") && !trimmed.contains("TODO:") {
            goals.push(trimmed.trim_start_matches("- ").to_string());
        }
    }
    goals
}

fn parse_requirements_block(content: &str, start_marker: &str, end_marker: &str) -> Vec<String> {
    let mut in_block = false;
    let mut requirements = Vec::new();

    for line in content.lines() {
        if line.contains(start_marker) {
            in_block = true;
            continue;
        }
        if line.contains(end_marker) {
            break;
        }
        if !in_block {
            continue;
        }

        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            continue;
        }

        let cols: Vec<&str> = trimmed
            .split('|')
            .map(str::trim)
            .filter(|col| !col.is_empty())
            .collect();
        if cols.len() < 2 {
            continue;
        }

        let id = cols[0];
        let requirement = cols[1];
        if id.eq_ignore_ascii_case("ID")
            || id.starts_with("---")
            || requirement.eq_ignore_ascii_case("TODO")
            || requirement.eq_ignore_ascii_case("Test")
            || requirement.contains("primary user workflow")
            || requirement.contains("operational reliability")
        {
            continue;
        }

        requirements.push(format!("{id}: {requirement}"));
    }

    requirements
}

fn detect_project_signals(board_dir: &Path) -> ProjectSignals {
    let project_root = if board_dir
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name == ".keel")
        .unwrap_or(false)
    {
        board_dir.parent().unwrap_or(board_dir)
    } else {
        board_dir
    };

    let flake_content = fs::read_to_string(project_root.join("flake.nix")).unwrap_or_default();
    let package_json = fs::read_to_string(project_root.join("package.json")).unwrap_or_default();

    let playwright_project = project_root.join("playwright.config.ts").exists()
        || project_root.join("playwright.config.js").exists()
        || package_json.contains("playwright");

    ProjectSignals {
        rust_workspace: project_root.join("Cargo.toml").exists(),
        node_workspace: project_root.join("package.json").exists(),
        playwright_project,
        vhs_available: flake_content.contains("pkgs.vhs")
            || flake_content.contains(" vhs")
            || flake_content.contains("\tvhs"),
        ffmpeg_available: flake_content.contains("pkgs.ffmpeg")
            || flake_content.contains(" ffmpeg")
            || flake_content.contains("\tffmpeg"),
    }
}

fn render_project_signals(signals: &ProjectSignals) -> Vec<String> {
    let mut out = Vec::new();
    if signals.rust_workspace {
        out.push("Cargo.toml (Rust workspace)".to_string());
    }
    if signals.node_workspace {
        out.push("package.json (Node workspace)".to_string());
    }
    if signals.playwright_project {
        out.push("Playwright config/dependency".to_string());
    }
    if signals.vhs_available {
        out.push("flake.nix includes vhs".to_string());
    }
    if signals.ffmpeg_available {
        out.push("flake.nix includes ffmpeg".to_string());
    }
    out
}

fn build_verification_recommendations(
    signals: &ProjectSignals,
    verification: &VerificationRollup,
) -> Vec<VerificationRecommendation> {
    let mut recommendations = Vec::new();

    if signals.rust_workspace {
        recommendations.push(VerificationRecommendation {
            technique: "Coverage Gate".to_string(),
            rationale: "Detected Cargo.toml: add coverage threshold checks to reduce silent test gaps (for example `just coverage`).".to_string(),
        });
        recommendations.push(VerificationRecommendation {
            technique: "Property/Fuzz Tests".to_string(),
            rationale: "Rust parser/state-machine code is present: add `cargo fuzz` targets for high-risk boundary behavior.".to_string(),
        });
    }

    if signals.playwright_project && !verification.used_techniques.contains("playwright") {
        recommendations.push(VerificationRecommendation {
            technique: "Playwright Video E2E".to_string(),
            rationale: "Detected Playwright project signals: record browser acceptance runs with video and traces for faster review.".to_string(),
        });
    }

    if signals.vhs_available && !verification.used_techniques.contains("vhs") {
        recommendations.push(VerificationRecommendation {
            technique: "VHS CLI Recording".to_string(),
            rationale: "Detected vhs in flake.nix: capture deterministic terminal acceptance evidence with tape-driven recordings.".to_string(),
        });
    }

    if verification.manual_criteria > 0 && !verification.used_techniques.contains("llm-judge") {
        recommendations.push(VerificationRecommendation {
            technique: "LLM-Judge Semantic Checks".to_string(),
            rationale: format!(
                "Detected {} manual criterion/criteria: convert high-value manual checks into repeatable `llm-judge` assertions where possible.",
                verification.manual_criteria
            ),
        });
    }

    recommendations
}

fn is_text_artifact(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".log", ".txt", ".md", ".json", ".yaml", ".yml", ".toml"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

fn is_media_artifact(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".gif", ".png", ".jpg", ".jpeg", ".webm", ".mp4", ".mov"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

fn render_planning_summary(report: &EpicShowReport) {
    println!();
    println!("{}", "Planning Summary".bold());
    println!(
        "  Problem: {}",
        report
            .prd
            .problem_statement
            .as_deref()
            .unwrap_or(PROBLEM_PLACEHOLDER)
    );

    if report.prd.goals.is_empty() {
        println!("  Goals:   {}", GOALS_PLACEHOLDER.dimmed());
    } else {
        println!("  Goals:");
        for goal in &report.prd.goals {
            println!("    - {}", goal);
        }
    }

    if report.prd.key_requirements.is_empty() {
        println!("  Key Requirements: {}", REQUIREMENTS_PLACEHOLDER.dimmed());
    } else {
        println!("  Key Requirements:");
        for req in report.prd.key_requirements.iter().take(5) {
            println!("    - {}", req);
        }
        if report.prd.key_requirements.len() > 5 {
            println!(
                "    - ... {} more",
                report.prd.key_requirements.len().saturating_sub(5)
            );
        }
    }
}

fn render_progress(report: &EpicShowReport) {
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
        Some(weeks) => format!("~{weeks:.1} weeks ({:.1} stories/week, 4w)", report.eta.throughput_stories_per_week),
        None if report.eta.remaining_stories == 0 => "complete".to_string(),
        None => ETA_PLACEHOLDER.to_string(),
    };
    println!("  ETA:     {eta}");
}

fn render_verification_readiness(report: &EpicShowReport) {
    println!();
    println!("{}", "Verification Readiness".bold());
    let total_criteria = report.verification.automated_criteria + report.verification.manual_criteria;
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

    println!("  Automated verification additions:");
    if report.recommendations.is_empty() {
        println!("    - {}", RECOMMENDATION_PLACEHOLDER.dimmed());
    } else {
        for recommendation in report.recommendations.iter().take(5) {
            println!(
                "    - {}: {}",
                recommendation.technique, recommendation.rationale
            );
        }
        if report.recommendations.len() > 5 {
            println!(
                "    - ... {} more",
                report.recommendations.len().saturating_sub(5)
            );
        }
    }
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
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::{Duration, Local};
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
            report.prd.problem_statement.as_deref(),
            Some("Teams cannot quickly understand planning intent and verification readiness.")
        );
        assert!(
            report
                .prd
                .goals
                .iter()
                .any(|goal| goal.contains("Improve planning readability"))
        );
        assert!(
            report
                .prd
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
        assert!(report.verification.linked_artifacts.iter().any(|a| a.ends_with("ac-1.log")));
        assert!(report.verification.linked_artifacts.iter().any(|a| a.ends_with("ac-2.gif")));
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

        assert!(report.prd.problem_statement.is_none());
        assert!(report.prd.goals.is_empty());
        assert!(report.prd.key_requirements.is_empty());
        assert!(report.verification.linked_artifacts.is_empty());
        assert!(report.recommendations.is_empty());

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
                .any(|s| s.contains("Cargo.toml (Rust workspace)"))
        );
        assert!(
            report
                .project_signals
                .iter()
                .any(|s| s.contains("flake.nix includes vhs"))
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|r| r.technique.contains("Coverage Gate"))
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|r| r.technique.contains("VHS CLI Recording"))
        );
        assert!(
            report
                .recommendations
                .iter()
                .any(|r| r.technique.contains("LLM-Judge Semantic Checks"))
        );
    }
}
