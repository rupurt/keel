//! List epics command

use anyhow::Result;

use crate::cli::table::Table;
use crate::domain::model::{Board, Epic};
use crate::infrastructure::loader::load_board;

/// List epics with optional status filter
pub fn run(status: Option<&str>) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let epics = list_epics(&board, status);

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

fn list_epics<'a>(board: &'a Board, status: Option<&str>) -> Vec<&'a Epic> {
    let mut epics: Vec<_> = board
        .epics
        .values()
        .filter(|e| {
            status.is_none() || status.map(|s| s == e.status().to_string()).unwrap_or(false)
        })
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

        let all = list_epics(&board, None);
        assert_eq!(all.len(), 3);

        let active = list_epics(&board, Some("active"));
        assert_eq!(active.len(), 1);
        assert_eq!(active[0].id(), "active-epic");

        let draft = list_epics(&board, Some("draft"));
        assert_eq!(draft.len(), 1);
        assert_eq!(draft[0].id(), "draft-epic");

        let done = list_epics(&board, Some("done"));
        assert_eq!(done.len(), 1);
        assert_eq!(done[0].id(), "done-epic");
    }
}
