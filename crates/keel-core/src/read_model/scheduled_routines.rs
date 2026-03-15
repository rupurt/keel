//! Scheduled routine projections for `keel next`.

use std::cmp::Ordering;

use chrono::{DateTime, Duration, Utc};

use crate::domain::model::{Board, Routine, Story};
use crate::read_model::routine_due_state::{RoutineDueCategory, evaluate_routine_due_state};

/// Stable scheduled state exposed by `keel next`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduledRoutineState {
    Due,
    Upcoming,
    Invalid,
}

/// Machine-readable reason for whether a scheduled routine is actionable.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduledRoutineGatingReason {
    DueNow,
    NotDueUntilNextEligible,
    InvalidCadence,
}

/// Projected routine scheduling metadata for rendering and queue gating.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduledRoutineProjection {
    pub id: String,
    pub title: String,
    pub target_scope: String,
    pub state: ScheduledRoutineState,
    pub actionable: bool,
    pub gating_reason: ScheduledRoutineGatingReason,
    pub next_eligible_at: Option<DateTime<Utc>>,
    pub countdown: Option<String>,
    pub error: Option<String>,
}

/// Optional board-level filter for scheduled routine projection.
#[derive(Debug, Clone, Copy, Default)]
pub struct RoutineScheduleFilter<'a> {
    pub mission_id: Option<&'a str>,
}

/// Project scheduled routine metadata at a deterministic reference time.
pub fn project_scheduled_routines(
    board: &Board,
    reference_time: DateTime<Utc>,
    filter: RoutineScheduleFilter<'_>,
) -> Vec<ScheduledRoutineProjection> {
    let mut scheduled: Vec<_> = board
        .routines
        .values()
        .filter(|routine| routine_matches_filter(board, routine, filter))
        .map(|routine| project_routine(routine, reference_time))
        .collect();

    scheduled.sort_by(compare_scheduled_routines);
    scheduled
}

/// Whether a story should be gated by projected routine schedule state.
pub fn story_is_gated_by_scheduled_routines(
    story: &Story,
    scheduled: &[ScheduledRoutineProjection],
) -> bool {
    let Some(scope) = story.scope() else {
        return false;
    };

    if scheduled.iter().any(|routine| {
        routine.actionable && target_scope_matches_story_scope(&routine.target_scope, scope)
    }) {
        return false;
    }

    scheduled.iter().any(|routine| {
        !routine.actionable
            && matches!(routine.state, ScheduledRoutineState::Upcoming)
            && target_scope_matches_story_scope(&routine.target_scope, scope)
    })
}

fn project_routine(routine: &Routine, reference_time: DateTime<Utc>) -> ScheduledRoutineProjection {
    match evaluate_routine_due_state(routine, reference_time) {
        Ok(state) => {
            let (scheduled_state, actionable, gating_reason) = match state.category {
                RoutineDueCategory::Due => (
                    ScheduledRoutineState::Due,
                    true,
                    ScheduledRoutineGatingReason::DueNow,
                ),
                RoutineDueCategory::NotDue => (
                    ScheduledRoutineState::Upcoming,
                    false,
                    ScheduledRoutineGatingReason::NotDueUntilNextEligible,
                ),
            };

            ScheduledRoutineProjection {
                id: routine.id().to_string(),
                title: routine.title().to_string(),
                target_scope: routine.target_scope().to_string(),
                state: scheduled_state,
                actionable,
                gating_reason,
                next_eligible_at: Some(state.next_eligible_at),
                countdown: Some(format_countdown(reference_time, state.next_eligible_at)),
                error: None,
            }
        }
        Err(error) => ScheduledRoutineProjection {
            id: routine.id().to_string(),
            title: routine.title().to_string(),
            target_scope: routine.target_scope().to_string(),
            state: ScheduledRoutineState::Invalid,
            actionable: false,
            gating_reason: ScheduledRoutineGatingReason::InvalidCadence,
            next_eligible_at: None,
            countdown: None,
            error: Some(error.to_string()),
        },
    }
}

fn compare_scheduled_routines(
    left: &ScheduledRoutineProjection,
    right: &ScheduledRoutineProjection,
) -> Ordering {
    state_rank(left.state)
        .cmp(&state_rank(right.state))
        .then_with(|| left.id.cmp(&right.id))
}

