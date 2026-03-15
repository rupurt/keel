//! Routine command implementations

pub mod list;
pub mod new;
pub mod show;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum RoutineAction {
    /// Create a new routine
    New {
        /// Routine title
        title: String,
        /// Canonical target scope (epic or epic/voyage)
        #[arg(long = "target-scope")]
        target_scope: String,
        /// Opaque cadence metadata entries as key=value pairs
        #[arg(long = "cadence", action = clap::ArgAction::Append, required = true)]
        cadence: Vec<String>,
    },
    /// List routines
    List,
    /// Show routine details
    Show {
        /// Routine ID or HEAD selector (HEAD, HEAD~, HEAD~~, HEAD^)
        id: String,
    },
}

/// Run a routine action through the routine interface adapter.
pub fn run(action: RoutineAction) -> Result<()> {
    match action {
        RoutineAction::New {
            title,
            target_scope,
            cadence,
        } => new::run(&title, &target_scope, &cadence).map(|_| ()),
        RoutineAction::List => list::run(),
        RoutineAction::Show { id } => show::run(&id),
    }
}
