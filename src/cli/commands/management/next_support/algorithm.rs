#![allow(dead_code)]
//! Pull-system decision algorithm for selecting the next task.

use anyhow::Result;
use std::path::Path;

use keel::domain::model::{Board, Story, StoryState};
use keel::domain::policy::queue::compare_work_item_ids;
use keel::read_model::queue_policy::{self, DraftVoyageQueueCategory};

#[derive(Debug, Clone)]
pub struct ItemFilter<'a> {
    pub mission_id: Option<&'a str>,
    pub actor_role: Option<&'a keel::domain::model::taxonomy::RoleTaxonomy>,
}

impl<'a> ItemFilter<'a> {
    pub fn none() -> Self {
        Self {
            mission_id: None,
            actor_role: None,
        }
    }

    pub fn matches_story(&self, board: &Board, story: &Story) -> bool {
        if let Some(id) = self.mission_id {
            if !board.is_story_in_mission(story, id) {
                return false;
            }
        }
        if let Some(role) = self.actor_role {
            if !keel::domain::model::taxonomy::actor_matches_story(role, story) {
                return false;
            }
        }
        true
    }

    pub fn matches_adr(&self, board: &Board, adr: &keel::domain::model::Adr) -> bool {
        if let Some(id) = self.mission_id {
            return board.is_adr_in_mission(adr, id);
        }
        true
    }

    pub fn matches_voyage(&self, board: &Board, voyage: &keel::domain::model::Voyage) -> bool {
        if let Some(id) = self.mission_id {
            return board.is_voyage_in_mission(voyage, id);
        }
        true
    }

    pub fn matches_bearing(&self, board: &Board, bearing: &keel::domain::model::Bearing) -> bool {
        if let Some(id) = self.mission_id {
            return board.is_bearing_in_mission(bearing, id);
        }
        true
    }
}

/// Single decision about what to work on next.
#[derive(Debug)]
pub enum NextDecision {
    /// Work on an existing story
    Work(StoryDecision),
    /// Proposed ADR needs review
    Decision(AdrDecision),
    /// Stories need human acceptance
    Accept(AcceptDecision),
    /// Research pipeline needs attention
    Research(ResearchDecision),
    /// No work found
    Empty(EmptyDecision),
    /// System is blocked on verification
    Blocked(BlockedDecision),
    /// Strategic gap (voyage needs stories)
    NeedsStories(DecomposeDecision),
    /// Strategic gap (voyage needs planning)
    NeedsPlanning(DecomposeDecision),
    /// Strategic gap (epic needs PRD)
    NeedsPRD(NeedsPRDDecision),
    /// Active mission needs work created
    Mission(MissionDecision),
    /// Multiple actionable missions
    Missions(MissionsDecision),
}

#[derive(Debug)]
pub struct MissionsDecision {
    pub missions: Vec<MissionDecision>,
}

#[derive(Debug)]
pub struct MissionDecision {
    pub mission: keel::domain::model::Mission,
    pub unmet_goals: Vec<keel::infrastructure::validation::charter::ParsedMissionGoal>,
    pub suggestion: String,
}

#[derive(Debug)]
pub struct StoryDecision {
    pub story: Story,
    pub is_continuation: bool,
    pub warning: Option<String>,
}

#[derive(Debug)]
pub struct AdrDecision {
    pub adrs: Vec<keel::domain::model::Adr>,
    pub blocked_stories: Vec<Story>,
}

#[derive(Debug)]
pub struct AcceptDecision {
    pub stories: Vec<Story>,
}

#[derive(Debug)]
pub struct ResearchDecision {
    pub bearings: Vec<keel::domain::model::Bearing>,
}

#[derive(Debug)]
pub struct EmptyDecision {
    pub suggestions: Vec<String>,
}

#[derive(Debug)]
pub struct BlockedDecision {
    pub story: Story,
    pub count: usize,
}

#[derive(Debug)]
pub struct DecomposeDecision {
    pub voyages: Vec<keel::domain::model::Voyage>,
}

#[derive(Debug)]
pub struct NeedsPRDDecision {
    pub epics: Vec<keel::domain::model::Epic>,
}

