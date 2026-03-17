//! Shared scheduled routine presentation helpers.

use chrono::{DateTime, SecondsFormat, Utc};

use keel::read_model::scheduled_routines::{ScheduledRoutineProjection, ScheduledRoutineState};

/// Render the stable human-readable routine schedule phrase used across CLI views.
pub fn describe_scheduled_routine(routine: &ScheduledRoutineProjection) -> String {
    match routine.state {
        ScheduledRoutineState::Due => format!(
            "due now; next run {}{}",
            routine.countdown.as_deref().unwrap_or("later"),
            next_eligible_suffix(routine.next_eligible_at)
        ),
        ScheduledRoutineState::Finished => format!(
            "finished this window; next run {}{}",
            routine.countdown.as_deref().unwrap_or("later"),
            next_eligible_suffix(routine.next_eligible_at)
        ),
        ScheduledRoutineState::Upcoming => format!(
            "next run {}{}",
            routine.countdown.as_deref().unwrap_or("later"),
            next_eligible_suffix(routine.next_eligible_at)
        ),
        ScheduledRoutineState::Invalid => format!(
            "invalid cadence: {}",
            routine.error.as_deref().unwrap_or("unknown error")
        ),
    }
}

fn next_eligible_suffix(next_eligible_at: Option<DateTime<Utc>>) -> String {
    next_eligible_at
        .map(|next| format!(" ({})", format_timestamp(next)))
        .unwrap_or_default()
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Secs, true)
}
