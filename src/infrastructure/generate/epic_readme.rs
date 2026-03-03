//! Epic README generation

use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::domain::model::{Board, Epic, VoyageState};

/// Generate and update an epic's README.md
pub fn generate(board_dir: &Path, board: &Board, epic: &Epic) -> anyhow::Result<()> {
    let content = generate_epic_readme(board, epic);
    let readme_path = board_dir.join("epics").join(epic.id()).join("README.md");

    // Update only the generated section
    let original = fs::read_to_string(&readme_path)?;
    let updated = crate::infrastructure::generate::voyage_readme::update_section(
        &original,
        "GENERATED",
        &content,
    )?;
    fs::write(readme_path, updated)?;
    Ok(())
}

/// Generate an epic's README.md content
pub fn generate_epic_readme(board: &Board, epic: &Epic) -> String {
    let mut output = String::new();

    let voyages = board.voyages_for_epic(epic);
    let done_count = voyages
        .iter()
        .filter(|v| v.status() == VoyageState::Done)
        .count();

    // Count stories for this epic
    let story_count = board
        .stories
        .values()
        .filter(|s| s.epic() == Some(epic.id()))
        .count();
    let done_stories = board
        .stories
        .values()
        .filter(|s| {
            s.epic() == Some(epic.id()) && s.stage == crate::domain::model::StoryState::Done
        })
        .count();

    // Progress summary
    writeln!(
        output,
        "**Progress:** {}/{} voyages complete, {}/{} stories done",
        done_count,
        voyages.len(),
        done_stories,
        story_count
    )
    .unwrap();

    // Full table with header (must be contiguous for markdown rendering)
    writeln!(output, "| Voyage | Status | Stories |").unwrap();
    writeln!(output, "|--------|--------|---------|").unwrap();

    if !voyages.is_empty() {
        let mut sorted_voyages = voyages;
        sorted_voyages.sort_by(|a, b| match (a.frontmatter.index, b.frontmatter.index) {
            (Some(ai), Some(bi)) if ai != bi => ai.cmp(&bi),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            _ => a.id().cmp(b.id()),
        });

        for v in sorted_voyages {
            let v_stories = board.stories_for_voyage(v);
            let v_done = v_stories
                .iter()
                .filter(|s| s.stage == crate::domain::model::StoryState::Done)
                .count();

            writeln!(
                output,
                "| [{}](voyages/{}/) | {} | {}/{} |",
                v.title(),
                v.id(),
                v.status(),
                v_done,
                v_stories.len()
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
    use std::fs;
    use tempfile::TempDir;

    fn create_test_board() -> TempDir {
        let temp = TempDir::new().unwrap();
        let root = temp.path();

        // Create flat stories directory (new structure)
        fs::create_dir_all(root.join("stories")).unwrap();

        fs::create_dir_all(root.join("epics/test-epic/voyages/01-first")).unwrap();
        fs::write(
            root.join("epics/test-epic/README.md"),
            r#"---
id: test-epic
title: Test Epic
---
"#,
        )
        .unwrap();
        fs::write(
            root.join("epics/test-epic/voyages/01-first/README.md"),
            r#"---
id: 01-first
title: First Voyage
status: in-progress
---
"#,
        )
        .unwrap();

        fs::write(
            root.join("stories/[FEAT][0001]-active.md"),
            r#"---
id: FEAT0001
title: Active Story
type: feat
status: in-progress
scope: test-epic/01-first
---
"#,
        )
        .unwrap();

        temp
    }

    #[test]
    fn generate_epic_readme_includes_voyages() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();
        let epic = board.epics.get("test-epic").unwrap();
        let readme = generate_epic_readme(&board, epic);

        // Generated content includes full table (header + rows)
        assert!(readme.contains("| Voyage | Status | Stories |"));
        assert!(readme.contains("|--------|--------|---------|"));
        assert!(readme.contains("01-first"));
        assert!(!readme.contains("<!-- BEGIN GENERATED -->"));
        assert!(!readme.contains("<!-- END GENERATED -->"));
    }

    #[test]
    fn generate_epic_readme_includes_progress() {
        let temp = create_test_board();
        let board = load_board(temp.path()).unwrap();
        let epic = board.epics.get("test-epic").unwrap();
        let readme = generate_epic_readme(&board, epic);

        assert!(readme.contains("**Progress:**"));
    }
}
