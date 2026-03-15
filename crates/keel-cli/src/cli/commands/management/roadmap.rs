//! `keel roadmap` command.

use anyhow::Result;
use std::path::Path;

use crate::cli::table::Table;
use keel::infrastructure::loader::load_board;
use keel::read_model;

/// Run the roadmap command.
pub fn run() -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir)
}

/// Run the roadmap command with an explicit board directory.
pub fn run_with_dir(board_dir: &Path) -> Result<()> {
    let output = build_roadmap_output(board_dir)?;
    print!("{output}");
    Ok(())
}

fn build_roadmap_output(board_dir: &Path) -> Result<String> {
    let board = load_board(board_dir)?;
    let projection = read_model::roadmap::project(&board);

    if projection.rows.is_empty() {
        return Ok("No roadmap items found on this board.\n".to_string());
    }

    let mut table = Table::new(&[
        "TYPE",
        "ID",
        "TITLE",
        "STATUS",
        "POSTURE",
        "BLOCKING_IDS",
        "BLOCKING_COUNT",
    ]);

    for row in projection.rows {
        table.row(&[
            row.entity_type.as_str(),
            &row.entity_id,
            &row.title,
            &row.status,
            row.posture.as_str(),
            &format_blocking_ids(&row.blocking_ids),
            &row.blocking_count().to_string(),
        ]);
    }

    Ok(table.render())
}

fn format_blocking_ids(blocking_ids: &[String]) -> String {
    if blocking_ids.is_empty() {
        return "[]".to_string();
    }

    format!("[{}]", blocking_ids.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::domain::model::StoryState;
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};
    use std::time::Instant;

    fn roadmap_fixture() -> tempfile::TempDir {
        let srs_content = "# SRS\n\n## Functional Requirements\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n| ID | Requirement | Scope | Source | Verification |\n|----|-------------|-------|--------|--------------|\n| SRS-01 | One | Scope | FR-01 | test |\n| SRS-02 | Two | Scope | FR-02 | test |\n| SRS-03 | Three | Scope | FR-03 | test |\n<!-- END FUNCTIONAL_REQUIREMENTS -->";

        TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .mission(TestMission::new("M2").status("active"))
            .epic(TestEpic::new("E1").mission("M1").index(2))
            .epic(TestEpic::new("E2").mission("M1").index(1))
            .epic(TestEpic::new("E3").mission("M2").index(1))
            .voyage(
                TestVoyage::new("V1", "E2")
                    .status("planned")
                    .index(1)
                    .srs_content(srs_content),
            )
            .voyage(
                TestVoyage::new("V2", "E2")
                    .status("draft")
                    .index(2)
                    .srs_content(srs_content),
            )
            .voyage(
                TestVoyage::new("V3", "E1")
                    .status("planned")
                    .index(1)
                    .srs_content(srs_content),
            )
            .story(
                TestStory::new("S1")
                    .scope("E2/V1")
                    .index(1)
                    .body("- [ ] [SRS-01/AC-01] ready story"),
            )
            .story(
                TestStory::new("S2")
                    .scope("E2/V1")
                    .index(2)
                    .blocked_by(&["S1"])
                    .body("- [ ] [SRS-02/AC-01] blocked story"),
            )
            .story(
                TestStory::new("S3")
                    .status(StoryState::Done)
                    .scope("E1/V3")
                    .index(1)
                    .body("- [x] [SRS-03/AC-01] completed story"),
            )
            .build()
    }

    #[test]
    fn roadmap_render_includes_posture() {
        let board_dir = roadmap_fixture();
        let output = build_roadmap_output(board_dir.path()).unwrap();

        assert!(output.contains("proceed"));
        assert!(output.contains("park"));
        assert!(output.contains("blocked"));
        assert!(output.contains("BLOCKING_IDS"));
    }

    #[test]
    fn roadmap_rows_include_blockers_and_deterministic_sort() {
        let board_dir = roadmap_fixture();
        let output = build_roadmap_output(board_dir.path()).unwrap();
        let board = keel::infrastructure::loader::load_board(board_dir.path()).unwrap();
        let projection = read_model::roadmap::project(&board);

        let blocked_story = projection
            .rows
            .iter()
            .find(|row| row.entity_id == "S2")
            .expect("blocked story row");

        assert_eq!(blocked_story.blocking_count(), 1);
        assert_eq!(blocked_story.blocking_ids, vec!["S1"]);

        let repeated = build_roadmap_output(board_dir.path()).unwrap();
        assert_eq!(output, repeated);
    }

    #[test]
    fn roadmap_output_is_deterministic() {
        let board_dir = roadmap_fixture();
        let first = build_roadmap_output(board_dir.path()).unwrap();
        let second = build_roadmap_output(board_dir.path()).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn roadmap_render_performance() {
        let board_dir = roadmap_fixture();
        let start = Instant::now();

        let _ = build_roadmap_output(board_dir.path()).unwrap();

        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 500,
            "roadmap command is too slow: {elapsed:?}"
        );
    }
}
