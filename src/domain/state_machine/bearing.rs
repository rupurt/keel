//! Bearing state machine
#![allow(dead_code)] // Types defined for future integration (SRS-10/11)
//!
//! Bearing (research item) lifecycle states and valid transitions:
//!
//! ```text
//!                                          ┌──────────┐
//!                                     ┌───►│   Laid   │ (graduated to epic)
//!                                     │    └──────────┘
//! ┌───────────┐    ┌────────────┐    ┌┴────────┐
//! │ Exploring │───►│ Evaluating │───►│  Ready  │
//! └───────────┘    └────────────┘    └┬────────┘
//!      │                │             │    └───►┌──────────┐
//!      │                │             │         │ Declined │
//!      │                │             │         └──────────┘
//!      ▼                ▼             ▼
//! ┌──────────┐    ┌──────────┐  ┌──────────┐
//! │  Parked  │◄───│  Parked  │◄─│  Parked  │
//! └──────────┘    └──────────┘  └──────────┘
//!      │
//!      ▼
//! (can resume to Exploring)
//! ```

use super::{InvalidTransition, StateMachine};

/// Bearing lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BearingState {
    /// Initial research phase
    Exploring,
    /// Actively gathering data (has SURVEY.md)
    Evaluating,
    /// Ready for decision (has ASSESSMENT.md)
    Ready,
    /// Graduated to epic via `lay` command
    Laid,
    /// Shelved for later
    Parked,
    /// Rejected with reason
    Declined,
}

/// Named transitions for bearings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BearingTransition {
    /// Move from exploring to evaluating (add SURVEY.md)
    Survey,
    /// Move from evaluating to ready (add ASSESSMENT.md)
    Assess,
    /// Graduate to epic
    Lay,
    /// Reject the bearing
    Decline,
    /// Park for later consideration
    Park,
    /// Resume from parked state
    Resume,
}

impl BearingTransition {
    /// Get the target state for this transition
    pub fn target_state(&self) -> BearingState {
        match self {
            BearingTransition::Survey => BearingState::Evaluating,
            BearingTransition::Assess => BearingState::Ready,
            BearingTransition::Lay => BearingState::Laid,
            BearingTransition::Decline => BearingState::Declined,
            BearingTransition::Park => BearingState::Parked,
            BearingTransition::Resume => BearingState::Exploring,
        }
    }

    /// Get valid source states for this transition
    pub fn valid_from(&self) -> &'static [BearingState] {
        match self {
            BearingTransition::Survey => &[BearingState::Exploring],
            BearingTransition::Assess => &[BearingState::Evaluating],
            BearingTransition::Lay => &[BearingState::Ready],
            BearingTransition::Decline => &[BearingState::Ready],
            BearingTransition::Park => &[
                BearingState::Exploring,
                BearingState::Evaluating,
                BearingState::Ready,
            ],
            BearingTransition::Resume => &[BearingState::Parked],
        }
    }
}

/// Bearing state machine instance
#[derive(Debug, Clone)]
pub struct BearingStateMachine {
    state: BearingState,
}

impl BearingStateMachine {
    /// Create a new bearing in Exploring state
    pub fn new() -> Self {
        Self {
            state: BearingState::Exploring,
        }
    }

    /// Create a bearing machine from an existing state
    pub fn from_state(state: BearingState) -> Self {
        Self { state }
    }

    /// Apply a named transition
    pub fn apply(
        &mut self,
        transition: BearingTransition,
    ) -> Result<(), InvalidTransition<BearingState>> {
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

impl Default for BearingStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine for BearingStateMachine {
    type State = BearingState;
    type Transition = BearingTransition;

    fn state(&self) -> BearingState {
        self.state
    }

    fn valid_transitions(&self) -> Vec<BearingTransition> {
        [
            BearingTransition::Survey,
            BearingTransition::Assess,
            BearingTransition::Lay,
            BearingTransition::Decline,
            BearingTransition::Park,
            BearingTransition::Resume,
        ]
        .into_iter()
        .filter(|t| t.valid_from().contains(&self.state))
        .collect()
    }

    fn can_transition(&self, to: BearingState) -> bool {
        self.valid_transitions()
            .iter()
            .any(|t| t.target_state() == to)
    }

    fn transition(&mut self, to: BearingState) -> Result<(), InvalidTransition<BearingState>> {
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
    fn bearing_starts_in_exploring() {
        let sm = BearingStateMachine::new();
        assert_eq!(sm.state(), BearingState::Exploring);
    }

    #[test]
    fn bearing_full_lifecycle_to_laid() {
        let mut sm = BearingStateMachine::new();

        // Exploring -> Evaluating -> Ready -> Laid
        sm.apply(BearingTransition::Survey).unwrap();
        assert_eq!(sm.state(), BearingState::Evaluating);

        sm.apply(BearingTransition::Assess).unwrap();
        assert_eq!(sm.state(), BearingState::Ready);

        sm.apply(BearingTransition::Lay).unwrap();
        assert_eq!(sm.state(), BearingState::Laid);
    }

    #[test]
    fn bearing_can_be_declined_from_ready() {
        let mut sm = BearingStateMachine::from_state(BearingState::Ready);

        sm.apply(BearingTransition::Decline).unwrap();
        assert_eq!(sm.state(), BearingState::Declined);
    }

    #[test]
    fn bearing_can_park_from_multiple_states() {
        // From exploring
        let mut sm = BearingStateMachine::new();
        assert!(sm.apply(BearingTransition::Park).is_ok());

        // From evaluating
        let mut sm = BearingStateMachine::from_state(BearingState::Evaluating);
        assert!(sm.apply(BearingTransition::Park).is_ok());

        // From ready
        let mut sm = BearingStateMachine::from_state(BearingState::Ready);
        assert!(sm.apply(BearingTransition::Park).is_ok());
    }

    #[test]
    fn bearing_can_resume_from_parked() {
        let mut sm = BearingStateMachine::from_state(BearingState::Parked);

        sm.apply(BearingTransition::Resume).unwrap();
        assert_eq!(sm.state(), BearingState::Exploring);
    }

    #[test]
    fn bearing_cannot_lay_from_exploring() {
        let mut sm = BearingStateMachine::new();

        // Cannot lay directly from exploring
        assert!(sm.apply(BearingTransition::Lay).is_err());
    }

    #[test]
    fn valid_transitions_from_ready() {
        let sm = BearingStateMachine::from_state(BearingState::Ready);
        let valid = sm.valid_transitions();

        assert!(valid.contains(&BearingTransition::Lay));
        assert!(valid.contains(&BearingTransition::Decline));
        assert!(valid.contains(&BearingTransition::Park));
        assert!(!valid.contains(&BearingTransition::Survey));
    }
}
