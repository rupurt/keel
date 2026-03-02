//! Board README generation

use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::domain::model::{BearingStatus, Board, EpicState, Story, StoryState, VoyageState};

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

    // Rejected section (collapsed - these need rework)
    let rejected: Vec<_> = stories_by_status(board, StoryState::Rejected);
    write_story_section(&mut output, board, "Rejected", &rejected, true);

    // Ready for Acceptance section
    let ready_for_acceptance: Vec<_> = stories_by_status(board, StoryState::NeedsHumanVerification);
    write_story_section(
        &mut output,
        board,
        "Ready for Acceptance",
        &ready_for_acceptance,
        false,
    );

    // In Progress section
    let in_progress: Vec<_> = stories_by_status(board, StoryState::InProgress);
    write_story_section(&mut output, board, "In Progress", &in_progress, false);

    // Backlog section
    let backlog: Vec<_> = stories_by_status(board, StoryState::Backlog);
    write_story_section(&mut output, board, "Backlog", &backlog, false);

    // Icebox section (collapsed)
    let icebox: Vec<_> = stories_by_status(board, StoryState::Icebox);
    write_story_section(&mut output, board, "Icebox", &icebox, true);

    // Bearings section (research phase)
    write_bearings_section(&mut output, board);

    // Epics section
    write_epics_section(&mut output, board);

    // Done section (collapsed)
    write_done_section(&mut output, board);

    output
}

