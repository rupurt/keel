//! Flow-lane projection derived from the resolved workflow topology.

use std::collections::BTreeMap;

use crate::domain::model::Board;
use crate::read_model::workflow_topology::{ResolvedWorkflowTopology, queue_source_catalog};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneFlowProjection {
    pub lanes: Vec<LaneFlowCard>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneFlowCard {
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub parallel: bool,
    pub manual_accept: bool,
    pub total_count: usize,
    pub source_counts: Vec<LaneSourceCount>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaneSourceCount {
    pub source: String,
    pub count: usize,
}

pub fn project(board: &Board, topology: &ResolvedWorkflowTopology) -> LaneFlowProjection {
    let source_counts = count_queue_sources(board);

    let lanes = topology
        .ordered_lanes()
        .into_iter()
        .map(|lane| {
            let source_counts: Vec<_> = lane
                .sources
                .iter()
                .map(|source| LaneSourceCount {
                    source: source.clone(),
                    count: *source_counts.get(source).unwrap_or(&0),
                })
                .collect();
            let total_count = source_counts.iter().map(|source| source.count).sum();

            LaneFlowCard {
                name: lane.name.clone(),
                description: lane.description.clone(),
                priority: lane.priority,
                parallel: lane.parallel,
                manual_accept: lane.manual_accept,
                total_count,
                source_counts,
            }
        })
        .collect();

    LaneFlowProjection { lanes }
}

fn count_queue_sources(board: &Board) -> BTreeMap<String, usize> {
    let mut counts: BTreeMap<String, usize> = queue_source_catalog()
        .iter()
        .map(|source| ((*source).to_string(), 0))
        .collect();

    for bearing in board.bearings.values() {
        if board.is_bearing_paused_by_mission(bearing) {
            continue;
        }
        increment_source_count(&mut counts, &format!("bearing.{}", bearing.status()));
    }

    for story in board.stories.values() {
        if board.is_story_paused_by_mission(story) {
            continue;
        }
        increment_source_count(&mut counts, &format!("story.{}", story.status()));
    }

    for voyage in board.voyages.values() {
        if board.is_voyage_paused_by_mission(voyage) {
            continue;
        }
        increment_source_count(&mut counts, &format!("voyage.{}", voyage.status()));
    }

    for mission in board.missions.values() {
        increment_source_count(&mut counts, &format!("mission.{}", mission.status()));
    }

    counts
}

fn increment_source_count(counts: &mut BTreeMap<String, usize>, source: &str) {
    if let Some(count) = counts.get_mut(source) {
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::config::{Config, LaneConfig, RoleFamilyConfig};
    use crate::read_model::workflow_topology;
    use crate::test_helpers::{
        TestBearing, TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage,
    };

    fn count_for_source(card: &LaneFlowCard, source: &str) -> Option<usize> {
        card.source_counts
            .iter()
            .find(|entry| entry.source == source)
            .map(|entry| entry.count)
    }

    #[test]
    fn project_orders_lanes_by_priority_and_counts_only_selected_sources() {
        let mut config = Config::default();
        config.workflow.defaults.management_lane = "review".to_string();
        config.workflow.defaults.delivery_lane = "delivery".to_string();

        config.roles.insert(
            "manager".to_string(),
            RoleFamilyConfig {
                default_lane: "review".to_string(),
                operational_contract: "manager-core".to_string(),
            },
        );
        config.roles.insert(
            "operator".to_string(),
            RoleFamilyConfig {
                default_lane: "delivery".to_string(),
                operational_contract: "operator-core".to_string(),
            },
        );
        config.roles.insert(
            "researcher".to_string(),
            RoleFamilyConfig {
                default_lane: "research".to_string(),
                operational_contract: "researcher-core".to_string(),
            },
        );

        config.lanes.insert(
            "review".to_string(),
            LaneConfig {
                description: "Manual review work".to_string(),
                include: vec!["story.needs-human-verification".to_string()],
                exclude: Vec::new(),
                parallel: false,
                manual_accept: true,
                priority: 300,
            },
        );
        config.lanes.insert(
            "delivery".to_string(),
            LaneConfig {
                description: "Delivery work".to_string(),
                include: vec!["story.*".to_string()],
                exclude: vec![
                    "story.done".to_string(),
                    "story.icebox".to_string(),
                    "story.needs-human-verification".to_string(),
                    "story.rejected".to_string(),
                ],
                parallel: true,
                manual_accept: false,
                priority: 200,
            },
        );
        config.lanes.insert(
            "research".to_string(),
            LaneConfig {
                description: "Research work".to_string(),
                include: vec!["bearing.exploring".to_string()],
                exclude: Vec::new(),
                parallel: false,
                manual_accept: false,
                priority: 100,
            },
        );

        let topology = workflow_topology::resolve(&config).unwrap();
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::NeedsHumanVerification))
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .story(TestStory::new("S3").status(StoryState::Done))
            .bearing(TestBearing::new("B1").status("exploring"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = project(&board, &topology);

        assert_eq!(
            projection
                .lanes
                .iter()
                .map(|lane| lane.name.as_str())
                .collect::<Vec<_>>(),
            vec!["review", "delivery", "research"]
        );

        let review = &projection.lanes[0];
        assert_eq!(review.total_count, 1);
        assert_eq!(
            count_for_source(review, "story.needs-human-verification"),
            Some(1)
        );

        let delivery = &projection.lanes[1];
        assert_eq!(delivery.total_count, 1);
        assert_eq!(count_for_source(delivery, "story.backlog"), Some(1));
        assert_eq!(count_for_source(delivery, "story.in-progress"), Some(0));
        assert_eq!(count_for_source(delivery, "story.done"), None);

        let research = &projection.lanes[2];
        assert_eq!(research.total_count, 1);
        assert_eq!(count_for_source(research, "bearing.exploring"), Some(1));
    }

    #[test]
    fn project_skips_child_work_for_paused_missions() {
        let mut config = Config::default();
        config.workflow.defaults.management_lane = "review".to_string();
        config.workflow.defaults.delivery_lane = "delivery".to_string();

        config.roles.insert(
            "manager".to_string(),
            RoleFamilyConfig {
                default_lane: "review".to_string(),
                operational_contract: "manager-core".to_string(),
            },
        );
        config.roles.insert(
            "operator".to_string(),
            RoleFamilyConfig {
                default_lane: "delivery".to_string(),
                operational_contract: "operator-core".to_string(),
            },
        );

        config.lanes.insert(
            "review".to_string(),
            LaneConfig {
                description: "Manual review work".to_string(),
                include: vec![
                    "mission.paused".to_string(),
                    "story.needs-human-verification".to_string(),
                ],
                exclude: Vec::new(),
                parallel: false,
                manual_accept: true,
                priority: 200,
            },
        );
        config.lanes.insert(
            "delivery".to_string(),
            LaneConfig {
                description: "Delivery work".to_string(),
                include: vec!["story.backlog".to_string(), "voyage.planned".to_string()],
                exclude: Vec::new(),
                parallel: true,
                manual_accept: false,
                priority: 100,
            },
        );

        let topology = workflow_topology::resolve(&config).unwrap();
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("paused"))
            .mission(TestMission::new("M2").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .epic(TestEpic::new("E2").mission("M2"))
            .voyage(TestVoyage::new("V1", "E1").status("planned"))
            .voyage(TestVoyage::new("V2", "E2").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("E1/V1")
                    .status(StoryState::NeedsHumanVerification),
            )
            .story(
                TestStory::new("S2")
                    .scope("E1/V1")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S3")
                    .scope("E2/V2")
                    .status(StoryState::Backlog),
            )
            .bearing(TestBearing::new("B1").mission("M1").status("exploring"))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        let projection = project(&board, &topology);
        let review = &projection.lanes[0];
        let delivery = &projection.lanes[1];

        assert_eq!(count_for_source(review, "mission.paused"), Some(1));
        assert_eq!(
            count_for_source(review, "story.needs-human-verification"),
            Some(0)
        );
        assert_eq!(count_for_source(delivery, "story.backlog"), Some(1));
        assert_eq!(count_for_source(delivery, "voyage.planned"), Some(1));
    }
}
