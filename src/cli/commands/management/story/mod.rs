//! Story command implementations

pub mod accept;
pub mod audit;
pub(crate) mod guidance;
pub mod ice;
pub mod link;
pub mod list;
pub mod new;
pub mod record;
pub mod reflect;
pub mod reject;
pub mod show;
pub mod start;
pub mod submit;
pub mod thaw;
pub mod unlink;

use anyhow::Result;
use clap::Subcommand;

use crate::infrastructure::config::find_board_dir;

#[derive(Subcommand, Debug)]
pub enum StoryAction {
    /// Create a new story
    New {
        /// Story title
        title: String,
        /// Story type
        #[arg(long, short = 't', default_value = "feat")]
        r#type: String,
    },
    /// Move story to in-progress
    Start {
        /// Story ID (supports fuzzy matching)
        id: String,
        /// Expected board version for optimistic locking (SRS-05)
        #[arg(long)]
        expect_version: Option<u64>,
    },
    /// Submit story for acceptance (in-progress -> needs-human-verification)
    Submit {
        /// Story ID (supports fuzzy matching)
        id: String,
    },
    /// Accept a story (needs-human-verification -> done)
    Accept {
        /// Story ID (supports fuzzy matching)
        id: String,
        /// Acknowledge manual verification steps have been verified by a human
        #[arg(long)]
        human: bool,
        /// Optional reflection observation to capture at acceptance time
        #[arg(long)]
        reflect: Option<String>,
    },
    /// Create a reflection scaffold for a story bundle
    Reflect {
        /// Story ID (supports fuzzy matching)
        id: String,
    },
    /// Reject a story (needs-human-verification -> rejected)
    Reject {
        /// Story ID (supports fuzzy matching)
        id: String,
        /// Reason for rejection
        reason: String,
    },
    /// Move story to icebox
    Ice {
        /// Story ID (supports fuzzy matching)
        id: String,
    },
    /// Move story from icebox to backlog
    Thaw {
        /// Story ID (supports fuzzy matching)
        id: String,
    },
    /// Show story details
    Show {
        /// Story ID (supports fuzzy matching)
        id: String,
    },
    /// List stories
    List {
        /// Filter by stage
        #[arg(
            long,
            short,
            value_parser = [
                "backlog",
                "in-progress",
                "needs-human-verification",
                "done",
                "rejected",
                "icebox",
            ]
        )]
        stage: Option<String>,
        /// Filter by epic
        #[arg(long, short)]
        epic: Option<String>,
        /// Only show stories that have reflections
        #[arg(long)]
        reflections: bool,
    },
    /// Link story to voyage
    Link {
        /// Story ID
        id: String,
        /// Voyage ID
        voyage: String,
    },
    /// Unlink story from voyage
    Unlink {
        /// Story ID
        id: String,
        /// Voyage ID
        voyage: String,
    },
    /// Record proof for an acceptance criterion
    Record {
        /// Story ID (supports fuzzy matching)
        id: String,
        /// Acceptance criterion number (1-based index)
        #[arg(long, short)]
        ac: Option<usize>,
        /// Command to run (overrides the one in the story)
        #[arg(long, short)]
        cmd: Option<String>,
        /// Manual proof text (for manual ACs)
        #[arg(long, short)]
        msg: Option<String>,
        /// Trigger LLM-Judge verification
        #[arg(long)]
        judge: bool,
        /// Attach existing files as proof
        #[arg(long = "file", short = 'f', action = clap::ArgAction::Append)]
        files: Vec<String>,
    },
}

/// Run a story action through the story interface adapter.
pub fn run(action: StoryAction) -> Result<()> {
    match action {
        StoryAction::New { title, r#type } => new::run(&title, &r#type, None, None),
        StoryAction::Start { id, expect_version } => {
            start::run(&find_board_dir()?, &id, expect_version)
        }
        StoryAction::Submit { id } => submit::run(&find_board_dir()?, &id),
        StoryAction::Accept { id, human, reflect } => {
            accept::run(&find_board_dir()?, &id, human, reflect.as_deref())
        }
        StoryAction::Reflect { id } => reflect::run(&find_board_dir()?, &id),
        StoryAction::Reject { id, reason } => reject::run(&find_board_dir()?, &id, &reason),
        StoryAction::Ice { id } => ice::run(&find_board_dir()?, &id),
        StoryAction::Thaw { id } => thaw::run(&find_board_dir()?, &id),
        StoryAction::Show { id } => show::run(&id),
        StoryAction::List {
            stage,
            epic,
            reflections,
        } => list::run(stage.as_deref(), epic.as_deref(), reflections),
        StoryAction::Link { id, voyage } => link::run(&id, &voyage),
        StoryAction::Unlink { id, voyage } => unlink::run(&id, &voyage),
        StoryAction::Record {
            id,
            ac,
            cmd,
            msg,
            judge,
            files,
        } => record::run(&find_board_dir()?, id, ac, cmd, msg, judge, files),
    }
}
