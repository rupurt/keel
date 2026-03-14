//! Mission commands - lifecycle management

use anyhow::Result;
use clap::Subcommand;

pub mod attach;
pub mod list;
pub mod new;
pub mod next;
pub mod show;

#[derive(Subcommand, Debug)]
pub enum MissionAction {
    /// Create a new mission
    New {
        /// Mission title
        title: String,
    },
    /// List all missions
    List,
    /// Show mission details
    Show {
        /// Mission ID or HEAD selector (HEAD, HEAD~, HEAD~~, HEAD^)
        id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
        /// Render a compact three-bullet summary
        #[arg(long, short)]
        compact: bool,
    },
    /// Attach a bearing to a mission
    Attach {
        /// Mission ID
        mission_id: String,
        /// Bearing ID
        bearing_id: String,
    },
    /// Show next steps across all roles for a mission
    Next {
        /// Mission ID (omit to auto-select the highest-priority actionable mission)
        id: Option<String>,
        /// Show a compact three-bullet status summary
        #[arg(long)]
        status: bool,
        /// Show an extended report with novel findings and strategic unblockers
        #[arg(long)]
        extended: bool,
    },
    /// Refine mission charter (elicitation)
    Refine {
        /// Mission ID
        id: String,
        /// Answer to the last question
        #[arg(long)]
        answer: Option<String>,
    },
    /// Activate a mission (Defining -> Active)
    Activate {
        /// Mission ID
        id: String,
    },
    /// Pause a mission (Active -> Paused)
    Pause {
        /// Mission ID
        id: String,
    },
    /// Achieve a mission (Active -> Achieved)
    Achieve {
        /// Mission ID
        id: String,
    },
    /// Verify a mission (Achieved -> Verified)
    Verify {
        /// Mission ID
        id: String,
    },
    /// Abandon a mission (Active or Paused -> Abandoned)
    Abandon {
        /// Mission ID
        id: String,
    },
    /// Add entry to mission log
    Log {
        /// Mission ID
        id: String,
        /// Log entry text
        #[arg(long)]
        entry: String,
    },
    /// Digest mission log entries
    Digest {
        /// Mission ID
        id: String,
    },
}

/// Run a mission action
pub fn run(action: MissionAction) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    match action {
        MissionAction::New { title } => run_new(&title),
        MissionAction::List => list::run(),
        MissionAction::Show { id, json, compact } => show::run(&id, json, compact),
        MissionAction::Attach {
            mission_id,
            bearing_id,
        } => attach::run(&mission_id, &bearing_id),
        MissionAction::Next {
            id,
            status,
            extended,
        } => next::run(id.as_deref(), status, extended),
        MissionAction::Refine { id, answer } => {
            keel::application::mission_lifecycle::MissionLifecycleService::refine(
                &board_dir,
                &id,
                answer.as_deref(),
            )
        }
        MissionAction::Activate { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::activate(&board_dir, &id)
        }
        MissionAction::Pause { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::pause(&board_dir, &id)
        }
        MissionAction::Achieve { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::achieve(&board_dir, &id)
        }
        MissionAction::Verify { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::verify(&board_dir, &id)
        }
        MissionAction::Abandon { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::abandon(&board_dir, &id)
        }
        MissionAction::Log { id, entry } => {
            keel::application::mission_lifecycle::MissionLifecycleService::log(
                &board_dir, &id, &entry,
            )
        }
        MissionAction::Digest { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::digest(&board_dir, &id)
        }
    }
}

pub fn run_new(title: &str) -> Result<()> {
    let mission_id = new::run(title)?;
    println!("Created mission: {}", mission_id);
    Ok(())
}
