//! `keel pulse` command adapter.

use std::path::Path;

use anyhow::Result;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::Serialize;

use keel::domain::model::Board;
use keel::infrastructure::loader::load_board;
use keel::read_model::scheduled_routines::{
    RoutineScheduleFilter, ScheduledRoutineGatingReason, ScheduledRoutineProjection,
    ScheduledRoutineState, project_scheduled_routines,
};

/// Run the pulse command.
pub fn run(json: bool) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let output = build_pulse_output_with_dir_at(&board_dir, json, Utc::now())?;
    print!("{output}");
    Ok(())
}

fn build_pulse_output_with_dir_at(
    board_dir: &Path,
    json: bool,
    reference_time: DateTime<Utc>,
) -> Result<String> {
    let board = load_board(board_dir)?;
    let cycle = build_pulse_cycle(&board, reference_time);
    if json {
        Ok(format!("{}\n", serde_json::to_string_pretty(&cycle)?))
    } else {
        Ok(render_pulse_human(&cycle))
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct PulseCycleOutput {
    mode: &'static str,
    evaluated: usize,
    would_trigger: usize,
    skipped: usize,
    routines: Vec<PulseRoutineSummary>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct PulseRoutineSummary {
    id: String,
    title: String,
    target_scope: String,
    outcome: PulseRoutineOutcome,
    reason: PulseRoutineReason,
    next_eligible_at: Option<DateTime<Utc>>,
    countdown: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PulseRoutineOutcome {
    WouldTrigger,
    Skipped,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum PulseRoutineReason {
    DueNow,
    NotDueUntilNextEligible,
    InvalidCadence,
}

fn build_pulse_cycle(board: &Board, reference_time: DateTime<Utc>) -> PulseCycleOutput {
    let scheduled =
        project_scheduled_routines(board, reference_time, RoutineScheduleFilter::default());
    let routines: Vec<_> = scheduled.iter().map(map_pulse_routine).collect();
    let would_trigger = routines
        .iter()
        .filter(|routine| matches!(routine.outcome, PulseRoutineOutcome::WouldTrigger))
        .count();
    let skipped = routines.len().saturating_sub(would_trigger);

    PulseCycleOutput {
        mode: "dry_run",
        evaluated: routines.len(),
        would_trigger,
        skipped,
        routines,
    }
}

fn map_pulse_routine(routine: &ScheduledRoutineProjection) -> PulseRoutineSummary {
    PulseRoutineSummary {
        id: routine.id.clone(),
        title: routine.title.clone(),
        target_scope: routine.target_scope.clone(),
        outcome: if matches!(routine.state, ScheduledRoutineState::Due) {
            PulseRoutineOutcome::WouldTrigger
        } else {
            PulseRoutineOutcome::Skipped
        },
        reason: match routine.gating_reason {
            ScheduledRoutineGatingReason::DueNow => PulseRoutineReason::DueNow,
            ScheduledRoutineGatingReason::NotDueUntilNextEligible => {
                PulseRoutineReason::NotDueUntilNextEligible
            }
            ScheduledRoutineGatingReason::InvalidCadence => PulseRoutineReason::InvalidCadence,
        },
        next_eligible_at: routine.next_eligible_at,
        countdown: routine.countdown.clone(),
        error: routine.error.clone(),
    }
}

fn render_pulse_human(cycle: &PulseCycleOutput) -> String {
    let mut lines = vec![
        "Pulse cycle (dry-run)".to_string(),
        format!("  Evaluated: {}", cycle.evaluated),
        format!("  Would trigger: {}", cycle.would_trigger),
        format!("  Skipped:   {}", cycle.skipped),
    ];

    let would_trigger: Vec<_> = cycle
        .routines
        .iter()
        .filter(|routine| matches!(routine.outcome, PulseRoutineOutcome::WouldTrigger))
        .collect();
    let skipped: Vec<_> = cycle
        .routines
        .iter()
        .filter(|routine| matches!(routine.outcome, PulseRoutineOutcome::Skipped))
        .collect();

    if !would_trigger.is_empty() {
        lines.push(String::new());
        lines.push("Would trigger routines:".to_string());
        lines.extend(would_trigger.into_iter().map(render_human_routine_line));
    }

    if !skipped.is_empty() {
        lines.push(String::new());
        lines.push("Skipped routines:".to_string());
        lines.extend(skipped.into_iter().map(render_human_routine_line));
    }

    lines.push(String::new());
    lines.join("\n")
}

fn render_human_routine_line(routine: &PulseRoutineSummary) -> String {
    format!(
        "  - {} | {} | {} | {}",
        routine.id,
        routine.title,
        routine.target_scope,
        human_reason(routine)
    )
}

fn human_reason(routine: &PulseRoutineSummary) -> String {
    match routine.reason {
        PulseRoutineReason::DueNow => format!(
            "due now | next run {}",
            format_optional_timestamp(routine.next_eligible_at)
        ),
        PulseRoutineReason::NotDueUntilNextEligible => {
            match (routine.next_eligible_at, routine.countdown.as_deref()) {
                (Some(next_eligible_at), Some(countdown)) => {
                    format!(
                        "not due until {} ({countdown})",
                        format_timestamp(next_eligible_at)
                    )
                }
                (Some(next_eligible_at), None) => {
                    format!("not due until {}", format_timestamp(next_eligible_at))
                }
                (None, Some(countdown)) => format!("not due yet ({countdown})"),
                (None, None) => "not due yet".to_string(),
            }
        }
        PulseRoutineReason::InvalidCadence => format!(
            "invalid cadence: {}",
            routine.error.as_deref().unwrap_or("unknown cadence error")
        ),
    }
}

fn format_optional_timestamp(value: Option<DateTime<Utc>>) -> String {
    value
        .map(format_timestamp)
        .unwrap_or_else(|| "unknown".to_string())
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Secs, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use keel::test_helpers::TestBoardBuilder;
    use serde_json::json;
    use std::fs;

    fn write_routine(root: &Path, id: &str, title: &str, target_scope: &str, cadence_block: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: {title}
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
    fn pulse_human_output_reports_evaluated_would_trigger_and_skipped_routines() {
        let temp = TestBoardBuilder::new().build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-upcoming",
            "Friday Review",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-invalid",
            "Broken Review",
            "E1/V1",
            "  timezone: America/Los_Angeles",
        );

        let rendered = build_pulse_output_with_dir_at(
            temp.path(),
            false,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        assert_eq!(
            rendered,
            concat!(
                "Pulse cycle (dry-run)\n",
                "  Evaluated: 3\n",
                "  Would trigger: 1\n",
                "  Skipped:   2\n",
                "\n",
                "Would trigger routines:\n",
                "  - routine-due | Weekly Review | E1/V1 | due now | next run 2026-01-12T17:00:00Z\n",
                "\n",
                "Skipped routines:\n",
                "  - routine-upcoming | Friday Review | E1/V1 | not due until 2026-01-05T19:00:00Z (in 1h)\n",
                "  - routine-invalid | Broken Review | E1/V1 | invalid cadence: Routine 'routine-invalid' is missing cadence.cron\n",
            )
        );
    }

    #[test]
    fn pulse_json_output_is_structured_for_scheduler_logs() {
        let temp = TestBoardBuilder::new().build();
        write_routine(
            temp.path(),
            "routine-due",
            "Weekly Review",
            "E1/V1",
            "  cron: 0 9 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-upcoming",
            "Friday Review",
            "E1/V1",
            "  cron: 0 11 * * 1\n  timezone: America/Los_Angeles",
        );
        write_routine(
            temp.path(),
            "routine-invalid",
            "Broken Review",
            "E1/V1",
            "  timezone: America/Los_Angeles",
        );

        let rendered = build_pulse_output_with_dir_at(
            temp.path(),
            true,
            Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap(),
        )
        .unwrap();

        let parsed: serde_json::Value = serde_json::from_str(&rendered).unwrap();
        assert_eq!(
            parsed,
            json!({
                "mode": "dry_run",
                "evaluated": 3,
                "would_trigger": 1,
                "skipped": 2,
                "routines": [
                    {
                        "id": "routine-due",
                        "title": "Weekly Review",
                        "target_scope": "E1/V1",
                        "outcome": "would_trigger",
                        "reason": "due_now",
                        "next_eligible_at": "2026-01-12T17:00:00Z",
                        "countdown": "in 6d 23h",
                        "error": null
                    },
                    {
                        "id": "routine-upcoming",
                        "title": "Friday Review",
                        "target_scope": "E1/V1",
                        "outcome": "skipped",
                        "reason": "not_due_until_next_eligible",
                        "next_eligible_at": "2026-01-05T19:00:00Z",
                        "countdown": "in 1h",
                        "error": null
                    },
                    {
                        "id": "routine-invalid",
                        "title": "Broken Review",
                        "target_scope": "E1/V1",
                        "outcome": "skipped",
                        "reason": "invalid_cadence",
                        "next_eligible_at": null,
                        "countdown": null,
                        "error": "Routine 'routine-invalid' is missing cadence.cron"
                    }
                ]
            })
        );
    }
}
