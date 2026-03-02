//! Voyage state machine
#![allow(dead_code)] // Types defined for future integration (SRS-10/11)
//!
//! Voyage lifecycle states and valid transitions:
//!
//! ```text
//! ┌─────────┐    ┌─────────┐    ┌────────────┐    ┌──────┐
//! │  Draft  │───►│ Planned │───►│ InProgress │───►│ Done │
//! └─────────┘    └─────────┘    └────────────┘    └──────┘
//!
//! Draft: Voyage being designed, no stories required yet
//! Planned: Has stories, planning complete, ready to start
//! InProgress: Actively being worked (has in-progress stories)
//! Done: All stories completed
//! ```

use serde::{Deserialize, Deserializer, Serialize};

use super::{InvalidTransition, StateMachine};

/// Voyage lifecycle states
///
/// This is the canonical type for voyage/epic states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum VoyageState {
    /// Voyage being designed, stories not yet required
    #[default]
    Draft,
    /// Planning complete, has stories, ready to start
    Planned,
    /// Actively being worked on
    InProgress,
    /// All stories completed
    Done,
}

// Canonical deserializer with explicit guidance for legacy tokens
impl<'de> Deserialize<'de> for VoyageState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "draft" => Ok(VoyageState::Draft),
            "planned" => Ok(VoyageState::Planned),
            "in-progress" => Ok(VoyageState::InProgress),
            "active" => Err(serde::de::Error::custom(
                "legacy voyage status `active` is no longer supported; use `in-progress`",
            )),
            "done" => Ok(VoyageState::Done),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &["draft", "planned", "in-progress", "done"],
            )),
        }
    }
}

impl std::fmt::Display for VoyageState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Planned => write!(f, "planned"),
            Self::InProgress => write!(f, "in-progress"),
            Self::Done => write!(f, "done"),
        }
    }
}

impl VoyageState {
    /// Returns `true` if the voyage is ready for story work.
    ///
    /// A voyage is ready for work when it has been planned (has stories
    /// defined) and can accept active story work. This includes:
    /// - `Planned`: Ready to start, stories can be picked up
    /// - `InProgress`: Actively being worked on
    ///
    /// Draft voyages are not ready (still being designed) and Done
    /// voyages are complete (no more work needed).
    ///
    /// # Examples
    /// ```
    /// use keel::state_machine::voyage::VoyageState;
    ///
    /// assert!(VoyageState::Planned.is_ready_for_work());
    /// assert!(VoyageState::InProgress.is_ready_for_work());
    /// assert!(!VoyageState::Draft.is_ready_for_work());
    /// assert!(!VoyageState::Done.is_ready_for_work());
    /// ```
    pub fn is_ready_for_work(&self) -> bool {
        matches!(self, Self::Planned | Self::InProgress)
    }
}

/// Named transitions for voyages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoyageTransition {
    /// Promote draft to planned (has stories)
    Plan,
    /// Start working on a planned voyage
    Start,
    /// Complete the voyage (all stories done)
    Complete,
}

impl VoyageTransition {
    /// Get the target state for this transition
    pub fn target_state(&self) -> VoyageState {
        match self {
            VoyageTransition::Plan => VoyageState::Planned,
            VoyageTransition::Start => VoyageState::InProgress,
            VoyageTransition::Complete => VoyageState::Done,
        }
    }

    /// Get valid source states for this transition
    pub fn valid_from(&self) -> &'static [VoyageState] {
        match self {
            VoyageTransition::Plan => &[VoyageState::Draft],
            VoyageTransition::Start => &[VoyageState::Planned],
            VoyageTransition::Complete => &[VoyageState::InProgress],
        }
    }
}

/// Voyage state machine instance
#[derive(Debug, Clone)]
pub struct VoyageStateMachine {
    state: VoyageState,
}

impl VoyageStateMachine {
    /// Create a new voyage in Draft state
    pub fn new() -> Self {
        Self {
            state: VoyageState::Draft,
        }
    }

    /// Create a voyage machine from an existing state
    pub fn from_state(state: VoyageState) -> Self {
        Self { state }
    }

    /// Apply a named transition
    pub fn apply(
        &mut self,
        transition: VoyageTransition,
    ) -> Result<(), InvalidTransition<VoyageState>> {
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

impl Default for VoyageStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine for VoyageStateMachine {
    type State = VoyageState;
    type Transition = VoyageTransition;

    fn state(&self) -> VoyageState {
        self.state
    }

    fn valid_transitions(&self) -> Vec<VoyageTransition> {
        [
            VoyageTransition::Plan,
            VoyageTransition::Start,
            VoyageTransition::Complete,
        ]
        .into_iter()
        .filter(|t| t.valid_from().contains(&self.state))
        .collect()
    }

    fn can_transition(&self, to: VoyageState) -> bool {
        self.valid_transitions()
            .iter()
            .any(|t| t.target_state() == to)
    }

    fn transition(&mut self, to: VoyageState) -> Result<(), InvalidTransition<VoyageState>> {
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
    fn voyage_starts_in_draft() {
        let sm = VoyageStateMachine::new();
        assert_eq!(sm.state(), VoyageState::Draft);
    }

    #[test]
    fn voyage_full_lifecycle() {
        let mut sm = VoyageStateMachine::new();

        // Draft -> Planned -> InProgress -> Done
        sm.apply(VoyageTransition::Plan).unwrap();
        assert_eq!(sm.state(), VoyageState::Planned);

        sm.apply(VoyageTransition::Start).unwrap();
        assert_eq!(sm.state(), VoyageState::InProgress);

        sm.apply(VoyageTransition::Complete).unwrap();
        assert_eq!(sm.state(), VoyageState::Done);
    }

    #[test]
    fn voyage_cannot_skip_states() {
        let mut sm = VoyageStateMachine::new();

        // Cannot start from draft (must plan first)
        assert!(sm.apply(VoyageTransition::Start).is_err());

        // Cannot complete from draft
        assert!(sm.apply(VoyageTransition::Complete).is_err());
    }

    #[test]
    fn voyage_cannot_go_backwards() {
        let mut sm = VoyageStateMachine::from_state(VoyageState::InProgress);

        // Cannot plan from in-progress
        assert!(sm.apply(VoyageTransition::Plan).is_err());
    }

    #[test]
    fn valid_transitions_from_planned() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let valid = sm.valid_transitions();

        assert!(valid.contains(&VoyageTransition::Start));
        assert!(!valid.contains(&VoyageTransition::Plan));
        assert!(!valid.contains(&VoyageTransition::Complete));
    }

    // Query method tests (SRS-06)

    #[test]
    fn is_ready_for_work_returns_true_for_planned_and_in_progress() {
        assert!(VoyageState::Planned.is_ready_for_work());
        assert!(VoyageState::InProgress.is_ready_for_work());
    }

    #[test]
    fn is_ready_for_work_returns_false_for_draft_and_done() {
        assert!(!VoyageState::Draft.is_ready_for_work());
        assert!(!VoyageState::Done.is_ready_for_work());
    }

    #[test]
    fn state_machine_trait_impl() {
        let mut sm = VoyageStateMachine::new();
        assert!(sm.can_transition(VoyageState::Planned));
        assert!(!sm.can_transition(VoyageState::InProgress));

        sm.transition(VoyageState::Planned).unwrap();
        assert_eq!(sm.state(), VoyageState::Planned);
    }

    #[test]
    fn voyage_state_rejects_active_with_replacement() {
        let err = serde_yaml::from_str::<VoyageState>("active").unwrap_err();
        let message = err.to_string();

        assert!(message.contains("active"));
        assert!(message.contains("in-progress"));
    }
}
