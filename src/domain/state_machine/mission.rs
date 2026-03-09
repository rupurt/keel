//! Mission state machine
#![allow(dead_code)]
//!
//! Mission (autonomous long-running harness session) lifecycle states and valid transitions:
//!
//! ```text
//! ┌──────────┐  activate  ┌────────┐  achieve  ┌──────────┐  verify  ┌──────────┐
//! │ Defining │───────────►│ Active │──────────►│ Achieved │────────►│ Verified │
//! └──────────┘            └────┬───┘           └──────────┘         └──────────┘
//!                              │                                      (terminal)
//!                              ├──pause──►┌────────┐
//!                              │          │ Paused │──resume──►Active
//!                              │          └───┬────┘
//!                              │              │
//!                              └──abandon──►┌─▼────────┐
//!                                           │Abandoned │
//!                                           └──────────┘
//!                                             (terminal)
//! ```

use serde::{Deserialize, Deserializer, Serialize};

use super::{InvalidTransition, StateMachine};

/// Mission lifecycle states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum MissionStatus {
    /// Initial goal-setting phase
    #[default]
    Defining,
    /// Mission is actively being pursued
    Active,
    /// All board-verifiable goals met, awaiting human verification
    Achieved,
    /// Human has verified mission completion (terminal)
    Verified,
    /// Temporarily paused
    Paused,
    /// Mission abandoned (terminal)
    Abandoned,
}

impl<'de> Deserialize<'de> for MissionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "defining" => Ok(MissionStatus::Defining),
            "active" => Ok(MissionStatus::Active),
            "achieved" => Ok(MissionStatus::Achieved),
            "verified" => Ok(MissionStatus::Verified),
            "paused" => Ok(MissionStatus::Paused),
            "abandoned" => Ok(MissionStatus::Abandoned),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &[
                    "defining",
                    "active",
                    "achieved",
                    "verified",
                    "paused",
                    "abandoned",
                ],
            )),
        }
    }
}

impl std::fmt::Display for MissionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MissionStatus::Defining => write!(f, "defining"),
            MissionStatus::Active => write!(f, "active"),
            MissionStatus::Achieved => write!(f, "achieved"),
            MissionStatus::Verified => write!(f, "verified"),
            MissionStatus::Paused => write!(f, "paused"),
            MissionStatus::Abandoned => write!(f, "abandoned"),
        }
    }
}

impl std::str::FromStr for MissionStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "defining" => Ok(MissionStatus::Defining),
            "active" => Ok(MissionStatus::Active),
            "achieved" => Ok(MissionStatus::Achieved),
            "verified" => Ok(MissionStatus::Verified),
            "paused" => Ok(MissionStatus::Paused),
            "abandoned" => Ok(MissionStatus::Abandoned),
            _ => Err(format!("Invalid mission status: {}", s)),
        }
    }
}

impl MissionStatus {
    /// Whether this is a terminal state (no further transitions possible)
    pub fn is_terminal(&self) -> bool {
        matches!(self, MissionStatus::Verified | MissionStatus::Abandoned)
    }
}

/// Named transitions for missions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionTransition {
    /// Move from Defining to Active (CHARTER goals must exist)
    Activate,
    /// Move from Active to Achieved (board-verifiable goals must pass)
    Achieve,
    /// Move from Achieved to Verified (human confirmation)
    Verify,
    /// Move from Active to Paused
    Pause,
    /// Move from Paused back to Active
    Resume,
    /// Move from Active or Paused to Abandoned
    Abandon,
}

impl MissionTransition {
    /// Get the target state for this transition
    pub fn target_state(&self) -> MissionStatus {
        match self {
            MissionTransition::Activate => MissionStatus::Active,
            MissionTransition::Achieve => MissionStatus::Achieved,
            MissionTransition::Verify => MissionStatus::Verified,
            MissionTransition::Pause => MissionStatus::Paused,
            MissionTransition::Resume => MissionStatus::Active,
            MissionTransition::Abandon => MissionStatus::Abandoned,
        }
    }

    /// Get valid source states for this transition
    pub fn valid_from(&self) -> &'static [MissionStatus] {
        match self {
            MissionTransition::Activate => &[MissionStatus::Defining],
            MissionTransition::Achieve => &[MissionStatus::Active],
            MissionTransition::Verify => &[MissionStatus::Achieved],
            MissionTransition::Pause => &[MissionStatus::Active],
            MissionTransition::Resume => &[MissionStatus::Paused],
            MissionTransition::Abandon => &[MissionStatus::Active, MissionStatus::Paused],
        }
    }
}

