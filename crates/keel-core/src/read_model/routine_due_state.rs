//! Standalone routine due-state evaluation.
//!
//! This module interprets the current canonical routine cadence mapping
//! (`cron` + `timezone`) against an injected reference time and produces a
//! deterministic due/not-due snapshot for later consumers.

use std::str::FromStr;

use anyhow::{Result, anyhow, bail};
use chrono::{DateTime, LocalResult, TimeZone, Utc};
use chrono_tz::Tz;
use croner::Cron;
use serde_yaml::{Mapping, Value};

use crate::domain::model::Routine;

/// Due-state category for a routine at a reference instant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineDueCategory {
    Due,
    NotDue,
}

/// Deterministic due-state projection for a routine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutineDueState {
    pub category: RoutineDueCategory,
    pub next_eligible_at: DateTime<Utc>,
}

impl RoutineDueState {
    pub fn is_due(&self) -> bool {
        matches!(self.category, RoutineDueCategory::Due)
    }
}

/// Evaluate the routine cadence mapping against a reference time.
pub fn evaluate_routine_due_state(
    routine: &Routine,
    reference_time: DateTime<Utc>,
) -> Result<RoutineDueState> {
    let cadence = CadenceSpec::parse(routine)?;
    let created_at = routine
        .frontmatter
        .created_at
        .ok_or_else(|| anyhow!("Routine '{}' is missing created_at", routine.id()))?;
    let created_at = localize_naive_datetime(routine, cadence.timezone, created_at, "created_at")?;
    let reference_time = reference_time.with_timezone(&cadence.timezone);
    let first_eligible_at = cadence
        .schedule
        .find_next_occurrence(&created_at, true)
        .map_err(|err| {
            anyhow!(
                "Routine '{}' failed to evaluate first eligible occurrence: {err}",
                routine.id()
            )
        })?;

    if reference_time < first_eligible_at {
        return Ok(RoutineDueState {
            category: RoutineDueCategory::NotDue,
            next_eligible_at: first_eligible_at.with_timezone(&Utc),
        });
    }

    let next_eligible_at = cadence
        .schedule
        .find_next_occurrence(&reference_time, false)
        .map_err(|err| {
            anyhow!(
                "Routine '{}' failed to evaluate next eligible occurrence: {err}",
                routine.id()
            )
        })?;

    Ok(RoutineDueState {
        category: RoutineDueCategory::Due,
        next_eligible_at: next_eligible_at.with_timezone(&Utc),
    })
}

#[derive(Debug, Clone)]
struct CadenceSpec {
    schedule: Cron,
    timezone: Tz,
}

impl CadenceSpec {
    fn parse(routine: &Routine) -> Result<Self> {
        let cadence = cadence_mapping(routine)?;
        let cron_expression = cadence_string(cadence, "cron", routine)?;
        let timezone_name = cadence_string(cadence, "timezone", routine)?;
        let schedule = Cron::from_str(cron_expression).map_err(|err| {
            anyhow!(
                "Routine '{}' has invalid cadence.cron '{}': {err}",
                routine.id(),
                cron_expression
            )
        })?;
        let timezone = timezone_name.parse::<Tz>().map_err(|err| {
            anyhow!(
                "Routine '{}' has invalid cadence.timezone '{}': {err}",
                routine.id(),
                timezone_name
            )
        })?;

        Ok(Self { schedule, timezone })
    }
}

fn cadence_mapping(routine: &Routine) -> Result<&Mapping> {
    routine
        .cadence()
        .as_mapping()
        .ok_or_else(|| anyhow!("Routine '{}' cadence must be a YAML mapping", routine.id()))
}

fn cadence_string<'a>(cadence: &'a Mapping, key: &str, routine: &Routine) -> Result<&'a str> {
    let value = cadence
        .get(Value::from(key))
        .ok_or_else(|| anyhow!("Routine '{}' is missing cadence.{}", routine.id(), key))?;

    value.as_str().ok_or_else(|| {
        anyhow!(
            "Routine '{}' cadence.{} must be a string",
            routine.id(),
            key
        )
    })
}

fn localize_naive_datetime(
    routine: &Routine,
    timezone: Tz,
    naive: chrono::NaiveDateTime,
    field_name: &str,
) -> Result<DateTime<Tz>> {
    match timezone.from_local_datetime(&naive) {
        LocalResult::Single(datetime) => Ok(datetime),
        LocalResult::Ambiguous(first, second) => bail!(
            "Routine '{}' has ambiguous {} '{}' in timezone '{}': {} or {}",
            routine.id(),
            field_name,
            naive,
            timezone,
            first,
            second
        ),
        LocalResult::None => bail!(
            "Routine '{}' has invalid {} '{}' in timezone '{}'",
            routine.id(),
            field_name,
            naive,
            timezone
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, TimeZone};
    use serde_yaml::Value;

    use crate::domain::model::{Routine, RoutineFrontmatter};

    fn build_routine() -> Routine {
        Routine::new(
            RoutineFrontmatter {
                id: "routine-weekly-review".to_string(),
                title: "Weekly Review".to_string(),
                cadence: serde_yaml::from_str::<Value>(
                    r#"
cron: 0 9 * * 1
timezone: America/Los_Angeles
"#,
                )
                .unwrap(),
                target_scope: "VDakmCGYi/VDcFd5kmn".to_string(),
                created_at: Some(
                    NaiveDate::from_ymd_opt(2026, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                updated_at: None,
                operator_signal: None,
                lineage: Vec::new(),
            },
            "# Blueprint\n",
            ".keel/routines/routine-weekly-review/README.md",
        )
    }

    #[test]
    fn evaluate_routine_due_state_returns_not_due_before_first_matching_window() {
        let routine = build_routine();
        let reference_time = Utc.with_ymd_and_hms(2026, 1, 5, 16, 0, 0).unwrap();

        let state = evaluate_routine_due_state(&routine, reference_time).unwrap();

        assert_eq!(state.category, RoutineDueCategory::NotDue);
        assert!(!state.is_due());
        assert_eq!(
            state.next_eligible_at,
            Utc.with_ymd_and_hms(2026, 1, 5, 17, 0, 0).unwrap()
        );
    }

    #[test]
    fn evaluate_routine_due_state_returns_due_and_next_eligible_after_matching_window() {
        let routine = build_routine();
        let reference_time = Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap();

        let state = evaluate_routine_due_state(&routine, reference_time).unwrap();

        assert_eq!(state.category, RoutineDueCategory::Due);
        assert!(state.is_due());
        assert_eq!(
            state.next_eligible_at,
            Utc.with_ymd_and_hms(2026, 1, 12, 17, 0, 0).unwrap()
        );
    }

    #[test]
    fn evaluate_routine_due_state_is_deterministic_for_identical_reference_time() {
        let routine = build_routine();
        let reference_time = Utc.with_ymd_and_hms(2026, 1, 5, 18, 0, 0).unwrap();

        let first = evaluate_routine_due_state(&routine, reference_time).unwrap();
        let second = evaluate_routine_due_state(&routine, reference_time).unwrap();

        assert_eq!(first, second);
    }
}
