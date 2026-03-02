//! Knowledge graph visualization command

use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::path::Path;

use crate::read_model::knowledge::scanner;

/// Run the graph command to visualize knowledge connections
pub fn run(board_dir: &Path) -> Result<()> {
    let knowledge_list = scanner::scan_all_knowledge(board_dir)?;

    if knowledge_list.is_empty() {
        println!("{}", "No knowledge found to graph.".yellow());
        return Ok(());
    }

    println!("{}", "Project Knowledge Graph".bold().underline());
    println!("Visualizing insights linked to project entities.");
    println!();

    // Group by source file
    let mut by_source: HashMap<String, Vec<_>> = HashMap::new();
    for k in &knowledge_list {
        let source_key = format!("{}:{}", k.source_type, k.source.display());
        by_source.entry(source_key).or_default().push(k);
    }

    let mut sources: Vec<_> = by_source.keys().collect();
    sources.sort();

    for source in sources {
        let insights = by_source.get(source).unwrap();
        println!("{} ({})", source.bold().cyan(), insights.len().dimmed());

        for (i, k) in insights.iter().enumerate() {
            let is_last = i == insights.len() - 1;
            let connector = if is_last { "└──" } else { "├──" };

            println!(
                "  {} [{}] {} ({})",
                connector.dimmed(),
                k.id.bold(),
                k.title,
                k.category.dimmed()
            );
        }
        println!();
    }

    Ok(())
}