/// Mission state machine instance
#[derive(Debug, Clone)]
pub struct MissionStateMachine {
    state: MissionStatus,
}

impl MissionStateMachine {
    /// Create a new mission in Defining state
    pub fn new() -> Self {
        Self {
            state: MissionStatus::Defining,
        }
    }

    /// Create a mission machine from an existing state
    pub fn from_state(state: MissionStatus) -> Self {
        Self { state }
    }

    /// Apply a named transition
    pub fn apply(
        &mut self,
        transition: MissionTransition,
    ) -> Result<(), InvalidTransition<MissionStatus>> {
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

impl Default for MissionStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl StateMachine for MissionStateMachine {
    type State = MissionStatus;
    type Transition = MissionTransition;

    fn state(&self) -> MissionStatus {
        self.state
    }

    fn valid_transitions(&self) -> Vec<MissionTransition> {
        [
            MissionTransition::Activate,
            MissionTransition::Achieve,
            MissionTransition::Verify,
            MissionTransition::Pause,
            MissionTransition::Resume,
            MissionTransition::Abandon,
        ]
        .into_iter()
        .filter(|t| t.valid_from().contains(&self.state))
        .collect()
    }

    fn can_transition(&self, to: MissionStatus) -> bool {
        self.valid_transitions()
            .iter()
            .any(|t| t.target_state() == to)
    }

    fn transition(&mut self, to: MissionStatus) -> Result<(), InvalidTransition<MissionStatus>> {
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

    // ============ MissionStatus Tests (SRS-02) ============

    #[test]
    fn mission_status_has_all_variants() {
        // SRS-02/AC-01: MissionStatus enum has all six variants
        let _defining = MissionStatus::Defining;
        let _active = MissionStatus::Active;
        let _achieved = MissionStatus::Achieved;
        let _verified = MissionStatus::Verified;
        let _paused = MissionStatus::Paused;
        let _abandoned = MissionStatus::Abandoned;
    }

    #[test]
    fn mission_status_default_is_defining() {
        assert_eq!(MissionStatus::default(), MissionStatus::Defining);
    }

    #[test]
    fn mission_status_serializes_kebab_case() {
        assert_eq!(
            serde_yaml::to_string(&MissionStatus::Defining)
                .unwrap()
                .trim(),
            "defining"
        );
        assert_eq!(
            serde_yaml::to_string(&MissionStatus::Active)
                .unwrap()
                .trim(),
            "active"
        );
        assert_eq!(
            serde_yaml::to_string(&MissionStatus::Achieved)
                .unwrap()
                .trim(),
            "achieved"
        );
        assert_eq!(
            serde_yaml::to_string(&MissionStatus::Verified)
                .unwrap()
                .trim(),
            "verified"
        );
        assert_eq!(
            serde_yaml::to_string(&MissionStatus::Paused)
                .unwrap()
                .trim(),
            "paused"
        );
        assert_eq!(
            serde_yaml::to_string(&MissionStatus::Abandoned)
                .unwrap()
                .trim(),
            "abandoned"
        );
    }

    #[test]
    fn mission_status_deserializes() {
        let status: MissionStatus = serde_yaml::from_str("defining").unwrap();
        assert_eq!(status, MissionStatus::Defining);

        let status: MissionStatus = serde_yaml::from_str("active").unwrap();
        assert_eq!(status, MissionStatus::Active);

        let status: MissionStatus = serde_yaml::from_str("achieved").unwrap();
        assert_eq!(status, MissionStatus::Achieved);

        let status: MissionStatus = serde_yaml::from_str("verified").unwrap();
        assert_eq!(status, MissionStatus::Verified);

        let status: MissionStatus = serde_yaml::from_str("paused").unwrap();
        assert_eq!(status, MissionStatus::Paused);

        let status: MissionStatus = serde_yaml::from_str("abandoned").unwrap();
        assert_eq!(status, MissionStatus::Abandoned);
    }

    #[test]
    fn mission_status_rejects_invalid() {
        let result: Result<MissionStatus, _> = serde_yaml::from_str("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn mission_status_display() {
        assert_eq!(MissionStatus::Defining.to_string(), "defining");
        assert_eq!(MissionStatus::Active.to_string(), "active");
        assert_eq!(MissionStatus::Achieved.to_string(), "achieved");
        assert_eq!(MissionStatus::Verified.to_string(), "verified");
        assert_eq!(MissionStatus::Paused.to_string(), "paused");
        assert_eq!(MissionStatus::Abandoned.to_string(), "abandoned");
    }

    #[test]
    fn mission_status_from_str() {
        assert_eq!(
            "defining".parse::<MissionStatus>().unwrap(),
            MissionStatus::Defining
        );
        assert_eq!(
            "active".parse::<MissionStatus>().unwrap(),
            MissionStatus::Active
        );
        assert_eq!(
            "achieved".parse::<MissionStatus>().unwrap(),
            MissionStatus::Achieved
        );
        assert_eq!(
            "verified".parse::<MissionStatus>().unwrap(),
            MissionStatus::Verified
        );
        assert_eq!(
            "paused".parse::<MissionStatus>().unwrap(),
            MissionStatus::Paused
        );
        assert_eq!(
            "abandoned".parse::<MissionStatus>().unwrap(),
            MissionStatus::Abandoned
        );
        // Case insensitive
        assert_eq!(
            "DEFINING".parse::<MissionStatus>().unwrap(),
            MissionStatus::Defining
        );
        // Invalid
        assert!("invalid".parse::<MissionStatus>().is_err());
    }

    #[test]
    fn mission_status_terminal_states() {
        assert!(!MissionStatus::Defining.is_terminal());
        assert!(!MissionStatus::Active.is_terminal());
        assert!(!MissionStatus::Achieved.is_terminal());
        assert!(MissionStatus::Verified.is_terminal());
        assert!(!MissionStatus::Paused.is_terminal());
        assert!(MissionStatus::Abandoned.is_terminal());
    }

    // ============ State Machine Transition Tests (SRS-03) ============

    #[test]
    fn mission_starts_in_defining() {
        let sm = MissionStateMachine::new();
        assert_eq!(sm.state(), MissionStatus::Defining);
    }

    #[test]
    fn mission_activate_defining_to_active() {
        // SRS-03/AC-01: activate: Defining→Active
        let mut sm = MissionStateMachine::new();
        sm.apply(MissionTransition::Activate).unwrap();
        assert_eq!(sm.state(), MissionStatus::Active);
    }

    #[test]
    fn mission_achieve_active_to_achieved() {
        // SRS-03/AC-02: achieve: Active→Achieved
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        sm.apply(MissionTransition::Achieve).unwrap();
        assert_eq!(sm.state(), MissionStatus::Achieved);
    }

    #[test]
    fn mission_verify_achieved_to_verified() {
        // SRS-03/AC-03: verify: Achieved→Verified
        let mut sm = MissionStateMachine::from_state(MissionStatus::Achieved);
        sm.apply(MissionTransition::Verify).unwrap();
        assert_eq!(sm.state(), MissionStatus::Verified);
    }

    #[test]
    fn mission_pause_active_to_paused() {
        // SRS-03/AC-04: pause: Active→Paused
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        sm.apply(MissionTransition::Pause).unwrap();
        assert_eq!(sm.state(), MissionStatus::Paused);
    }

    #[test]
    fn mission_resume_paused_to_active() {
        // SRS-03/AC-04: resume: Paused→Active
        let mut sm = MissionStateMachine::from_state(MissionStatus::Paused);
        sm.apply(MissionTransition::Resume).unwrap();
        assert_eq!(sm.state(), MissionStatus::Active);
    }

    #[test]
    fn mission_abandon_from_active() {
        // SRS-03/AC-05: abandon: Active→Abandoned
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        sm.apply(MissionTransition::Abandon).unwrap();
        assert_eq!(sm.state(), MissionStatus::Abandoned);
    }

    #[test]
    fn mission_abandon_from_paused() {
        // SRS-03/AC-05: abandon: Paused→Abandoned
        let mut sm = MissionStateMachine::from_state(MissionStatus::Paused);
        sm.apply(MissionTransition::Abandon).unwrap();
        assert_eq!(sm.state(), MissionStatus::Abandoned);
    }

    #[test]
    fn mission_full_lifecycle_to_verified() {
        let mut sm = MissionStateMachine::new();
        // Defining → Active → Achieved → Verified
        sm.apply(MissionTransition::Activate).unwrap();
        sm.apply(MissionTransition::Achieve).unwrap();
        sm.apply(MissionTransition::Verify).unwrap();
        assert_eq!(sm.state(), MissionStatus::Verified);
    }

    #[test]
    fn mission_pause_resume_cycle() {
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        sm.apply(MissionTransition::Pause).unwrap();
        assert_eq!(sm.state(), MissionStatus::Paused);
        sm.apply(MissionTransition::Resume).unwrap();
        assert_eq!(sm.state(), MissionStatus::Active);
    }

    // ============ Invalid Transition Tests (SRS-03/AC-06) ============

    #[test]
    fn mission_cannot_achieve_from_defining() {
        let mut sm = MissionStateMachine::new();
        let err = sm.apply(MissionTransition::Achieve).unwrap_err();
        assert_eq!(err.from, MissionStatus::Defining);
        assert_eq!(err.to, MissionStatus::Achieved);
    }

    #[test]
    fn mission_cannot_verify_from_active() {
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        assert!(sm.apply(MissionTransition::Verify).is_err());
    }

    #[test]
    fn mission_cannot_activate_from_active() {
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        assert!(sm.apply(MissionTransition::Activate).is_err());
    }

    #[test]
    fn mission_cannot_pause_from_defining() {
        let mut sm = MissionStateMachine::new();
        assert!(sm.apply(MissionTransition::Pause).is_err());
    }

    #[test]
    fn mission_cannot_resume_from_active() {
        let mut sm = MissionStateMachine::from_state(MissionStatus::Active);
        assert!(sm.apply(MissionTransition::Resume).is_err());
    }

    #[test]
    fn mission_cannot_abandon_from_defining() {
        let mut sm = MissionStateMachine::new();
        assert!(sm.apply(MissionTransition::Abandon).is_err());
    }

    #[test]
    fn mission_cannot_transition_from_verified() {
        // Verified is terminal
        let sm = MissionStateMachine::from_state(MissionStatus::Verified);
        assert!(sm.valid_transitions().is_empty());
    }

    #[test]
    fn mission_cannot_transition_from_abandoned() {
        // Abandoned is terminal
        let sm = MissionStateMachine::from_state(MissionStatus::Abandoned);
        assert!(sm.valid_transitions().is_empty());
    }

    #[test]
    fn mission_invalid_transition_has_descriptive_error() {
        let mut sm = MissionStateMachine::new();
        let err = sm.apply(MissionTransition::Achieve).unwrap_err();
        assert!(
            err.message.contains("Cannot"),
            "Error should describe the failure: {}",
            err.message
        );
    }

    // ============ StateMachine Trait Tests ============

    #[test]
    fn valid_transitions_from_defining() {
        let sm = MissionStateMachine::new();
        let valid = sm.valid_transitions();
        assert_eq!(valid.len(), 1);
        assert!(valid.contains(&MissionTransition::Activate));
    }

    #[test]
    fn valid_transitions_from_active() {
        let sm = MissionStateMachine::from_state(MissionStatus::Active);
        let valid = sm.valid_transitions();
        assert_eq!(valid.len(), 3);
        assert!(valid.contains(&MissionTransition::Achieve));
        assert!(valid.contains(&MissionTransition::Pause));
        assert!(valid.contains(&MissionTransition::Abandon));
    }

    #[test]
    fn valid_transitions_from_paused() {
        let sm = MissionStateMachine::from_state(MissionStatus::Paused);
        let valid = sm.valid_transitions();
        assert_eq!(valid.len(), 2);
        assert!(valid.contains(&MissionTransition::Resume));
        assert!(valid.contains(&MissionTransition::Abandon));
    }

    #[test]
    fn valid_transitions_from_achieved() {
        let sm = MissionStateMachine::from_state(MissionStatus::Achieved);
        let valid = sm.valid_transitions();
        assert_eq!(valid.len(), 1);
        assert!(valid.contains(&MissionTransition::Verify));
    }

    #[test]
    fn can_transition_checks_correctly() {
        let sm = MissionStateMachine::new();
        assert!(sm.can_transition(MissionStatus::Active));
        assert!(!sm.can_transition(MissionStatus::Achieved));
        assert!(!sm.can_transition(MissionStatus::Paused));
    }

    #[test]
    fn transition_by_target_state() {
        let mut sm = MissionStateMachine::new();
        sm.transition(MissionStatus::Active).unwrap();
        assert_eq!(sm.state(), MissionStatus::Active);
    }

    #[test]
    fn transition_by_target_state_rejects_invalid() {
        let mut sm = MissionStateMachine::new();
        let err = sm.transition(MissionStatus::Achieved).unwrap_err();
        assert!(err.message.contains("No valid transition"));
    }
}
