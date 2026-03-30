//! Mission commands - lifecycle management

use anyhow::Result;
use clap::{ArgGroup, Subcommand};

pub mod attach;
pub mod list;
pub mod new;
pub mod next;
pub mod play;
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
        /// Render the watch constraint visualization
        #[arg(long)]
        scene: bool,
    },
    /// Attach a mission child entity to a mission
    #[command(group(
        ArgGroup::new("target")
            .args(["bearing", "epic", "adr"])
            .required(true)
            .multiple(false)
    ))]
    Attach {
        /// Mission ID
        mission_id: String,
        /// Bearing ID to attach
        #[arg(long, value_name = "BEARING_ID")]
        bearing: Option<String>,
        /// Epic ID to attach
        #[arg(long, value_name = "EPIC_ID")]
        epic: Option<String>,
        /// ADR ID to attach
        #[arg(long, value_name = "ADR_ID")]
        adr: Option<String>,
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
        /// Render a visual representation of the current board scene
        #[arg(long)]
        scene: bool,
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
    /// Resume a mission (Paused -> Active)
    Resume {
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
        /// Verification artifact (e.g. GIF)
        #[arg(long, short)]
        artifact: Option<String>,
    },
    /// Play mission verification artifacts
    Play {
        /// Mission ID (optional, if omitted plays verified missions in last-completed order)
        id: Option<String>,
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
pub fn run(ctx: &spoke_auth::ExecutionContext, action: MissionAction) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    match action {
        MissionAction::New { title } => run_new(&title),
        MissionAction::List => list::run(),
        MissionAction::Show {
            id,
            json,
            compact,
            scene,
        } => show::run(&id, json, compact, scene),
        MissionAction::Attach {
            mission_id,
            bearing,
            epic,
            adr,
        } => attach::run(
            &mission_id,
            bearing.as_deref(),
            epic.as_deref(),
            adr.as_deref(),
        ),
        MissionAction::Next {
            id,
            status,
            extended,
            scene,
        } => next::run(id.as_deref(), status, extended, scene),
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
        MissionAction::Resume { id } => {
            keel::application::mission_lifecycle::MissionLifecycleService::resume(&board_dir, &id)
        }
        MissionAction::Achieve { id } => {
            let result = keel::application::mission_lifecycle::MissionLifecycleService::achieve(
                ctx, &board_dir, &id,
            );
            if result.is_ok() {
                crate::cli::presentation::audio::play(
                    crate::cli::presentation::audio::SoundEvent::MissionAchieved,
                );
            }
            result
        }
        MissionAction::Verify { id, artifact } => {
            let result = keel::application::mission_lifecycle::MissionLifecycleService::verify(
                &board_dir,
                &id,
                artifact.as_deref(),
            );
            if result.is_ok() {
                crate::cli::presentation::audio::play(
                    crate::cli::presentation::audio::SoundEvent::MissionVerified,
                );
            }
            result
        }
        MissionAction::Play { id } => play::run(id.as_deref()),
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
