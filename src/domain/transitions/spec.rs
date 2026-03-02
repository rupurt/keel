#![allow(dead_code)]
//! Transition specifications for the state machine.
//!
//! This module defines declarative specifications for state transitions,
//! aligned with the 2-queue pull system mental model:
//!
//! ```text
//! HUMAN QUEUE                         AGENT QUEUE
//! ┌─────────────────┐                 ┌─────────────────┐
//! │ • to accept     │   ←─ submit ──  │ • backlog       │
//! │ • to start      │   ── reject →   │ • in-progress   │
//! │ • research      │   ── start →    │                 │
//! └─────────────────┘                 └─────────────────┘
//!         │                                   ↑
//!         └── accept ──────────────→ Done ────┘
//!
//! PARKING (outside flow)
//! ┌─────────────────┐
//! │ • icebox        │  ←─ ice ── (any except done)
//! └─────────────────┘  ── thaw → backlog
//! ```

use crate::domain::model::StoryState;

/// Which timestamp fields to update during a transition.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct TimestampUpdates {
    /// Update the updated_at field to current datetime
    pub updated_at: bool,
    /// Update the submitted_at field to current datetime
    pub submitted_at: bool,
    /// Update the completed_at field to current datetime
    pub completed_at: bool,
    /// Update the started field to today (used by voyages)
    pub started: bool,
}

impl TimestampUpdates {
    /// Create a new TimestampUpdates with only updated_at set
    pub const fn updated_only() -> Self {
        Self {
            updated_at: true,
            submitted_at: false,
            completed_at: false,
            started: false,
        }
    }

    /// Create a new TimestampUpdates with updated_at and submitted_at set
    pub const fn with_submitted() -> Self {
        Self {
            updated_at: true,
            submitted_at: true,
            completed_at: false,
            started: false,
        }
    }

    /// Create a new TimestampUpdates with updated_at and completed_at set
    pub const fn with_completed() -> Self {
        Self {
            updated_at: true,
            submitted_at: false,
            completed_at: true,
            started: false,
        }
    }

    /// Create a new TimestampUpdates with started set (for voyage start)
    pub const fn with_started() -> Self {
        Self {
            updated_at: false,
            submitted_at: false,
            completed_at: false,
            started: true,
        }
    }

    /// Create a new TimestampUpdates with completed_at set (for voyage done)
    pub const fn voyage_completed() -> Self {
        Self {
            updated_at: false,
            submitted_at: false,
            completed_at: true,
            started: false,
        }
    }
}

/// Specification for a state transition.
///
/// Transitions are declarative: they specify the valid source stages,
/// target stage, and which timestamp fields to update. The transition
/// engine uses these specs to execute transitions uniformly.
#[derive(Debug, Clone)]
pub struct TransitionSpec {
    /// Human-readable name for error messages (e.g., "start", "submit")
    pub name: &'static str,
    /// Valid source stages (transition allowed from these)
    pub from: &'static [StoryState],
    /// Target stage
    pub to: StoryState,
    /// Which timestamp fields to update
    pub timestamps: TimestampUpdates,
}

impl TransitionSpec {
    /// Check if a stage is a valid source for this transition
    pub fn is_valid_source(&self, stage: StoryState) -> bool {
        self.from.contains(&stage)
    }
}

/// Pre-defined transitions aligned with the 2-queue pull system.
pub mod transitions {
    use super::*;

    /// Agent pulls story from backlog to work on.
    ///
    /// Valid from: backlog, rejected (restart), icebox
    /// Target: in-progress
    pub const START: TransitionSpec = TransitionSpec {
        name: "start",
        from: &[
            StoryState::Backlog,
            StoryState::Rejected,
            StoryState::Icebox,
        ],
        to: StoryState::InProgress,
        timestamps: TimestampUpdates::updated_only(),
    };

    /// Agent completes work, pushes to human queue for review.
    ///
    /// Valid from: in-progress
    /// Target: needs-human-verification
    pub const SUBMIT: TransitionSpec = TransitionSpec {
        name: "submit",
        from: &[StoryState::InProgress],
        to: StoryState::NeedsHumanVerification,
        timestamps: TimestampUpdates::with_submitted(),
    };

