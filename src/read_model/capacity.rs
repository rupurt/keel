//! Canonical capacity projection service.
//!
//! This read model centralizes epic execution capacity calculations so flow
//! renderers and diagnostics consume one deterministic type and algorithm.

use std::collections::HashMap;

use crate::domain::model::{Board, StoryState};
use crate::read_model::execution_queue::{BacklogQueueState, classify_backlog_story};
use crate::read_model::traceability::derive_implementation_dependencies;

/// Summary of execution capacity per epic.
#[derive(Debug, Clone)]
pub struct SystemCapacity {
    pub epics: Vec<EpicCapacityReport>,
}

#[derive(Debug, Clone)]
pub struct EpicCapacity {
    pub ready: usize,
    pub in_flight: usize,
    pub blocked: usize,
    pub inactive: usize,
    pub done: usize,
}

#[derive(Debug, Clone)]
pub struct EpicCapacityReport {
    pub id: String,
    pub title: String,
    pub charge_state: ChargeState,
    pub capacity: EpicCapacity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChargeState {
    Blocked,
    Discharged,
    Trickle,
    Charged,
    Supercharged,
    Overloaded,
}

/// Build canonical capacity projection from a board snapshot.
pub fn project(board: &Board) -> SystemCapacity {
    let deps = derive_implementation_dependencies(board);
    let mut epic_map: HashMap<String, EpicCapacityReport> = HashMap::new();

    for epic in board.epics.values() {
        epic_map.insert(
            epic.id().to_string(),
            EpicCapacityReport {
                id: epic.id().to_string(),
                title: epic.frontmatter.title.clone(),
                charge_state: ChargeState::Discharged,
                capacity: EpicCapacity {
                    ready: 0,
                    in_flight: 0,
                    blocked: 0,
                    inactive: 0,
                    done: 0,
                },
            },
        );
    }

    for story in board.stories.values() {
        let Some(epic_id) = story.epic() else {
            continue;
        };
        let Some(report) = epic_map.get_mut(epic_id) else {
            continue;
        };

        match story.status {
            StoryState::Done => report.capacity.done += 1,
            StoryState::InProgress => report.capacity.in_flight += 1,
            StoryState::Backlog => match classify_backlog_story(board, story, &deps) {
                BacklogQueueState::Ready => report.capacity.ready += 1,
                BacklogQueueState::Blocked => report.capacity.blocked += 1,
            },
            StoryState::Icebox | StoryState::Rejected => report.capacity.inactive += 1,
            _ => {}
        }
    }

    let mut epics: Vec<_> = epic_map
        .into_values()
        .map(|mut report| {
            report.charge_state = classify_charge(report.capacity.ready, report.capacity.blocked);
            report
        })
        .collect();

    // Sort: blocked first, then highest charge, then alphabetical ID.
    epics.sort_by(|a, b| {
        b.charge_state
            .cmp(&a.charge_state)
            .then_with(|| a.id.cmp(&b.id))
    });

    SystemCapacity { epics }
}

fn classify_charge(ready: usize, blocked: usize) -> ChargeState {
    if blocked > 0 && ready == 0 {
        return ChargeState::Blocked;
    }

    match ready {
        0 => ChargeState::Discharged,
        1..=2 => ChargeState::Trickle,
        3..=5 => ChargeState::Charged,
        6..=10 => ChargeState::Supercharged,
        _ => ChargeState::Overloaded,
    }
}

#[cfg(test)]
mod tests {
    use super::project;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn blocked_stories_identified_from_unmet_deps() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .status(StoryState::InProgress)
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .status(StoryState::Backlog)
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let cap = project(&board);
        let keel_cap = cap.epics.iter().find(|e| e.id == "keel").unwrap();

        assert_eq!(keel_cap.capacity.blocked, 1);
        assert_eq!(keel_cap.capacity.ready, 0);
    }

    #[test]
    fn stories_with_all_done_deps_not_blocked() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\n| SRS-02 | req2 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(TestVoyage::new("01-test", "keel").srs_content(srs))
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .status(StoryState::Done)
                    .body("- [x] [SRS-01/AC-01] req1"),
            )
            .story(
                TestStory::new("S2")
                    .scope("keel/01-test")
                    .status(StoryState::Backlog)
                    .body("- [ ] [SRS-02/AC-01] req2"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let cap = project(&board);
        let keel_cap = cap.epics.iter().find(|e| e.id == "keel").unwrap();

        assert_eq!(keel_cap.capacity.blocked, 0);
        assert_eq!(keel_cap.capacity.ready, 1);
    }

    #[test]
    fn draft_voyage_backlog_story_is_blocked_capacity() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req1 | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("keel"))
            .voyage(
                TestVoyage::new("01-test", "keel")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("keel/01-test")
                    .status(StoryState::Backlog)
                    .body("- [ ] [SRS-01/AC-01] req1"),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let cap = project(&board);
        let keel_cap = cap.epics.iter().find(|e| e.id == "keel").unwrap();

        assert_eq!(keel_cap.capacity.blocked, 1);
        assert_eq!(keel_cap.capacity.ready, 0);
    }
}
