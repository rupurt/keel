//! Workshop command - focus on items requiring human attention

use anyhow::Result;
use crate::cli::presentation::terminal::get_terminal_width;
use keel::infrastructure::loader::load_board;
use keel::read_model::{workflow_lane_flow, workflow_topology};
use owo_colors::OwoColorize;

/// Run the workshop command
pub fn run(board_dir: &std::path::Path) -> Result<()> {
    let board = load_board(board_dir)?;
    let topology = workflow_topology::load_for_board(board_dir)?;
    let lane_flow = workflow_lane_flow::project(&board, &topology);
    
    let width = get_terminal_width();
    
    println!("\n    ┌{}┐", "─".repeat(width.saturating_sub(6)));
    println!("    │ {: <width$} │", "THE WORKSHOP".bold(), width = width.saturating_sub(8));
    println!("    └{}┘", "─".repeat(width.saturating_sub(6)));
    
    let mut human_items = Vec::new();
    for lane in &lane_flow.lanes {
        if lane.manual_accept && lane.total_count > 0 {
            for source in &lane.source_counts {
                if source.count > 0 {
                    let items: Vec<_> = match source.source.as_str() {
                        "story.needs-human-verification" => board.stories.values()
                            .filter(|s| s.status == keel::domain::model::StoryState::NeedsHumanVerification)
                            .map(|s| format!("Story {} - {}", s.id().yellow(), s.title()))
                            .collect(),
                        "mission.achieved" => board.missions.values()
                            .filter(|m| m.status() == keel::domain::model::MissionStatus::Achieved)
                            .map(|m| format!("Mission {} - {}", m.id().cyan(), m.title()))
                            .collect(),
                        "voyage.draft" => board.voyages.values()
                            .filter(|v| v.status() == keel::domain::state_machine::voyage::VoyageState::Draft)
                            .map(|v| format!("Voyage {} - {}", v.id().magenta(), v.title()))
                            .collect(),
                        "bearing.exploring" => board.bearings.values()
                            .filter(|b| b.status() == keel::domain::model::BearingStatus::Exploring)
                            .map(|b| format!("Bearing {} - {}", b.id().green(), b.title()))
                            .collect(),
                        "bearing.evaluating" => board.bearings.values()
                            .filter(|b| b.status() == keel::domain::model::BearingStatus::Evaluating)
                            .map(|b| format!("Bearing {} - {}", b.id().green(), b.title()))
                            .collect(),
                        "bearing.ready" => board.bearings.values()
                            .filter(|b| b.status() == keel::domain::model::BearingStatus::Ready)
                            .map(|b| format!("Bearing {} - {}", b.id().green(), b.title()))
                            .collect(),
                        _ => vec![],
                    };
                    human_items.extend(items);
                }
            }
        }
    }

    if human_items.is_empty() {
        println!("\n    The workshop is clean. No items require immediate human attention.");
        println!("    Run `keel flow` to see the autonomous pipeline state.");
    } else {
        println!("\n    {} messy items on the bench:", human_items.len().red().bold());
        for item in human_items {
            println!("      - {}", item);
        }
        println!("\n    \"A broken workshop is a messy workshop!\"");
    }

    Ok(())
}
