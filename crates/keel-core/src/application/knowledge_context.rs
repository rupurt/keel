//! Shared knowledge context surfacing for execution/planning flows.

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::read_model::knowledge::RankedKnowledge;

/// Collect ranked knowledge for a scope.
pub fn ranked_for_scope(
    board_dir: &Path,
    epic: Option<&str>,
    scope: Option<&str>,
    limit: usize,
) -> Result<Vec<RankedKnowledge>> {
    let all_knowledge = crate::read_model::knowledge::scan_all_knowledge(board_dir)?;
    Ok(crate::read_model::knowledge::rank_relevant_knowledge(
        all_knowledge,
        epic,
        scope,
        limit,
    ))
}

/// Print ranked knowledge with shared formatting. Returns whether anything was printed.
pub fn surface_ranked_knowledge(
    board_dir: &Path,
    heading: &str,
    epic: Option<&str>,
    scope: Option<&str>,
    limit: usize,
    guidance: Option<&str>,
) -> Result<bool> {
    let ranked = ranked_for_scope(board_dir, epic, scope, limit)?;
    if ranked.is_empty() {
        return Ok(false);
    }

    println!();
    println!("{}", heading.yellow().bold());
    for ranked_unit in ranked {
        let knowledge = ranked_unit.knowledge;
        println!(
            "  - [{}] {} (score {:.2}, conf {:.2}, recency {:.2})",
            knowledge.id.cyan(),
            knowledge.title,
            ranked_unit.score,
            knowledge.confidence,
            ranked_unit.recency
        );
        println!("    Insight: {}", knowledge.insight);
    }

    if let Some(message) = guidance {
        println!();
        println!("{message}");
    }

    Ok(true)
}
