//! Shared terminal utilities for CLI presentation.

/// Get the current terminal width, falling back to a default.
pub fn get_terminal_width() -> usize {
    terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(100)
}
