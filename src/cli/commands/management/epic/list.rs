//! List epics command

use anyhow::Result;

use crate::cli::table::Table;
use crate::domain::model::{Board, Epic};
use crate::infrastructure::loader::load_board;

const DEFAULT_EPIC_STATUSES: &[&str] = &["draft", "active"];
const ALLOWED_EPIC_STATUSES: &[&str] = &["draft", "active", "done"];

/// List epics with optional status filters.
pub fn run(status_filters: &[String]) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let status_filter = crate::cli::commands::management::status_filter::resolve_status_filter(
        status_filters,
        DEFAULT_EPIC_STATUSES,
        ALLOWED_EPIC_STATUSES,
    )?;
    let epics = collect_epics(&board, &status_filter);

    if epics.is_empty() {
        let done_hidden = board
            .epics
            .values()
            .any(|epic| epic.status().to_string() == "done" && !status_filter.contains("done"));
        let tip = if done_hidden {
            crate::cli::commands::management::status_filter::build_append_status_suggestion(
                &["done"],
                "completed epics",
            )
        } else {
            None
        };
        let state = crate::cli::commands::management::status_filter::build_empty_list_state(
            "epics",
            !board.epics.is_empty(),
            &status_filter,
            tip,
        );
        crate::cli::commands::management::status_filter::print_empty_list_state(&state);
        return Ok(());
    }

    let mut table = Table::new(&["ID", "TITLE", "STATUS"]);
    for epic in epics {
        table.row(&[
            &crate::cli::style::styled_epic_id(epic.id()),
            &epic.frontmatter.title,
            &epic.status().to_string(),
        ]);
    }
    table.print();

    Ok(())
}

#[cfg(test)]
fn list_epics<'a>(board: &'a Board, status_filters: &[String]) -> Result<Vec<&'a Epic>> {
    let status_filter = crate::cli::commands::management::status_filter::resolve_status_filter(
        status_filters,
        DEFAULT_EPIC_STATUSES,
        ALLOWED_EPIC_STATUSES,
    )?;

    Ok(collect_epics(board, &status_filter))
}

fn collect_epics<'a>(
    board: &'a Board,
    status_filter: &crate::cli::commands::management::status_filter::StatusFilter,
) -> Vec<&'a Epic> {
    let mut epics: Vec<_> = board
        .epics
        .values()
        .filter(|e| status_filter.contains(&e.status().to_string()))
        .collect();

    epics.sort_by(|a, b| a.id().cmp(b.id()));
    epics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};

    #[test]
    fn test_list_epics_filtering() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("active-epic"))
            .epic(TestEpic::new("draft-epic"))
            .epic(TestEpic::new("done-epic"))
            .voyage(TestVoyage::new("a1", "active-epic").status("in-progress"))
            .voyage(TestVoyage::new("d1", "draft-epic").status("draft"))
            .voyage(TestVoyage::new("x1", "done-epic").status("done"))
            .build();
        let board = load_board(temp.path()).unwrap();

        let default = list_epics(&board, &[]).unwrap();
        assert_eq!(default.len(), 2);
        assert_eq!(default[0].id(), "active-epic");
        assert_eq!(default[1].id(), "draft-epic");

        let active = list_epics(&board, &["active".to_string()]).unwrap();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id(), "active-epic");

        let draft = list_epics(&board, &["draft".to_string()]).unwrap();
        assert_eq!(draft.len(), 1);
        assert_eq!(draft[0].id(), "draft-epic");

        let done = list_epics(&board, &["+done".to_string()]).unwrap();
        assert_eq!(done.len(), 3);

        let done_only = list_epics(&board, &["done".to_string()]).unwrap();
        assert_eq!(done_only.len(), 1);
        assert_eq!(done_only[0].id(), "done-epic");

        let draft_and_done =
            list_epics(&board, &["draft".to_string(), "+done".to_string()]).unwrap();
        assert_eq!(draft_and_done.len(), 2);
        assert_eq!(draft_and_done[0].id(), "done-epic");
        assert_eq!(draft_and_done[1].id(), "draft-epic");

        let done = done_only;
        assert_eq!(done.len(), 1);
        assert_eq!(done[0].id(), "done-epic");
    }
}
