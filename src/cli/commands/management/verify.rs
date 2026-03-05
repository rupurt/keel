//! verify command — execute verification proofs

use anyhow::Result;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::guidance::CanonicalGuidance;
use super::verification_guidance::{
    guidance_for_verify_story, print_human, verify_error_with_recovery,
};
use crate::infrastructure::config;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;
use crate::infrastructure::verification::parse_verify_annotations;
use crate::read_model::verification_techniques::{self, ProjectStack, TechniqueModality};

const MAX_RECOMMENDATIONS: usize = 3;

#[derive(Debug, Clone, Serialize)]
struct VerifyStoryPayload {
    story_id: String,
    title: String,
    scope: Option<String>,
    passed: bool,
    requires_human_review: bool,
    results: Vec<verification::VerificationResult>,
}

#[derive(Debug, Clone, Serialize)]
struct VerifyRunPayload {
    target: String,
    reports: Vec<VerifyStoryPayload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    guidance: Option<CanonicalGuidance>,
}

#[derive(Debug, Clone, Serialize)]
struct VerifyRecommendRow {
    id: String,
    modality: String,
    command: String,
}

#[derive(Debug, Clone, Serialize)]
struct VerifyRecommendPayload {
    recommendations: Vec<VerifyRecommendRow>,
    diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct VerifyDetectTechniqueRow {
    id: String,
    detected: bool,
    disabled: bool,
    active: bool,
    modality: String,
    command: String,
}

#[derive(Debug, Clone, Serialize)]
struct VerifyDetectPayload {
    project_root: String,
    hints: Vec<String>,
    detected_files: Vec<String>,
    stack_confidence: BTreeMap<String, f64>,
    techniques: Vec<VerifyDetectTechniqueRow>,
    diagnostics: Vec<String>,
}

/// Run the verify command
pub fn run(board_dir: &Path, id: Option<&str>, all: bool, json: bool) -> Result<()> {
    let result = (|| {
        let board = load_board(board_dir)?;

        if all || id.is_none() {
            let mut reports = verification::verify_all(board_dir)?;
            reports.sort_by(|left, right| left.story_id.cmp(&right.story_id));
            let payload = build_payload(&board, reports, "all".to_string(), None);

            if json {
                println!("{}", serde_json::to_string_pretty(&payload)?);
                return Ok(());
            }

            for report in payload.reports {
                if let Some(story) = board.stories.get(&report.story_id) {
                    let id_styled = crate::cli::style::styled_story_id(story.id());
                    let scope_styled = crate::cli::style::styled_scope(story.scope());
                    println!("\n{} {} [{}]", id_styled, story.title(), scope_styled);
                    verification::print_terminal_report(&verification::VerificationReport {
                        story_id: report.story_id.clone(),
                        results: report.results.clone(),
                    });
                }
            }
            Ok(())
        } else if let Some(id) = id {
            let story = board.require_story(id)?;
            let content = std::fs::read_to_string(&story.path)?;
            let report = verification::verify_story(board_dir, story.id(), &content)?;
            let guidance = guidance_for_verify_story(story.id(), story.stage, &report);

            if json {
                let payload = build_payload(
                    &board,
                    vec![report],
                    story.id().to_string(),
                    guidance.clone(),
                );
                println!("{}", serde_json::to_string_pretty(&payload)?);
                return Ok(());
            }

            let id_styled = crate::cli::style::styled_story_id(story.id());
            let scope_styled = crate::cli::style::styled_scope(story.scope());
            println!("\n{} {} [{}]", id_styled, story.title(), scope_styled);
            verification::print_terminal_report(&report);
            print_human(guidance.as_ref());
            Ok(())
        } else {
            unreachable!()
        }
    })();

    result.map_err(|error| verify_error_with_recovery(id, error))
}

pub fn recommend(board_dir: &Path, json: bool) -> Result<()> {
    let payload = build_recommend_payload(board_dir)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    for line in render_recommend_payload(&payload) {
        println!("{line}");
    }

    Ok(())
}

pub fn detect(board_dir: &Path, json: bool) -> Result<()> {
    let payload = build_detect_payload(board_dir)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    for line in render_detect_payload(&payload) {
        println!("{line}");
    }

    Ok(())
}

fn build_payload(
    board: &crate::domain::model::Board,
    reports: Vec<verification::VerificationReport>,
    target: String,
    guidance: Option<CanonicalGuidance>,
) -> VerifyRunPayload {
    let reports = reports
        .into_iter()
        .map(|report| {
            let passed = report.passed();
            let requires_human_review = report.requires_human_review();
            let (title, scope) = board
                .stories
                .get(&report.story_id)
                .map(|story| (story.title().to_string(), story.scope().map(str::to_string)))
                .unwrap_or_else(|| ("(story not found)".to_string(), None));
            VerifyStoryPayload {
                story_id: report.story_id,
                title,
                scope,
                passed,
                requires_human_review,
                results: report.results,
            }
        })
        .collect();

    VerifyRunPayload {
        target,
        reports,
        guidance,
    }
}

fn build_recommend_payload(board_dir: &Path) -> Result<VerifyRecommendPayload> {
    let (config, _) = config::load_config();
    let project_root = resolve_project_root(board_dir, &config);
    let used_techniques = collect_used_techniques(board_dir)?;
    let status_report =
        verification_techniques::resolve_technique_status_report(&project_root, &used_techniques);
    let ranked_report =
        verification_techniques::build_show_recommendation_report(&project_root, &used_techniques);

    let mut recommendations = Vec::new();
    for ranked in ranked_report
        .recommendations
        .into_iter()
        .take(MAX_RECOMMENDATIONS)
    {
        if let Some(status) = status_report
            .techniques
            .iter()
            .find(|status| status.id == ranked.id)
            .filter(|status| status.detected && status.active)
        {
            recommendations.push(VerifyRecommendRow {
                id: status.id.clone(),
                modality: modality_name(status.modality).to_string(),
                command: status.default_command.clone(),
            });
        }
    }

    if recommendations.is_empty() {
        recommendations = status_report
            .techniques
            .iter()
            .filter(|status| status.detected && status.active)
            .map(|status| VerifyRecommendRow {
                id: status.id.clone(),
                modality: modality_name(status.modality).to_string(),
                command: status.default_command.clone(),
            })
            .collect();
    }

    let mut diagnostics = status_report.diagnostics;
    diagnostics.extend(ranked_report.diagnostics);
    diagnostics.sort();
    diagnostics.dedup();

    Ok(VerifyRecommendPayload {
        recommendations,
        diagnostics,
    })
}

fn build_detect_payload(board_dir: &Path) -> Result<VerifyDetectPayload> {
    let (config, _) = config::load_config();
    let project_root = resolve_project_root(board_dir, &config);
    let used_techniques = collect_used_techniques(board_dir)?;
    let status_report =
        verification_techniques::resolve_technique_status_report(&project_root, &used_techniques);
    let signal_report = verification_techniques::detect_project_signals(&project_root);

    let techniques = status_report
        .techniques
        .iter()
        .map(|status| VerifyDetectTechniqueRow {
            id: status.id.clone(),
            detected: status.detected,
            disabled: status.disabled,
            active: status.active,
            modality: modality_name(status.modality).to_string(),
            command: status.default_command.clone(),
        })
        .collect();

    let mut stack_confidence = BTreeMap::new();
    for (stack, confidence) in signal_report.stack_confidence {
        stack_confidence.insert(stack_name(stack).to_string(), confidence);
    }

    Ok(VerifyDetectPayload {
        project_root: project_root.display().to_string(),
        hints: signal_report.hints.into_iter().collect(),
        detected_files: signal_report.detected_files,
        stack_confidence,
        techniques,
        diagnostics: status_report.diagnostics,
    })
}

fn render_recommend_payload(payload: &VerifyRecommendPayload) -> Vec<String> {
    let mut lines = Vec::new();
    if payload.recommendations.is_empty() {
        lines.push("[verification.recommend]".to_string());
        lines.push("none = true".to_string());
    } else {
        for recommendation in &payload.recommendations {
            lines.push("[[verification.recommend]]".to_string());
            lines.push(format!("id = \"{}\"", recommendation.id));
            lines.push(format!("modality = \"{}\"", recommendation.modality));
            lines.push(format!("command = \"{}\"", recommendation.command));
            lines.push(String::new());
        }
    }

    if !payload.diagnostics.is_empty() {
        for diagnostic in &payload.diagnostics {
            lines.push("[[verification.recommend.diagnostics]]".to_string());
            lines.push(format!("message = \"{}\"", diagnostic));
            lines.push(String::new());
        }
    }

    lines
}

fn render_detect_payload(payload: &VerifyDetectPayload) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push("[verification.detect]".to_string());
    lines.push(format!("project_root = \"{}\"", payload.project_root));
    lines.push(format!("hints = {:?}", payload.hints));
    lines.push(format!("detected_files = {:?}", payload.detected_files));
    lines.push(String::new());

