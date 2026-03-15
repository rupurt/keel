//! Story state machine
#![allow(dead_code)] // Some methods defined for future use
//!
//! Story lifecycle states and valid transitions:
//!
//! ```text
//!                    ┌──────────────────────────────────────┐
//!                    │                                      │
//!                    ▼                                      │
//! ┌─────────┐    ┌────────────┐    ┌───────────────────┐    │
//! │ Backlog │───►│ InProgress │───►│ NeedsHumanVerification │────┤
//! └─────────┘    └────────────┘    └───────────────────┘    │
//!     │              │                     │                │
//!     │              │                     ▼                │
//!     │              │              ┌──────────┐            │
//!     │              │              │   Done   │            │
//!     │              │              └──────────┘            │
//!     │              │                                      │
//!     │              ▼                                      │
//!     │         ┌──────────┐                                │
//!     └────────►│  Icebox  │◄───────────────────────────────┘
//!               └──────────┘
//!                    │
//!                    ▼
//!               (can return to Backlog via thaw)
//! ```

use serde::{Deserialize, Deserializer, Serialize};

use super::{InvalidTransition, StateMachine};

/// Story lifecycle states
///
/// This is the canonical type for story states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum StoryState {
    /// Story in backlog, not yet started
    Backlog,
    /// Story actively being worked on
    InProgress,
    /// Story submitted, awaiting human verification
    NeedsHumanVerification,
    /// Story completed and accepted
    Done,
    /// Story rejected (returns to InProgress on restart)
    Rejected,
    /// Story shelved for later
    Icebox,
}

// Canonical deserializer with explicit guidance for legacy tokens
impl<'de> Deserialize<'de> for StoryState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "backlog" => Ok(StoryState::Backlog),
            "in-progress" => Ok(StoryState::InProgress),
            "needs-human-verification" => Ok(StoryState::NeedsHumanVerification),
            "ready-for-acceptance" => Err(serde::de::Error::custom(
                "legacy story status `ready-for-acceptance` is no longer supported; use `needs-human-verification`",
            )),
            "done" => Ok(StoryState::Done),
            "rejected" => Ok(StoryState::Rejected),
            "icebox" => Ok(StoryState::Icebox),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &[
                    "backlog",
                    "in-progress",
                    "needs-human-verification",
                    "done",
                    "rejected",
                    "icebox",
                ],
            )),
        }
    }
}

impl StoryState {
    /// Returns `true` if the story is actively being worked on.
    ///
    /// Active stories are those that require attention from either the
    /// implementer or verifier. This includes:
    /// - `InProgress`: Currently being implemented
    /// - `NeedsHumanVerification`: Awaiting human review
    /// - `Rejected`: Needs rework after failed verification
    ///
    /// # Examples
    /// ```
    /// use keel::domain::state_machine::story::StoryState;
    ///
    /// assert!(StoryState::InProgress.is_active());
    /// assert!(StoryState::NeedsHumanVerification.is_active());
    /// assert!(StoryState::Rejected.is_active());
    /// assert!(!StoryState::Backlog.is_active());
    /// ```
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            Self::InProgress | Self::NeedsHumanVerification | Self::Rejected
        )
    }

    /// Returns `true` if the story is ready to be started.
    ///
    /// Only stories in `Backlog` state can be picked up for work.
    /// Stories in `Icebox` must first be thawed back to `Backlog`.
    ///
    /// # Examples
    /// ```
    /// use keel::domain::state_machine::story::StoryState;
    ///
    /// assert!(StoryState::Backlog.is_workable());
    /// assert!(!StoryState::InProgress.is_workable());
    /// assert!(!StoryState::Icebox.is_workable());
    /// ```
    pub fn is_workable(&self) -> bool {
        matches!(self, Self::Backlog)
    }

    /// Returns `true` if the story has reached a terminal state.
    ///
    /// Terminal states indicate the story lifecycle is complete.
    /// Currently only `Done` is terminal - stories can be iced/thawed
    /// indefinitely, and rejected stories return to active work.
    ///
    /// # Examples
    /// ```
    /// use keel::domain::state_machine::story::StoryState;
    ///
    /// assert!(StoryState::Done.is_terminal());
    /// assert!(!StoryState::InProgress.is_terminal());
    /// assert!(!StoryState::Icebox.is_terminal());
    /// ```
    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Done)
    }

    /// Returns the directory name for this stage
    pub fn dir_name(&self) -> &'static str {
        match self {
            StoryState::Backlog => "backlog",
            StoryState::InProgress => "in-progress",
            StoryState::NeedsHumanVerification => "needs-human-verification",
            StoryState::Done => "done",
            StoryState::Rejected => "rejected",
            StoryState::Icebox => "icebox",
        }
    }

    /// Returns all possible directory names for this stage.
    #[allow(dead_code)] // Available for directory scanning
    pub fn dir_names(&self) -> &'static [&'static str] {
        match self {
            StoryState::NeedsHumanVerification => &["needs-human-verification"],
            StoryState::Backlog => &["backlog"],
            StoryState::InProgress => &["in-progress"],
            StoryState::Done => &["done"],
            StoryState::Rejected => &["rejected"],
            StoryState::Icebox => &["icebox"],
        }
    }

    /// Check if this stage represents a completed state
    #[deprecated(
        since = "0.1.0",
        note = "use `is_terminal()` instead for clearer semantics"
    )]
    pub fn is_done(&self) -> bool {
        *self == StoryState::Done
    }
}

