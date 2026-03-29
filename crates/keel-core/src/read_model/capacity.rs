//! Canonical capacity projection service.
//!
//! This read model centralizes epic execution capacity calculations so flow
//! renderers and diagnostics consume one deterministic type and algorithm.

use std::collections::HashMap;

use crate::domain::model::{Board, Story, StoryState};
use crate::read_model::execution_queue::{BacklogQueueState, classify_backlog_story};
use crate::read_model::traceability::derive_implementation_dependencies;

/// Summary of execution capacity per epic.
#[derive(Debug, Clone)]
pub struct SystemCapacity {
    pub epics: Vec<EpicCapacityReport>,
    pub watches: Vec<WatchCapacityReport>,
}

#[derive(Debug, Clone, Default)]
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
    pub status: crate::domain::model::EpicState,
    pub index: Option<u32>,
    pub charge_state: ChargeState,
    pub capacity: EpicCapacity,
}

#[derive(Debug, Clone)]
pub struct WatchCapacityReport {
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
    let mut watch_map: HashMap<String, WatchCapacityReport> = HashMap::new();

    for epic in board.epics.values() {
        if board.is_epic_paused_by_mission(epic.id()) {
            continue;
        }

        epic_map.insert(
            epic.id().to_string(),
            EpicCapacityReport {
                id: epic.id().to_string(),
                title: epic.frontmatter.title.clone(),
                status: epic.status(),
                index: epic.frontmatter.index,
                charge_state: ChargeState::Discharged,
                capacity: EpicCapacity::default(),
            },
        );
    }

    for story in board.stories.values() {
        if board.is_story_paused_by_mission(story) {
            continue;
        }

        if let Some(epic_id) = story.epic() {
            let Some(report) = epic_map.get_mut(epic_id) else {
                continue;
            };

            apply_story_capacity(board, story, &deps, &mut report.capacity);
            continue;
        }

        let Some(watch_id) = materialized_watch_scope(board, story) else {
            continue;
        };
        let Some(watch) = board.find_watch(watch_id) else {
            continue;
        };

        let report =
            watch_map
                .entry(watch.id().to_string())
                .or_insert_with(|| WatchCapacityReport {
                    id: watch.id().to_string(),
                    title: watch.title().to_string(),
                    charge_state: ChargeState::Discharged,
                    capacity: EpicCapacity::default(),
                });

        apply_story_capacity(board, story, &deps, &mut report.capacity);
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

    let mut watches: Vec<_> = watch_map
        .into_values()
        .map(|mut report| {
            report.charge_state = classify_charge(report.capacity.ready, report.capacity.blocked);
            report
        })
        .collect();

    watches.sort_by(|a, b| {
        b.charge_state
            .cmp(&a.charge_state)
            .then_with(|| a.id.cmp(&b.id))
    });

    SystemCapacity { epics, watches }
}

fn apply_story_capacity(
    board: &Board,
    story: &Story,
    deps: &HashMap<String, Vec<String>>,
    capacity: &mut EpicCapacity,
) {
    match story.status {
        StoryState::Done => capacity.done += 1,
        StoryState::InProgress => capacity.in_flight += 1,
        StoryState::Backlog => match classify_backlog_story(board, story, deps) {
            BacklogQueueState::Ready => capacity.ready += 1,
            BacklogQueueState::Blocked => capacity.blocked += 1,
        },
        StoryState::Icebox | StoryState::Rejected => capacity.inactive += 1,
        _ => {}
    }
}

fn materialized_watch_scope<'a>(board: &'a Board, story: &'a Story) -> Option<&'a str> {
    let materialization_key = story.materialization_key.as_deref()?;
    let (routine_id, _) = materialization_key.split_once('@')?;
    let routine = board.find_routine(routine_id)?;
    let target_scope = routine.target_scope();
    board.find_watch(target_scope)?;
    Some(target_scope)
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
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestMission, TestStory, TestVoyage};
    use std::fs;

    fn write_watch(root: &std::path::Path, id: &str, title: &str) {
        let dir = root.join("watches").join(id);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("README.md"),
            format!("---\nid: {id}\ntitle: {title}\nlimit_hours: 12\n---\n\n# {title}\n"),
        )
        .unwrap();
    }

    fn write_routine(root: &std::path::Path, id: &str, title: &str, target_scope: &str) {
        let dir = root.join("routines").join(id);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("README.md"),
            format!(
                "---\nid: {id}\ntitle: {title}\ncadence:\n  cron: 0 9 * * 1\n  timezone: UTC\ntarget-scope: {target_scope}\n---\n\n# Blueprint\n\n- Review the watch backlog.\n"
            ),
        )
        .unwrap();
    }

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

    #[test]
    fn watch_scoped_materialized_story_surfaces_watch_capacity() {
        let temp = TestBoardBuilder::new().build();
        write_watch(temp.path(), "W1", "Standard Operations");
        write_routine(temp.path(), "routine-watch", "Watch Review", "W1");

        let story_dir = temp.path().join("stories").join("S1");
        fs::create_dir_all(story_dir.join("EVIDENCE")).unwrap();
        fs::write(
            story_dir.join("README.md"),
            concat!(
                "---\n",
                "id: S1\n",
                "title: Watch Review\n",
                "type: feat\n",
                "status: backlog\n",
                "index: 1\n",
                "---\n\n",
                "<!-- keel:pulse-materialization: routine-watch@2026-01-12T17:00:00Z -->\n\n",
                "# Watch Review\n\n",
                "## Acceptance Criteria\n\n",
                "- [ ] [SRS-ROUTINE/AC-01] Review the watch backlog.\n"
            ),
        )
        .unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let cap = project(&board);

        assert_eq!(cap.watches.len(), 1);
        assert_eq!(cap.watches[0].id, "W1");
        assert_eq!(cap.watches[0].title, "Standard Operations");
        assert_eq!(cap.watches[0].capacity.ready, 1);
        assert_eq!(cap.watches[0].capacity.blocked, 0);
    }

    #[test]
    fn project_skips_epics_and_stories_under_paused_missions() {
        let srs = "# SRS\n\n## Functional Requirements\nBEGIN FUNCTIONAL_REQUIREMENTS\n| SRS-01 | req | test |\nEND FUNCTIONAL_REQUIREMENTS";
        let temp = TestBoardBuilder::new()
            .mission(TestMission::new("M1").status("paused"))
            .mission(TestMission::new("M2").status("active"))
            .epic(TestEpic::new("E1").mission("M1"))
            .epic(TestEpic::new("E2").mission("M2"))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .status("planned")
                    .srs_content(srs),
            )
            .voyage(
                TestVoyage::new("V2", "E2")
                    .status("planned")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .scope("E1/V1")
                    .status(StoryState::Backlog),
            )
            .story(
                TestStory::new("S2")
                    .scope("E2/V2")
                    .status(StoryState::Backlog),
            )
            .build();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let cap = project(&board);

        assert_eq!(cap.epics.len(), 1);
        assert_eq!(cap.epics[0].id, "E2");
        assert_eq!(cap.epics[0].capacity.ready, 1);
    }
}
