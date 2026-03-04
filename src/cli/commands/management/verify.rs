//! verify command — execute verification proofs

use anyhow::Result;
use std::path::Path;

use super::verification_guidance::{
    guidance_for_verify_story, print_human, verify_error_with_recovery,
};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;

/// Run the verify command
pub fn run(board_dir: &Path, id: Option<&str>, all: bool) -> Result<()> {
    let result = (|| {
        let board = load_board(board_dir)?;

        if all || id.is_none() {
            let reports = verification::verify_all(board_dir)?;
            for report in reports {
                if let Some(story) = board.stories.get(&report.story_id) {
                    let id_styled = crate::cli::style::styled_story_id(story.id());
                    let scope_styled = crate::cli::style::styled_scope(story.scope());
                    println!("\n{} {} [{}]", id_styled, story.title(), scope_styled);
                    verification::print_terminal_report(&report);
                }
            }
            Ok(())
        } else if let Some(id) = id {
            let story = board.require_story(id)?;
            let content = std::fs::read_to_string(&story.path)?;
            let report = verification::verify_story(board_dir, story.id(), &content)?;

            let id_styled = crate::cli::style::styled_story_id(story.id());
            let scope_styled = crate::cli::style::styled_scope(story.scope());
            println!("\n{} {} [{}]", id_styled, story.title(), scope_styled);
            verification::print_terminal_report(&report);

            let guidance = guidance_for_verify_story(story.id(), story.stage, &report);
            print_human(guidance.as_ref());
            Ok(())
        } else {
            unreachable!()
        }
    })();

    result.map_err(|error| verify_error_with_recovery(id, error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn verify_run_not_found_includes_recovery_guidance() {
        let temp = TestBoardBuilder::new().build();

        let err = run(temp.path(), Some("MISSING"), false)
            .unwrap_err()
            .to_string();
        assert!(err.contains("Story not found: MISSING"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story list"));
    }
}
