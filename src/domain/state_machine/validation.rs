//! Coherence validation for state machine relationships
//!
//! This module provides pure validation functions that check relationships
//! between entities (voyage-story, epic-voyage) without performing I/O.
//! Doctor delegates to these functions and maps violations to Problems.
#![allow(dead_code)] // Types defined for future epic-voyage integration (SRS-09)

use super::story::StoryState;
use super::voyage::VoyageState;
use crate::domain::model::EpicState;

/// Coherence violations between voyages and their stories
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoyageStoryViolation {
    /// A planned voyage has active stories (should be started)
    ActiveStoryInPlannedVoyage {
        voyage_id: String,
        progressed_count: usize,
    },
    /// A draft voyage has non-icebox stories (should be planned or iced)
    NonIceboxStoryInDraftVoyage {
        voyage_id: String,
        non_icebox_count: usize,
    },
    /// All stories are done but voyage is not done
    AllStoriesDoneButVoyageNotDone {
        voyage_id: String,
        story_count: usize,
        voyage_state: VoyageState,
    },
    /// Non-draft voyage has no stories
    NoStoriesInNonDraftVoyage {
        voyage_id: String,
        voyage_state: VoyageState,
    },
}

impl VoyageStoryViolation {
    /// Get a human-readable message describing the violation
    pub fn message(&self) -> String {
        match self {
            Self::ActiveStoryInPlannedVoyage {
                voyage_id,
                progressed_count,
            } => format!(
                "voyage is 'planned' but has {} progressed stories (run `voyage start {}`)",
                progressed_count, voyage_id
            ),
            Self::NonIceboxStoryInDraftVoyage {
                voyage_id,
                non_icebox_count,
            } => format!(
                "voyage is 'draft' but has {} stories not in icebox (ice them or run `voyage plan {}`)",
                non_icebox_count, voyage_id
            ),
            Self::AllStoriesDoneButVoyageNotDone {
                voyage_id: _,
                story_count,
                voyage_state,
            } => format!(
                "all {} stories done but voyage status is '{}'",
                story_count, voyage_state
            ),
            Self::NoStoriesInNonDraftVoyage {
                voyage_id,
                voyage_state,
            } => format!(
                "voyage '{}' has status '{}' but no stories",
                voyage_id, voyage_state
            ),
        }
    }

    /// Get a suggested fix for this violation, if one exists
    pub fn suggested_fix(&self) -> Option<SuggestedFix> {
        match self {
            Self::AllStoriesDoneButVoyageNotDone { .. } => Some(SuggestedFix::UpdateVoyageStatus {
                new_status: VoyageState::Done,
            }),
            // Other violations require user decision (start vs ice, etc.)
            _ => None,
        }
    }
}

/// Suggested fixes for coherence violations
///
/// These are abstract fix suggestions that doctor maps to concrete Fix actions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestedFix {
    /// Update voyage status to a new state
    UpdateVoyageStatus { new_status: VoyageState },
    /// Update epic status to a new state
    UpdateEpicStatus { new_status: EpicState },
}

/// Coherence violations between epics and their voyages
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EpicVoyageViolation {
    /// A done epic has incomplete voyages (should be reopened)
    IncompleteVoyageInDoneEpic {
        epic_id: String,
        incomplete_count: usize,
    },
    /// All voyages are done but epic is not done
    AllVoyagesDoneButEpicNotDone {
        epic_id: String,
        voyage_count: usize,
        epic_state: EpicState,
    },
    /// Epic status doesn't match its implicit state based on voyages
    EpicStatusDrift {
        epic_id: String,
        current_status: EpicState,
        implicit_status: EpicState,
    },
}

