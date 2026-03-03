//! Status command - display board status summary

use anyhow::Result;

use crate::domain::model::Board;
use crate::infrastructure::loader::load_board;
use crate::read_model::flow_status;

/// Run the status command.
pub fn run(board_dir: &std::path::Path) -> Result<()> {
    let board = load_board(board_dir)?;
    let metrics = calculate_metrics(&board);

    println!("Board Status Summary");
    println!("====================");
    println!();

    println!("Execution:");
    println!("  Backlog:       {}", metrics.execution.backlog_count);
    println!("  In Progress:   {}", metrics.execution.in_progress_count);
    println!("  Total Stories: {}", metrics.execution.total_count);
    println!();

    println!("Planning:");
    println!("  Draft Voyages:   {}", metrics.planning.draft_count);
    println!("  Planned Voyages: {}", metrics.planning.planned_count);
    println!();

    println!("Research:");
    println!("  Exploring: {}", metrics.research.exploring_count);
    println!("  Evaluating: {}", metrics.research.evaluating_count);
    println!("  Parked:    {}", metrics.research.parked_count);
    println!();

    println!("Verification:");
    println!("  Needs Review:  {}", metrics.verification.count);
    if metrics.verification.count > 0 {
        println!(
            "  Average Age:   {:.1} days",
            metrics.verification.average_age_days
        );
    }

    Ok(())
}

/// Calculate status metrics from the canonical projection service.
pub fn calculate_metrics(board: &Board) -> flow_status::StatusMetrics {
    flow_status::project(board).status
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn status_counts_stories_by_frontmatter_status() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Active Story")
                    .stage(StoryState::InProgress),
            )
            .story(
                TestStory::new("FEAT0002")
                    .title("Pending Story")
                    .stage(StoryState::Backlog),
            )
            .story(
                TestStory::new("FEAT0003")
                    .title("Complete Story")
                    .stage(StoryState::Done),
            )
            .build();
        let board = load_board(temp.path()).unwrap();

        // Count by frontmatter status (source of truth)
        let in_progress = board
            .stories
            .values()
            .filter(|s| s.frontmatter.status == StoryState::InProgress)
            .count();
        let backlog = board
            .stories
            .values()
            .filter(|s| s.frontmatter.status == StoryState::Backlog)
            .count();
        let done = board
            .stories
            .values()
            .filter(|s| s.frontmatter.status == StoryState::Done)
            .count();

        assert_eq!(in_progress, 1);
        assert_eq!(backlog, 1);
        assert_eq!(done, 1);
    }

    #[test]
    fn status_uses_frontmatter_not_directory() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("STORY1")
                    .title("Test Story")
                    .stage(StoryState::InProgress),
            )
            .build();
        let board = load_board(temp.path()).unwrap();

        let metrics = calculate_metrics(&board);
        assert_eq!(metrics.execution.in_progress_count, 1);
    }
}
