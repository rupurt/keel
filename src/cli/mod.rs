//! CLI layer modules: command adapters and terminal presentation.

use std::path::PathBuf;

use anyhow::Result;

pub mod command_tree;
pub mod commands;
pub mod presentation;
mod runtime;
pub mod style;
pub mod table;

pub use command_tree::build_cli;
pub use runtime::run;

/// Resolve board directory for CLI command execution.
pub fn resolve_board_dir() -> Result<PathBuf> {
    crate::infrastructure::config::find_board_dir()
}