/// Calculate the single most important next action.
pub fn calculate_next(
    board: &Board,
    board_dir: &Path,
    agent_mode: bool,
    filter: &ItemFilter,
) -> Result<NextDecision> {
    let metrics = keel::read_model::flow_status::project(board);
    let queue_policy_snapshot = queue_policy::project(&metrics);

    // 1. Check for blocking verification backlog (human only)
    if !agent_mode && queue_policy_snapshot.verification.blocks_human_next() {
        let ready = board
            .stories
            .values()
            .filter(|s| filter.matches_story(board, s))
            .find(|s| s.status == StoryState::NeedsHumanVerification)
            .cloned();

        if let Some(ready) = ready {
            return Ok(NextDecision::Blocked(BlockedDecision {
                story: ready,
                count: metrics.verification.count,
            }));
        }
    }

    // 2. Check for proposed ADRs (human only)
    if !agent_mode {
        let adrs: Vec<_> = board
            .adrs
            .values()
            .filter(|a| a.status() == keel::domain::model::AdrStatus::Proposed)
            .filter(|a| filter.matches_adr(board, a))
            .cloned()
            .collect();

        if !adrs.is_empty() {
            let blocked_stories: Vec<_> = board
                .stories
                .values()
                .filter(|s| s.status == StoryState::Backlog)
                .filter(|s| {
                    s.frontmatter
                        .governed_by
                        .iter()
                        .any(|id| adrs.iter().any(|a| a.id() == id))
                })
                .cloned()
                .collect();

            return Ok(NextDecision::Decision(AdrDecision {
                adrs,
                blocked_stories,
            }));
        }
    }

    // 3. Acceptance (human only)
    if !agent_mode {
        let stories: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::NeedsHumanVerification)
            .filter(|s| filter.matches_story(board, s))
            .cloned()
            .collect();

        if !stories.is_empty() {
            return Ok(NextDecision::Accept(AcceptDecision { stories }));
        }
    }

    // 4. Research (human only)
    if !agent_mode {
        let bearings: Vec<_> = board
            .bearings
            .values()
            .filter(|b| {
                matches!(
                    b.frontmatter.status,
                    keel::domain::model::BearingStatus::Exploring
                        | keel::domain::model::BearingStatus::Evaluating
                )
            })
            .filter(|b| filter.matches_bearing(board, b))
            .cloned()
            .collect();

        if !bearings.is_empty() {
            return Ok(NextDecision::Research(ResearchDecision { bearings }));
        }
    }

    // 5. Strategy: Decompose or Plan (human only)
    if !agent_mode {
        let mut needs_prd = Vec::new();
        let mut needs_stories = Vec::new();
        let mut needs_planning = Vec::new();

        // 5a. Check for draft epics (missing PRD content)
        for epic in board
            .epics
            .values()
            .filter(|e| e.status() == keel::domain::model::EpicState::Draft)
            .filter(|e| {
                filter
                    .mission_id
                    .map(|id| e.frontmatter.mission.as_deref() == Some(id))
                    .unwrap_or(true)
            })
            .cloned()
        {
            let prd_path = epic.path.parent().unwrap().join("PRD.md");
            if !keel::infrastructure::validation::structural::check_epic_prd_authored_content(
                &prd_path,
            )
            .is_empty()
            {
                needs_prd.push(epic);
            }
        }

        if !needs_prd.is_empty() {
            return Ok(NextDecision::NeedsPRD(NeedsPRDDecision {
                epics: needs_prd,
            }));
        }

        // 5b. Check for draft voyages
        for voyage in board
            .voyages
            .values()
            .filter(|v| v.status() == keel::domain::state_machine::voyage::VoyageState::Draft)
            .filter(|v| filter.matches_voyage(board, v))
            .cloned()
        {
            let story_count = board.stories_for_voyage(&voyage).len();
            match queue_policy::classify_draft_voyage(story_count) {
                DraftVoyageQueueCategory::NeedsStories => needs_stories.push(voyage),
                DraftVoyageQueueCategory::NeedsPlanning => needs_planning.push(voyage),
            }
        }

        if !needs_stories.is_empty() {
            return Ok(NextDecision::NeedsStories(DecomposeDecision {
                voyages: needs_stories,
            }));
        }

        if !needs_planning.is_empty() {
            return Ok(NextDecision::NeedsPlanning(DecomposeDecision {
                voyages: needs_planning,
            }));
        }
    }

    // 6. Implementation work selection (agent only)
    if agent_mode {
        // 6a. Continue in-progress work (actor-specific)
        let in_progress: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::InProgress)
            .filter(|s| filter.matches_story(board, s))
            .collect();

        if let Some(story) = in_progress.first() {
            return Ok(NextDecision::Work(StoryDecision {
                story: (*story).clone(),
                is_continuation: true,
                warning: None,
            }));
        }

        // 6b. Select from backlog
        let deps = keel::read_model::traceability::derive_implementation_dependencies(board);
        let workable_backlog: Vec<_> = board
            .stories
            .values()
            .filter(|s| s.status == StoryState::Backlog)
            .filter(|s| {
                keel::domain::state_machine::invariants::story_workable(s, board, board_dir)
            })
            .filter(|s| filter.matches_story(board, s))
            .collect();

        let mut candidates: Vec<_> = workable_backlog
            .iter()
            .copied()
            .filter(|s| {
                // Unblocked if no dependencies OR all dependencies are Done
                deps.get(s.id()).is_none_or(|dep_ids| {
                    dep_ids.iter().all(|id| {
                        board
                            .stories
                            .get(id)
                            .map(|dep| dep.status == StoryState::Done)
                            .unwrap_or(false)
                    })
                })
            })
            .collect();

        candidates.sort_by(|a, b| compare_work_item_ids(a.id(), b.id()));

        if let Some(story) = candidates.first() {
            return Ok(NextDecision::Work(StoryDecision {
                story: (*story).clone(),
                is_continuation: false,
                warning: None,
            }));
        }
    }

    // 7. Mission Steering (Shared)
    let actionable_missions: Vec<_> = board
        .missions
        .values()
        .filter(|m| m.status() == keel::domain::model::MissionStatus::Active)
        .filter(|m| filter.mission_id.map(|id| m.id() == id).unwrap_or(true))
        .filter_map(|mission| {
            let charter_path = mission.path.parent().unwrap().join("CHARTER.md");
            let content = std::fs::read_to_string(&charter_path).unwrap_or_default();
            let goals = keel::infrastructure::validation::charter::parse_mission_goals(&content);

            let unmet_goals: Vec<_> = goals
                .iter()
                .filter(|g| {
                    matches!(
                        g.verification,
                        keel::infrastructure::validation::charter::GoalVerification::Board(_)
                    ) && !is_goal_met(board, g.verification.raw())
                })
                .cloned()
                .collect();

            if !unmet_goals.is_empty() {
                let epics = board.epics_for_mission(mission.id());
                let bearings = board.bearings_for_mission(mission.id());

                let suggestion = if bearings.iter().any(|b| {
                    matches!(
                        b.frontmatter.status,
                        keel::domain::model::BearingStatus::Exploring
                            | keel::domain::model::BearingStatus::Evaluating
                    )
                }) {
                    "Complete active research bearings".to_string()
                } else if let Some(epic) = epics
                    .iter()
                    .find(|e| e.status() == keel::domain::model::EpicState::Draft)
                {
                    let prd_path = epic.path.parent().unwrap().join("PRD.md");
                    if keel::infrastructure::validation::structural::check_epic_prd_authored_content(
                        &prd_path,
                    )
                    .is_empty()
                    {
                        format!("Decompose Epic {} into voyages", epic.id())
                    } else {
                        format!("Author PRD for Epic {}", epic.id())
                    }
                } else if let Some(voyage) = board.voyages.values().find(|v| {
                    v.status() == keel::domain::state_machine::voyage::VoyageState::Draft
                        && epics.iter().any(|e| e.id() == v.epic_id)
                }) {
                    format!("Decompose or author planning for Voyage {}", voyage.id())
                } else if epics
                    .iter()
                    .any(|e| e.status() != keel::domain::model::EpicState::Done)
                {
                    "Progress existing mission-scoped epics".to_string()
                } else if bearings.is_empty() && epics.is_empty() {
                    "Create first bearing or epic for mission".to_string()
                } else {
                    "Create next bearing or epic to address unmet goals".to_string()
                };

                Some(MissionDecision {
                    mission: mission.clone(),
                    unmet_goals,
                    suggestion,
                })
            } else {
                None
            }
        })
        .collect();

    if !actionable_missions.is_empty() {
        if actionable_missions.len() == 1 {
            return Ok(NextDecision::Mission(
                actionable_missions.into_iter().next().unwrap(),
            ));
        } else {
            return Ok(NextDecision::Missions(MissionsDecision {
                missions: actionable_missions,
            }));
        }
    }

    Ok(NextDecision::Empty(EmptyDecision {
        suggestions: if agent_mode {
            vec!["Board is empty. Add new bearings or epics to begin.".to_string()]
        } else {
            vec![
                "Refuel the backlog".to_string(),
                "Check for drifted research".to_string(),
            ]
        },
    }))
}

