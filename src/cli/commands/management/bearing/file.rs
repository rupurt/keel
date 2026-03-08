//! Bearing file command.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::cli::commands::management::file_support::{
    KnownDocument, print_markdown_file, resolve_bundle_document_path,
};
use crate::infrastructure::config::find_board_dir;
use crate::infrastructure::loader::load_board;

const BEARING_DOCUMENTS: &[KnownDocument] = &[
    KnownDocument::new("README", "README.md"),
    KnownDocument::new("BRIEF", "BRIEF.md"),
    KnownDocument::new("SURVEY", "SURVEY.md"),
    KnownDocument::new("ASSESSMENT", "ASSESSMENT.md"),
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
    let bearing = board.require_bearing(id)?;
    resolve_bundle_document_path(
        &bearing.path,
        "Bearing",
        bearing.id(),
        file,
        BEARING_DOCUMENTS,
    )
}
