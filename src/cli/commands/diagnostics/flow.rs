//! Flow command - aggregate pull-system diagnostics

use anyhow::Result;

use crate::cli::presentation::flow::display::render_annotated_flow;
use crate::cli::presentation::terminal::get_terminal_width;
use crate::infrastructure::loader::load_board;
use crate::read_model::flow_status;

/// Run the flow command
pub fn run(board_dir: &std::path::Path, no_color: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let projection = flow_status::project(&board);
    let width = get_terminal_width();

    let output = render_annotated_flow(&board, &projection.flow, width, no_color);
    println!("{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn test_flow_run() {
        let temp = TestBoardBuilder::new().build();
        let result = run(temp.path(), true);
        assert!(result.is_ok());
    }
}
