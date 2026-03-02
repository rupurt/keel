//! Bearing transition specifications.
//!
//! Extends the transition pattern to bearings, which have side effects
//! (file creation) as part of certain transitions.

use crate::domain::model::BearingStatus;

/// Side effect that occurs during a transition.
#[derive(Debug, Clone)]
pub enum TransitionSideEffect {
    /// No side effect, just status update
    None,
    /// Create a file from a template
    CreateFile {
        /// Template content to render
        template: &'static str,
        /// Filename to create (e.g., "SURVEY.md")
        filename: &'static str,
    },
}

/// Specification for a bearing state transition.
#[derive(Debug, Clone)]
pub struct BearingTransitionSpec {
    /// Human-readable name for error messages
    pub name: &'static str,
    /// Valid source statuses
    pub from: &'static [BearingStatus],
    /// Target status
    pub to: BearingStatus,
    /// Side effect to execute (file creation, etc.)
    pub side_effect: TransitionSideEffect,
}

impl BearingTransitionSpec {
    /// Check if a status is a valid source for this transition
    pub fn is_valid_source(&self, status: BearingStatus) -> bool {
        self.from.contains(&status)
    }
}

/// Pre-defined bearing transitions.
pub mod bearing_transitions {
    use super::*;
    use crate::infrastructure::templates;

    /// Survey: exploring → evaluating, creates SURVEY.md
    pub const SURVEY: BearingTransitionSpec = BearingTransitionSpec {
        name: "survey",
        from: &[BearingStatus::Exploring],
        to: BearingStatus::Evaluating,
        side_effect: TransitionSideEffect::CreateFile {
            template: templates::bearing::SURVEY,
            filename: "SURVEY.md",
        },
    };

    /// Assess: evaluating → ready, creates ASSESSMENT.md
    pub const ASSESS: BearingTransitionSpec = BearingTransitionSpec {
        name: "assess",
        from: &[BearingStatus::Evaluating],
        to: BearingStatus::Ready,
        side_effect: TransitionSideEffect::CreateFile {
            template: templates::bearing::ASSESSMENT,
            filename: "ASSESSMENT.md",
        },
    };

    /// Park: any active state → parked
    pub const PARK: BearingTransitionSpec = BearingTransitionSpec {
        name: "park",
        from: &[
            BearingStatus::Exploring,
            BearingStatus::Evaluating,
            BearingStatus::Ready,
        ],
        to: BearingStatus::Parked,
        side_effect: TransitionSideEffect::None,
    };

    /// Decline: ready → declined (requires reason, handled separately)
    #[allow(dead_code)] // For future use when decline adds reason via side effect
    pub const DECLINE: BearingTransitionSpec = BearingTransitionSpec {
        name: "decline",
        from: &[BearingStatus::Ready],
        to: BearingStatus::Declined,
        side_effect: TransitionSideEffect::None,
    };
}
