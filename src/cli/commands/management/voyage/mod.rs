//! Voyage command implementations

pub mod done;
pub mod list;
pub mod new;
pub mod plan;
pub mod show;
pub mod start;

use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum VoyageAction {
    /// Create a new voyage
    New {
        /// Voyage name
        name: String,
        /// Associated epic
        #[arg(long, short)]
        epic: String,
        /// Goal/value proposition for the voyage
        #[arg(long, short, required = true)]
        goal: Option<String>,
    },
    /// Start a voyage
    Start {
        /// Voyage ID (supports fuzzy matching)
        id: String,
        /// Force start even if preconditions fail
        #[arg(long, short)]
        force: bool,
        /// Expected board version for optimistic locking (SRS-05)
        #[arg(long)]
        expect_version: Option<u64>,
    },
    /// Plan a voyage (promote from draft to planned)
    Plan {
        /// Voyage ID (supports fuzzy matching)
        id: String,
        /// Skip interactive review (still validates)
        #[arg(long)]
        no_review: bool,
    },
    /// Complete a voyage
    Done {
        /// Voyage ID (supports fuzzy matching)
        id: String,
        /// What went well?
        #[arg(long)]
        well: Option<String>,
        /// What was hard?
        #[arg(long)]
        hard: Option<String>,
        /// What would you do differently?
        #[arg(long)]
        different: Option<String>,
    },
    /// Show voyage details
    Show {
        /// Voyage ID (supports fuzzy matching)
        id: String,
    },
    /// List voyages
    List {
        /// Filter by epic
        #[arg(long, short)]
        epic: Option<String>,
        /// Filter by status
        #[arg(long, short, value_parser = ["draft", "planned", "in-progress", "done"])]
        status: Option<String>,
    },
}

/// Run a voyage action through the voyage interface adapter.
pub fn run(action: VoyageAction) -> Result<()> {
    match action {
        VoyageAction::New { name, epic, goal } => new::run(&name, &epic, goal.as_deref()),
        VoyageAction::Start {
            id,
            force,
            expect_version,
        } => start::run(&id, force, expect_version),
        VoyageAction::Plan { id, no_review } => plan::run(&id, no_review),
        VoyageAction::Done {
            id,
            well,
            hard,
            different,
        } => done::run(&id, well, hard, different),
        VoyageAction::Show { id } => show::run(&id),
        VoyageAction::List { epic, status } => list::run(epic.as_deref(), status.as_deref()),
    }
}
