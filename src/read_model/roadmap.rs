//! Canonical roadmap read-model projection.
//!
//! Builds a deterministic, posture-aware view of missions, epics, voyages, and
//! stories for management command output.

use std::collections::HashMap;

use crate::domain::model::{
    Board, Epic, EpicState, Mission, Story, StoryState, Voyage, VoyageState,
};
use crate::domain::state_machine::invariants::story_workable;
use crate::domain::state_machine::mission::MissionStatus;
use crate::infrastructure::utils::cmp_optional_index_then_id;
use crate::read_model::traceability::derive_implementation_dependencies;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadmapEntityKind {
    Mission,
    Epic,
    Voyage,
    Story,
}

impl RoadmapEntityKind {
    fn rank(&self) -> u8 {
        match self {
            Self::Mission => 0,
            Self::Epic => 1,
            Self::Voyage => 2,
            Self::Story => 3,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mission => "mission",
            Self::Epic => "epic",
            Self::Voyage => "voyage",
            Self::Story => "story",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadmapPosture {
    Proceed,
    Park,
    Blocked,
}

impl RoadmapPosture {
    fn rank(&self) -> u8 {
        match self {
            Self::Blocked => 0,
            Self::Park => 1,
            Self::Proceed => 2,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Park => "park",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapRow {
    pub entity_type: RoadmapEntityKind,
    pub entity_id: String,
    pub title: String,
    pub status: String,
    pub posture: RoadmapPosture,
    pub priority: Option<u32>,
    pub blocking_ids: Vec<String>,
}

impl RoadmapRow {
    pub fn blocking_count(&self) -> usize {
        self.blocking_ids.len()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoadmapProjection {
    pub rows: Vec<RoadmapRow>,
}

pub fn project(board: &Board) -> RoadmapProjection {
    let dependencies = derive_implementation_dependencies(board);
    let mut rows = Vec::new();

    let mut stories_by_voyage: HashMap<String, Vec<RoadmapRow>> = HashMap::new();
    let mut voyage_rows_by_epic: HashMap<String, Vec<RoadmapRow>> = HashMap::new();
    let mut epic_rows_by_mission: HashMap<String, Vec<RoadmapRow>> = HashMap::new();

    for story in sorted_stories(board) {
        if story.status().is_terminal() {
            continue;
        }

        let blocking_ids = unresolved_story_blockers(story, &dependencies, board);
        let posture = derive_story_posture(story, board, &blocking_ids);
        let row = RoadmapRow {
            entity_type: RoadmapEntityKind::Story,
            entity_id: story.id().to_string(),
            title: story.title().to_string(),
            status: story.status().to_string(),
            posture,
            priority: story.index(),
            blocking_ids,
        };

        if let Some(voyage_id) = story_parent_voyage_id(story.scope()) {
            stories_by_voyage
                .entry(voyage_id.to_string())
                .or_default()
                .push(row.clone());
        }

        rows.push(row);
    }

    for voyage in sorted_voyages(board) {
        if voyage.status() == VoyageState::Done {
            continue;
        }

        let voyage_children = stories_by_voyage
            .get(voyage.id())
            .cloned()
            .unwrap_or_default();
        let blockers = non_proceeding_children(&voyage_children);
        let posture = derive_voyage_posture(voyage, &blockers, &voyage_children);
        let row = RoadmapRow {
            entity_type: RoadmapEntityKind::Voyage,
            entity_id: voyage.id().to_string(),
            title: voyage.title().to_string(),
            status: voyage.status().to_string(),
            posture,
            priority: voyage.index(),
            blocking_ids: blockers,
        };

        voyage_rows_by_epic
            .entry(voyage.epic_id.clone())
            .or_default()
            .push(row.clone());
        rows.push(row);
    }

    for epic in sorted_epics(board) {
        if epic.status() == EpicState::Done {
            continue;
        }

        let epic_children = voyage_rows_by_epic
            .get(epic.id())
            .cloned()
            .unwrap_or_default();
        let blockers = non_proceeding_children(&epic_children);
        let posture = derive_epic_posture(epic, &blockers, !epic_children.is_empty());
        let row = RoadmapRow {
            entity_type: RoadmapEntityKind::Epic,
            entity_id: epic.id().to_string(),
            title: epic.title().to_string(),
            status: epic.status().to_string(),
            posture,
            priority: epic.index(),
            blocking_ids: blockers,
        };

        if let Some(mission_id) = &epic.frontmatter.mission {
            epic_rows_by_mission
                .entry(mission_id.clone())
                .or_default()
                .push(row.clone());
        }
        rows.push(row);
    }

    for mission in sorted_missions(board) {
        if mission.status().is_terminal() {
            continue;
        }

        let mission_children = epic_rows_by_mission
            .get(mission.id())
            .cloned()
            .unwrap_or_default();
        let blockers = non_proceeding_children(&mission_children);
        let posture = derive_mission_posture(mission, &blockers, !mission_children.is_empty());
        rows.push(RoadmapRow {
            entity_type: RoadmapEntityKind::Mission,
            entity_id: mission.id().to_string(),
            title: mission.title().to_string(),
            status: mission.status().to_string(),
            posture,
            priority: None,
            blocking_ids: blockers,
        });
    }

    for row in rows.iter_mut() {
        row.blocking_ids = dedupe_sorted(row.blocking_ids.clone());
    }

    rows.sort_by(compare_rows);
    rows.dedup_by(|left, right| {
        left.entity_type == right.entity_type && left.entity_id == right.entity_id
    });

    RoadmapProjection { rows }
}

fn story_parent_voyage_id(scope: Option<&str>) -> Option<&str> {
    scope.and_then(|scope| scope.split_once('/').map(|(_, voyage_id)| voyage_id))
}

fn derive_story_posture(story: &Story, board: &Board, blocking_ids: &[String]) -> RoadmapPosture {
    if story.status() != StoryState::Backlog {
        return RoadmapPosture::Park;
    }

    if !blocking_ids.is_empty() {
        return RoadmapPosture::Blocked;
    }

    if story_workable(story, board, &board.root) {
        RoadmapPosture::Proceed
    } else {
        RoadmapPosture::Park
    }
}

fn unresolved_story_blockers(
    story: &Story,
    dependencies: &HashMap<String, Vec<String>>,
    board: &Board,
) -> Vec<String> {
    dependencies
        .get(story.id())
        .map_or_else(Vec::new, |dependencies| {
            dependencies
                .iter()
                .filter_map(|dependency_id| {
                    board
                        .find_story(dependency_id)
                        .is_none_or(|dependency| !dependency.status().is_terminal())
                        .then_some(dependency_id.clone())
                })
                .collect()
        })
}

fn derive_voyage_posture(
    voyage: &Voyage,
    blockers: &[String],
    children: &[RoadmapRow],
) -> RoadmapPosture {
    if voyage.status() == VoyageState::Draft {
        return RoadmapPosture::Park;
    }
    if !blockers.is_empty() {
        return RoadmapPosture::Blocked;
    }
    if children.is_empty() {
        return RoadmapPosture::Park;
    }
    if children
        .iter()
        .any(|child| child.posture == RoadmapPosture::Park)
    {
        return RoadmapPosture::Park;
    }

    RoadmapPosture::Proceed
}

fn derive_epic_posture(epic: &Epic, blockers: &[String], has_children: bool) -> RoadmapPosture {
    if !has_children || epic.status() == EpicState::Draft {
        RoadmapPosture::Park
    } else if !blockers.is_empty() {
        RoadmapPosture::Blocked
    } else {
        RoadmapPosture::Proceed
    }
}

fn derive_mission_posture(
    mission: &Mission,
    blockers: &[String],
    has_children: bool,
) -> RoadmapPosture {
    let posture = match mission.status() {
        MissionStatus::Defining | MissionStatus::Active => {
            if has_children {
                RoadmapPosture::Proceed
            } else {
                RoadmapPosture::Park
            }
        }
        MissionStatus::Paused
        | MissionStatus::Achieved
        | MissionStatus::Verified
        | MissionStatus::Abandoned => RoadmapPosture::Park,
    };

    if blockers.is_empty() {
        posture
    } else {
        RoadmapPosture::Blocked
    }
}

fn non_proceeding_children(children: &[RoadmapRow]) -> Vec<String> {
    children
        .iter()
        .filter(|child| child.posture != RoadmapPosture::Proceed)
        .map(|child| child.entity_id.clone())
        .collect()
}

fn dedupe_sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values.dedup();
    values
}

fn compare_rows(left: &RoadmapRow, right: &RoadmapRow) -> std::cmp::Ordering {
    left.posture
        .rank()
        .cmp(&right.posture.rank())
        .then_with(|| left.entity_type.rank().cmp(&right.entity_type.rank()))
        .then_with(|| {
            let left_priority = left.priority.unwrap_or(u32::MAX);
            let right_priority = right.priority.unwrap_or(u32::MAX);
            left_priority.cmp(&right_priority)
        })
        .then_with(|| left.entity_id.cmp(&right.entity_id))
}

fn sorted_missions(board: &Board) -> Vec<&Mission> {
    let mut missions: Vec<_> = board.missions.values().collect();
    missions.sort_by(|left, right| left.id().cmp(right.id()));
    missions
}

fn sorted_epics(board: &Board) -> Vec<&crate::domain::model::Epic> {
    let mut epics: Vec<_> = board.epics.values().collect();
    epics.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    epics
}

fn sorted_voyages(board: &Board) -> Vec<&Voyage> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    voyages
}

fn sorted_stories(board: &Board) -> Vec<&Story> {
    let mut stories: Vec<_> = board.stories.values().collect();
    stories.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    stories
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};

    #[test]
    fn project_marks_dependency_blocked_stories() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .epic(TestEpic::new("E1").mission("M1").index(1))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .status("planned")
                    .index(1)
                    .srs_content("# SRS\n\n## Functional Requirements\n\n<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->\n| ID | Requirement | Scope | Source | Verification |\n|----|-------------|-------|--------|--------------|\n| SRS-01 | Story one | Scope | FR-01 | test |\n<!-- END FUNCTIONAL_REQUIREMENTS -->\n"),
            )
            .story(
                TestStory::new("S1")
                    .scope("E1/V1")
                    .body("- [ ] [SRS-01/AC-01] story one"),
            )
            .story(
                TestStory::new("S2")
                    .scope("E1/V1")
                    .blocked_by(&["S1"])
                    .body("- [ ] [SRS-01/AC-02] blocked story"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let projection = project(&board);
        let blocked_story = projection
            .rows
            .into_iter()
            .find(|row| row.entity_id == "S2")
            .unwrap();

        assert_eq!(blocked_story.posture, RoadmapPosture::Blocked);
        assert_eq!(blocked_story.blocking_ids, vec!["S1"]);
        assert_eq!(blocked_story.blocking_count(), 1);
    }
}