fn is_goal_met(board: &Board, target: &str) -> bool {
    let target = target.trim();
    if target.is_empty() || target == "..." {
        return false;
    }

    if let Some(epic) = board.epics.get(target) {
        return epic.status() == keel::domain::model::EpicState::Done;
    }
    if let Some(voyage) = board.voyages.get(target) {
        return voyage.status() == keel::domain::state_machine::voyage::VoyageState::Done;
    }
    if let Some(story) = board.stories.get(target) {
        return story.status == keel::domain::model::StoryState::Done;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use keel::domain::model::StoryState;
    use keel::test_helpers::{TestBearing, TestBoardBuilder, TestStory, TestMission, TestAdr};

    fn assert_human_queue_decision(decision: &NextDecision) {
        match decision {
            NextDecision::Work(_) => panic!("human mode must not return execution work"),
            NextDecision::Decision(_) => {}
            NextDecision::Accept(_) => {}
            NextDecision::Research(_) => {}
            NextDecision::Empty(_) => {}
            NextDecision::Blocked(_) => {}
            NextDecision::NeedsStories(_) => {}
            NextDecision::NeedsPlanning(_) => {}
            NextDecision::NeedsPRD(_) => {}
            NextDecision::Mission(_) => {}
            NextDecision::Missions(_) => {}
        }
    }

    #[test]
    fn human_mode_finds_adr_decisions() {
        let temp = TestBoardBuilder::new()
            .adr(
                TestAdr::new("ADR1").title("ADR 1").status("proposed")
            )
            .build();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let next = calculate_next(&board, temp.path(), false, &ItemFilter::none()).unwrap();

        assert_human_queue_decision(&next);
        match next {
            NextDecision::Decision(d) => assert_eq!(d.adrs[0].id(), "ADR1"),
            other => panic!("expected ADR decision, got {other:?}"),
        }
    }

    #[test]
    fn human_mode_finds_research() {
        let temp = TestBoardBuilder::new()
            .bearing(TestBearing::new("B1").status("exploring"))
            .build();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let next = calculate_next(&board, Path::new("test"), false, &ItemFilter::none()).unwrap();

        assert_human_queue_decision(&next);
        match next {
            NextDecision::Research(d) => assert_eq!(d.bearings[0].id(), "B1"),
            other => panic!("expected research decision, got {other:?}"),
        }
    }

    #[test]
    fn agent_mode_prefers_in_progress() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::InProgress))
            .story(TestStory::new("S2").status(StoryState::Backlog))
            .build();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let next = calculate_next(&board, temp.path(), true, &ItemFilter::none()).unwrap();

        match next {
            NextDecision::Work(d) => {
                assert_eq!(d.story.id(), "S1");
                assert!(d.is_continuation);
            }
            other => panic!("expected work decision, got {other:?}"),
        }
    }

    #[test]
    fn agent_mode_selects_backlog() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").status(StoryState::Backlog))
            .build();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let next = calculate_next(&board, temp.path(), true, &ItemFilter::none()).unwrap();

        match next {
            NextDecision::Work(d) => {
                assert_eq!(d.story.id(), "S1");
                assert!(!d.is_continuation);
            }
            other => panic!("expected work decision, got {other:?}"),
        }
    }

    #[test]
    fn agent_mode_reports_mission_when_queue_empty_but_goals_unmet() {
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("active"))
            .build();

        let charter_path = temp.path().join("missions/M1/CHARTER.md");
        let charter = r#"
## Goals
| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Test goal | board: E1 |
"#;
        fs::write(charter_path, charter).unwrap();

        let board = keel::infrastructure::loader::load_board(temp.path()).unwrap();
        let next = calculate_next(&board, temp.path(), true, &ItemFilter::none()).unwrap();

        match next {
            NextDecision::Mission(d) => {
                assert_eq!(d.mission.id(), "M1");
                assert_eq!(d.unmet_goals.len(), 1);
            }
            other => panic!("expected mission decision, got {other:?}"),
        }
    }
}
