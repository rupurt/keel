//! Canonical flow metrics projection shared by diagnostics and queue policy.

use crate::domain::model::{Board, EpicState, StoryState, VoyageState};
use crate::read_model::execution_queue;

/// High-level summary of board-wide flow state.
#[derive(Debug, Default)]
pub struct FlowMetrics {
    pub execution: ExecutionMetrics,
    pub planning: PlanningMetrics,
    pub research: ResearchMetrics,
    pub verification: VerificationMetrics,
    pub governance: GovernanceMetrics,
    pub done_count: usize,
}

#[derive(Debug, Default)]
pub struct ExecutionMetrics {
    pub backlog_count: usize,
    pub backlog_ready_count: usize,
    pub backlog_blocked_count: usize,
    pub in_progress_count: usize,
    pub active_voyages_count: usize,
}

#[derive(Debug, Default)]
pub struct PlanningMetrics {
    pub draft_count: usize,
    pub planned_count: usize,
    pub epics_needing_voyages: usize,
}

#[derive(Debug, Default)]
pub struct ResearchMetrics {
    pub exploring_count: usize,
    pub surveying_count: usize,
    pub assessing_count: usize,
    pub laid_count: usize,
    pub parked_count: usize,
}

#[derive(Debug, Default)]
pub struct VerificationMetrics {
    pub count: usize,
    pub avg_age_days: f64,
    pub max_age_days: usize,
    pub items: Vec<(String, usize)>, // (story_id, age_days)
}

#[derive(Debug, Default)]
pub struct GovernanceMetrics {
    pub proposed_count: usize,
    pub accepted_count: usize,
}

/// Calculate board-wide flow metrics.
pub fn calculate_metrics(board: &Board) -> FlowMetrics {
    let mut metrics = FlowMetrics::default();

    // 1. Execution
    let backlog_stories: Vec<_> = board
        .stories
        .values()
        .filter(|s| s.status == StoryState::Backlog)
        .collect();
    metrics.execution.backlog_count = backlog_stories.len();
    let (ready, blocked) = execution_queue::backlog_queue_counts(board);
    metrics.execution.backlog_ready_count = ready;
    metrics.execution.backlog_blocked_count = blocked;

    metrics.execution.in_progress_count = board
        .stories
        .values()
        .filter(|s| s.status == StoryState::InProgress)
        .count();
    metrics.execution.active_voyages_count = board
        .voyages
        .values()
        .filter(|v| v.status() == VoyageState::InProgress)
        .count();

    // 2. Planning
    metrics.planning.draft_count = board
        .voyages
        .values()
        .filter(|v| v.status() == VoyageState::Draft)
        .count();
    metrics.planning.planned_count = board
        .voyages
        .values()
        .filter(|v| v.status() == VoyageState::Planned)
        .count();
    metrics.planning.epics_needing_voyages = board
        .epics
        .values()
        .filter(|e| e.status() == EpicState::Draft)
        .filter(|e| board.voyages_for_epic(e).is_empty())
        .count();

    // 3. Research
    metrics.research.exploring_count = board
        .bearings
        .values()
        .filter(|b| b.frontmatter.status == crate::domain::model::BearingStatus::Exploring)
        .count();
    metrics.research.surveying_count = board
        .bearings
        .values()
        .filter(|b| {
            b.frontmatter.status == crate::domain::model::BearingStatus::Evaluating && b.has_survey
        })
        .count();
    metrics.research.assessing_count = board
        .bearings
        .values()
        .filter(|b| {
            b.frontmatter.status == crate::domain::model::BearingStatus::Evaluating && !b.has_survey
        })
        .count();
    metrics.research.laid_count = board
        .bearings
        .values()
        .filter(|b| b.frontmatter.status == crate::domain::model::BearingStatus::Laid)
        .count();
    metrics.research.parked_count = board
        .bearings
        .values()
        .filter(|b| b.frontmatter.status == crate::domain::model::BearingStatus::Parked)
        .count();

    // 4. Verification
    metrics.verification.count = board
        .stories
        .values()
        .filter(|s| s.status == StoryState::NeedsHumanVerification)
        .count();
    // TODO: Age calculation

    // 5. Governance
    metrics.governance.proposed_count = board
        .adrs
        .values()
        .filter(|a| a.status() == crate::domain::model::AdrStatus::Proposed)
        .count();
    metrics.governance.accepted_count = board
        .adrs
        .values()
        .filter(|a| a.status() == crate::domain::model::AdrStatus::Accepted)
        .count();

    // 6. Done
    metrics.done_count = board
        .stories
        .values()
        .filter(|s| s.status == StoryState::Done)
        .count();

    metrics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn calculate_counts_stories_by_stage() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::InProgress))
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .story(TestStory::new("S3").status(StoryState::Done))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.execution.in_progress_count, 1);
        assert_eq!(m.execution.backlog_count, 1);
        assert_eq!(m.execution.backlog_ready_count, 1);
        assert_eq!(m.execution.backlog_blocked_count, 0);
        assert_eq!(m.done_count, 1);
    }

    #[test]
    fn calculate_splits_backlog_ready_and_blocked_by_dependencies() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | first req | test |\n| SRS-02 | second req | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-flow", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-flow")
                    .status(StoryState::InProgress)
                    .body("- [ ] [SRS-01/AC-01] First requirement"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-flow")
                    .status(StoryState::Backlog)
                    .body("- [ ] [SRS-02/AC-01] Depends on S1"),
            )
            .story(
                TestStory::new("S3")
                    .scope("keel/01-flow")
                    .status(StoryState::Backlog)
                    .body("- [ ] [SRS-01/AC-02] Parallel with S1"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.execution.backlog_count, 2);
        assert_eq!(m.execution.backlog_ready_count, 1);
        assert_eq!(m.execution.backlog_blocked_count, 1);
    }

    #[test]
    fn calculate_treats_backlog_in_draft_voyage_as_blocked() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(
                TestVoyage::new("01-test", "keel")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .status(StoryState::Backlog)
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.execution.backlog_count, 1);
        assert_eq!(m.execution.backlog_ready_count, 0);
        assert_eq!(m.execution.backlog_blocked_count, 1);
    }

    #[test]
    fn calculate_counts_voyages_by_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("in-progress"))
            .voyage(TestVoyage::new("v2", "e1").status("planned"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.execution.active_voyages_count, 1);
        assert_eq!(m.planning.planned_count, 1);
    }

    #[test]
    fn calculate_counts_draft_voyages_separately() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("draft"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.planning.draft_count, 1);
    }

    #[test]
    fn calculate_counts_epics_needing_voyages() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1")) // Draft epic with no voyages
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.planning.epics_needing_voyages, 1);
    }

    #[test]
    fn calculate_governance_metrics() {
        use crate::test_helpers::TestAdr;
        let temp = TestBoardBuilder::new()
            .adr(TestAdr::new("A1").status("proposed"))
            .adr(TestAdr::new("A2").status("accepted"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let m = calculate_metrics(&board);

        assert_eq!(m.governance.proposed_count, 1);
        assert_eq!(m.governance.accepted_count, 1);
    }
}