impl std::fmt::Display for StoryState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoryState::Backlog => write!(f, "backlog"),
            StoryState::InProgress => write!(f, "in-progress"),
            StoryState::NeedsHumanVerification => write!(f, "needs-human-verification"),
            StoryState::Done => write!(f, "done"),
            StoryState::Rejected => write!(f, "rejected"),
            StoryState::Icebox => write!(f, "icebox"),
        }
    }
}

/// Named transitions for stories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoryTransition {
    /// Start working on a backlog story
    Start,
    /// Submit story for verification
    Submit,
    /// Accept a verified story
    Accept,
    /// Reject a story (needs more work)
    Reject,
    /// Restart a rejected story
    Restart,
    /// Move to icebox
    Ice,
    /// Restore from icebox to backlog
    Thaw,
    /// Auto-complete when all verification passes (no manual checks)
    SubmitDone,
}

impl StoryTransition {
    /// Get the target state for this transition
    pub fn target_state(&self) -> StoryState {
        match self {
            StoryTransition::Start | StoryTransition::Restart => StoryState::InProgress,
            StoryTransition::Submit => StoryState::NeedsHumanVerification,
            StoryTransition::Accept | StoryTransition::SubmitDone => StoryState::Done,
            StoryTransition::Reject => StoryState::Rejected,
            StoryTransition::Ice => StoryState::Icebox,
            StoryTransition::Thaw => StoryState::Backlog,
        }
    }

    /// Get valid source states for this transition
    pub fn valid_from(&self) -> &'static [StoryState] {
        match self {
            StoryTransition::Start => &[StoryState::Backlog, StoryState::Icebox],
            StoryTransition::Submit => &[StoryState::InProgress],
            StoryTransition::Accept => &[StoryState::NeedsHumanVerification],
            StoryTransition::Reject => &[StoryState::NeedsHumanVerification],
            StoryTransition::Restart => &[StoryState::Rejected],
            StoryTransition::SubmitDone => &[StoryState::InProgress],
            StoryTransition::Ice => &[
                StoryState::Backlog,
                StoryState::InProgress,
                StoryState::NeedsHumanVerification,
                StoryState::Rejected,
            ],
            StoryTransition::Thaw => &[StoryState::Icebox],
        }
    }
}

/// Story state machine instance
#[derive(Debug, Clone)]
pub struct StoryStateMachine {
    state: StoryState,
}

