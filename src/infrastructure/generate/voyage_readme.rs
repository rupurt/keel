//! Voyage README generation

use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::domain::model::{Board, StoryState, Voyage, VoyageState};

/// Generate and update a voyage's README.md
pub fn generate(_board_dir: &Path, board: &Board, voyage: &Voyage) -> anyhow::Result<()> {
    let readme_path = voyage.path.clone();
    let mut original = fs::read_to_string(&readme_path)?;

    // 1. Update Documents section
    let doc_content = generate_documents_table_for_state(voyage.status());
    if original.contains("<!-- BEGIN DOCUMENTS -->") {
        original = update_section(&original, "DOCUMENTS", &doc_content)?;
    }

    // 2. Update Stories section
    let stories_content = generate_voyage_readme(board, voyage);
    let updated = update_section(&original, "GENERATED", &stories_content)?;

    fs::write(readme_path, updated)?;
    Ok(())
}

/// Update content between <!-- BEGIN {tag} --> and <!-- END {tag} -->
pub fn update_section(original: &str, tag: &str, new_content: &str) -> anyhow::Result<String> {
    let start_marker = format!("<!-- BEGIN {} -->", tag);
    let end_marker = format!("<!-- END {} -->", tag);

    let start_pos = original
        .find(&start_marker)
        .ok_or_else(|| anyhow::anyhow!("Missing {} marker", start_marker))?;
    let search_start = start_pos + start_marker.len();
    let end_pos = original[search_start..]
        .rfind(&end_marker)
        .map(|rel| search_start + rel)
        .ok_or_else(|| anyhow::anyhow!("Missing {} marker", end_marker))?;

    let mut result = original[..start_pos].to_string();
    result.push_str(&start_marker);
    result.push('\n');
    result.push_str(new_content.trim());
    result.push('\n');
    result.push_str(&end_marker);
    result.push_str(&original[end_pos + end_marker.len()..]);

    Ok(result)
}

/// Generate the default documents table for migration/fixup paths.
///
/// This intentionally excludes report artifacts because they are only
/// valid for done voyages.
#[allow(dead_code)]
pub fn generate_documents_table() -> String {
    generate_documents_table_with_reports(false)
}

/// Generate documents table based on voyage state.
pub fn generate_documents_table_for_state(state: VoyageState) -> String {
    generate_documents_table_with_reports(matches!(state, VoyageState::Done))
}

fn generate_documents_table_with_reports(include_reports: bool) -> String {
    let mut output = String::new();
    writeln!(output, "| Document | Description |").unwrap();
    writeln!(output, "|----------|-------------|").unwrap();
    writeln!(
        output,
        "| [SRS.md](SRS.md) | Requirements and verification criteria |"
    )
    .unwrap();
    writeln!(
        output,
        "| [SDD.md](SDD.md) | Architecture and implementation details |"
    )
    .unwrap();

    if include_reports {
        writeln!(output, "| [VOYAGE_REPORT.md](VOYAGE_REPORT.md) | Narrative summary of implementation and evidence |").unwrap();
        writeln!(output, "| [COMPLIANCE_REPORT.md](COMPLIANCE_REPORT.md) | Traceability matrix and verification proof |").unwrap();
    }

    output.trim().to_string()
}

/// Generate a voyage's README.md content (Stories section)
pub fn generate_voyage_readme(board: &Board, voyage: &Voyage) -> String {
    let mut output = String::new();

    let stories = board.stories_for_voyage(voyage);
    let done_count = stories
        .iter()
        .filter(|s| s.stage == StoryState::Done)
        .count();

    // Generated content - Stories header should be above the markers in template
    writeln!(
        output,
        "**Progress:** {}/{} stories complete",
        done_count,
        stories.len()
    )
    .unwrap();
    writeln!(output).unwrap();

    if !stories.is_empty() {
        writeln!(output, "| Title | Type | Status |").unwrap();
        writeln!(output, "|-------|------|--------|").unwrap();

        let mut sorted_stories = stories;
        sorted_stories.sort_by(|a, b| a.id().cmp(b.id()));

        for story in sorted_stories {
            // Link is relative to voyage directory: epics/<epic>/voyages/<id>/
            // Stories are at: stories/<id>/README.md
            let link = format!(
                "[{}](../../../../stories/{}/README.md)",
                story.title(),
                story.id()
            );

            writeln!(
                output,
                "| {} | {} | {} |",
                link,
                story.story_type(),
                story.stage
            )
            .unwrap();
        }
    }

    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn generate_voyage_readme_includes_stories() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Active Story")
                    .stage(StoryState::InProgress)
                    .scope("test-epic/01-first"),
            )
            .story(
                TestStory::new("FEAT0002")
                    .title("Done Story")
                    .stage(StoryState::Done)
                    .scope("test-epic/01-first"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.voyages.get("01-first").unwrap();
        let readme = generate_voyage_readme(&board, voyage);

        assert!(readme.contains("FEAT0001"));
        assert!(readme.contains("FEAT0002"));
    }

    #[test]
    fn test_update_section() {
        let original = "Header\n<!-- BEGIN TEST -->\nOld\n<!-- END TEST -->\nFooter";
        let updated = update_section(original, "TEST", "New").unwrap();
        assert_eq!(
            updated,
            "Header\n<!-- BEGIN TEST -->\nNew\n<!-- END TEST -->\nFooter"
        );
    }

    #[test]
    fn test_update_section_collapses_duplicate_markers() {
        let original = "Header\n<!-- BEGIN TEST -->\n<!-- BEGIN TEST -->\nOld\n<!-- END TEST -->\n<!-- END TEST -->\nFooter";
        let updated = update_section(original, "TEST", "New").unwrap();
        assert_eq!(
            updated,
            "Header\n<!-- BEGIN TEST -->\nNew\n<!-- END TEST -->\nFooter"
        );
    }

    #[test]
    fn generate_voyage_readme_includes_progress() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("FEAT0001")
                    .stage(StoryState::Done)
                    .scope("test-epic/01-first"),
            )
            .story(
                TestStory::new("FEAT0002")
                    .stage(StoryState::InProgress)
                    .scope("test-epic/01-first"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.voyages.get("01-first").unwrap();
        let readme = generate_voyage_readme(&board, voyage);

        assert!(readme.contains("**Progress:** 1/2 stories complete"));
    }

    #[test]
    fn generate_voyage_readme_includes_story_titles() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Active Story")
                    .scope("test-epic/01-first"),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.voyages.get("01-first").unwrap();
        let readme = generate_voyage_readme(&board, voyage);

        assert!(readme.contains("| Title | Type | Status |"));
        assert!(readme.contains("Active Story"));
    }

    #[test]
    fn documents_table_hides_reports_for_non_done_voyages() {
        let docs = generate_documents_table_for_state(VoyageState::InProgress);
        assert!(docs.contains("| [SRS.md](SRS.md) |"));
        assert!(docs.contains("| [SDD.md](SDD.md) |"));
        assert!(!docs.contains("VOYAGE_REPORT.md"));
        assert!(!docs.contains("COMPLIANCE_REPORT.md"));
    }

    #[test]
    fn documents_table_includes_reports_for_done_voyages() {
        let docs = generate_documents_table_for_state(VoyageState::Done);
        assert!(docs.contains("VOYAGE_REPORT.md"));
        assert!(docs.contains("COMPLIANCE_REPORT.md"));
    }
}
