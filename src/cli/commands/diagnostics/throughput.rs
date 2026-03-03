//! Throughput command - weekly throughput and timing sparklines.

use anyhow::Result;

use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::presentation::theme::Theme;
use crate::cli::presentation::throughput::graphs::render_throughput_graphs;
use crate::infrastructure::loader::load_board;

/// Run the throughput command
pub fn run(board_dir: &std::path::Path, no_color: bool) -> Result<()> {
    let board = load_board(board_dir)?;
    let history = crate::read_model::throughput_history::project_default(&board);
    crate::infrastructure::throughput_history_store::save_if_changed(board_dir, &history)?;
    let width = get_terminal_width();
    let use_color = Theme::should_use_color(no_color);
    let theme = Theme::for_color_mode(use_color);

    let output = render_throughput_graphs(&history, width, &theme);
    println!("{}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn test_throughput_run() {
        let temp = TestBoardBuilder::new().build();
        let result = run(temp.path(), true);
        assert!(result.is_ok());
    }
}
