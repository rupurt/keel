//! Bearing file command.

use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::cli::commands::management::file_support::{
    KnownDocument, print_markdown_file, resolve_bundle_document_path,
};
use keel::infrastructure::config::find_board_dir;
use keel::infrastructure::loader::load_board;

const BEARING_DOCUMENTS: &[KnownDocument] = &[
    KnownDocument::new("README", "README.md"),
    KnownDocument::new("BRIEF", "BRIEF.md"),
    KnownDocument::new("EVIDENCE", "EVIDENCE.md"),
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

#[cfg(test)]
mod tests {
    use super::resolve_path;
    use keel::infrastructure::loader::load_board;
    use keel::test_helpers::{TestBearing, TestBoardBuilder};
    use std::fs;

    #[test]
    fn bearing_hard_cutover_rejects_legacy_survey_paths() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01"))
            .build();

        fs::write(temp.path().join("bearings/BRG-01/SURVEY.md"), "# Survey\n").unwrap();

        let board = load_board(temp.path()).unwrap();
        assert!(!board.bearings["BRG-01"].has_evidence);

        let err = resolve_path(temp.path(), "BRG-01", "SURVEY")
            .unwrap_err()
            .to_string();

        assert!(err.contains("Unknown Bearing document 'SURVEY'"));
        assert!(err.contains("Available files: README, BRIEF, EVIDENCE, ASSESSMENT"));
    }

    #[test]
    fn bearing_file_surfaces_evidence_document() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("BRG-01").has_evidence(true))
            .build();

        let expected = temp.path().join("bearings/BRG-01/EVIDENCE.md");
        let resolved = resolve_path(temp.path(), "BRG-01", "evidence.Md").unwrap();

        assert_eq!(resolved, expected);
        let content = fs::read_to_string(resolved).unwrap();
        assert!(content.contains("— Evidence"));
        assert!(content.contains("## Sources"));
    }
}
