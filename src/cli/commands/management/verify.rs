//! verify command — execute verification proofs

use anyhow::Result;
use std::path::Path;

use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification;

/// Run the verify command
pub fn run(board_dir: &Path, id: Option<&str>, all: bool) -> Result<()> {
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
        Ok(())
    } else {
        unreachable!()
    }
}
