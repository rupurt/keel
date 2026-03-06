#![allow(dead_code)]
//! Pull-system decision algorithm for selecting the next task.

use anyhow::Result;
use std::path::Path;

use crate::domain::model::{Board, Story, StoryState};
use crate::domain::policy::queue::compare_work_item_ids;
use crate::read_model::queue_policy::{self, DraftVoyageQueueCategory};

/// Single decision about what to work on next.
#[derive(Debug)]
pub enum NextDecision {
    /// Work on an existing story
    Work(StoryDecision),
    /// Proposed ADR needs review
    Decision(AdrDecision),
    /// Stories need human acceptance
    Accept(AcceptDecision),
    /// Research pipeline needs attention
    Research(ResearchDecision),
    /// No work found
    Empty(EmptyDecision),
    /// System is blocked on verification
    Blocked(BlockedDecision),
    /// Strategic gap (voyage needs stories)
    NeedsStories(DecomposeDecision),
    /// Strategic gap (voyage needs planning)
    NeedsPlanning(DecomposeDecision),
}

#[derive(Debug)]
pub struct StoryDecision {
    pub story: Story,
    pub is_continuation: bool,
    pub warning: Option<String>,
}

#[derive(Debug)]
pub struct AdrDecision {
    pub adrs: Vec<crate::domain::model::Adr>,
    pub blocked_stories: Vec<Story>,
}

#[derive(Debug)]
pub struct AcceptDecision {
    pub stories: Vec<Story>,
}

#[derive(Debug)]
pub struct ResearchDecision {
    pub bearings: Vec<crate::domain::model::Bearing>,
}

#[derive(Debug)]
pub struct EmptyDecision {
    pub suggestions: Vec<String>,
}

#[derive(Debug)]
pub struct BlockedDecision {
    pub story: Story,
    pub count: usize,
}

#[derive(Debug)]
pub struct DecomposeDecision {
    pub voyages: Vec<crate::domain::model::Voyage>,
}

