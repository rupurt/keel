//! Knowledge command implementations

pub mod explore;
pub mod graph;
pub mod impact;
pub mod list;
pub mod show;

use anyhow::Result;
use clap::Subcommand;
use std::path::Path;

#[derive(Subcommand, Debug)]
pub enum KnowledgeAction {
    /// List all knowledge units
    List {
        /// Filter by category
        #[arg(long, short)]
        category: Option<String>,
        /// Only show pending (unapplied) knowledge
        #[arg(long, short)]
        pending: bool,
    },
    /// Show detailed knowledge unit
    Show {
        /// Knowledge ID (e.g., L001)
        id: String,
    },
    /// Explore thematic threads and rising patterns
    Explore,
    /// Visualize the knowledge graph (connections between insights and entities)
    Graph,
    /// Impact/Drift analysis: identify where knowledge is missing or successfully applied
    Impact,
}

pub fn run(board_dir: &Path, action: KnowledgeAction) -> Result<()> {
    match action {
        KnowledgeAction::List { category, pending } => {
            list::run(board_dir, category.as_deref(), pending)
        }
        KnowledgeAction::Show { id } => show::run(board_dir, &id),
        KnowledgeAction::Explore => explore::run(board_dir),
        KnowledgeAction::Graph => graph::run(board_dir),
        KnowledgeAction::Impact => impact::run(board_dir),
    }
}