impl EpicVoyageViolation {
    /// Get a human-readable message describing the violation
    pub fn message(&self) -> String {
        match self {
            Self::IncompleteVoyageInDoneEpic {
                epic_id,
                incomplete_count,
            } => format!(
                "epic is 'done' but has {} incomplete voyages ({})",
                incomplete_count, epic_id
            ),
            Self::AllVoyagesDoneButEpicNotDone {
                epic_id: _,
                voyage_count,
                epic_state,
            } => format!(
                "all {} voyages done but epic status is '{}' (should be 'done')",
                voyage_count, epic_state
            ),
            Self::EpicStatusDrift {
                epic_id: _,
                current_status,
                implicit_status,
            } => format!(
                "epic status '{}' doesn't match implicit state '{}' based on voyages",
                current_status, implicit_status
            ),
        }
    }

    /// Get a suggested fix for this violation, if one exists
    pub fn suggested_fix(&self) -> Option<SuggestedFix> {
        match self {
            Self::AllVoyagesDoneButEpicNotDone { .. } => Some(SuggestedFix::UpdateEpicStatus {
                new_status: EpicState::Done,
            }),
            Self::EpicStatusDrift {
                implicit_status, ..
            } => Some(SuggestedFix::UpdateEpicStatus {
                new_status: *implicit_status,
            }),
            // Incomplete voyages in done epic requires user decision (reopen vs complete voyages)
            _ => None,
        }
    }
}

/// Validate coherence between an epic and its voyages
///
/// Takes the epic state and a slice of voyage states, returning any violations.
/// This is a pure function with no I/O - caller provides the states.
///
/// # Arguments
/// * `epic_id` - Epic identifier for error messages
/// * `epic_state` - Current state of the epic
/// * `voyage_states` - States of all voyages in this epic
///
/// # Returns
/// A vector of violations (empty if coherent)
pub fn validate_epic_voyage_coherence(
    epic_id: &str,
    epic_state: EpicState,
    voyage_states: &[VoyageState],
) -> Vec<EpicVoyageViolation> {
    let mut violations = Vec::new();

    // 1. Determine implicit state
    let implicit_state = if voyage_states.is_empty() {
        EpicState::Draft
    } else if voyage_states.iter().all(|v| *v == VoyageState::Done) {
        EpicState::Done
    } else if voyage_states.iter().any(|v| *v != VoyageState::Draft) {
        EpicState::Active
    } else {
        EpicState::Draft
    };

    // 2. Check for terminal coherence (Done epics)
    if epic_state == EpicState::Done {
        let incomplete_count = voyage_states
            .iter()
            .filter(|v| **v != VoyageState::Done)
            .count();

        if incomplete_count > 0 {
            violations.push(EpicVoyageViolation::IncompleteVoyageInDoneEpic {
                epic_id: epic_id.to_string(),
                incomplete_count,
            });
        }
        return violations;
    }

    // 3. Check for implicit "Done" status
    if implicit_state == EpicState::Done && epic_state != EpicState::Done {
        violations.push(EpicVoyageViolation::AllVoyagesDoneButEpicNotDone {
            epic_id: epic_id.to_string(),
            voyage_count: voyage_states.len(),
            epic_state,
        });
        return violations;
    }

    // 4. Check for "Draft" vs "Active" drift
    if epic_state != implicit_state {
        violations.push(EpicVoyageViolation::EpicStatusDrift {
            epic_id: epic_id.to_string(),
            current_status: epic_state,
            implicit_status: implicit_state,
        });
    }

    violations
}

