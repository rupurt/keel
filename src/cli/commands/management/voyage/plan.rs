//! Plan voyage command - promote draft voyages to planned status

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::domain::model::{StoryState, VoyageState};
use crate::domain::state_machine::{
    EnforcementPolicy, TransitionEntity, TransitionIntent, VoyageTransition, enforce_transition,
    format_enforcement_error,
};
use crate::domain::transitions::{TimestampUpdates, update_frontmatter};
use crate::infrastructure::config::find_board_dir;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::validation::parse_acceptance_criteria;

/// Run the plan-voyage command
pub fn run(id: &str, no_review: bool) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, id, no_review)
}

/// Run the plan-voyage command with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str, no_review: bool) -> Result<()> {
    let board = load_board(board_dir)?;

    // Find the voyage
    let voyage = board.require_voyage(id)?;

    // Check voyage transition/completion gates
    let stories = board.stories_for_voyage(voyage);
    let intent = TransitionIntent::Voyage(VoyageTransition::Plan);
    let enforcement = enforce_transition(
        &board,
        TransitionEntity::Voyage(voyage),
        intent,
        EnforcementPolicy::RUNTIME,
    );
    if !enforcement.allows_transition() {
        return Err(anyhow!(format_enforcement_error(
            &format!("voyage {}", voyage.id()),
            intent,
            &enforcement.blocking_problems
        )));
    }

    // Validate stories (if review enabled)
    if !no_review {
        println!("Reviewing {} stories...", stories.len());

        let mut issues = Vec::new();
        for story in &stories {
            let content = fs::read_to_string(&story.path)
                .with_context(|| format!("Failed to read story: {}", story.path.display()))?;

            let criteria = parse_acceptance_criteria(&content);
            if !criteria.has_section {
                issues.push(format!("  - {} has no acceptance criteria", story.id()));
            }
        }

        if !issues.is_empty() {
            println!("\nReview found issues:");
            for issue in &issues {
                println!("{}", issue);
            }
            println!();
            return Err(anyhow!(
                "Review failed: {} stories with issues",
                issues.len()
            ));
        }

        println!("Review passed: all stories have acceptance criteria\n");
    }

    // Promote draft-scoped stories from icebox to backlog
    let mut thawed = 0usize;
    for story in stories.iter().filter(|s| s.stage == StoryState::Icebox) {
        let content = fs::read_to_string(&story.path)
            .with_context(|| format!("Failed to read story: {}", story.path.display()))?;

        let updated_content = update_frontmatter(
            &content,
            StoryState::Backlog,
            &TimestampUpdates::updated_only(),
        )?;

        fs::write(&story.path, updated_content)
            .with_context(|| format!("Failed to write thawed story: {}", story.path.display()))?;

        thawed += 1;
    }

    // Update status to planned
    let content = fs::read_to_string(&voyage.path)
        .with_context(|| format!("Failed to read voyage: {}", voyage.path.display()))?;

    let updated_content = update_frontmatter(
        &content,
        VoyageState::Planned,
        &TimestampUpdates::updated_only(),
    )?;

    fs::write(&voyage.path, updated_content)
        .with_context(|| format!("Failed to write voyage: {}", voyage.path.display()))?;

    println!("Planned voyage: {}", voyage.id());
    if thawed > 0 {
        println!("  Thawed {} draft story(s) to backlog", thawed);
    }
    println!("  {} stories ready for execution", stories.len());

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn plan_voyage_updates_status_to_planned() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("0001")
                    .scope("test-epic/01-draft")
                    .body("\n## Acceptance Criteria\n\n- [ ] test"),
            )
            .build();

        run_with_dir(temp.path(), "01-draft", true).unwrap();

        let content = fs::read_to_string(
            temp.path()
                .join("epics/test-epic/voyages/01-draft/README.md"),
        )
        .unwrap();

        assert!(content.contains("status: planned"));
    }

    #[test]
    fn plan_voyage_errors_on_not_draft() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("02-planned", "test-epic").status("planned"))
            .story(TestStory::new("0001").scope("test-epic/02-planned"))
            .build();

        let result = run_with_dir(temp.path(), "02-planned", true);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("must be draft"), "Error was: {}", err);
    }

    #[test]
    fn plan_voyage_errors_on_no_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("03-empty-draft", "test-epic").status("draft"))
            .build();

        let result = run_with_dir(temp.path(), "03-empty-draft", true);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("no stories"), "Error was: {}", err);
    }

    #[test]
    fn plan_voyage_review_checks_acceptance_criteria() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("0002")
                    .scope("test-epic/01-draft")
                    .body("# No AC here"),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-draft", false);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("stories with issues"), "Error was: {}", err);
    }

    #[test]
    fn plan_voyage_review_passes_with_acceptance_criteria() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("0003")
                    .scope("test-epic/01-draft")
                    .body("\n## Acceptance Criteria\n\n- [ ] test"),
            )
            .build();

        let result = run_with_dir(temp.path(), "01-draft", false);

        assert!(result.is_ok(), "Expected success, got: {:?}", result);
    }

    #[test]
    fn plan_voyage_blocks_uncovered_requirements() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS")
            )
            .story(
                TestStory::new("0003")
                    .scope("test-epic/01-draft")
                    .body("\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] test")
            )
            .build();

        let result = run_with_dir(temp.path(), "01-draft", true);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("requirements not covered"));
        assert!(err.contains("SRS-02"));
    }

    #[test]
    fn plan_voyage_allows_covered_requirements() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content("# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS")
            )
            .story(
                TestStory::new("0007")
                    .scope("test-epic/01-draft")
                    .body("\n## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] test")
            )
            .story(
                TestStory::new("0008")
                    .scope("test-epic/01-draft")
                    .body("\n## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] test")
            )
            .build();

        let result = run_with_dir(temp.path(), "01-draft", true);

        assert!(result.is_ok());
    }

    #[test]
    fn plan_voyage_thaws_draft_scoped_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-draft", "test-epic").status("draft"))
            .story(
                TestStory::new("0005")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Icebox),
            )
            .story(
                TestStory::new("0006")
                    .scope("test-epic/01-draft")
                    .stage(StoryState::Backlog),
            )
            .build();

        run_with_dir(temp.path(), "01-draft", true).unwrap();

        let thawed_story = fs::read_to_string(temp.path().join("stories/0005/README.md")).unwrap();
        let backlog_story = fs::read_to_string(temp.path().join("stories/0006/README.md")).unwrap();

        assert!(thawed_story.contains("status: backlog"));
        assert!(backlog_story.contains("status: backlog"));
    }
}
