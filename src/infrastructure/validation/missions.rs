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

pub fn mission_non_terminal_children(board: &Board, mission: &Mission) -> Vec<String> {
    let mut children = Vec::new();

    for epic in board.epics_for_mission(mission.id()) {
        if epic.status() != EpicState::Done {
            children.push(format!("epic {} ({})", epic.id(), epic.status()));
        }
    }

    for bearing in board.bearings_for_mission(mission.id()) {
        if !bearing.is_complete() {
            children.push(format!("bearing {} ({})", bearing.id(), bearing.status()));
        }
    }

    for adr in board
        .adrs
        .values()
        .filter(|adr| adr.frontmatter.mission.as_deref() == Some(mission.id()))
    {
        if !adr.status().is_terminal() {
            children.push(format!("adr {} ({})", adr.id(), adr.status()));
        }
    }

    children.sort();
    children
}

pub fn check_mission_terminal_children(board: &Board, mission: &Mission) -> Vec<Problem> {
    let non_terminal_children = mission_non_terminal_children(board, mission);
    if non_terminal_children.is_empty() {
        return Vec::new();
    }

    vec![
        Problem::error(
            mission.path.clone(),
            format!(
                "Mission {} has non-terminal child entities: {}. Complete, lay, park, decline, or otherwise terminalize all mission children before mission completion.",
                mission.id(),
                non_terminal_children.join(", "),
            ),
        )
        .with_scope(mission.id())
        .with_category(GapCategory::Coherence)
        .with_check_id(CheckId::MissionNonTerminalChildren),
    ]
}