impl StoryStateMachine {
    /// Create a new story in Backlog state
    pub fn new() -> Self {
        Self {
            state: StoryState::Backlog,
        }
    }

    /// Create a story machine from an existing state
    pub fn from_state(state: StoryState) -> Self {
        Self { state }
    }

    /// Apply a named transition
    pub fn apply(
        &mut self,
        transition: StoryTransition,
    ) -> Result<(), InvalidTransition<StoryState>> {
        if transition.valid_from().contains(&self.state) {
            self.state = transition.target_state();
            Ok(())
        } else {
            Err(InvalidTransition {
                from: self.state,
                to: transition.target_state(),
                message: format!("Cannot {:?} from {:?} state", transition, self.state),
            })
        }
    }
}

impl Default for StoryStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine for StoryStateMachine {
    type State = StoryState;
    type Transition = StoryTransition;

    fn state(&self) -> StoryState {
        self.state
    }

    fn valid_transitions(&self) -> Vec<StoryTransition> {
        [
            StoryTransition::Start,
            StoryTransition::Submit,
            StoryTransition::Accept,
            StoryTransition::Reject,
            StoryTransition::Restart,
            StoryTransition::Ice,
            StoryTransition::Thaw,
            StoryTransition::SubmitDone,
        ]
        .into_iter()
        .filter(|t| t.valid_from().contains(&self.state))
        .collect()
    }

    fn can_transition(&self, to: StoryState) -> bool {
        self.valid_transitions()
            .iter()
            .any(|t| t.target_state() == to)
    }