/// Calculate the single most important next action.
pub fn calculate_next(
    board: &Board,
    board_dir: &Path,
    agent_mode: bool,
    actor_role: Option<&crate::domain::model::taxonomy::RoleTaxonomy>,
) -> Result<NextDecision> {
    let mut agent_backlog_blocked_by_dependencies = false;

    let projection = crate::read_model::flow_status::project(board);
    let metrics = &projection.flow;
    let queue_policy_snapshot = queue_policy::project(metrics);

    // 1. Check for blocking verification backlog (human only)
    // If we've reached or exceeded capacity, we MUST stop and clear the queue
    if !agent_mode && queue_policy_snapshot.verification.blocks_human_next() {
        let ready = board
            .stories
            .values()
            .find(|s| s.status == StoryState::NeedsHumanVerification)
            .cloned()
            .unwrap();
        return Ok(NextDecision::Blocked(BlockedDecision {
            story: ready,
            count: metrics.verification.count,
        }));
    }

    // 2. Check for proposed ADRs (human only)
    if !agent_mode && metrics.governance.proposed_count > 0 {
        let adrs: Vec<_> = board
            .adrs
            .values()
            .filter(|a| a.status() == crate::domain::model::AdrStatus::Proposed)
            .cloned()
            .collect();

        let blocked_stories: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::Backlog)
            .filter(|s| {
                s.frontmatter
                    .governed_by
                    .iter()
                    .any(|id| adrs.iter().any(|a| a.id() == id))
            })
            .cloned()
            .collect();

        return Ok(NextDecision::Decision(AdrDecision {
            adrs,
            blocked_stories,
        }));
    }

    // 3. Acceptance (human only)
    // If there is ANY work to accept, prioritize it before starting new work
    if !agent_mode && queue_policy_snapshot.verification.has_items() {
        let stories: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::NeedsHumanVerification)
            .cloned()
            .collect();

        return Ok(NextDecision::Accept(AcceptDecision { stories }));
    }

    // 4. Research (human only)
    if !agent_mode && queue_policy_snapshot.has_research_work {
        let bearings: Vec<_> = board
            .bearings
            .values()
            .filter(|b| {
                matches!(
                    b.frontmatter.status,
                    crate::domain::model::BearingStatus::Exploring
                        | crate::domain::model::BearingStatus::Evaluating
                )
            })
            .cloned()
            .collect();

        return Ok(NextDecision::Research(ResearchDecision { bearings }));
    }

    // 5. Strategy: Decompose or Plan (human only)
    if !agent_mode && queue_policy_snapshot.has_planning_work {
        let mut needs_stories = Vec::new();
        let mut needs_planning = Vec::new();
        for voyage in board
            .voyages
            .values()
            .filter(|v| v.status() == crate::domain::model::VoyageState::Draft)
            .cloned()
        {
            let story_count = board.stories_for_voyage(&voyage).len();
            match queue_policy::classify_draft_voyage(story_count) {
                DraftVoyageQueueCategory::NeedsStories => needs_stories.push(voyage),
                DraftVoyageQueueCategory::NeedsPlanning => needs_planning.push(voyage),
            }
        }

        if !needs_stories.is_empty() {
            return Ok(NextDecision::NeedsStories(DecomposeDecision {
                voyages: needs_stories,
            }));
        }

        if !needs_planning.is_empty() {
            return Ok(NextDecision::NeedsPlanning(DecomposeDecision {
                voyages: needs_planning,
            }));
        }
    }

    // 6. Implementation work selection (agent only)
    if agent_mode {
        // 6a. Continue in-progress work (actor-specific)
        let in_progress: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::InProgress)
            .filter(|s| {
                actor_role
                    .map(|role| crate::domain::model::taxonomy::actor_matches_story(role, s))
                    .unwrap_or(true)
            })
            .collect();

        if let Some(story) = in_progress.first() {
            return Ok(NextDecision::Work(StoryDecision {
                story: (*story).clone(),
                is_continuation: true,
                warning: None,
            }));
        }

        // 6b. Select from backlog
        let deps = crate::read_model::traceability::derive_implementation_dependencies(board);
        let workable_backlog: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::Backlog)
            .filter(|s| {
                crate::domain::state_machine::invariants::story_workable(s, board, board_dir)
            })
            .filter(|s| {
                actor_role
                    .map(|role| crate::domain::model::taxonomy::actor_matches_story(role, s))
                    .unwrap_or(true)
            })
            .collect();

        let mut candidates: Vec<_> = workable_backlog
            .iter()
            .copied()
            .filter(|s| {
                // Unblocked if no dependencies OR all dependencies are Done
                deps.get(s.id()).is_none_or(|dep_ids| {
                    dep_ids.iter().all(|id| {
                        board
                            .stories
                            .get(id)
                            .map(|dep| dep.status == StoryState::Done)
                            .unwrap_or(false)
                    })
                })
            })
            .collect();

        candidates.sort_by(|a, b| compare_work_item_ids(a.id(), b.id()));

        if let Some(story) = candidates.first() {
            return Ok(NextDecision::Work(StoryDecision {
                story: (*story).clone(),
                is_continuation: false,
                warning: None,
            }));
        }

        if !workable_backlog.is_empty() {
            agent_backlog_blocked_by_dependencies = true;
        }
    }

    let mut suggestions = Vec::new();
    if agent_mode {
        if agent_backlog_blocked_by_dependencies {
            suggestions.push(
                "All workable backlog stories are blocked by implementation dependencies."
                    .to_string(),
            );
            suggestions.push(
                "Run `keel next --agent --parallel` to inspect sequential chains.".to_string(),
            );
        }
        if queue_policy_snapshot.verification.has_items() {
            suggestions.push("Waiting for human acceptance of completed work".to_string());
        }
        if queue_policy_snapshot.has_planning_work {
            suggestions.push("Waiting for human to plan draft voyages".to_string());
        }
        if suggestions.is_empty() {
            suggestions.push("Board is empty. Add new bearings or epics to begin.".to_string());
        }
    } else {
        suggestions.push("Refuel the backlog".to_string());
        suggestions.push("Check for drifted research".to_string());
    }

    Ok(NextDecision::Empty(EmptyDecision { suggestions }))
}

