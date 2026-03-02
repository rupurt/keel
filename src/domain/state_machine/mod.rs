//! Entity state machines with defined lifecycle transitions
//!
//! Provides formal state machine definitions for Story, Voyage, and Bearing
//! entities with validated state transitions.
//!
//! Note: Bearing transitions with side effects (file creation) are now handled
//! by the unified transition engine in `crate::domain::transitions::bearing`.

mod bearing;
mod enforcement;
mod flow;
mod formatting;
mod gating;
pub mod invariants;
pub mod preconditions;
pub mod story;
pub mod validation;
pub mod voyage;

// Re-exports for future integration (SRS-10, SRS-11)
#[allow(unused_imports)] // Used by commands in SRS-02, SRS-03
pub use crate::infrastructure::validation::{Problem, Severity};
#[allow(unused_imports)]
pub use bearing::{BearingState, BearingTransition};
#[allow(unused_imports)]
pub use enforcement::{
    BlockingMode, EnforcementPolicy, EnforcementResult, TransitionEntity, TransitionIntent,
    classify_findings, enforce_transition, format_enforcement_error,
};
#[allow(unused_imports)]
pub use flow::{FlowState, FlowStateMachine, QueueDepths};
#[allow(unused_imports)]
pub use formatting::format_transition_error;
#[allow(unused_imports)]
pub use gating::{
    VoyageCompletionPolicy, evaluate_epic_done, evaluate_story_transition,
    evaluate_voyage_completion, evaluate_voyage_transition, format_gate_error,
};
#[allow(unused_imports)]
pub use preconditions::{PreconditionError, Preconditions, TransitionContext};
#[allow(unused_imports)]
pub use story::{StoryState, StoryStateMachine, StoryTransition};
// Validation types are re-exported from this module's invariants facade.
// Import from `state_machine::invariants`, not directly from `validation`.
#[allow(unused_imports)]
pub use voyage::{VoyageState, VoyageTransition};

/// Common trait for state machines with defined transitions
#[allow(dead_code)] // Planned for integration in SRS-10/11
pub trait StateMachine: Sized {
    /// The state type for this machine
    type State: Copy + PartialEq;
    /// The transition type for this machine
    type Transition;

    /// Get the current state
    fn state(&self) -> Self::State;

    /// Get valid transitions from the current state
    fn valid_transitions(&self) -> Vec<Self::Transition>;

    /// Check if a transition is valid from the current state
    fn can_transition(&self, to: Self::State) -> bool;

    /// Attempt to transition to a new state, returning error if invalid
    fn transition(&mut self, to: Self::State) -> Result<(), InvalidTransition<Self::State>>;
}

/// Error returned when an invalid state transition is attempted
#[allow(dead_code)] // Used in state machine apply() methods
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidTransition<S> {
    /// Current state
    pub from: S,
    /// Attempted target state
    pub to: S,
    /// Human-readable message
    pub message: String,
}

impl<S: std::fmt::Debug> std::fmt::Display for InvalidTransition<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl<S: std::fmt::Debug> std::error::Error for InvalidTransition<S> {}
