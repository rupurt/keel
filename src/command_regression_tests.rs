//! Command behavior regression suite for migration parity.
//!
//! These tests cover key command-adjacent flows (`next`, `flow`, lifecycle
//! transitions) so refactors preserve observed behavior.

use crate::cli::commands::management::next::NextDecision;
use crate::domain::model::StoryState;
use crate::domain::policy::queue::{
    FLOW_VERIFY_BLOCK_THRESHOLD, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD,
};
use crate::test_helpers::{TestBoardBuilder, TestStory};
use std::fs;

fn board_with_verification_and_ready(verify_count: usize, ready_count: usize) -> tempfile::TempDir {
    let mut builder = TestBoardBuilder::new();
    for i in 0..verify_count {
        let id = format!("VERIFY{:02}", i + 1);
        builder = builder.story(TestStory::new(&id).stage(StoryState::NeedsHumanVerification));
    }
    for i in 0..ready_count {
        let id = format!("READY{:02}", i + 1);
        builder = builder.story(TestStory::new(&id).stage(StoryState::Backlog));
    }
    builder.build()
}

#[test]
fn regression_next_and_flow_align_on_human_blocked_boundary() {
    let temp = board_with_verification_and_ready(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD, 1);
    let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

    let next =
        crate::cli::commands::management::next::calculate_next(&board, temp.path(), false, None)
            .unwrap();
    assert!(
        matches!(next, NextDecision::Blocked(_)),
        "human next should be blocked at policy threshold"
    );

    let metrics = crate::cli::presentation::flow::metrics::calculate_metrics(&board);
    let health = crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(&metrics);
    assert!(
        health.action_summary.to_lowercase().contains("blocked"),
        "flow summary should indicate blocked human queue at threshold"
    );
}

#[test]
fn regression_next_and_flow_align_on_flow_blocked_boundary() {
    let temp = board_with_verification_and_ready(FLOW_VERIFY_BLOCK_THRESHOLD + 1, 1);
    let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

    let next =
        crate::cli::commands::management::next::calculate_next(&board, temp.path(), false, None)
            .unwrap();
    assert!(
        matches!(next, NextDecision::Blocked(_)),
        "human next should be blocked when flow is verify-blocked"
    );

    let metrics = crate::cli::presentation::flow::metrics::calculate_metrics(&board);
    let health = crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(&metrics);
    assert!(
        health
            .action_summary
            .to_lowercase()
            .contains("verification queue is blocked"),
        "flow summary should indicate verification queue blocked"
    );
}

#[test]
fn regression_story_lifecycle_command_chain_reaches_done() {
    let temp = TestBoardBuilder::new()
        .story(
            TestStory::new("REGCHAIN1")
                .stage(StoryState::Backlog)
                .body(
                    "## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] Manual check <!-- verify: manual, SRS-01:start:end -->",
                ),
        )
        .build();

    crate::cli::commands::management::story::start::run(temp.path(), "REGCHAIN1", None).unwrap();

    let evidence_dir = temp.path().join("stories/REGCHAIN1/EVIDENCE");
    fs::create_dir_all(&evidence_dir).unwrap();
    fs::write(
        temp.path().join("stories/REGCHAIN1/REFLECT.md"),
        "### L001: Insight",
    )
    .unwrap();

    crate::cli::commands::management::story::submit::run(temp.path(), "REGCHAIN1").unwrap();
    crate::cli::commands::management::story::accept::run(temp.path(), "REGCHAIN1", true, None)
        .unwrap();

    let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
    let story = board.require_story("REGCHAIN1").unwrap();
    assert_eq!(story.stage, StoryState::Done);

    let content = fs::read_to_string(temp.path().join("stories/REGCHAIN1/README.md")).unwrap();
    assert!(content.contains("submitted_at:"));
    assert!(content.contains("completed_at:"));
}
