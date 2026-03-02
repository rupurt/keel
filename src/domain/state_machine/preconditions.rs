//! Precondition validation for state machine transitions
//!
//! This module provides the `Preconditions` trait that validates invariants
//! before state transitions execute. Commands use this to fail fast with
//! helpful error messages when preconditions aren't met.

use super::story::{StoryState, StoryStateMachine, StoryTransition};
use super::voyage::{VoyageState, VoyageStateMachine, VoyageTransition};

/// Context for evaluating transition preconditions
///
/// Contains references to related entities needed to validate
/// cross-entity invariants (e.g., voyage state when starting a story).
#[derive(Debug, Clone, Default)]
pub struct TransitionContext {
    /// The voyage this entity belongs to, if any
    #[allow(dead_code)]
    pub voyage_state: Option<VoyageState>,
    /// Story states in the voyage (for voyage transitions)
    #[allow(dead_code)]
    pub story_states: Vec<StoryState>,
}

impl TransitionContext {
    /// Create a context with just a voyage state (for story transitions)
    #[cfg(test)]
    pub fn with_voyage(voyage_state: VoyageState) -> Self {
        Self {
            voyage_state: Some(voyage_state),
            story_states: Vec::new(),
        }
    }

    /// Create a context with story states (for voyage transitions)
    #[allow(dead_code)] // Used in SRS-03 (story state on voyage start)
    pub fn with_stories(story_states: Vec<StoryState>) -> Self {
        Self {
            voyage_state: None,
            story_states,
        }
    }
}

/// Errors that can occur when checking preconditions
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PreconditionError {
    /// Voyage is not ready for work (story start requires Planned/InProgress)
    VoyageNotReady {
        voyage_state: VoyageState,
        required_states: &'static [VoyageState],
    },
    /// Stories exist in invalid states for voyage transition
    InvalidStoryStates {
        transition: &'static str,
        invalid_states: Vec<StoryState>,
        allowed_states: &'static [StoryState],
    },
    /// Generic precondition failure
    #[allow(dead_code)] // For future custom precondition errors
    Custom(String),
}

impl std::fmt::Display for PreconditionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::VoyageNotReady {
                voyage_state,
                required_states,
            } => {
                let required: Vec<_> = required_states.iter().map(|s| s.to_string()).collect();
                write!(
                    f,
                    "Voyage is '{}' but must be {} to start story work",
                    voyage_state,
                    required.join(" or ")
                )
            }
            Self::InvalidStoryStates {
                transition,
                invalid_states,
                allowed_states,
            } => {
                let invalid: Vec<_> = invalid_states.iter().map(|s| s.to_string()).collect();
                let allowed: Vec<_> = allowed_states.iter().map(|s| s.to_string()).collect();
                write!(
                    f,
                    "Cannot {} voyage: stories in {} state(s), must be {}",
                    transition,
                    invalid.join(", "),
                    allowed.join(" or ")
                )
            }
            Self::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PreconditionError {}

/// Trait for checking preconditions before state transitions
///
/// Implementors validate that invariants hold before a transition
/// is allowed to proceed. This enables fail-fast behavior with
/// helpful error messages.
#[allow(dead_code)]
pub trait Preconditions {
    /// The transition type being validated
    type Transition;

    /// Check if a transition is allowed given the current context
    ///
    /// Returns `Ok(())` if the transition can proceed, or an error
    /// describing why the precondition failed.
    fn check_preconditions(
        &self,
        transition: &Self::Transition,
        ctx: &TransitionContext,
    ) -> Result<(), PreconditionError>;
}

impl Preconditions for StoryStateMachine {
    type Transition = StoryTransition;

    fn check_preconditions(
        &self,
        transition: &Self::Transition,
        ctx: &TransitionContext,
    ) -> Result<(), PreconditionError> {
        match transition {
            StoryTransition::Start | StoryTransition::Thaw => {
                // Story start/thaw requires voyage to be ready for work
                if let Some(voyage_state) = ctx.voyage_state
                    && !voyage_state.is_ready_for_work()
                {
                    return Err(PreconditionError::VoyageNotReady {
                        voyage_state,
                        required_states: &[VoyageState::Planned, VoyageState::InProgress],
                    });
                }
                // Unscoped stories (no voyage) can always start/thaw
                Ok(())
            }
            // Other transitions have no cross-entity preconditions
            _ => Ok(()),
        }
    }
}

impl Preconditions for VoyageStateMachine {
    type Transition = VoyageTransition;

