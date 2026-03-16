//! List watches command

use anyhow::Result;

use crate::cli::table::Table;
use keel::domain::model::Watch;
use keel::infrastructure::loader::load_board;

/// List all watches
pub fn run() -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;

    let mut watches: Vec<&Watch> = board.watches.values().collect();
    watches.sort_by(|a, b| a.id().cmp(b.id()));

    if watches.is_empty() {
        println!("No watches found on this board.");
        return Ok(());
    }

    let mut table = Table::new(&["ID", "TITLE", "LIMIT"]);
    for watch in watches {
        table.row(&[
            &crate::cli::style::styled_watch_id(watch.id()),
            watch.title(),
            &format!("{}h", watch.limit_hours()),
        ]);
    }
    table.print();

    Ok(())
}
