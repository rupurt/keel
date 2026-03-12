//! Process manager for cross-aggregate lifecycle coordination.

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

use crate::application::domain_events::DomainEvent;
use crate::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use crate::domain::model::{Board, StoryState, VoyageState};
use crate::infrastructure::loader::load_board;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProcessAction {
    StartVoyage { voyage_id: String },
    CompleteVoyage { voyage_id: String },
    CompleteEpic { epic_id: String },
}

pub trait ProcessActionExecutor: Send + Sync {
    fn start_voyage(&self, voyage_id: &str) -> Result<()>;
    fn complete_voyage(&self, voyage_id: &str) -> Result<()>;
    fn complete_epic(&self, epic_id: &str) -> Result<()>;
}

trait ProcessReactor: Send + Sync {
    fn name(&self) -> &'static str;

    fn priority(&self) -> u8 {
        100
    }

    fn plan(&self, board: &Board, event: &DomainEvent) -> Vec<ProcessAction>;
}

struct LifecyclePlanningReactor;

impl ProcessReactor for LifecyclePlanningReactor {
    fn name(&self) -> &'static str {
        "lifecycle-planner"
    }

    fn plan(&self, board: &Board, event: &DomainEvent) -> Vec<ProcessAction> {
        match event {
            DomainEvent::StoryStarted {
                scope: Some(scope), ..
            } => plan_story_started_actions(board, scope),
            DomainEvent::StoryAccepted {
                scope: Some(scope), ..
            } => plan_story_accepted_actions(board, scope),
            DomainEvent::VoyageCompleted { voyage_id, epic_id } => {
                plan_voyage_completed_actions(board, voyage_id, epic_id)
            }
            DomainEvent::StoryStarted { scope: None, .. }
            | DomainEvent::StoryAccepted { scope: None, .. } => Vec::new(),
        }
    }
}

fn default_reactors() -> Vec<Box<dyn ProcessReactor>> {
    vec![Box::new(LifecyclePlanningReactor)]
}

pub struct LiveProcessActionExecutor {
    service: Arc<VoyageEpicLifecycleService>,
}

impl LiveProcessActionExecutor {
    pub fn new(service: Arc<VoyageEpicLifecycleService>) -> Self {
        Self { service }
    }
}

impl ProcessActionExecutor for LiveProcessActionExecutor {
    fn start_voyage(&self, voyage_id: &str) -> Result<()> {
        self.service.start_voyage(voyage_id, false, None)
    }

    fn complete_voyage(&self, voyage_id: &str) -> Result<()> {
        self.service.complete_voyage(voyage_id, None, None, None)
    }

    fn complete_epic(&self, epic_id: &str) -> Result<()> {
        // Epics are currently completed via derived state, but we might want
        // a formal transition if we add more frontmatter logic.
        // For now, we'll just log it or perform any necessary side effects.
        println!("[process-manager] Finalizing epic {}", epic_id);
        Ok(())
    }
}

pub struct DomainProcessManager<E = LiveProcessActionExecutor> {
    executor: E,
    reactors: Vec<Box<dyn ProcessReactor>>,
}

impl<E: ProcessActionExecutor> DomainProcessManager<E> {
    pub fn new(executor: E) -> Self {
        Self {
            executor,
            reactors: default_reactors(),
        }
    }

    #[cfg(test)]
    fn with_reactors(executor: E, reactors: Vec<Box<dyn ProcessReactor>>) -> Self {
        Self { executor, reactors }
    }

    pub fn handle(&self, board_dir: &Path, event: DomainEvent) -> Result<()> {
        log_event(&event);

        let board = load_board(board_dir)?;
        let actions = self.plan_actions(&board, &event);
        for action in actions {
            self.execute_action(action)?;
        }

        Ok(())
    }

    fn execute_action(&self, action: ProcessAction) -> Result<()> {
        match action {
            ProcessAction::StartVoyage { voyage_id } => {
                println!(
                    "[process-manager] Auto-starting voyage {} after story activity.",
                    voyage_id
                );
                self.executor.start_voyage(&voyage_id)
            }
            ProcessAction::CompleteVoyage { voyage_id } => {
                println!(
                    "[process-manager] Auto-completing voyage {} because all stories are done.",
                    voyage_id
                );
                self.executor.complete_voyage(&voyage_id)
            }
            ProcessAction::CompleteEpic { epic_id } => {
                println!(
                    "[process-manager] Finalizing epic {} because all voyages are done.",
                    epic_id
                );
                self.executor.complete_epic(&epic_id)
            }
        }
    }