    fn check_preconditions(
        &self,
        transition: &Self::Transition,
        ctx: &TransitionContext,
    ) -> Result<(), PreconditionError> {
        match transition {
            VoyageTransition::Start => {
                // Voyage start requires all stories to be in Backlog or Icebox
                let allowed = &[StoryState::Backlog, StoryState::Icebox];
                let invalid: Vec<_> = ctx
                    .story_states
                    .iter()
                    .filter(|s| !allowed.contains(s))
                    .copied()
                    .collect();

                if !invalid.is_empty() {
                    return Err(PreconditionError::InvalidStoryStates {
                        transition: "start",
                        invalid_states: invalid,
                        allowed_states: allowed,
                    });
                }
                Ok(())
            }
            // Other transitions have no story-state preconditions
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ============ StoryStateMachine Preconditions ============

    #[test]
    fn story_start_succeeds_with_planned_voyage() {
        let sm = StoryStateMachine::new();
        let ctx = TransitionContext::with_voyage(VoyageState::Planned);

        let result = sm.check_preconditions(&StoryTransition::Start, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn story_start_succeeds_with_in_progress_voyage() {
        let sm = StoryStateMachine::new();
        let ctx = TransitionContext::with_voyage(VoyageState::InProgress);

        let result = sm.check_preconditions(&StoryTransition::Start, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn story_start_fails_with_draft_voyage() {
        let sm = StoryStateMachine::new();
        let ctx = TransitionContext::with_voyage(VoyageState::Draft);

        let result = sm.check_preconditions(&StoryTransition::Start, &ctx);
        assert!(matches!(
            result,
            Err(PreconditionError::VoyageNotReady { .. })
        ));
    }

    #[test]
    fn story_start_fails_with_done_voyage() {
        let sm = StoryStateMachine::new();
        let ctx = TransitionContext::with_voyage(VoyageState::Done);

        let result = sm.check_preconditions(&StoryTransition::Start, &ctx);
        assert!(matches!(
            result,
            Err(PreconditionError::VoyageNotReady { .. })
        ));
    }

    #[test]
    fn story_start_succeeds_without_voyage() {
        let sm = StoryStateMachine::new();
        let ctx = TransitionContext::default();

        let result = sm.check_preconditions(&StoryTransition::Start, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn story_submit_has_no_preconditions() {
        let sm = StoryStateMachine::from_state(StoryState::InProgress);
        let ctx = TransitionContext::default();

        let result = sm.check_preconditions(&StoryTransition::Submit, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn story_thaw_fails_with_draft_voyage() {
        let sm = StoryStateMachine::from_state(StoryState::Icebox);
        let ctx = TransitionContext::with_voyage(VoyageState::Draft);

        let result = sm.check_preconditions(&StoryTransition::Thaw, &ctx);
        assert!(matches!(
            result,
            Err(PreconditionError::VoyageNotReady { .. })
        ));
    }

    // ============ VoyageStateMachine Preconditions ============

    #[test]
    fn voyage_start_succeeds_with_backlog_stories() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let ctx = TransitionContext::with_stories(vec![StoryState::Backlog, StoryState::Backlog]);

        let result = sm.check_preconditions(&VoyageTransition::Start, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn voyage_start_succeeds_with_icebox_stories() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let ctx = TransitionContext::with_stories(vec![StoryState::Icebox, StoryState::Backlog]);

        let result = sm.check_preconditions(&VoyageTransition::Start, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn voyage_start_fails_with_in_progress_stories() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let ctx =
            TransitionContext::with_stories(vec![StoryState::Backlog, StoryState::InProgress]);

        let result = sm.check_preconditions(&VoyageTransition::Start, &ctx);
        assert!(matches!(
            result,
            Err(PreconditionError::InvalidStoryStates { .. })
        ));
    }

    #[test]
    fn voyage_start_fails_with_done_stories() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let ctx = TransitionContext::with_stories(vec![StoryState::Backlog, StoryState::Done]);

        let result = sm.check_preconditions(&VoyageTransition::Start, &ctx);
        assert!(matches!(
            result,
            Err(PreconditionError::InvalidStoryStates { .. })
        ));
    }

    #[test]
    fn voyage_start_fails_with_needs_verification_stories() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let ctx = TransitionContext::with_stories(vec![StoryState::NeedsHumanVerification]);

        let result = sm.check_preconditions(&VoyageTransition::Start, &ctx);
        assert!(matches!(
            result,
            Err(PreconditionError::InvalidStoryStates { .. })
        ));
    }

    #[test]
    fn voyage_start_succeeds_with_no_stories() {
        let sm = VoyageStateMachine::from_state(VoyageState::Planned);
        let ctx = TransitionContext::with_stories(vec![]);

        let result = sm.check_preconditions(&VoyageTransition::Start, &ctx);
        assert!(result.is_ok());
    }

    #[test]
    fn voyage_complete_has_no_story_preconditions() {
        let sm = VoyageStateMachine::from_state(VoyageState::InProgress);
        let ctx = TransitionContext::with_stories(vec![StoryState::InProgress]);

        let result = sm.check_preconditions(&VoyageTransition::Complete, &ctx);
        assert!(result.is_ok());
    }

    // ============ Error Message Tests ============

    #[test]
    fn voyage_not_ready_error_message_is_helpful() {
        let error = PreconditionError::VoyageNotReady {
            voyage_state: VoyageState::Draft,
            required_states: &[VoyageState::Planned, VoyageState::InProgress],
        };

        let msg = error.to_string();
        assert!(msg.contains("draft"));
        assert!(msg.contains("planned"));
        assert!(msg.contains("in-progress"));
    }

    #[test]
    fn invalid_story_states_error_message_is_helpful() {
        let error = PreconditionError::InvalidStoryStates {
            transition: "start",
            invalid_states: vec![StoryState::InProgress, StoryState::Done],
            allowed_states: &[StoryState::Backlog, StoryState::Icebox],
        };

        let msg = error.to_string();
        assert!(msg.contains("start"));
        assert!(msg.contains("in-progress"));
        assert!(msg.contains("done"));
        assert!(msg.contains("backlog"));
    }
}
