//! Shared elapsed-duration rendering helpers for CLI output.

use chrono::NaiveDateTime;
use owo_colors::OwoColorize;

/// Return a dimmed completed timestamp with optional dimmed elapsed length.
pub fn render_completed_with_length(
    started_at: Option<NaiveDateTime>,
    completed_at: NaiveDateTime,
) -> String {
    let completed = format!("{}", completed_at.dimmed());
    match elapsed_duration(started_at, completed_at) {
        Some(length) => format!("{completed} ({})", length.dimmed()),
        None => completed,
    }
}

/// Return a human-readable elapsed duration if start/end ordering is valid.
pub fn elapsed_duration(
    started_at: Option<NaiveDateTime>,
    ended_at: NaiveDateTime,
) -> Option<String> {
    let started_at = started_at?;
    if ended_at < started_at {
        return None;
    }
    Some(format_elapsed_duration(
        (ended_at - started_at).num_seconds(),
    ))
}

/// Format seconds into compact day/hour/minute text.
pub fn format_elapsed_duration(total_seconds: i64) -> String {
    if total_seconds < 60 {
        return "<1m".to_string();
    }

    let mut seconds = total_seconds;
    let days = seconds / 86_400;
    seconds %= 86_400;
    let hours = seconds / 3_600;
    seconds %= 3_600;
    let minutes = seconds / 60;

    if days > 0 {
        format!("{days}d {hours}h {minutes}m")
    } else if hours > 0 {
        format!("{hours}h {minutes}m")
    } else {
        format!("{minutes}m")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn elapsed_duration_handles_missing_or_invalid_ranges() {
        let dt = NaiveDate::from_ymd_opt(2026, 3, 4)
            .unwrap()
            .and_hms_opt(10, 0, 0)
            .unwrap();
        assert!(elapsed_duration(None, dt).is_none());
        assert!(elapsed_duration(Some(dt), dt - chrono::Duration::minutes(1)).is_none());
    }

    #[test]
    fn format_elapsed_duration_uses_compact_units() {
        assert_eq!(format_elapsed_duration(30), "<1m");
        assert_eq!(format_elapsed_duration(60 * 45), "45m");
        assert_eq!(format_elapsed_duration(60 * 90), "1h 30m");
        assert_eq!(format_elapsed_duration(60 * 60 * 26 + 60 * 5), "1d 2h 5m");
    }
}
