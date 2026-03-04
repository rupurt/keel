//! verify command — execute verification proofs

use anyhow::Result;
use serde::Serialize;
use std::path::Path;

use super::guidance::CanonicalGuidance;
use super::verification_guidance::{
    guidance_for_verify_story, print_human, verify_error_with_recovery,
};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

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
}
