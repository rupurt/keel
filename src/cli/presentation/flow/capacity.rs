//! Capacity projection adapter for flow rendering.
//!
//! Re-exports canonical capacity DTOs and uses the read-model projection.

use anyhow::Result;

use crate::domain::model::Board;
use crate::infrastructure::loader::load_board;
use crate::read_model::capacity;

pub use crate::read_model::capacity::{
    ChargeState, EpicCapacity, EpicCapacityReport, SystemCapacity,
};

/// Calculate capacity via canonical read-model projection.
pub fn calculate_system_capacity(board: &Board) -> SystemCapacity {
    capacity::project(board)
}

/// Render-oriented capacity runner.
pub fn run(board_dir: &std::path::Path) -> Result<()> {
    let board = load_board(board_dir)?;
    let capacity = calculate_system_capacity(&board);

    println!("System Capacity Matrix");
    println!("======================");
    println!();

    for epic in capacity.epics {
        println!("{}:", epic.id);
        println!("  Charge:    {:?}", epic.charge_state);
        println!("  Ready:     {}", epic.capacity.ready);
        println!("  In-Flight: {}", epic.capacity.in_flight);
        println!("  Blocked:   {}", epic.capacity.blocked);
        println!();
    }

    Ok(())
}
