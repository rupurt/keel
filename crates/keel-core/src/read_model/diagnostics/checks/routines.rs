use crate::domain::model::{Board, VoyageState};
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

/// Check that routine target-scope references existing, non-terminal entities.
pub fn check_routine_scope_coherence(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for routine in board.routines.values() {
        let target_scope = routine.target_scope().trim();
        if target_scope.is_empty() {
            continue;
        }

        // Watch scope: plain ID matching a known watch
        if board.find_watch(target_scope).is_some() {
            continue; // Valid watch scope
        }

        if let Some((epic_id, voyage_id)) = target_scope.split_once('/') {
            if board.find_epic(epic_id).is_none() {
                problems.push(
                    Problem::warning(
                        routine.path.clone(),
                        format!(
                            "Routine '{}' targets missing epic '{}'",
                            routine.id(),
                            epic_id
                        ),
                    )
                    .with_check_id(CheckId::RoutineScopeIncoherence),
                );
            }
            match board.find_voyage(voyage_id) {
                None => {
                    problems.push(
                        Problem::warning(
                            routine.path.clone(),
                            format!(
                                "Routine '{}' targets missing voyage '{}'",
                                routine.id(),
                                voyage_id
                            ),
                        )
                        .with_check_id(CheckId::RoutineScopeIncoherence),
                    );
                }
                Some(voyage) if voyage.status() == VoyageState::Done => {
                    problems.push(
                        Problem::error(
                            routine.path.clone(),
                            format!(
                                "Routine '{}' targets terminal voyage '{}'",
                                routine.id(),
                                voyage_id
                            ),
                        )
                        .with_check_id(CheckId::RoutineScopeIncoherence),
                    );
                }
                _ => {}
            }
        } else if board.find_epic(target_scope).is_none() {
            problems.push(
                Problem::warning(
                    routine.path.clone(),
                    format!(
                        "Routine '{}' targets missing epic '{}'",
                        routine.id(),
                        target_scope
                    ),
                )
                .with_check_id(CheckId::RoutineScopeIncoherence),
            );
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use std::fs;

    fn write_routine(root: &std::path::Path, id: &str, target_scope: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: Test Routine
cadence:
  cron: 0 9 * * 1
  timezone: America/Los_Angeles
target-scope: {target_scope}
created_at: 2026-03-12T09:00:00
updated_at: 2026-03-12T09:00:00
---

# Blueprint
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn scope_coherence_warns_on_missing_epic() {
        let temp = TestBoardBuilder::new().build();
        write_routine(temp.path(), "r1", "nonexistent-epic");
        let board = load_board(temp.path()).unwrap();

        let problems = check_routine_scope_coherence(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Warning);
        assert!(problems[0].message.contains("missing epic"));
    }

    #[test]
    fn scope_coherence_warns_on_missing_voyage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .build();
        write_routine(temp.path(), "r1", "E1/nonexistent-voyage");
        let board = load_board(temp.path()).unwrap();

        let problems = check_routine_scope_coherence(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Warning);
        assert!(problems[0].message.contains("missing voyage"));
    }

    #[test]
    fn scope_coherence_errors_on_terminal_voyage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1").status("done"))
            .build();
        write_routine(temp.path(), "r1", "E1/V1");
        let board = load_board(temp.path()).unwrap();

        let problems = check_routine_scope_coherence(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert!(problems[0].message.contains("terminal voyage"));
    }

    #[test]
    fn scope_coherence_passes_for_valid_scope() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1").status("in-progress"))
            .build();
        write_routine(temp.path(), "r1", "E1/V1");
        let board = load_board(temp.path()).unwrap();

        let problems = check_routine_scope_coherence(&board);
        assert!(problems.is_empty());
    }
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
