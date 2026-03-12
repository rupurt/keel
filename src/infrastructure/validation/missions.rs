use crate::domain::model::{Board, EpicState, Mission};

use super::{CheckId, GapCategory, Problem};

/// Active missions must have at least one non-draft epic or a bearing in lineage.
pub fn check_mission_actionable_lineage_readiness(
    board: &Board,
    mission: &Mission,
) -> Vec<Problem> {
    if board.mission_child_count(mission.id()) == 0 {
        return Vec::new();
    }

    let has_non_draft_epic = board
        .epics_for_mission(mission.id())
        .into_iter()
        .any(|epic| epic.status() != EpicState::Draft);
    let has_bearing = !board.bearings_for_mission(mission.id()).is_empty();

    if has_non_draft_epic || has_bearing {
        return Vec::new();
    }

    vec![
        Problem::error(
            mission.path.clone(),
            format!(
                "Mission {} cannot be active without at least one planned epic or bearing. Draft epics alone do not create actionable mission work.",
                mission.id()
            ),
        )
        .with_scope(mission.id())
        .with_category(GapCategory::Coherence)
        .with_check_id(CheckId::MissionDefinitionReadiness),
    ]
}