/// Helper to select which mode calculate_next should run in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Agent,
    Human,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::policy::queue::{
        FLOW_VERIFY_BLOCK_THRESHOLD, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD,
    };
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    fn assert_human_queue_decision(next: NextDecision) {
        match next {
            NextDecision::Decision(_)
            | NextDecision::Accept(_)
            | NextDecision::Research(_)
            | NextDecision::Empty(_)
            | NextDecision::Blocked(_)
            | NextDecision::NeedsStories(_)
            | NextDecision::NeedsPlanning(_) => {}
            NextDecision::Work(d) => panic!(
                "human mode must not return implementation work (got story {})",
                d.story.id()
            ),
        }
    }

    fn build_board_with_verification_and_ready(
        verify_count: usize,
        ready_count: usize,
    ) -> (tempfile::TempDir, Board) {
        let mut builder = TestBoardBuilder::new();

        for i in 0..verify_count {
            let id = format!("V{}", i + 1);
            builder = builder.story(TestStory::new(&id).status(StoryState::NeedsHumanVerification));
        }

        for i in 0..ready_count {
            let id = format!("R{}", i + 1);
            builder = builder.story(TestStory::new(&id).status(StoryState::Backlog));
        }

        let temp = builder.build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        (temp, board)
    }

    fn flow_action_summary(board: &Board) -> String {
        let metrics = crate::read_model::flow_status::project(board).flow;
        crate::cli::presentation::flow::bottleneck::analyze_two_actor_health(&metrics)
            .action_summary
    }

    #[test]
    fn selects_backlog_story_when_nothing_else() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), true, None).unwrap();
        if let NextDecision::Work(d) = next {
            assert_eq!(d.story.id(), "S1");
        } else {
            panic!("Expected Work decision");
        }
    }

    #[test]
    fn prefers_in_progress_over_backlog() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .story(TestStory::new("S2").status(StoryState::InProgress))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), true, None).unwrap();
        if let NextDecision::Work(d) = next {
            assert_eq!(d.story.id(), "S2");
            assert!(d.is_continuation);
        } else {
            panic!("Expected Work decision");
        }
    }

    #[test]
    fn human_mode_finds_adr_decisions() {
        let temp = TestBoardBuilder::new()
            .adr(crate::test_helpers::TestAdr::new("ADR-1").status("proposed"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert!(matches!(next, NextDecision::Decision(_)));
    }

    #[test]
    fn human_mode_finds_research() {
        let mut board = Board::default();
        let bearing = crate::test_helpers::BearingFactory::new("B1")
            .status(crate::domain::model::BearingStatus::Exploring)
            .build();
        board.bearings.insert(bearing.id().to_string(), bearing);

        let next = calculate_next(&board, Path::new("test"), false, None).unwrap();
        assert!(matches!(next, NextDecision::Research(_)));
    }

    #[test]
    fn human_mode_finds_needs_stories() {
        let temp = TestBoardBuilder::new()
            .epic(crate::test_helpers::TestEpic::new("E1"))
            .voyage(crate::test_helpers::TestVoyage::new("V1", "E1").status("draft"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert!(matches!(next, NextDecision::NeedsStories(_)));
    }

    #[test]
    fn empty_decision_when_no_work() {
        let temp = TestBoardBuilder::new().build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), true, None).unwrap();
        assert!(matches!(next, NextDecision::Empty(_)));
    }

    #[test]
    fn human_mode_blocks_when_verify_queue_reaches_threshold() {
        let mut builder = TestBoardBuilder::new();
        for i in 0..HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD {
            let id = format!("S{}", i + 1);
            builder = builder.story(TestStory::new(&id).status(StoryState::NeedsHumanVerification));
        }
        let temp = builder.build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        match next {
            NextDecision::Blocked(d) => assert_eq!(d.count, HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD),
            _ => panic!("Expected Blocked decision"),
        }
    }

    #[test]
    fn human_mode_accepts_when_verify_queue_below_block_threshold() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::NeedsHumanVerification))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert!(matches!(next, NextDecision::Accept(_)));
    }

    #[test]
    fn backlog_work_ordering_uses_policy_comparator() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), true, None).unwrap();
        match next {
            NextDecision::Work(d) => assert_eq!(d.story.id(), "S1"),
            _ => panic!("Expected Work decision"),
        }
    }

    #[test]
    fn human_mode_never_returns_work_when_only_in_progress_exists() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::InProgress))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert_human_queue_decision(next);
    }

    #[test]
    fn human_mode_never_returns_work_when_only_backlog_exists() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert_human_queue_decision(next);
    }

    #[test]
    fn human_mode_never_returns_work_in_mixed_execution_queues() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::InProgress))
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert_human_queue_decision(next);
    }

    #[test]
    fn human_next_and_flow_align_on_verification_attention_boundary() {
        let (temp, board) =
            build_board_with_verification_and_ready(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD - 1, 1);

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert!(matches!(next, NextDecision::Accept(_)));

        let summary = flow_action_summary(&board).to_lowercase();
        assert!(summary.contains("accept"));
    }

    #[test]
    fn human_next_and_flow_align_on_human_blocked_boundary() {
        let (temp, board) =
            build_board_with_verification_and_ready(HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD, 1);

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert!(matches!(next, NextDecision::Blocked(_)));

        let summary = flow_action_summary(&board).to_lowercase();
        assert!(summary.contains("blocked"));
        assert!(summary.contains("accept"));
    }

    #[test]
    fn human_next_and_flow_align_on_flow_blocked_boundary() {
        let (temp, board) =
            build_board_with_verification_and_ready(FLOW_VERIFY_BLOCK_THRESHOLD + 1, 1);

        let next = calculate_next(&board, temp.path(), false, None).unwrap();
        assert!(matches!(next, NextDecision::Blocked(_)));

        let summary = flow_action_summary(&board).to_lowercase();
        assert!(summary.contains("blocked"));
        assert!(summary.contains("accept"));
    }

    #[test]
    fn agent_mode_reports_dependency_blocked_backlog_instead_of_empty_board() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(
                TestVoyage::new("01-test", "keel")
                    .status("planned")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-01] req1\n- [ ] [SRS-02/AC-01] req2"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .body("- [ ] [SRS-01/AC-02] req1b\n- [ ] [SRS-02/AC-02] req2b"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let next = calculate_next(&board, temp.path(), true, None).unwrap();
        match next {
            NextDecision::Empty(empty) => {
                assert!(
                    empty
                        .suggestions
                        .iter()
                        .any(|s| s.contains("blocked by implementation dependencies"))
                );
                assert!(
                    !empty
                        .suggestions
                        .iter()
                        .any(|s| s.contains("Board is empty"))
                );
            }
            _ => panic!("Expected Empty decision with dependency-blocked suggestions"),
        }
    }
}
