//! Capacity projection adapter for flow rendering.
//!
//! Re-exports canonical capacity DTOs and uses the read-model projection.

use crate::domain::model::Board;
use crate::read_model::capacity;

pub use crate::read_model::capacity::{
    ChargeState, EpicCapacity, EpicCapacityReport, SystemCapacity,
};

/// Calculate capacity via canonical read-model projection.
pub fn calculate_system_capacity(board: &Board) -> SystemCapacity {
    capacity::project(board)
}
