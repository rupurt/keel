use crate::domain::model::Board;
use crate::infrastructure::validation::{CheckId, Fix, Problem, Severity};

/// Check that all routines have valid cadence configuration.
pub fn check_routine_cadence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for routine in board.routines.values() {
        if let Some(cron_val) = routine.frontmatter.cadence.get("cron") {
            if cron_val.as_str().is_none() {
                problems.push(
                    Problem::error(
                        routine.path.clone(),
                        format!(
                            "Routine '{}' has invalid cadence.cron (expected string)",
                            routine.id()
                        ),
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
                        format!(
                            "Routine '{}' has invalid cadence.timezone (expected string)",
                            routine.id()
                        ),
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

/// Check routine ID-filename consistency
pub fn check_routine_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for routine in board.routines.values() {
        let Some(bundle_name) = routine
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
        else {
            continue;
        };

        let frontmatter_id = routine.id();

        if frontmatter_id != bundle_name {
            let old_path = routine.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(Problem {
                severity: Severity::Error,
                path: routine.path.clone(),
                message: format!(
                    "routine directory name '{}' does not match frontmatter id '{}'",
                    bundle_name, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: Some(routine.id().to_string()),
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}
