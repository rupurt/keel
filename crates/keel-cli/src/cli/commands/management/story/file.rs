//! Story file command.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::cli::commands::management::file_support::{
    KnownDocument, print_markdown_file, resolve_bundle_document_path,
};
use keel::infrastructure::config::find_board_dir;
use keel::infrastructure::loader::load_board;

const STORY_DOCUMENTS: &[KnownDocument] = &[
    KnownDocument::new("README", "README.md"),
    KnownDocument::new("REFLECT", "REFLECT.md"),
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
    let story = board.require_story(id)?;
    resolve_bundle_document_path(&story.path, "Story", story.id(), file, STORY_DOCUMENTS)
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestBoardBuilder, TestStory};

    #[test]
    fn resolve_path_uses_strict_story_id_lookup() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("FEAT0001"))
            .build();

        let err = resolve_path(temp.path(), "0001", "README")
            .unwrap_err()
            .to_string();
        assert!(err.contains("Story not found: 0001"));
    }
}
