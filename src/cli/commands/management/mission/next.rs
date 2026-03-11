use anyhow::Result;
use std::collections::BTreeSet;
use owo_colors::OwoColorize;

use keel::infrastructure::loader::load_board;
use keel::read_model::workflow_topology;
use crate::cli::commands::management::next_support::{calculate_next, format_decision, ItemFilter};
use keel::domain::model::MissionStatus;

/// Run the mission next command
pub fn run(mission_id: &str) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    
    let mission = board.require_mission(mission_id)?;
    
    if mission.status() == MissionStatus::Verified {
        // "when the mission has been verified returns nothing and a non zero exit code"
        std::process::exit(1);
    }

    let topology = workflow_topology::load_for_board(&board_dir)?;
    
    // Get all unique role families from the topology
    let mut role_families: BTreeSet<String> = topology.roles.keys().cloned().collect();
    // Also include default examples
    role_families.insert(topology.management_role_example().to_string());
    role_families.insert(topology.delivery_role_example().to_string());

    println!("Next steps for mission {}:", mission_id.bold());
    println!();

    let mut found_any = false;

    for role_name in role_families {
        let role_taxonomy = keel::domain::model::taxonomy::parse(&role_name)?;
        let actor_context = topology.resolve_actor_context(&role_taxonomy)?;
        
        let agent_mode = matches!(
            actor_context.queue_lane,
            keel::read_model::queue_policy::ActorQueueLane::Execution
        );

        let filter = ItemFilter {
            mission_id: Some(mission_id),
            actor_role: Some(&role_taxonomy),
        };

        let decision = calculate_next(&board, &board_dir, agent_mode, &filter)?;
        
        // Only show if it's not Empty or if we want to show everything?
        // The user said "shows the next step across all roles to resolve indecision points"
        // Let's show the role and its decision.
        
        println!("{}:", role_name.bold().blue());
        println!("  {}", format_decision(&decision));
        found_any = true;
    }

    if !found_any {
        println!("No active roles or next steps found for this mission.");
    }

    Ok(())
}