    fn plan_actions(&self, board: &Board, event: &DomainEvent) -> Vec<ProcessAction> {
        let mut reactors: Vec<_> = self
            .reactors
            .iter()
            .map(|reactor| reactor.as_ref())
            .collect();
        reactors.sort_by(|left, right| {
            left.priority()
                .cmp(&right.priority())
                .then_with(|| left.name().cmp(right.name()))
        });

        let mut actions = Vec::new();
        for reactor in reactors {
            let planned = reactor.plan(board, event);
            if !planned.is_empty() {
                println!(
                    "[process-manager] reactor={} planned_actions={}",
                    reactor.name(),
                    planned.len()
                );
            }
            actions.extend(planned);
        }

        actions
    }
}

fn log_event(event: &DomainEvent) {
    match event {
        DomainEvent::StoryStarted { story_id, scope } => println!(
            "[process-manager] event={} story={} scope={}",
            event.name(),
            story_id,
            scope.as_deref().unwrap_or("<none>")
        ),
        DomainEvent::StoryAccepted { story_id, scope } => println!(
            "[process-manager] event={} story={} scope={}",
            event.name(),
            story_id,
            scope.as_deref().unwrap_or("<none>")
        ),
        DomainEvent::VoyageCompleted { voyage_id, epic_id } => println!(
            "[process-manager] event={} voyage={} epic={}",
            event.name(),
            voyage_id,
            epic_id
        ),
    }
}

fn plan_story_started_actions(board: &Board, scope: &str) -> Vec<ProcessAction> {
    let Some(voyage) = board.voyages.values().find(|v| v.scope_path() == scope) else {
        return Vec::new();
    };

    if voyage.status() != VoyageState::Planned {
        return Vec::new();
    }

    vec![ProcessAction::StartVoyage {
        voyage_id: voyage.id().to_string(),
    }]
}

fn plan_story_accepted_actions(board: &Board, scope: &str) -> Vec<ProcessAction> {
    let Some(voyage) = board.voyages.values().find(|v| v.scope_path() == scope) else {
        return Vec::new();
    };

    if voyage.status() != VoyageState::InProgress {
        return Vec::new();
    }

    let stories = board.stories_for_voyage(voyage);
    if stories.is_empty() {
        return Vec::new();
    }

    let all_done = stories.iter().all(|story| story.status == StoryState::Done);
    if !all_done {
        return Vec::new();
    }

    vec![ProcessAction::CompleteVoyage {
        voyage_id: voyage.id().to_string(),
    }]
}

