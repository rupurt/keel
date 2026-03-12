use crate::domain::model::{Board, EpicState, Mission};

use super::{CheckId, GapCategory, Problem};

/// Active missions must have at least one non-draft epic so there is planned work.
pub fn check_mission_planned_epic_readiness(board: &Board, mission: &Mission) -> Vec<Problem> {
    if board.mission_child_count(mission.id()) == 0 {
        return Vec::new();
    }

    if board
        .epics_for_mission(mission.id())
        .into_iter()
        .any(|epic| epic.status() != EpicState::Draft)
    {
        return Vec::new();
    }

    vec![
        Problem::error(
            mission.path.clone(),
            format!(
                "Mission {} cannot be active without at least one planned epic. Draft epics do not create actionable mission work.",
                mission.id()
            ),
        )
        .with_scope(mission.id())
        .with_category(GapCategory::Coherence)
        .with_check_id(CheckId::MissionDefinitionReadiness),
    ]
}