    lines.push("[verification.detect.stack_confidence]".to_string());
    for (stack, confidence) in &payload.stack_confidence {
        lines.push(format!("{stack} = {confidence:.2}"));
    }
    lines.push(String::new());

    for technique in &payload.techniques {
        lines.push("[[verification.detect.techniques]]".to_string());
        lines.push(format!("id = \"{}\"", technique.id));
        lines.push(format!("detected = {}", technique.detected));
        lines.push(format!("disabled = {}", technique.disabled));
        lines.push(format!("active = {}", technique.active));
        lines.push(format!("modality = \"{}\"", technique.modality));
        lines.push(format!("command = \"{}\"", technique.command));
        lines.push(String::new());
    }

    if !payload.diagnostics.is_empty() {
        for diagnostic in &payload.diagnostics {
            lines.push("[[verification.detect.diagnostics]]".to_string());
            lines.push(format!("message = \"{}\"", diagnostic));
            lines.push(String::new());
        }
    }

    lines
}

fn collect_used_techniques(board_dir: &Path) -> Result<BTreeSet<String>> {
    let board = load_board(board_dir)?;
    let mut used = BTreeSet::new();

    for story in board.stories.values() {
        let Ok(content) = std::fs::read_to_string(&story.path) else {
            continue;
        };
        for annotation in parse_verify_annotations(&content) {
            if let Some(command) = annotation.command {
                for technique_id in verification_techniques::infer_used_technique_ids(&command) {
                    used.insert(technique_id);
                }
            }
        }
    }

    Ok(used)
}

