//! Knowledge command implementations

pub mod explore;
pub mod graph;
pub mod impact;
pub mod list;
pub mod prune;
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
        /// Sort mode: id | story
        #[arg(long, value_name = "MODE", default_value = "id")]
        sort: String,
    },
    /// Show detailed knowledge unit
    Show {
        /// Canonical knowledge ID (9-character base62)
        id: String,
    },
    /// Explore thematic threads and rising patterns
    Explore,
    /// Visualize the knowledge graph (connections between insights and entities)
    Graph {
        /// Zoom level to render
        #[arg(long, value_name = "LEVEL", default_value = "world")]
        zoom: String,
        /// Center the graph on a specific node branch
        #[arg(long, value_name = "ID")]
        focus: Option<String>,
        /// Render once and exit even in an interactive terminal
        #[arg(long = "static")]
        static_output: bool,
    },
    /// Impact/Drift analysis: identify where knowledge is missing or successfully applied
    Impact,
    /// Prune duplicate knowledge and refresh canonical knowledge files
    Prune,
}

pub fn run(_ctx: &spoke_auth::ExecutionContext, board_dir: &Path, action: KnowledgeAction) -> Result<()> {
    match action {
        KnowledgeAction::List {
            category,
            pending,
            sort,
        } => {
            let sort = match sort.as_str() {
                "id" => keel::read_model::knowledge::KnowledgeSort::Id,
                "story" => keel::read_model::knowledge::KnowledgeSort::Story,
                other => anyhow::bail!("invalid sort mode '{}'; expected 'id' or 'story'", other),
            };
            list::run(board_dir, category.as_deref(), pending, sort)
        }
        KnowledgeAction::Show { id } => show::run(board_dir, &id),
        KnowledgeAction::Explore => explore::run(board_dir),
        KnowledgeAction::Graph {
            zoom,
            focus,
            static_output,
        } => {
            let zoom = zoom.parse().map_err(anyhow::Error::msg)?;
            graph::run(board_dir, zoom, focus.as_deref(), static_output)
        }
        KnowledgeAction::Impact => impact::run(board_dir),
        KnowledgeAction::Prune => prune::run(board_dir),
    }
}
