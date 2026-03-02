//! Epic command implementations

pub mod done;
pub mod list;
pub mod new;
pub mod reopen;
pub mod show;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum EpicAction {
    /// Create a new epic
    New {
        /// Epic name
        name: String,
        /// Epic description
        #[arg(long, short)]
        description: Option<String>,
        /// Goal/value proposition for the epic
        #[arg(long, short, required = true)]
        goal: Option<String>,
    },
    /// Complete an epic
    Done {
        /// Epic ID (supports fuzzy matching)
        id: String,
    },
    /// Reopen a completed epic
    Reopen {
        /// Epic ID (supports fuzzy matching)
        id: String,
    },
    /// Show epic details
    Show {
        /// Epic ID (supports fuzzy matching)
        id: String,
    },
    /// List epics
    List {
        /// Filter by status
        #[arg(long, short, value_parser = ["strategic", "tactical", "done"])]
        status: Option<String>,
    },
}

/// Run an epic action through the epic interface adapter.
pub fn run(action: EpicAction) -> Result<()> {
    match action {
        EpicAction::New {
            name,
            description,
            goal,
        } => new::run(&name, description.as_deref(), goal.as_deref()),
        EpicAction::Done { id } => done::run(&id),
        EpicAction::Reopen { id } => reopen::run(&id),
        EpicAction::Show { id } => show::run(&id),
        EpicAction::List { status } => list::run(status.as_deref()),
    }
}
