//! List voyages command

use crate::cli::table::Table;
use crate::domain::model::{Board, Voyage};
use crate::infrastructure::loader::load_board;
use anyhow::Result;

const DEFAULT_VOYAGE_STATUSES: &[&str] = &["draft", "planned", "in-progress"];
const ALLOWED_VOYAGE_STATUSES: &[&str] = &["draft", "planned", "in-progress", "done"];

/// Check if a planning document has been filled in (vs still containing template placeholders)
pub fn doc_status(voyage_dir: &std::path::Path, filename: &str) -> &'static str {
    let path = voyage_dir.join(filename);
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            // Check for TODO: but only outside of <!-- ... --> comments
            let mut search_text = content.clone();
            while let Some(start) = search_text.find("<!--") {
                if let Some(end) = search_text[start..].find("-->") {
                    search_text.replace_range(start..start + end + 3, "");
                } else {
                    break;
                }
            }

            if search_text.contains("TODO:") {
                "todo"
            } else {
                "done"
            }
        }
        Err(_) => "missing",
    }
}

/// List voyages with optional filters
pub fn run(epic: Option<&str>, status_filters: &[String]) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let status_filter = crate::cli::commands::management::status_filter::resolve_status_filter(
        status_filters,
        DEFAULT_VOYAGE_STATUSES,
        ALLOWED_VOYAGE_STATUSES,
    )?;
    let voyages = collect_voyages(&board, epic, &status_filter);

    if voyages.is_empty() {
        let done_hidden = board
            .voyages
            .values()
            .any(|voyage| voyage.status().to_string() == "done" && !status_filter.contains("done"));
        let tip = if done_hidden {
            crate::cli::commands::management::status_filter::build_append_status_suggestion(
                &["done"],
                "completed voyages",
            )
        } else {
            None
        };
        let state = crate::cli::commands::management::status_filter::build_empty_list_state(
            "voyages",
            !board.voyages.is_empty(),
            &status_filter,
            tip,
        );
        crate::cli::commands::management::status_filter::print_empty_list_state(&state);
        return Ok(());
    }

    let mut table = Table::new(&["ID", "TITLE", "STATUS", "SRS", "SDD", "STORIES", "EPIC"]);
    for v in voyages {
        // Get voyage directory (parent of README.md path)
        let voyage_dir = v.path.parent().unwrap_or(&v.path);

        let srs = doc_status(voyage_dir, "SRS.md");
        let sdd = doc_status(voyage_dir, "SDD.md");

        // Count stories for this voyage
        let stories = board.stories_for_voyage(v);
        let done = stories
            .iter()
            .filter(|s| s.status == crate::domain::model::StoryState::Done)
            .count();
        let total = stories.len();
        let story_count = format!("{}/{}", done, total);

        table.row(&[
            &crate::cli::style::styled_voyage_id(v.id()),
            &v.frontmatter.title,
            &v.status().to_string(),
            srs,
            sdd,
            &story_count,
            &crate::cli::style::styled_epic_id(&v.epic_id),
        ]);
    }
    table.print();

    Ok(())
}

#[cfg(test)]
fn list_voyages<'a>(
    board: &'a Board,
    epic: Option<&str>,
    status_filters: &[String],
) -> Result<Vec<&'a Voyage>> {
    let status_filter = crate::cli::commands::management::status_filter::resolve_status_filter(
        status_filters,
        DEFAULT_VOYAGE_STATUSES,
        ALLOWED_VOYAGE_STATUSES,
    )?;

    Ok(collect_voyages(board, epic, &status_filter))
}

fn collect_voyages<'a>(
    board: &'a Board,
    epic: Option<&str>,
    status_filter: &crate::cli::commands::management::status_filter::StatusFilter,
) -> Vec<&'a Voyage> {
    let mut voyages: Vec<_> = board
        .voyages
        .values()
        .filter(|v| epic.is_none() || epic.map(|e| e == v.epic_id).unwrap_or(false))
        .filter(|v| status_filter.contains(&v.status().to_string()))
        .collect();

    voyages.sort_by(|a, b| {
        // 1. Sort by Epic index
        let epic_a = board.epics.get(&a.epic_id);
        let epic_b = board.epics.get(&b.epic_id);

        let epic_cmp = match (
            epic_a.and_then(|e| e.index()),
            epic_b.and_then(|e| e.index()),
        ) {
            (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.epic_id.cmp(&b.epic_id),
        };

        if epic_cmp != std::cmp::Ordering::Equal {
            return epic_cmp;
        }

        // 2. Sort by Voyage index
        let voyage_cmp = match (a.index(), b.index()) {
            (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => a.id().cmp(b.id()),
        };

        if voyage_cmp != std::cmp::Ordering::Equal {
            return voyage_cmp;
        }

        a.id().cmp(b.id())
    });
    voyages
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};

    #[test]
    fn test_list_voyages_filtering() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .epic(TestEpic::new("epic2"))
            .voyage(TestVoyage::new("01-v1", "epic1").status("draft"))
            .voyage(TestVoyage::new("01-v2", "epic2").status("planned"))
            .build();
        let board = load_board(temp.path()).unwrap();

        let all = list_voyages(&board, None, &[]).unwrap();
        assert_eq!(all.len(), 2);

        let by_epic = list_voyages(&board, Some("epic1"), &[]).unwrap();
        assert_eq!(by_epic.len(), 1);
        assert_eq!(by_epic[0].id(), "01-v1");

        let by_status = list_voyages(&board, None, &["planned".to_string()]).unwrap();
        assert_eq!(by_status.len(), 1);
        assert_eq!(by_status[0].id(), "01-v2");
    }

    #[test]
    fn test_list_voyages_default_excludes_done_but_plus_done_adds_it_back() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("01-draft", "epic1").status("draft"))
            .voyage(TestVoyage::new("02-done", "epic1").status("done"))
            .build();
        let board = load_board(temp.path()).unwrap();

        let default = list_voyages(&board, None, &[]).unwrap();
        assert_eq!(default.len(), 1);
        assert_eq!(default[0].id(), "01-draft");

        let with_done = list_voyages(&board, None, &["+done".to_string()]).unwrap();
        assert_eq!(with_done.len(), 2);

        let done_only = list_voyages(&board, None, &["done".to_string()]).unwrap();
        assert_eq!(done_only.len(), 1);
        assert_eq!(done_only[0].id(), "02-done");
    }

    #[test]
    fn test_doc_status() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path();

        // Missing
        assert_eq!(doc_status(path, "SRS.md"), "missing");

        // Todo
        std::fs::write(path.join("SRS.md"), "TODO: fill this").unwrap();
        assert_eq!(doc_status(path, "SRS.md"), "todo");

        // Done
        std::fs::write(path.join("SRS.md"), "Requirement 1").unwrap();
        assert_eq!(doc_status(path, "SRS.md"), "done");
    }
}
