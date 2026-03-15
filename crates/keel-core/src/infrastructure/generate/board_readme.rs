//! Board README generation

use std::fmt::Write;

use crate::domain::model::{BearingStatus, Board};
use crate::infrastructure::bearing_readiness::evaluate_bearing_readiness;
use crate::infrastructure::utils::cmp_optional_index_then_id;

/// Generate and save the board-level README.md
pub fn generate(board: &Board) -> anyhow::Result<()> {
    let content = generate_board_readme(board);
    let readme_path = board.root.join("README.md");
    super::artifact_io::write_if_changed(&readme_path, &content)?;
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

    // Visible bearings stay in the main table so laid research remains easy to scan.
    let visible: Vec<_> = board
        .bearings
        .values()
        .filter(|b| {
            matches!(
                b.frontmatter.status,
                BearingStatus::Exploring
                    | BearingStatus::Evaluating
                    | BearingStatus::Ready
                    | BearingStatus::Laid
            )
        })
        .collect();

    if !visible.is_empty() {
        let (config, _source) = crate::infrastructure::config::load_config();
        let weights = config.current_weights();
        writeln!(
            output,
            "| Bearing | Status | Evidence | Assessment | Readiness | EV | Laid |"
        )
        .unwrap();
        writeln!(
            output,
            "|---------|--------|----------|------------|-----------|----|------|"
        )
        .unwrap();

        let mut sorted = visible;
        sort_indexed(
            &mut sorted,
            |bearing| bearing.frontmatter.index,
            |bearing| bearing.id(),
        );

        for bearing in sorted {
            let evidence = if bearing.has_evidence { "✓" } else { "-" };
            let assessment = if bearing.has_assessment { "✓" } else { "-" };
            let readiness = evaluate_bearing_readiness(&board.root, bearing, Some(&weights));
            let laid = if bearing.frontmatter.status == BearingStatus::Laid {
                "✓"
            } else {
                "-"
            };
            let ev = readiness
                .score
                .as_ref()
                .map(|score| format!("{:.2}", score.weighted_score))
                .unwrap_or_else(|| "-".to_string());
            writeln!(
                output,
                "| [{}](bearings/{}/) | {} | {} | {} | {} | {} | {} |",
                bearing.title(),
                bearing.id(),
                bearing.frontmatter.status,
                evidence,
                assessment,
                readiness.short_status(),
                ev,
                laid
            )
            .unwrap();
        }
        writeln!(output).unwrap();
    }

    // Archived bearings remain collapsed so the board stays focused on active and laid research.
    let archived: Vec<_> = board
        .bearings
        .values()
        .filter(|b| {
            matches!(
                b.frontmatter.status,
                BearingStatus::Parked | BearingStatus::Declined
            )
        })
        .collect();

    if !archived.is_empty() {
        writeln!(output, "<details>").unwrap();
        writeln!(output, "<summary>Archived Bearings</summary>").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "| Bearing | Status |").unwrap();
        writeln!(output, "|---------|--------|").unwrap();

        let mut sorted = archived;
        sort_indexed(
            &mut sorted,
            |bearing| bearing.frontmatter.index,
            |bearing| bearing.id(),
        );

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
    use crate::test_helpers::{TestBearing, TestBoardBuilder, TestEpic, TestStory, TestVoyage};

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
                    .status(crate::domain::model::StoryState::InProgress),
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
            .bearing(
                TestBearing::new("test-research")
                    .status("exploring")
                    .has_evidence(true),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Bearings"));
        assert!(readme.contains("test-research"));
        assert!(
            readme.contains("| Bearing | Status | Evidence | Assessment | Readiness | EV | Laid |")
        );
    }

    #[test]
    fn generate_board_readme_keeps_laid_bearings_in_main_table() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("done-epic"))
            .bearing(
                TestBearing::new("laid-bearing")
                    .status("laid")
                    .has_evidence(true)
                    .has_assessment(true),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("[Test Bearing](bearings/laid-bearing/) | laid | ✓ | ✓ |"));
        assert!(readme.contains("| complete scoring | - | ✓ |"));
        assert!(!readme.contains("<summary>Completed Bearings</summary>"));
    }

    #[test]
    fn generate_board_readme_keeps_parked_and_declined_bearings_collapsed() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("done-epic"))
            .bearing(TestBearing::new("parked-bearing").status("parked"))
            .bearing(TestBearing::new("declined-bearing").status("declined"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("<summary>Archived Bearings</summary>"));
        assert!(readme.contains("[Test Bearing](bearings/parked-bearing/) | parked |"));
        assert!(readme.contains("[Test Bearing](bearings/declined-bearing/) | declined |"));
    }
}
