//! Explore thematic threads and rising patterns command

use anyhow::Result;
use chrono::Utc;
use owo_colors::OwoColorize;
use std::path::Path;

use crate::cli::table::Table;
use crate::read_model::knowledge::navigator::{self, DetectionConfig};
use crate::read_model::knowledge::scanner;

/// Run the explore command to surface rising patterns
pub fn run(board_dir: &Path) -> Result<()> {
    let knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    let signals: Vec<_> = knowledge_list
        .iter()
        .filter_map(|k: &crate::read_model::knowledge::Knowledge| k.to_signal())
        .collect();

    if signals.is_empty() {
        println!("{}", "No reflection signals found to analyze.".yellow());
        return Ok(());
    }

    let config = DetectionConfig::default();
    let now = Utc::now();
    let patterns = navigator::detect_rising_patterns(&signals, now, &config);

    if patterns.is_empty() {
        println!(
            "{}",
            "No significant rising patterns detected in the current window.".yellow()
        );
        return Ok(());
    }

    println!("{}", "Rising Thematic Threads".bold().underline());
    println!("Surfacing recurring patterns from recent reflections.");
    println!();

    let mut table = Table::new(&["RANK", "PATTERN", "TREND", "CONF", "EVIDENCE"]);
    for p in &patterns {
        let trend_display = format!("+{:>3.0}%", p.trend_delta() * 100.0);
        let trend_styled = if p.trend_delta() > 0.2 {
            trend_display.green().bold().to_string()
        } else {
            trend_display.yellow().to_string()
        };

        let conf_display = format!("{:.2}", p.confidence());
        let conf_styled = if p.confidence() > 0.8 {
            conf_display.green().to_string()
        } else {
            conf_display.dimmed().to_string()
        };

        table.row(&[
            &p.rank().unwrap_or(0).to_string(),
            &p.pattern_id().bold().cyan().to_string(),
            &trend_styled,
            &conf_styled,
            &format!("{} refs", p.evidence_ids().len()),
        ]);
    }
    table.print();

    println!();
    println!("{}", "Suggested Action:".bold());
    println!(
        "High-trend patterns indicate areas where a new {} or {} might be needed to formalize the knowledge.",
        "Bearing".cyan(),
        "ADR".purple()
    );

    Ok(())
}
