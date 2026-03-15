//! ADR file command.

use std::path::Path;

use anyhow::Result;

use crate::cli::commands::management::file_support::print_markdown_file;
use keel::infrastructure::config::find_board_dir;
use keel::infrastructure::loader::load_board;

pub fn run(id: &str, raw: bool) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, id, raw)
}

pub fn run_with_dir(board_dir: &Path, id: &str, raw: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let adr = board.require_adr(id)?;
    print_markdown_file(&adr.path, raw)
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestAdr, TestBoardBuilder};

    #[test]
    fn run_with_dir_uses_strict_adr_id_lookup() {
        let temp = TestBoardBuilder::new().adr(TestAdr::new("ADR-001")).build();

        let err = run_with_dir(temp.path(), "001", true)
            .unwrap_err()
            .to_string();
        assert!(err.contains("ADR not found: 001"));
    }
}