    /// Human accepts work from queue, story is done.
    ///
    /// Valid from: needs-human-verification
    /// Target: done
    pub const ACCEPT: TransitionSpec = TransitionSpec {
        name: "accept",
        from: &[StoryState::NeedsHumanVerification],
        to: StoryState::Done,
        timestamps: TimestampUpdates::with_completed(),
    };

    /// Human rejects work, returns to agent queue.
    ///
    /// Valid from: needs-human-verification
    /// Target: rejected
    pub const REJECT: TransitionSpec = TransitionSpec {
        name: "reject",
        from: &[StoryState::NeedsHumanVerification],
        to: StoryState::Rejected,
        timestamps: TimestampUpdates::updated_only(),
    };

    /// Remove story from flow to parking (icebox).
    ///
    /// Valid from: any active stage except done
    /// Target: icebox
    pub const ICE: TransitionSpec = TransitionSpec {
        name: "ice",
        from: &[
            StoryState::Backlog,
            StoryState::InProgress,
            StoryState::NeedsHumanVerification,
            StoryState::Rejected,
        ],
        to: StoryState::Icebox,
        timestamps: TimestampUpdates::updated_only(),
    };

    /// Return story from parking to agent queue.
    ///
    /// Valid from: icebox
    /// Target: backlog
    pub const THAW: TransitionSpec = TransitionSpec {
        name: "thaw",
        from: &[StoryState::Icebox],
        to: StoryState::Backlog,
        timestamps: TimestampUpdates::updated_only(),
    };

