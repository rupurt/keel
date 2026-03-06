//! Epic command implementations

pub mod list;
pub mod new;
pub mod show;

use anyhow::Result;
use clap::Subcommand;

const EPIC_STATUS_VALUES: &[&str] = &["draft", "active", "done"];

pub(crate) fn parse_epic_status(value: &str) -> Result<String, String> {
    crate::cli::commands::management::status_filter::validate_status_arg(value, EPIC_STATUS_VALUES)
}

#[derive(Subcommand, Debug)]
pub enum EpicAction {
    /// Create a new epic
    New {
        /// Epic name
        name: String,
        /// Problem statement for the epic
        #[arg(long, short, required = true)]
        problem: String,
    },
    /// Show epic details
    Show {
        /// Epic ID (supports fuzzy matching)
        id: String,
    },
    /// List epics
    List {
        /// Filter by status. Repeat to override or use + / - to modify the default active view.
        #[arg(long, short, action = clap::ArgAction::Append, value_parser = parse_epic_status)]
        status: Vec<String>,
    },
}

/// Run an epic action through the epic interface adapter.
pub fn run(action: EpicAction) -> Result<()> {
    match action {
        EpicAction::New { name, problem } => new::run(&name, &problem),
        EpicAction::Show { id } => show::run(&id),
        EpicAction::List { status } => list::run(&status),
    }
}
