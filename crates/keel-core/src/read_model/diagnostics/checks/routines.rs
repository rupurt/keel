use crate::domain::model::Board;
use crate::infrastructure::validation::{CheckId, Fix, Problem};

/// Check that all routines have valid cadence configuration.
pub fn check_routine_cadence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for routine in board.routines.values() {
        if let Some(cron_val) = routine.frontmatter.cadence.get("cron") {
            if cron_val.as_str().is_none() {
                problems.push(
                    Problem::error(
                        routine.path.clone(),
                        format!("Routine '{}' has invalid cadence.cron (expected string)", routine.id()),
                    )
                    .with_check_id(CheckId::RoutineInvalidCadence)
                    .with_fix(Fix::SetFrontmatterField {
                        path: routine.path.clone(),
                        field: "cadence.cron".to_string(),
                        value: "0 9 * * 1".to_string(), // Default to Monday 9am
                    }),
                );
            }
        } else {
            problems.push(
                Problem::error(
                    routine.path.clone(),
                    format!("Routine '{}' is missing cadence.cron", routine.id()),
                )
                .with_check_id(CheckId::RoutineInvalidCadence)
                .with_fix(Fix::SetFrontmatterField {
                    path: routine.path.clone(),
                    field: "cadence.cron".to_string(),
                    value: "0 9 * * 1".to_string(),
                }),
            );
        }

        if let Some(tz_val) = routine.frontmatter.cadence.get("timezone") {
            if tz_val.as_str().is_none() {
                problems.push(
                    Problem::error(
                        routine.path.clone(),
                        format!("Routine '{}' has invalid cadence.timezone (expected string)", routine.id()),
                    )
                    .with_check_id(CheckId::RoutineInvalidCadence)
                    .with_fix(Fix::SetFrontmatterField {
                        path: routine.path.clone(),
                        field: "cadence.timezone".to_string(),
                        value: "America/Los_Angeles".to_string(),
                    }),
                );
            }
        } else {
            problems.push(
                Problem::error(
                    routine.path.clone(),
                    format!("Routine '{}' is missing cadence.timezone", routine.id()),
                )
                .with_check_id(CheckId::RoutineInvalidCadence)
                .with_fix(Fix::SetFrontmatterField {
                    path: routine.path.clone(),
                    field: "cadence.timezone".to_string(),
                    value: "America/Los_Angeles".to_string(),
                }),
            );
        }
    }

    problems
}