    /// Auto-complete story when all automated verification passes and no manual required.
    ///
    /// Valid from: in-progress
    /// Target: done
    pub const SUBMIT_DONE: TransitionSpec = TransitionSpec {
        name: "submit-done",
        from: &[StoryState::InProgress],
        to: StoryState::Done,
        timestamps: TimestampUpdates::with_completed(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_transition_spec_is_correct() {
        let spec = &transitions::START;
        assert_eq!(spec.name, "start");
        assert_eq!(spec.to, StoryState::InProgress);
        assert!(spec.is_valid_source(StoryState::Backlog));
        assert!(spec.is_valid_source(StoryState::Rejected));
        assert!(spec.is_valid_source(StoryState::Icebox));
        assert!(!spec.is_valid_source(StoryState::InProgress));
        assert!(!spec.is_valid_source(StoryState::Done));
        assert!(spec.timestamps.updated_at);
        assert!(!spec.timestamps.submitted_at);
        assert!(!spec.timestamps.completed_at);
    }

    #[test]
    fn submit_transition_spec_is_correct() {
        let spec = &transitions::SUBMIT;
        assert_eq!(spec.name, "submit");
        assert_eq!(spec.to, StoryState::NeedsHumanVerification);
        assert!(spec.is_valid_source(StoryState::InProgress));
        assert!(!spec.is_valid_source(StoryState::Backlog));
        assert!(spec.timestamps.updated_at);
        assert!(spec.timestamps.submitted_at);
        assert!(!spec.timestamps.completed_at);
    }

    #[test]
    fn accept_transition_spec_is_correct() {
        let spec = &transitions::ACCEPT;
        assert_eq!(spec.name, "accept");
        assert_eq!(spec.to, StoryState::Done);
        assert!(spec.is_valid_source(StoryState::NeedsHumanVerification));
        assert!(!spec.is_valid_source(StoryState::InProgress));
        assert!(spec.timestamps.updated_at);
        assert!(!spec.timestamps.submitted_at);
        assert!(spec.timestamps.completed_at);
    }

    #[test]
    fn reject_transition_spec_is_correct() {
        let spec = &transitions::REJECT;
        assert_eq!(spec.name, "reject");
        assert_eq!(spec.to, StoryState::Rejected);
        assert!(spec.is_valid_source(StoryState::NeedsHumanVerification));
        assert!(!spec.is_valid_source(StoryState::InProgress));
        assert!(spec.timestamps.updated_at);
        assert!(!spec.timestamps.submitted_at);
        assert!(!spec.timestamps.completed_at);
    }

    #[test]
    fn ice_transition_spec_is_correct() {
        let spec = &transitions::ICE;
        assert_eq!(spec.name, "ice");
        assert_eq!(spec.to, StoryState::Icebox);
        assert!(spec.is_valid_source(StoryState::Backlog));
        assert!(spec.is_valid_source(StoryState::InProgress));
        assert!(spec.is_valid_source(StoryState::NeedsHumanVerification));
        assert!(spec.is_valid_source(StoryState::Rejected));
        assert!(!spec.is_valid_source(StoryState::Done));
        assert!(!spec.is_valid_source(StoryState::Icebox));
    }

    #[test]
    fn thaw_transition_spec_is_correct() {
        let spec = &transitions::THAW;
        assert_eq!(spec.name, "thaw");
        assert_eq!(spec.to, StoryState::Backlog);
        assert!(spec.is_valid_source(StoryState::Icebox));
        assert!(!spec.is_valid_source(StoryState::Backlog));
        assert!(!spec.is_valid_source(StoryState::Done));
    }

    #[test]
    fn all_transitions_have_updated_at() {
        // Every transition should update the updated_at field
        let all_transitions = [
            &transitions::START,
            &transitions::SUBMIT,
            &transitions::ACCEPT,
            &transitions::REJECT,
            &transitions::ICE,
            &transitions::THAW,
            &transitions::SUBMIT_DONE,
        ];
        for spec in all_transitions {
            assert!(
                spec.timestamps.updated_at,
                "Transition '{}' should have updated_at=true",
                spec.name
            );
        }
    }

    // ============ Drift Detection ============
    //
    // These tests ensure TransitionSpecs stay in sync with the
    // canonical StoryTransition rules in state_machine/story.rs.
    // If a spec and the SM disagree on valid source stages, these fail.

    use crate::domain::state_machine::story::StoryTransition;
    use std::collections::HashSet;

    /// Assert that a TransitionSpec's `from` stages match the union of
    /// `valid_from()` for the given StoryTransitions.
    fn assert_spec_matches_sm(spec: &TransitionSpec, sm_transitions: &[StoryTransition]) {
        let spec_from: HashSet<StoryState> = spec.from.iter().copied().collect();
        let sm_from: HashSet<StoryState> = sm_transitions
            .iter()
            .flat_map(|t| t.valid_from().iter().copied())
            .collect();

        assert_eq!(
            spec_from, sm_from,
            "Drift detected for '{}' spec!\n  spec.from:  {:?}\n  SM union:   {:?}",
            spec.name, spec_from, sm_from
        );

        // Also verify target stage matches
        for t in sm_transitions {
            assert_eq!(
                spec.to,
                t.target_state(),
                "Target state mismatch for '{}': spec says {:?}, SM says {:?}",
                spec.name,
                spec.to,
                t.target_state()
            );
        }
    }

    #[test]
    fn spec_start_matches_state_machine() {
        assert_spec_matches_sm(
            &transitions::START,
            &[StoryTransition::Start, StoryTransition::Restart],
        );
    }

    #[test]
    fn spec_submit_matches_state_machine() {
        assert_spec_matches_sm(&transitions::SUBMIT, &[StoryTransition::Submit]);
    }

    #[test]
    fn spec_accept_matches_state_machine() {
        assert_spec_matches_sm(&transitions::ACCEPT, &[StoryTransition::Accept]);
    }

    #[test]
    fn spec_reject_matches_state_machine() {
        assert_spec_matches_sm(&transitions::REJECT, &[StoryTransition::Reject]);
    }

    #[test]
    fn spec_ice_matches_state_machine() {
        assert_spec_matches_sm(&transitions::ICE, &[StoryTransition::Ice]);
    }

    #[test]
    fn spec_thaw_matches_state_machine() {
        assert_spec_matches_sm(&transitions::THAW, &[StoryTransition::Thaw]);
    }

    #[test]
    fn spec_submit_done_matches_state_machine() {
        assert_spec_matches_sm(&transitions::SUBMIT_DONE, &[StoryTransition::SubmitDone]);
    }

    #[test]
    fn submit_done_transition_spec_is_correct() {
        let spec = &transitions::SUBMIT_DONE;
        assert_eq!(spec.name, "submit-done");
        assert_eq!(spec.to, StoryState::Done);
        assert!(spec.is_valid_source(StoryState::InProgress));
        assert!(!spec.is_valid_source(StoryState::Backlog));
        assert!(!spec.is_valid_source(StoryState::NeedsHumanVerification));
        assert!(spec.timestamps.updated_at);
        assert!(!spec.timestamps.submitted_at);
        assert!(spec.timestamps.completed_at);
    }
}
