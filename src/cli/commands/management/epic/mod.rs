//! Epic command implementations

pub mod list;
pub mod new;
pub mod show;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum EpicAction {
    /// Create a new epic
    New {
        /// Epic name
        name: String,
        /// Goal/value proposition for the epic
        #[arg(long, short, required = true)]
        goal: String,
    },
    /// Show epic details
    Show {
        /// Epic ID (supports fuzzy matching)
        id: String,
    },
    /// List epics
    List {
        /// Filter by status
        #[arg(long, short, value_parser = ["draft", "active", "done"])]
        status: Option<String>,
    },
}

/// Run an epic action through the epic interface adapter.
pub fn run(action: EpicAction) -> Result<()> {
    match action {
        EpicAction::New { name, goal } => new::run(&name, &goal),
        EpicAction::Show { id } => show::run(&id),
        EpicAction::List { status } => list::run(status.as_deref()),
    }
}
