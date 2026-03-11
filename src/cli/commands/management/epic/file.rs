//! Epic file command.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::cli::commands::management::file_support::{
    KnownDocument, print_markdown_file, resolve_bundle_document_path,
};
use keel::infrastructure::config::find_board_dir;
use keel::infrastructure::loader::load_board;

const EPIC_DOCUMENTS: &[KnownDocument] = &[
    KnownDocument::new("README", "README.md"),
    KnownDocument::new("PRD", "PRD.md"),
    KnownDocument::new("PRESS_RELEASE", "PRESS_RELEASE.md"),
];

pub fn run(id: &str, file: &str, raw: bool) -> Result<()> {
    let board_dir = find_board_dir()?;
    run_with_dir(&board_dir, id, file, raw)
}

pub fn run_with_dir(board_dir: &Path, id: &str, file: &str, raw: bool) -> Result<()> {
    let path = resolve_path(board_dir, id, file)?;
    print_markdown_file(&path, raw)
}

fn resolve_path(board_dir: &Path, id: &str, file: &str) -> Result<PathBuf> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(id)?;
    resolve_bundle_document_path(&epic.path, "Epic", epic.id(), file, EPIC_DOCUMENTS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestBoardBuilder, TestEpic};
    use std::fs;

    #[test]
    fn resolve_path_accepts_mixed_case_optional_extension() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .build();

        let path = resolve_path(temp.path(), "epic-1", "prd.Md").unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("# PRD"));
    }
}