fn stories_by_status(board: &Board, status: StoryState) -> Vec<&Story> {
    let mut stories: Vec<_> = board
        .stories
        .values()
        .filter(|s| s.frontmatter.status == status)
        .collect();
    stories.sort_by(|a, b| match (a.frontmatter.index, b.frontmatter.index) {
        (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        _ => a.id().cmp(b.id()),
    });
    stories
}

fn format_scope_links(board: &Board, scope: Option<&str>) -> String {
    let Some(s) = scope else {
        return "-".to_string();
    };

    if s.contains('/') {
        let parts: Vec<_> = s.split('/').collect();
        let epic_id = parts[0];
        let voyage_id = parts[1];

        let epic_link = if let Some(epic) = board.epics.get(epic_id) {
            format!("[{}](epics/{}/)", epic.title(), epic_id)
        } else {
            epic_id.to_string()
        };

        let voyage_link = if let Some(voyage) = board.voyages.get(voyage_id) {
            format!(
                "[{}](epics/{}/voyages/{}/)",
                voyage.title(),
                epic_id,
                voyage_id
            )
        } else {
            voyage_id.to_string()
        };

        format!("{} / {}", epic_link, voyage_link)
    } else if let Some(epic) = board.epics.get(s) {
        format!("[{}](epics/{}/)", epic.title(), s)
    } else {
        s.to_string()
    }
}

fn write_story_section(
    output: &mut String,
    board: &Board,
    title: &str,
    stories: &[&Story],
    collapsed: bool,
) {
    if collapsed {
        writeln!(output, "<details>").unwrap();
        writeln!(output, "<summary><h2>{}</h2></summary>", title).unwrap();
        writeln!(output).unwrap();
    } else {
        writeln!(output, "## {}", title).unwrap();
        writeln!(output).unwrap();
    }

    writeln!(output, "| Story | Type | Scope |").unwrap();
    writeln!(output, "|-------|------|-------|").unwrap();

    for story in stories {
        // Stories are now in story bundle directories: stories/[ID]/README.md
        let relative_path = format!("stories/{}/README.md", story.id());
        let link = format!("[{}]({})", story.title(), relative_path);
        let scope_links = format_scope_links(board, story.frontmatter.scope.as_deref());

        writeln!(
            output,
            "| {} | {} | {} |",
            link,
            story.story_type(),
            scope_links
        )
        .unwrap();
    }

    if collapsed {
        writeln!(output).unwrap();
        writeln!(output, "</details>").unwrap();
    }

    writeln!(output).unwrap();
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
        sorted.sort_by(|a, b| match (a.frontmatter.index, b.frontmatter.index) {
            (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => a.id().cmp(b.id()),
        });

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
    epics.sort_by(|a, b| match (a.frontmatter.index, b.frontmatter.index) {
        (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        _ => a.id().cmp(b.id()),
    });

    for epic in epics {
        // Skip done epics (they go in Done section)
        if epic.status() == EpicState::Done {
            continue;
        }

        let voyages = board.voyages_for_epic(epic);
        let done_count = voyages
            .iter()
            .filter(|v| v.status() == VoyageState::Done)
            .count();

        writeln!(
            output,
            "### [{}](epics/{}/) ({}) - {} voyages, {} done",
            epic.title(),
            epic.id(),
            epic.status(),
            voyages.len(),
            done_count
        )
        .unwrap();
        writeln!(output).unwrap();

        if !voyages.is_empty() {
            writeln!(output, "| Voyage | Status |").unwrap();
            writeln!(output, "|--------|--------|").unwrap();

            let mut sorted_voyages = voyages;
            sorted_voyages.sort_by(|a, b| match (a.frontmatter.index, b.frontmatter.index) {
                (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                _ => a.id().cmp(b.id()),
            });

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

fn write_done_section(output: &mut String, board: &Board) {
    // Done epics
    let done_epics: Vec<_> = board
        .epics
        .values()
        .filter(|e| e.status() == EpicState::Done)
        .collect();

    if done_epics.is_empty() {
        return;
    }

    writeln!(output, "<details>").unwrap();
    writeln!(output, "<summary><h2>Done</h2></summary>").unwrap();
    writeln!(output).unwrap();

    // Done epics first
    if !done_epics.is_empty() {
        writeln!(output, "### Completed Epics").unwrap();
        writeln!(output).unwrap();

        let mut sorted_epics = done_epics;
        sorted_epics.sort_by(|a, b| a.id().cmp(b.id()));

        for epic in sorted_epics {
            writeln!(output, "- [{}](epics/{}/)", epic.title(), epic.id()).unwrap();
        }
    }

    writeln!(output).unwrap();
    writeln!(output, "</details>").unwrap();
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
    fn generate_board_readme_includes_in_progress() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0001")
                    .title("Active Story")
                    .stage(StoryState::InProgress),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## In Progress"));
        assert!(readme.contains("0001"));
    }

    #[test]
    fn generate_board_readme_includes_backlog() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0002")
                    .title("Pending Bug")
                    .stage(StoryState::Backlog),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Backlog"));
        assert!(readme.contains("0002"));
    }

    #[test]
    fn generate_board_readme_includes_epics() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic").status("tactical"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("in-progress"))
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Epics"));
        assert!(readme.contains("test-epic"));
        assert!(readme.contains("01-first"));
    }

    #[test]
    fn generate_board_readme_includes_ready_for_acceptance() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0003")
                    .title("Awaiting Review")
                    .stage(StoryState::NeedsHumanVerification),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        assert!(readme.contains("## Ready for Acceptance"));
        assert!(readme.contains("0003"));
    }

    #[test]
    fn generate_board_readme_includes_rejected() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0004")
                    .title("Needs Rework")
                    .stage(StoryState::Rejected),
            )
            .build();
        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        // Rejected is collapsed
        assert!(readme.contains("<summary><h2>Rejected</h2></summary>"));
        assert!(readme.contains("0004"));
    }

    #[test]
    fn generate_board_readme_links_to_story_bundles() {
        let temp = TestBoardBuilder::new()
            .story(
                TestStory::new("0003")
                    .title("Test Story")
                    .stage(StoryState::InProgress),
            )
            .build();

        let board = load_board(temp.path()).unwrap();
        let readme = generate_board_readme(&board);

        // Story should appear in In Progress section
        assert!(readme.contains("## In Progress"));
        assert!(readme.contains("0003"));

        // The link should go to story bundle README
        assert!(readme.contains("stories/0003/README.md"));
    }

    #[test]
    fn generate_board_readme_includes_bearings() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("done-epic").status("done"))
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
