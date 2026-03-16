//! Watch commands - time constraint management

use anyhow::Result;
use clap::Subcommand;

pub mod list;
pub mod new;
pub mod show;

#[derive(Subcommand, Debug)]
pub enum WatchAction {
    /// Create a new watch constraint
    New {
        /// Watch title
        title: String,
        /// Time limit in hours
        #[arg(long, default_value = "12")]
        limit: u32,
    },
    /// List all watches
    List,
    /// Show watch details
    Show {
        /// Watch ID
        id: String,
    },
}

/// Run a watch action
pub fn run(action: WatchAction) -> Result<()> {
    match action {
        WatchAction::New { title, limit } => run_new(&title, limit),
        WatchAction::List => list::run(),
        WatchAction::Show { id } => show::run(&id),
    }
}

pub fn run_new(title: &str, limit: u32) -> Result<()> {
    let watch_id = new::run(title, limit)?;
    println!("Created watch: {}", watch_id);
    Ok(())
}
