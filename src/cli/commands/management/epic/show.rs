//! Show epic command

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::style;
use crate::infrastructure::loader::load_board;

use std::path::Path;

/// Show epic details
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Show epic details with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;

    let epic = board.require_epic(id)?;

    let width = crate::cli::presentation::flow::display::get_terminal_width();
    println!("{}", style::heavy_rule(width, None));
    println!(
        "{}",
        style::header(epic.id(), &epic.frontmatter.title, style::styled_epic_id)
    );
    println!("{}", style::heavy_rule(width, None));
    println!();

    println!("Status:   {}", style::styled_epic_stage(&epic.status()));
    if let Some(desc) = &epic.frontmatter.description {
        println!("Desc:     {}", desc);
    }
    println!();
    println!("Path: {}", epic.path.display().dimmed());

    // List voyages
    let voyages = board.voyages_for_epic_id(epic.id());
    if !voyages.is_empty() {
        println!();
        println!("Voyages:");
        for ms in voyages {
            println!(
                "  {} - {} ({})",
                style::styled_voyage_id(ms.id()),
                ms.frontmatter.title,
                style::styled_voyage_stage(&ms.status())
            );
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};

    #[test]
    fn test_show_epic() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1").status("tactical"))
            .voyage(TestVoyage::new("v1", "epic1").status("planned"))
            .build();

        let result = run_with_dir(temp.path(), "epic1");
        assert!(result.is_ok());
    }

    #[test]
    fn test_show_epic_not_found() {
        let temp = TestBoardBuilder::new().build();

        let result = run_with_dir(temp.path(), "NONEXISTENT");
        assert!(result.is_err());
    }
}