fn state_rank(state: ScheduledRoutineState) -> u8 {
    match state {
        ScheduledRoutineState::Due => 0,
        ScheduledRoutineState::Upcoming => 1,
        ScheduledRoutineState::Invalid => 2,
    }
}

fn routine_matches_filter(
    board: &Board,
    routine: &Routine,
    filter: RoutineScheduleFilter<'_>,
) -> bool {
    filter
        .mission_id
        .is_none_or(|mission_id| routine_in_mission(board, routine, mission_id))
}

fn routine_in_mission(board: &Board, routine: &Routine, mission_id: &str) -> bool {
    routine
        .target_scope()
        .split('/')
        .next()
        .is_some_and(|epic_id| board.is_epic_in_mission(epic_id, mission_id))
}

fn target_scope_matches_story_scope(target_scope: &str, story_scope: &str) -> bool {
    story_scope == target_scope || story_scope.starts_with(&format!("{target_scope}/"))
}

fn format_countdown(reference_time: DateTime<Utc>, next_eligible_at: DateTime<Utc>) -> String {
    let delta = next_eligible_at - reference_time;
    if delta <= Duration::zero() {
        return "now".to_string();
    }

    let total_minutes = delta.num_minutes();
    if total_minutes <= 0 {
        return "in <1m".to_string();
    }

    let days = total_minutes / (24 * 60);
    let hours = (total_minutes % (24 * 60)) / 60;
    let minutes = total_minutes % 60;

    match (days, hours, minutes) {
        (0, 0, m) => format!("in {m}m"),
        (0, h, 0) => format!("in {h}h"),
        (0, h, m) => format!("in {h}h {m}m"),
        (d, 0, _) => format!("in {d}d"),
        (d, h, _) => format!("in {d}d {h}h"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::TimeZone;
    use std::fs;
    use std::path::Path;

    fn write_routine(root: &Path, id: &str, target_scope: &str, cadence_block: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: {id}
cadence:
{cadence_block}
target-scope: {target_scope}
created_at: 2026-01-01T00:00:00
updated_at: 2026-01-01T00:00:00
---

# Blueprint
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn project_scheduled_routines_reports_due_upcoming_and_invalid_states() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1"))
            .build();
        write_routine(
            temp.path(),
            "routine-due",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-upcoming",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-invalid",
            "E1/V1",
            "  timezone: America/Los_Angeles",
        );

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let reference_time = Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap();

        let scheduled =
            project_scheduled_routines(&board, reference_time, RoutineScheduleFilter::default());

        assert_eq!(scheduled.len(), 3);
        assert_eq!(scheduled[0].id, "routine-due");
        assert_eq!(scheduled[0].state, ScheduledRoutineState::Due);
        assert!(scheduled[0].actionable);
        assert_eq!(
            scheduled[0].gating_reason,
            ScheduledRoutineGatingReason::DueNow
        );

        assert_eq!(scheduled[1].id, "routine-upcoming");
        assert_eq!(scheduled[1].state, ScheduledRoutineState::Upcoming);
        assert!(!scheduled[1].actionable);
        assert_eq!(
            scheduled[1].gating_reason,
            ScheduledRoutineGatingReason::NotDueUntilNextEligible
        );

        assert_eq!(scheduled[2].id, "routine-invalid");
        assert_eq!(scheduled[2].state, ScheduledRoutineState::Invalid);
        assert!(!scheduled[2].actionable);
        assert_eq!(
            scheduled[2].gating_reason,
            ScheduledRoutineGatingReason::InvalidCadence
        );
        assert!(scheduled[2].error.is_some());
    }

    #[test]
    fn story_is_gated_only_when_matching_scope_has_no_due_routine() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .epic(TestEpic::new("E2"))
            .voyage(TestVoyage::new("V1", "E1"))
            .voyage(TestVoyage::new("V2", "E2"))
            .story(TestStory::new("S1").scope("E1/V1"))
            .story(TestStory::new("S2").scope("E2/V2"))
            .build();
        write_routine(
            temp.path(),
            "routine-upcoming",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-due",
            "E2/V2",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let reference_time = Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap();
        let scheduled =
            project_scheduled_routines(&board, reference_time, RoutineScheduleFilter::default());

        assert!(story_is_gated_by_scheduled_routines(
            board.stories.get("S1").unwrap(),
            &scheduled
        ));
        assert!(!story_is_gated_by_scheduled_routines(
            board.stories.get("S2").unwrap(),
            &scheduled
        ));
    }
}
