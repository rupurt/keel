//! verify command — execute verification proofs

use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use super::guidance::CanonicalGuidance;
use super::verification_guidance::{
    guidance_for_verify_story, print_human, verify_error_with_recovery,
};
use crate::infrastructure::config;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;
use crate::infrastructure::verification::parse_verify_annotations;
use crate::read_model::verification_techniques::{self, TechniqueModality};

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
    label: String,
    name: String,
    modality: String,
    command: String,
}

#[derive(Debug, Clone, Serialize)]
struct VerifyRecommendPayload {
    recommendations: Vec<VerifyRecommendRow>,
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

pub fn run_legacy(id: Option<&str>) -> Result<()> {
    let target = id
        .map(|story_id| format!("keel verify run {story_id}"))
        .unwrap_or_else(|| "keel verify run --all".to_string());
    anyhow::bail!(
        "Legacy `keel verify` invocation is no longer supported.\nRecovery step:\n  {target}"
    );
}

pub fn recommend(board_dir: &Path, json: bool) -> Result<()> {
    let payload = build_recommend_payload(board_dir)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&payload)?);
        return Ok(());
    }

    println!("[verification.recommend]");
    println!("count = {}", payload.recommendations.len());
    println!();
    println!("# Detected + active techniques:");
    if payload.recommendations.is_empty() {
        println!("  (none)");
    } else {
        for recommendation in &payload.recommendations {
            println!("  - label = \"{}\"", recommendation.label);
            println!("    name = \"{}\"", recommendation.name);
            println!("    modality = \"{}\"", recommendation.modality);
            println!("    command = \"{}\"", recommendation.command);
        }
    }

    if !payload.diagnostics.is_empty() {
        println!();
        println!("# Config diagnostics:");
        for diagnostic in &payload.diagnostics {
            println!("  - {}", diagnostic);
        }
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

    let recommendations = status_report
        .techniques
        .iter()
        .filter(|status| status.detected && status.active)
        .map(|status| VerifyRecommendRow {
            label: status.id.clone(),
            name: status.label.clone(),
            modality: modality_name(status.modality).to_string(),
            command: status.default_command.clone(),
        })
        .collect();

    Ok(VerifyRecommendPayload {
        recommendations,
        diagnostics: status_report.diagnostics,
    })
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
    fn verify_root_fails_fast_with_run_guidance() {
        let err = run_legacy(Some("S1")).unwrap_err().to_string();
        assert!(err.contains("Legacy `keel verify` invocation is no longer supported."));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel verify run S1"));
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
                .all(|row| row.label != "rust-unit-tests")
        );
        assert!(
            payload
                .recommendations
                .iter()
                .all(|row| row.label != "browser-playwright-e2e")
        );
        assert!(
            payload
                .recommendations
                .iter()
                .any(|row| row.label == "llm-judge")
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
    fn verify_recommend_json_contract() {
        let payload = VerifyRecommendPayload {
            recommendations: vec![VerifyRecommendRow {
                label: "llm-judge".to_string(),
                name: "LLM-Judge".to_string(),
                modality: "judge".to_string(),
                command: "llm-judge".to_string(),
            }],
            diagnostics: Vec::new(),
        };

        let value = serde_json::to_value(payload).unwrap();
        assert!(value["recommendations"].is_array());
        let first = &value["recommendations"][0];
        assert!(first.get("label").is_some());
        assert!(first.get("name").is_some());
        assert!(first.get("modality").is_some());
        assert!(first.get("command").is_some());
    }
}