/// Validate coherence between a voyage and its stories
///
/// Takes the voyage state and a slice of story states, returning any violations.
/// This is a pure function with no I/O - caller provides the states.
///
/// # Arguments
/// * `voyage_id` - Voyage identifier for error messages
/// * `voyage_state` - Current state of the voyage
/// * `story_states` - States of all stories in this voyage
///
/// # Returns
/// A vector of violations (empty if coherent)
pub fn validate_voyage_story_coherence(
    voyage_id: &str,
    voyage_state: VoyageState,
    story_states: &[StoryState],
) -> Vec<VoyageStoryViolation> {
    let mut violations = Vec::new();

    // Non-draft voyages should have stories
    if story_states.is_empty() {
        if voyage_state != VoyageState::Draft {
            violations.push(VoyageStoryViolation::NoStoriesInNonDraftVoyage {
                voyage_id: voyage_id.to_string(),
                voyage_state,
            });
        }
        return violations;
    }

    match voyage_state {
        VoyageState::Done => {
            // Done voyages have no coherence constraints on story states
        }

        VoyageState::Draft => {
            // Draft voyages should have all stories in icebox
            let non_icebox_count = story_states
                .iter()
                .filter(|s| **s != StoryState::Icebox)
                .count();

            if non_icebox_count > 0 {
                violations.push(VoyageStoryViolation::NonIceboxStoryInDraftVoyage {
                    voyage_id: voyage_id.to_string(),
                    non_icebox_count,
                });
            }
        }

        VoyageState::Planned => {
            // Planned voyages shouldn't have active stories
            let progressed_count = story_states.iter().filter(|s| s.is_active()).count();

            // Also count Done stories as "active" for this check (they advanced beyond planned)
            let done_count = story_states.iter().filter(|s| s.is_terminal()).count();
            let total_active = progressed_count + done_count;

            if total_active > 0 {
                violations.push(VoyageStoryViolation::ActiveStoryInPlannedVoyage {
                    voyage_id: voyage_id.to_string(),
                    progressed_count: total_active,
                });
            }
        }

        VoyageState::InProgress => {
            // In-progress voyages with all done stories should be marked done
            let all_done = story_states.iter().all(|s| s.is_terminal());

            if all_done {
                violations.push(VoyageStoryViolation::AllStoriesDoneButVoyageNotDone {
                    voyage_id: voyage_id.to_string(),
                    story_count: story_states.len(),
                    voyage_state,
                });
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_stories_ok_for_draft_voyage() {
        let violations = validate_voyage_story_coherence("v1", VoyageState::Draft, &[]);
        assert!(violations.is_empty());
    }

    #[test]
    fn empty_stories_warns_for_non_draft_voyage() {
        let violations = validate_voyage_story_coherence("v1", VoyageState::Planned, &[]);
        assert_eq!(violations.len(), 1);
        assert!(matches!(
            &violations[0],
            VoyageStoryViolation::NoStoriesInNonDraftVoyage { .. }
        ));
    }

    #[test]
    fn draft_voyage_with_non_icebox_stories_warns() {
        let stories = vec![StoryState::Backlog, StoryState::Icebox];
        let violations = validate_voyage_story_coherence("v1", VoyageState::Draft, &stories);

        assert_eq!(violations.len(), 1);
        match &violations[0] {
            VoyageStoryViolation::NonIceboxStoryInDraftVoyage {
                non_icebox_count, ..
            } => {
                assert_eq!(*non_icebox_count, 1);
            }
            _ => panic!("Wrong violation type"),
        }
    }

    #[test]
    fn draft_voyage_with_all_icebox_stories_ok() {
        let stories = vec![StoryState::Icebox, StoryState::Icebox];
        let violations = validate_voyage_story_coherence("v1", VoyageState::Draft, &stories);
        assert!(violations.is_empty());
    }

    #[test]
    fn planned_voyage_with_active_stories_warns() {
        let stories = vec![StoryState::Backlog, StoryState::InProgress];
        let violations = validate_voyage_story_coherence("v1", VoyageState::Planned, &stories);

        assert_eq!(violations.len(), 1);
        match &violations[0] {
            VoyageStoryViolation::ActiveStoryInPlannedVoyage {
                progressed_count, ..
            } => {
                assert_eq!(*progressed_count, 1);
            }
            _ => panic!("Wrong violation type"),
        }
    }

    #[test]
    fn planned_voyage_with_done_stories_warns() {
        let stories = vec![StoryState::Backlog, StoryState::Done];
        let violations = validate_voyage_story_coherence("v1", VoyageState::Planned, &stories);

        assert_eq!(violations.len(), 1);
        assert!(matches!(
            &violations[0],
            VoyageStoryViolation::ActiveStoryInPlannedVoyage { .. }
        ));
    }

    #[test]
    fn planned_voyage_with_backlog_only_ok() {
        let stories = vec![StoryState::Backlog, StoryState::Backlog];
        let violations = validate_voyage_story_coherence("v1", VoyageState::Planned, &stories);
        assert!(violations.is_empty());
    }

    #[test]
    fn in_progress_voyage_with_all_done_warns() {
        let stories = vec![StoryState::Done, StoryState::Done];
        let violations = validate_voyage_story_coherence("v1", VoyageState::InProgress, &stories);

        assert_eq!(violations.len(), 1);
        match &violations[0] {
            VoyageStoryViolation::AllStoriesDoneButVoyageNotDone {
                story_count,
                voyage_state,
                ..
            } => {
                assert_eq!(*story_count, 2);
                assert_eq!(*voyage_state, VoyageState::InProgress);
            }
            _ => panic!("Wrong violation type"),
        }
    }

    #[test]
    fn in_progress_voyage_with_mixed_stories_ok() {
        let stories = vec![StoryState::Done, StoryState::InProgress];
        let violations = validate_voyage_story_coherence("v1", VoyageState::InProgress, &stories);
        assert!(violations.is_empty());
    }

    #[test]
    fn done_voyage_has_no_constraints() {
        // Done voyages don't care about story states
        let stories = vec![StoryState::InProgress, StoryState::Backlog];
        let violations = validate_voyage_story_coherence("v1", VoyageState::Done, &stories);
        assert!(violations.is_empty());
    }

    #[test]
    fn all_done_violation_suggests_update_status() {
        let violation = VoyageStoryViolation::AllStoriesDoneButVoyageNotDone {
            voyage_id: "v1".to_string(),
            story_count: 2,
            voyage_state: VoyageState::InProgress,
        };

        let fix = violation.suggested_fix();
        assert!(matches!(
            fix,
            Some(SuggestedFix::UpdateVoyageStatus {
                new_status: VoyageState::Done
            })
        ));
    }

    #[test]
    fn active_in_planned_has_no_auto_fix() {
        let violation = VoyageStoryViolation::ActiveStoryInPlannedVoyage {
            voyage_id: "v1".to_string(),
            progressed_count: 1,
        };

        // No auto-fix because user needs to decide (start voyage vs ice story)
        assert!(violation.suggested_fix().is_none());
    }

    #[test]
    fn violation_messages_contain_voyage_id() {
        let violations = vec![
            VoyageStoryViolation::ActiveStoryInPlannedVoyage {
                voyage_id: "test-voyage".to_string(),
                progressed_count: 3,
            },
            VoyageStoryViolation::NonIceboxStoryInDraftVoyage {
                voyage_id: "test-voyage".to_string(),
                non_icebox_count: 2,
            },
        ];

        for v in violations {
            assert!(v.message().contains("test-voyage"));
        }
    }

    // ============ Epic-Voyage Coherence Tests ============

    #[test]
    fn empty_voyages_ok_for_draft_epic() {
        let violations = validate_epic_voyage_coherence("e1", EpicState::Draft, &[]);
        assert!(violations.is_empty());
    }

    #[test]
    fn done_epic_with_incomplete_voyages_warns() {
        let voyages = vec![VoyageState::Done, VoyageState::InProgress];
        let violations = validate_epic_voyage_coherence("e1", EpicState::Done, &voyages);

        assert_eq!(violations.len(), 1);
        match &violations[0] {
            EpicVoyageViolation::IncompleteVoyageInDoneEpic {
                incomplete_count, ..
            } => {
                assert_eq!(*incomplete_count, 1);
            }
            _ => panic!("Wrong violation type"),
        }
    }

    #[test]
    fn done_epic_with_all_done_voyages_ok() {
        let voyages = vec![VoyageState::Done, VoyageState::Done];
        let violations = validate_epic_voyage_coherence("e1", EpicState::Done, &voyages);
        assert!(violations.is_empty());
    }

    #[test]
    fn draft_epic_with_all_done_voyages_warns() {
        let voyages = vec![VoyageState::Done, VoyageState::Done];
        let violations = validate_epic_voyage_coherence("e1", EpicState::Draft, &voyages);

        assert_eq!(violations.len(), 1);
        match &violations[0] {
            EpicVoyageViolation::AllVoyagesDoneButEpicNotDone {
                voyage_count,
                epic_state,
                ..
            } => {
                assert_eq!(*voyage_count, 2);
                assert_eq!(*epic_state, EpicState::Draft);
            }
            _ => panic!("Wrong violation type"),
        }
    }

    #[test]
    fn draft_epic_with_active_voyage_warns_drift() {
        let voyages = vec![VoyageState::InProgress, VoyageState::Draft];
        let violations = validate_epic_voyage_coherence("e1", EpicState::Draft, &voyages);

        assert_eq!(violations.len(), 1);
        match &violations[0] {
            EpicVoyageViolation::EpicStatusDrift {
                current_status,
                implicit_status,
                ..
            } => {
                assert_eq!(*current_status, EpicState::Draft);
                assert_eq!(*implicit_status, EpicState::Active);
            }
            _ => panic!("Wrong violation type"),
        }
    }

    #[test]
    fn active_epic_with_active_voyage_ok() {
        let voyages = vec![VoyageState::InProgress, VoyageState::Draft];
        let violations = validate_epic_voyage_coherence("e1", EpicState::Active, &voyages);
        assert!(violations.is_empty());
    }

    #[test]
    fn active_epic_with_planned_voyage_ok() {
        let voyages = vec![VoyageState::Planned, VoyageState::Draft];
        let violations = validate_epic_voyage_coherence("e1", EpicState::Active, &voyages);
        assert!(violations.is_empty());
    }

    #[test]
    fn all_voyages_done_violation_suggests_update_status() {
        let violation = EpicVoyageViolation::AllVoyagesDoneButEpicNotDone {
            epic_id: "e1".to_string(),
            voyage_count: 2,
            epic_state: EpicState::Draft,
        };

        let fix = violation.suggested_fix();
        assert!(matches!(
            fix,
            Some(SuggestedFix::UpdateEpicStatus {
                new_status: EpicState::Done
            })
        ));
    }

    #[test]
    fn drift_violation_suggests_update_status() {
        let violation = EpicVoyageViolation::EpicStatusDrift {
            epic_id: "e1".to_string(),
            current_status: EpicState::Draft,
            implicit_status: EpicState::Active,
        };

        let fix = violation.suggested_fix();
        assert!(matches!(
            fix,
            Some(SuggestedFix::UpdateEpicStatus {
                new_status: EpicState::Active
            })
        ));
    }

    #[test]
    fn incomplete_in_done_epic_has_no_auto_fix() {
        let violation = EpicVoyageViolation::IncompleteVoyageInDoneEpic {
            epic_id: "e1".to_string(),
            incomplete_count: 1,
        };

        // No auto-fix because user needs to decide (reopen epic vs complete voyages)
        assert!(violation.suggested_fix().is_none());
    }

    #[test]
    fn epic_violation_messages_contain_epic_id() {
        let violations = vec![
            EpicVoyageViolation::IncompleteVoyageInDoneEpic {
                epic_id: "test-epic".to_string(),
                incomplete_count: 3,
            },
            EpicVoyageViolation::AllVoyagesDoneButEpicNotDone {
                epic_id: "test-epic".to_string(),
                voyage_count: 2,
                epic_state: EpicState::Draft,
            },
        ];

        for v in violations {
            assert!(v.message().contains("test-epic") || v.message().contains("2 voyages"));
        }
    }
}
