//! Process manager for cross-aggregate lifecycle coordination.

use std::path::Path;

use anyhow::Result;

use crate::application::domain_events::DomainEvent;
use crate::application::voyage_epic_lifecycle::VoyageEpicLifecycleService;
use crate::domain::model::{Board, StoryState, VoyageState};
use crate::infrastructure::loader::load_board;

#[derive(Debug, Clone, PartialEq, Eq)]
enum ProcessAction {
    StartVoyage { voyage_id: String },
    CompleteVoyage { voyage_id: String },
}

pub trait ProcessActionExecutor {
    fn start_voyage(&self, board_dir: &Path, voyage_id: &str) -> Result<()>;
    fn complete_voyage(&self, board_dir: &Path, voyage_id: &str) -> Result<()>;
}

pub struct LiveProcessActionExecutor;

impl ProcessActionExecutor for LiveProcessActionExecutor {
    fn start_voyage(&self, board_dir: &Path, voyage_id: &str) -> Result<()> {
        VoyageEpicLifecycleService::start_voyage(board_dir, voyage_id, false, None)
    }

    fn complete_voyage(&self, board_dir: &Path, voyage_id: &str) -> Result<()> {
        VoyageEpicLifecycleService::complete_voyage(board_dir, voyage_id, None, None, None)
    }
}

pub struct DomainProcessManager<E = LiveProcessActionExecutor> {
    executor: E,
}

impl Default for DomainProcessManager<LiveProcessActionExecutor> {
    fn default() -> Self {
        Self::new(LiveProcessActionExecutor)
    }
}

impl<E: ProcessActionExecutor> DomainProcessManager<E> {
    pub fn new(executor: E) -> Self {
        Self { executor }
    }

    pub fn handle(&self, board_dir: &Path, event: DomainEvent) -> Result<()> {
        log_event(&event);

        let board = load_board(board_dir)?;
        let actions = Self::plan_actions(&board, &event);
        for action in actions {
            self.execute_action(board_dir, action)?;
        }

        Ok(())
    }

    fn execute_action(&self, board_dir: &Path, action: ProcessAction) -> Result<()> {
        match action {
            ProcessAction::StartVoyage { voyage_id } => {
                println!(
                    "[process-manager] Auto-starting voyage {} after story activity.",
                    voyage_id
                );
                self.executor.start_voyage(board_dir, &voyage_id)
            }
            ProcessAction::CompleteVoyage { voyage_id } => {
                println!(
                    "[process-manager] Auto-completing voyage {} because all stories are done.",
                    voyage_id
                );
                self.executor.complete_voyage(board_dir, &voyage_id)
            }
        }
    }

    fn plan_actions(board: &Board, event: &DomainEvent) -> Vec<ProcessAction> {
        match event {
            DomainEvent::StoryStarted {
                scope: Some(scope), ..
            } => plan_story_started_actions(board, scope),
            DomainEvent::StoryAccepted {
                scope: Some(scope), ..
            } => plan_story_accepted_actions(board, scope),
            DomainEvent::VoyageCompleted { .. } => Vec::new(),
            DomainEvent::StoryStarted { scope: None, .. }
            | DomainEvent::StoryAccepted { scope: None, .. } => Vec::new(),
        }
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

    impl ProcessActionExecutor for MockExecutor {
        fn start_voyage(&self, _board_dir: &Path, voyage_id: &str) -> Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("start:{voyage_id}"));
            Ok(())
        }

        fn complete_voyage(&self, _board_dir: &Path, voyage_id: &str) -> Result<()> {
            self.calls
                .lock()
                .unwrap()
                .push(format!("complete:{voyage_id}"));
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
}