fn plan_voyage_completed_actions(
    board: &Board,
    _voyage_id: &str,
    epic_id: &str,
) -> Vec<ProcessAction> {
    let Some(epic) = board.epics.get(epic_id) else {
        return Vec::new();
    };

    let voyages = board.voyages_for_epic_id(epic.id());
    if voyages.is_empty() {
        return Vec::new();
    }

    let all_done = voyages.iter().all(|v| v.status() == VoyageState::Done);
    if !all_done {
        return Vec::new();
    }

    vec![ProcessAction::CompleteEpic {
        epic_id: epic.id().to_string(),
    }]
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[derive(Clone)]
    struct MockExecutor {
        calls: Arc<Mutex<Vec<String>>>,
    }

    struct TestReactor {
        name: &'static str,
        priority: u8,
        actions: Vec<ProcessAction>,
    }

    impl ProcessReactor for TestReactor {
        fn name(&self) -> &'static str {
            self.name
        }

        fn priority(&self) -> u8 {
            self.priority
        }

        fn plan(&self, _board: &Board, _event: &DomainEvent) -> Vec<ProcessAction> {
            self.actions.clone()
        }
    }

    impl ProcessActionExecutor for MockExecutor {
        fn start_voyage(&self, voyage_id: &str) -> Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("start:{voyage_id}"));
            Ok(())
        }

        fn complete_voyage(&self, voyage_id: &str) -> Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("complete:{voyage_id}"));
            Ok(())
        }

        fn complete_epic(&self, epic_id: &str) -> Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("complete_epic:{epic_id}"));
            Ok(())
        }
    }

    #[test]
    fn story_started_event_starts_planned_voyage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .status(StoryState::Backlog),
            )
            .build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::new(MockExecutor {
            calls: calls.clone(),
        });

        manager
            .handle(
                temp.path(),
                DomainEvent::StoryStarted {
                    story_id: "S1".to_string(),
                    scope: Some("e1/v1".to_string()),
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert_eq!(calls.as_slice(), ["start:v1"]);
    }

    #[test]
    fn story_accepted_event_completes_voyage_when_all_stories_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("in-progress"))
            .story(TestStory::new("S1").scope("e1/v1").status(StoryState::Done))
            .story(TestStory::new("S2").scope("e1/v1").status(StoryState::Done))
            .build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::new(MockExecutor {
            calls: calls.clone(),
        });

        manager
            .handle(
                temp.path(),
                DomainEvent::StoryAccepted {
                    story_id: "S2".to_string(),
                    scope: Some("e1/v1".to_string()),
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert_eq!(calls.as_slice(), ["complete:v1"]);
    }

    #[test]
    fn voyage_completed_event_completes_epic_when_all_voyages_done() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("done"))
            .voyage(TestVoyage::new("v2", "e1").status("done"))
            .build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::new(MockExecutor {
            calls: calls.clone(),
        });

        manager
            .handle(
                temp.path(),
                DomainEvent::VoyageCompleted {
                    voyage_id: "v2".to_string(),
                    epic_id: "e1".to_string(),
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert_eq!(calls.as_slice(), ["complete_epic:e1"]);
    }

    #[test]
    fn story_accepted_event_noops_when_voyage_not_ready() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("in-progress"))
            .story(TestStory::new("S1").scope("e1/v1").status(StoryState::Done))
            .story(
                TestStory::new("S2")
                    .scope("e1/v1")
                    .status(StoryState::InProgress),
            )
            .build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::new(MockExecutor {
            calls: calls.clone(),
        });

        manager
            .handle(
                temp.path(),
                DomainEvent::StoryAccepted {
                    story_id: "S1".to_string(),
                    scope: Some("e1/v1".to_string()),
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert!(calls.is_empty());
    }

    #[test]
    fn voyage_completed_event_noops() {
        let temp = TestBoardBuilder::new().epic(TestEpic::new("e1")).build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::new(MockExecutor {
            calls: calls.clone(),
        });

        manager
            .handle(
                temp.path(),
                DomainEvent::VoyageCompleted {
                    voyage_id: "v1".to_string(),
                    epic_id: "e1".to_string(),
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert!(calls.is_empty());
    }

    #[test]
    fn process_manager_reactors_use_named_planner_units() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1"))
            .voyage(TestVoyage::new("v1", "e1").status("planned"))
            .story(
                TestStory::new("S1")
                    .scope("e1/v1")
                    .status(StoryState::Backlog),
            )
            .build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::new(MockExecutor {
            calls: calls.clone(),
        });

        manager
            .handle(
                temp.path(),
                DomainEvent::StoryStarted {
                    story_id: "S1".to_string(),
                    scope: Some("e1/v1".to_string()),
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert_eq!(calls.as_slice(), ["start:v1"]);
    }

    #[test]
    fn process_manager_reactors_are_deterministic() {
        let temp = TestBoardBuilder::new().build();

        let calls = Arc::new(Mutex::new(Vec::new()));
        let manager = DomainProcessManager::with_reactors(
            MockExecutor {
                calls: calls.clone(),
            },
            vec![
                Box::new(TestReactor {
                    name: "zeta",
                    priority: 50,
                    actions: vec![ProcessAction::StartVoyage {
                        voyage_id: "zeta".to_string(),
                    }],
                }),
                Box::new(TestReactor {
                    name: "alpha",
                    priority: 10,
                    actions: vec![ProcessAction::StartVoyage {
                        voyage_id: "alpha".to_string(),
                    }],
                }),
                Box::new(TestReactor {
                    name: "beta",
                    priority: 10,
                    actions: vec![ProcessAction::StartVoyage {
                        voyage_id: "beta".to_string(),
                    }],
                }),
            ],
        );

        manager
            .handle(
                temp.path(),
                DomainEvent::StoryStarted {
                    story_id: "S1".to_string(),
                    scope: None,
                },
            )
            .unwrap();

        let calls = calls.lock().unwrap();
        assert_eq!(
            calls.as_slice(),
            ["start:alpha", "start:beta", "start:zeta"]
        );
    }
}
