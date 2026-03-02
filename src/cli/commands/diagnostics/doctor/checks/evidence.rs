#![allow(dead_code)]

use std::fs;

use super::super::types::*;
use super::super::{CRITERIA_RE, EVIDENCE_PHASE_RE};
use crate::domain::model::{Board, StoryState, VoyageState};
use crate::domain::state_machine::{
    EnforcementPolicy, TransitionEntity, TransitionIntent, VoyageCompletionPolicy,
    VoyageTransition, enforce_transition, evaluate_voyage_completion,
};

/// Invalid phases (like `middle`) silently change verification semantics --
/// the phase suffix becomes part of the command instead of being parsed
/// as traceability metadata.
pub fn check_evidence_phase_syntax(board: &Board) -> Vec<Problem> {
    let valid_phases = ["start", "continues", "end", "start:end"];
    let mut problems = Vec::new();

    for story in board.stories.values() {
        if story.stage != StoryState::InProgress
            && story.stage != StoryState::NeedsHumanVerification
            && story.stage != StoryState::Done
        {
            continue;
        }

        let content = match fs::read_to_string(&story.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let mut bad_phases = Vec::new();

        for line in content.lines() {
            // Only check lines that are acceptance criteria with verify annotations
            if !CRITERIA_RE.is_match(line) {
                continue;
            }
            let Some(verify_start) = line.find("verify:") else {
                continue;
            };
            let verify_content = &line[verify_start + 7..];
            // Strip trailing -->
            let verify_content = verify_content
                .rfind("-->")
                .map(|i| &verify_content[..i])
                .unwrap_or(verify_content)
                .trim();

            if let Some(caps) = EVIDENCE_PHASE_RE.captures(verify_content)
                && let Some(phase_match) = caps.get(1)
            {
                let phase = phase_match.as_str();
                if !valid_phases.contains(&phase) {
                    bad_phases.push(phase.to_string());
                }
            }
        }

        if !bad_phases.is_empty() {
            problems.push(Problem {
                severity: Severity::Error,
                path: story.path.clone(),
                message: format!(
                    "{}: invalid evidence phase(s): {} (valid: start, continues, end, start:end)",
                    story.id(),
                    bad_phases.join(", ")
                ),
                fix: None,
                scope: story.scope().map(|s| s.to_string()),
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
}

/// Check that requirements have complete evidence chains via verification annotations.
///
/// A complete chain is either:
/// - A single `start:end` (one-shot)
/// - A `start` followed by optional `continues` and ending with `end`
pub fn check_evidence_chains(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        let gate_problems = match voyage.status() {
            VoyageState::InProgress => {
                // Doctor consumes the same transition/completion gate outputs as runtime,
                // but with reporting policy to keep warning findings non-blocking.
                let enforcement = enforce_transition(
                    board,
                    TransitionEntity::Voyage(voyage),
                    TransitionIntent::Voyage(VoyageTransition::Complete),
                    EnforcementPolicy::REPORTING,
                );
                enforcement.gate_problems
            }
            VoyageState::Done => evaluate_voyage_completion(
                board,
                voyage,
                None,
                VoyageCompletionPolicy {
                    strict: true,
                    require_all_stories_done: false,
                    require_evidence: true,
                },
            ),
            _ => Vec::new(),
        };
        let has_partial_chain_issue = gate_problems.iter().any(|problem| {
            problem.message.ends_with("has start but missing end")
                || problem.message.ends_with("has end but missing start")
                || problem.message.ends_with("has continues but missing start")
        });

        let mut voyage_problems = Vec::new();
        for problem in gate_problems {
            let message = normalize_evidence_summary_message(&problem.message)
                .unwrap_or_else(|| problem.message.clone());

            if voyage.status() == VoyageState::Done
                && has_partial_chain_issue
                && message.contains("has no evidence chain")
            {
                continue;
            }

            voyage_problems.push(Problem { message, ..problem });
        }

        problems.extend(voyage_problems);
    }

    problems
}

fn normalize_evidence_summary_message(message: &str) -> Option<String> {
    let after_prefix = message.strip_prefix("0/")?;
    let (count, _) = after_prefix.split_once("/requirements have evidence chains")?;

    Some(format!("0/{count} requirements have evidence chains"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::state_machine::{
        EnforcementPolicy, TransitionEntity, TransitionIntent, VoyageTransition, enforce_transition,
    };
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn test_check_evidence_phase_syntax_valid() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("S1")
                    .stage(StoryState::InProgress)
                    .body("- [ ] [SRS-01/AC-01] t1 <!-- verify: cargo test -- srs:start -->"),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let problems = check_evidence_phase_syntax(&board);
        // EVIDENCE_PHASE_RE is empty regex in mod.rs, so this might not match anything yet
        assert!(problems.is_empty());
    }

    #[test]
    fn test_normalize_evidence_summary_message() {
        assert_eq!(
            normalize_evidence_summary_message("0/5/requirements have evidence chains"),
            Some("0/5 requirements have evidence chains".to_string())
        );
        assert_eq!(normalize_evidence_summary_message("invalid"), None);
    }

    #[test]
    fn check_evidence_chains_matches_reporting_enforcement_for_in_progress_voyage() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | Requirement 1 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-in-progress", "test-epic")
                    .status("in-progress")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("test-epic/01-in-progress")
                    .stage(StoryState::InProgress)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Partial chain <!-- verify: manual, SRS-01:start -->",
                    ),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("01-in-progress").unwrap();

        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Voyage(voyage),
            TransitionIntent::Voyage(VoyageTransition::Complete),
            EnforcementPolicy::REPORTING,
        );
        assert!(
            enforcement.blocking_problems.is_empty(),
            "reporting policy should keep findings non-blocking"
        );

        let doctor_problems = check_evidence_chains(&board);
        assert!(
            doctor_problems
                .iter()
                .any(|problem| problem.severity == Severity::Warning),
            "doctor should surface reporting-mode warnings"
        );

        for gate_problem in enforcement.gate_problems {
            let expected = normalize_evidence_summary_message(&gate_problem.message)
                .unwrap_or_else(|| gate_problem.message.clone());
            assert!(
                doctor_problems
                    .iter()
                    .any(|problem| problem.message == expected),
                "doctor finding missing expected gate message: {expected}"
            );
        }
    }
}