    fn transition(&mut self, to: StoryState) -> Result<(), InvalidTransition<StoryState>> {
        // Find a transition that leads to the target state
        if let Some(transition) = self
            .valid_transitions()
            .into_iter()
            .find(|t| t.target_state() == to)
        {
            self.apply(transition)
        } else {
            Err(InvalidTransition {
                from: self.state,
                to,
                message: format!("No valid transition from {:?} to {:?}", self.state, to),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn story_starts_in_backlog() {
        let sm = StoryStateMachine::new();
        assert_eq!(sm.state(), StoryState::Backlog);
    }

    #[test]
    fn story_can_start_from_backlog() {
        let mut sm = StoryStateMachine::new();
        assert!(sm.apply(StoryTransition::Start).is_ok());
        assert_eq!(sm.state(), StoryState::InProgress);
    }

    #[test]
    fn story_cannot_submit_from_backlog() {
        let mut sm = StoryStateMachine::new();
        assert!(sm.apply(StoryTransition::Submit).is_err());
    }

    #[test]
    fn story_full_lifecycle() {
        let mut sm = StoryStateMachine::new();

        // Backlog -> InProgress -> NeedsHumanVerification -> Done
        sm.apply(StoryTransition::Start).unwrap();
        sm.apply(StoryTransition::Submit).unwrap();
        sm.apply(StoryTransition::Accept).unwrap();

        assert_eq!(sm.state(), StoryState::Done);
    }

    #[test]
    fn story_rejection_cycle() {
        let mut sm = StoryStateMachine::new();

        sm.apply(StoryTransition::Start).unwrap();
        sm.apply(StoryTransition::Submit).unwrap();
        sm.apply(StoryTransition::Reject).unwrap();

        assert_eq!(sm.state(), StoryState::Rejected);

        // Can restart from rejected
        sm.apply(StoryTransition::Restart).unwrap();
        assert_eq!(sm.state(), StoryState::InProgress);
    }

    #[test]
    fn story_icebox_from_multiple_states() {
        // From backlog
        let mut sm = StoryStateMachine::new();
        assert!(sm.apply(StoryTransition::Ice).is_ok());
        assert_eq!(sm.state(), StoryState::Icebox);

        // From in-progress
        let mut sm = StoryStateMachine::from_state(StoryState::InProgress);
        assert!(sm.apply(StoryTransition::Ice).is_ok());

        // From needs-verification
        let mut sm = StoryStateMachine::from_state(StoryState::NeedsHumanVerification);
        assert!(sm.apply(StoryTransition::Ice).is_ok());

        // From rejected
        let mut sm = StoryStateMachine::from_state(StoryState::Rejected);
        assert!(sm.apply(StoryTransition::Ice).is_ok());
    }

    #[test]
    fn story_start_from_icebox() {
        let mut sm = StoryStateMachine::from_state(StoryState::Icebox);
        assert!(sm.apply(StoryTransition::Start).is_ok());
        assert_eq!(sm.state(), StoryState::InProgress);
    }

    #[test]
    fn story_submit_done_lifecycle() {
        let mut sm = StoryStateMachine::new();
        sm.apply(StoryTransition::Start).unwrap();
        sm.apply(StoryTransition::SubmitDone).unwrap();
        assert_eq!(sm.state(), StoryState::Done);
    }

    #[test]
    fn story_thaw_returns_to_backlog() {
        let mut sm = StoryStateMachine::from_state(StoryState::Icebox);
        sm.apply(StoryTransition::Thaw).unwrap();
        assert_eq!(sm.state(), StoryState::Backlog);
    }

    #[test]
    fn valid_transitions_from_backlog() {
        let sm = StoryStateMachine::new();
        let valid = sm.valid_transitions();

        assert!(valid.contains(&StoryTransition::Start));
        assert!(valid.contains(&StoryTransition::Ice));
        assert!(!valid.contains(&StoryTransition::Submit));
    }

    // Query method tests (SRS-03, SRS-04, SRS-05)

    #[test]
    fn is_active_returns_true_for_active_states() {
        assert!(StoryState::InProgress.is_active());
        assert!(StoryState::NeedsHumanVerification.is_active());
        assert!(StoryState::Rejected.is_active());
    }

    #[test]
    fn is_active_returns_false_for_inactive_states() {
        assert!(!StoryState::Backlog.is_active());
        assert!(!StoryState::Done.is_active());
        assert!(!StoryState::Icebox.is_active());
    }

    #[test]
    fn is_workable_returns_true_only_for_backlog() {
        assert!(StoryState::Backlog.is_workable());

        assert!(!StoryState::InProgress.is_workable());
        assert!(!StoryState::NeedsHumanVerification.is_workable());
        assert!(!StoryState::Done.is_workable());
        assert!(!StoryState::Rejected.is_workable());
        assert!(!StoryState::Icebox.is_workable());
    }

    #[test]
    fn is_terminal_returns_true_only_for_done() {
        assert!(StoryState::Done.is_terminal());

        assert!(!StoryState::Backlog.is_terminal());
        assert!(!StoryState::InProgress.is_terminal());
        assert!(!StoryState::NeedsHumanVerification.is_terminal());
        assert!(!StoryState::Rejected.is_terminal());
        assert!(!StoryState::Icebox.is_terminal());
    }

    #[test]
    fn story_state_dir_name() {
        assert_eq!(StoryState::Backlog.dir_name(), "backlog");
        assert_eq!(StoryState::InProgress.dir_name(), "in-progress");
        assert_eq!(
            StoryState::NeedsHumanVerification.dir_name(),
            "needs-human-verification"
        );
        assert_eq!(StoryState::Done.dir_name(), "done");
        assert_eq!(StoryState::Rejected.dir_name(), "rejected");
        assert_eq!(StoryState::Icebox.dir_name(), "icebox");
    }

    #[test]
    fn story_state_display() {
        assert_eq!(StoryState::Backlog.to_string(), "backlog");
        assert_eq!(StoryState::InProgress.to_string(), "in-progress");
    }

    #[test]
    fn story_state_rejects_ready_for_acceptance_with_replacement() {
        let err = serde_yaml::from_str::<StoryState>("ready-for-acceptance").unwrap_err();
        let message = err.to_string();

        assert!(message.contains("ready-for-acceptance"));
        assert!(message.contains("needs-human-verification"));
    }
}
