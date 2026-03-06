//! Board README generation

use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::domain::model::{BearingStatus, Board};
use crate::infrastructure::utils::cmp_optional_index_then_id;

/// Generate and save the board-level README.md
pub fn generate(board_dir: &Path, board: &Board) -> anyhow::Result<()> {
    let content = generate_board_readme(board);
    let readme_path = board_dir.join("README.md");
    fs::write(readme_path, content)?;
    Ok(())
}

/// Generate the main board README.md content
pub fn generate_board_readme(board: &Board) -> String {
    let mut output = String::new();

    // Header
    writeln!(output, "# Planning Board").unwrap();
    writeln!(output).unwrap();
    writeln!(output, "> [!NOTE]").unwrap();
    writeln!(
        output,
        "> Auto-generated from story frontmatter. Run `keel generate` to update."
    )
    .unwrap();
    writeln!(output).unwrap();

    // Bearings section (research phase)
    write_bearings_section(&mut output, board);

    // Epics section
    write_epics_section(&mut output, board);

    output
}

fn write_bearings_section(output: &mut String, board: &Board) {
    if board.bearings.is_empty() {
        return;
    }

    writeln!(output, "## Bearings").unwrap();
    writeln!(output).unwrap();

    // Active bearings (exploring, evaluating, ready)
    let active: Vec<_> = board
        .bearings
        .values()
        .filter(|b| {
            matches!(
                b.frontmatter.status,
                BearingStatus::Exploring | BearingStatus::Evaluating | BearingStatus::Ready
            )
        })
        .collect();

    if !active.is_empty() {
        writeln!(output, "| Bearing | Status | Survey | Assessment |").unwrap();
        writeln!(output, "|---------|--------|--------|------------|").unwrap();

        let mut sorted = active;
        sort_indexed(
            &mut sorted,
            |bearing| bearing.frontmatter.index,
            |bearing| bearing.id(),
        );

        for bearing in sorted {
            let survey = if bearing.has_survey { "✓" } else { "-" };
            let assessment = if bearing.has_assessment { "✓" } else { "-" };
            writeln!(
                output,
                "| [{}](bearings/{}/) | {} | {} | {} |",
                bearing.title(),
                bearing.id(),
                bearing.frontmatter.status,
                survey,
                assessment
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    // Completed bearings (laid, parked, declined) - collapsed
    let completed: Vec<_> = board
        .bearings
        .values()
        .filter(|b| {
            matches!(
                b.frontmatter.status,
                BearingStatus::Laid | BearingStatus::Parked | BearingStatus::Declined
            )
        })
        .collect();

    if !completed.is_empty() {
        writeln!(output, "<details>").unwrap();
        writeln!(output, "<summary>Completed Bearings</summary>").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "| Bearing | Status |").unwrap();
        writeln!(output, "|---------|--------|").unwrap();

        let mut sorted = completed;
        sorted.sort_by(|a, b| a.id().cmp(b.id()));

        for bearing in sorted {
            writeln!(
                output,
                "| [{}](bearings/{}/) | {} |",
                bearing.title(),
                bearing.id(),
                bearing.frontmatter.status
            )
            .unwrap();
        }
        writeln!(output).unwrap();
        writeln!(output, "</details>").unwrap();
        writeln!(output).unwrap();
    }
}

fn write_epics_section(output: &mut String, board: &Board) {
    writeln!(output, "## Epics").unwrap();
    writeln!(output).unwrap();

    // Sort epics by index
    let mut epics: Vec<_> = board.epics.values().collect();
    sort_indexed(&mut epics, |epic| epic.frontmatter.index, |epic| epic.id());

    for epic in epics {
        let voyages = board.voyages_for_epic(epic);

        writeln!(
            output,
            "### [{}](epics/{}/) ({})",
            epic.title(),
            epic.id(),
            epic.status()
        )
        .unwrap();
        writeln!(output).unwrap();

        if !voyages.is_empty() {
            writeln!(output, "| Voyage | Status |").unwrap();
            writeln!(output, "|--------|--------|").unwrap();

            let mut sorted_voyages = voyages;
            sort_indexed(
                &mut sorted_voyages,
                |voyage| voyage.frontmatter.index,
                |voyage| voyage.id(),
            );

            for v in sorted_voyages {
                writeln!(
                    output,
                    "| [{}](epics/{}/voyages/{}/) | {} |",
                    v.title(),
                    epic.id(),
                    v.id(),
                    v.status()
                )
                .unwrap();
            }
            writeln!(output).unwrap();
        }
    }
}

fn sort_indexed<T, FIndex, FId>(items: &mut Vec<&T>, index_of: FIndex, id_of: FId)
where
    FIndex: Fn(&T) -> Option<u32>,
    FId: for<'a> Fn(&'a T) -> &'a str,
{
    items.sort_by(|left, right| {
        cmp_optional_index_then_id(index_of(left), id_of(left), index_of(right), id_of(right))
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn generate_board_readme_includes_header() {
        let temp = TestBoardBuilder::new().build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("# Planning Board"));
        assert!(readme.contains("Auto-generated"));
    }

    #[test]
    fn generate_board_readme_omits_story_status_sections() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Active Story")
                    .stage(crate::domain::model::StoryState::InProgress),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(!readme.contains("## Rejected"));
        assert!(!readme.contains("## Ready for Acceptance"));
        assert!(!readme.contains("## In Progress"));
        assert!(!readme.contains("## Backlog"));
        assert!(!readme.contains("## Icebox"));
        assert!(!readme.contains("0001"));
    }

    #[test]
    fn generate_board_readme_includes_epics() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Epics"));
        assert!(readme.contains("test-epic"));
        assert!(readme.contains("01-first"));
    }

    #[test]
    fn generate_board_readme_keeps_done_epics_in_epics_section() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("done-epic").title("Done Epic"))
            .voyage(
                TestVoyage::new("01-finished", "done-epic")
                    .title("Finished Voyage")
                    .status("done"),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Epics"));
        assert!(readme.contains("[Done Epic](epics/done-epic/) (done)"));
        assert!(readme.contains("[Finished Voyage](epics/done-epic/voyages/01-finished/)"));
        assert!(!readme.contains("<summary><h2>Done</h2></summary>"));
    }

    #[test]
    fn generate_board_readme_includes_bearings() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("done-epic"))
            .adr(crate::test_helpers::TestAdr::new("ADR-0001").status("proposed")) // minimal valid board
            .build();

        // Create a bearing manually since TestBoardBuilder might not have a helper for it yet
        fs::create_dir_all(temp.path().join("bearings/test-research")).unwrap();
        fs::write(
            temp.path().join("bearings/test-research/README.md"),
            r#"---
id: test-research
title: Test Research
status: exploring
created_at: 2026-01-29T12:00:00
---
"#,
        )
        .unwrap();

        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Bearings"));
        assert!(readme.contains("test-research"));
    }
}