fn resolve_project_root(board_dir: &Path, config: &config::Config) -> PathBuf {
    if config.board_dir() == "." {
        board_dir.to_path_buf()
    } else if board_dir.ends_with(config.board_dir()) {
        board_dir.parent().unwrap_or(board_dir).to_path_buf()
    } else {
        board_dir.to_path_buf()
    }
}

fn modality_name(modality: TechniqueModality) -> &'static str {
    match modality {
        TechniqueModality::Command => "command",
        TechniqueModality::Recording => "recording",
        TechniqueModality::Judge => "judge",
    }
}

fn stack_name(stack: ProjectStack) -> &'static str {
    match stack {
        ProjectStack::Rust => "rust",
        ProjectStack::Browser => "browser",
        ProjectStack::Cli => "cli",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;
    use std::fs;

    #[test]
    fn verify_run_not_found_includes_recovery_guidance() {
        let temp = TestBoardBuilder::new().build();

        let err = run(temp.path(), Some("MISSING"), false, false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("Story not found: MISSING"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story list"));
    }

    #[test]
    fn verify_run_json_contract() {
        let temp = TestBoardBuilder::new().build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let payload = build_payload(&board, Vec::new(), "all".to_string(), None);
        let value = serde_json::to_value(payload).unwrap();
        assert_eq!(value["target"], "all");
        assert!(value["reports"].is_array());
        assert!(value.get("guidance").is_none());
    }

    #[test]
    fn verify_recommend_filters_detected_active() {
        let temp = TestBoardBuilder::new().build();
        fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname=\"demo\"\nversion=\"0.1.0\"\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("keel.toml"),
            r#"[verification.techniques]
disable = ["rust-unit-tests"]
"#,
        )
        .unwrap();

        let payload = build_recommend_payload(temp.path()).unwrap();

        assert!(
            payload
                .recommendations
                .iter()
                .all(|row| row.id != "rust-unit-tests")
        );
        assert!(
            payload
                .recommendations
                .iter()
                .all(|row| row.id != "browser-playwright-e2e")
        );
        assert!(
            payload
                .recommendations
                .iter()
                .any(|row| row.id == "llm-judge")
        );
    }

    #[test]
    fn verify_recommend_has_no_execution_side_effects() {
        let temp = TestBoardBuilder::new()
            .story(
                crate::test_helpers::TestStory::new("S1").body(
                    "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] noop <!-- verify: sh -c 'touch SHOULD_NOT_EXIST', SRS-01:start:end -->",
                ),
            )
            .build();
        let marker = temp.path().join("SHOULD_NOT_EXIST");
        assert!(!marker.exists());

        let _ = build_recommend_payload(temp.path()).unwrap();

        assert!(!marker.exists());
    }

    #[test]
    fn verify_recommend_limits_output_size() {
        let temp = TestBoardBuilder::new().build();
        fs::write(
            temp.path().join("Cargo.toml"),
            "[package]\nname=\"demo\"\nversion=\"0.1.0\"\n",
        )
        .unwrap();
        fs::write(
            temp.path().join("flake.nix"),
            "buildInputs = [ pkgs.vhs pkgs.ffmpeg ];",
        )
        .unwrap();

        let payload = build_recommend_payload(temp.path()).unwrap();
        assert!(payload.recommendations.len() <= MAX_RECOMMENDATIONS);
    }

    #[test]
    fn verify_recommend_json_contract() {
        let payload = VerifyRecommendPayload {
            recommendations: vec![VerifyRecommendRow {
                id: "llm-judge".to_string(),
                modality: "judge".to_string(),
                command: "llm-judge".to_string(),
            }],
            diagnostics: Vec::new(),
        };

        let value = serde_json::to_value(payload).unwrap();
        assert!(value["recommendations"].is_array());
        let first = &value["recommendations"][0];
        assert!(first.get("id").is_some());
        assert!(first.get("modality").is_some());
        assert!(first.get("command").is_some());
    }

    #[test]
    fn verify_detect_json_contract() {
        let payload = VerifyDetectPayload {
            project_root: "/tmp/project".to_string(),
            hints: vec!["cargo".to_string(), "rust".to_string()],
            detected_files: vec!["Cargo.toml".to_string()],
            stack_confidence: BTreeMap::from([
                ("browser".to_string(), 0.0),
                ("cli".to_string(), 0.55),
                ("rust".to_string(), 0.75),
            ]),
            techniques: vec![VerifyDetectTechniqueRow {
                id: "rust-unit-tests".to_string(),
                detected: true,
                disabled: false,
                active: true,
                modality: "command".to_string(),
                command: "cargo test".to_string(),
            }],
            diagnostics: Vec::new(),
        };

        let value = serde_json::to_value(payload).unwrap();
        assert!(value.get("project_root").is_some());
        assert!(value.get("hints").is_some());
        assert!(value.get("detected_files").is_some());
        assert!(value.get("stack_confidence").is_some());
        assert!(value.get("techniques").is_some());
        let first = &value["techniques"][0];
        assert!(first.get("id").is_some());
        assert!(first.get("detected").is_some());
        assert!(first.get("disabled").is_some());
        assert!(first.get("active").is_some());
        assert!(first.get("modality").is_some());
        assert!(first.get("command").is_some());
    }
}
