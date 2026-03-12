//! Shared terminal utilities for CLI presentation.

/// Get the current terminal size, falling back to a sane default.
pub fn get_terminal_size() -> (usize, usize) {
    terminal_size::terminal_size()
        .map(|(w, h)| (w.0 as usize, h.0 as usize))
        .unwrap_or((100, 28))
}

/// Get the current terminal width, falling back to a default.
pub fn get_terminal_width() -> usize {
    get_terminal_size().0
}
