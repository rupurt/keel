use std::path::Path;

use anyhow::Result;

use super::super::types::*;
use crate::domain::model::{Board, EpicState};
use crate::domain::state_machine::invariants;
use crate::infrastructure::validation::structural;

/// Scan epic files for structural problems
/// Returns (problems, epic_count)
pub fn scan_epic_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    structural::scan_epic_files(board_dir)
}

/// Check a single epic file for problems
pub fn check_epic_file(path: &Path) -> Option<Problem> {
    structural::check_epic_file(path)
}

/// Check epic README structure for rendering issues
pub fn check_epic_readme_structure(path: &Path) -> Vec<Problem> {
    structural::check_epic_readme_structure(path)
}

/// Check epic PRD.md structure for section markers and content
pub fn check_epic_prd_structure(path: &Path) -> Vec<Problem> {
    structural::check_epic_prd_structure(path)
}

/// Check epic title case
pub fn check_epic_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let title = &epic.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(Problem {
                severity: Severity::Warning,
                path: epic.path.clone(),
                message: format!("title '{}' should use Title Case", title),
                fix: Some(Fix::UpdateTitle {
                    path: epic.path.clone(),
                    new_title,
                }),
                scope: None,
                category: Some(GapCategory::Convention),
                check_id: CheckId::TitleCaseViolation,
            });
        }
    }

    problems
}

/// Check epic date field consistency
pub fn check_epic_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        problems.extend(structural::check_date_consistency(
            &epic.path,
            CheckId::EpicDateConsistency,
        ));
    }

    problems
}

/// Check coherence between epic derived status and voyage states.
pub fn check_epic_status_drift(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let voyage_states: Vec<_> = board
            .voyages_for_epic(epic)
            .into_iter()
            .map(|voyage| voyage.status())
            .collect();

        let violations =
            invariants::validate_epic_voyage_coherence(epic.id(), epic.status(), &voyage_states);

        for violation in violations {
            problems.push(Problem {
                severity: Severity::Error,
                path: epic.path.clone(),
                message: violation.message(),
                fix: None,
                scope: Some(epic.id().to_string()),
                category: Some(GapCategory::Coherence),
                check_id: CheckId::EpicStatusDrift,
            });
        }
    }

    problems
}

/// Check completion gates for done epics
pub fn check_epic_done_gates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        if epic.status() == EpicState::Done {
            problems.extend(crate::domain::state_machine::evaluate_epic_done(
                board, epic,
            ));
        }
    }

    problems
}

/// Check epic ID-directory consistency
pub fn check_epic_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        // Extract directory name from path: epics/{dir-name}/README.md
        let dir_name = epic
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        let frontmatter_id = epic.id();

        if let Some(dir) = dir_name
            && dir != frontmatter_id
        {
            let old_path = epic.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(Problem {
                severity: Severity::Error,
                path: epic.path.clone(),
                message: format!(
                    "directory name '{}' differs from frontmatter id '{}'",
                    dir, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: Some(epic.id().to_string()),
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}

/// Check for duplicate epic IDs
pub fn check_epic_duplicates(board_dir: &Path) -> Vec<Problem> {
    use crate::domain::model::EpicFrontmatter;
    use crate::infrastructure::parser::parse_frontmatter;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    let mut problems = Vec::new();
    let mut id_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return problems;
    }

    if let Ok(entries) = fs::read_dir(epics_dir) {
        for entry in entries.flatten() {
            if !entry.path().is_dir() {
                continue;
            }

            let readme_path = entry.path().join("README.md");
            if !readme_path.exists() {
                continue;
            }

            if let Ok(content) = fs::read_to_string(&readme_path)
                && let Ok((fm, _)) = parse_frontmatter::<EpicFrontmatter>(&content)
            {
                id_to_paths.entry(fm.id).or_default().push(readme_path);
            }
        }
    }

    for (id, paths) in id_to_paths {
        if paths.len() > 1 {
            for path in &paths {
                let other_paths: Vec<_> = paths.iter().filter(|p| *p != path).collect();
                problems.push(
                    Problem::error(
                        path.clone(),
                        format!(
                            "duplicate epic ID '{}' (also in: {})",
                            id,
                            other_paths
                                .iter()
                                .map(|p| p.display().to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )
                    .with_check_id(CheckId::EpicDuplicateId),
                );
            }
        }
    }

    problems
}

/// Check that epics have a PRESS_RELEASE.md and placeholders are cleared
pub fn check_epic_press_release(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let pr_path = epic.path.parent().unwrap().join("PRESS_RELEASE.md");

        if !pr_path.exists() {
            problems.push(
                Problem::warning(epic.path.clone(), "epic is missing PRESS_RELEASE.md")
                    .with_check_id(CheckId::EpicMissingPressRelease)
                    .with_scope(epic.id()),
            );
            continue;
        }

        let content = match std::fs::read_to_string(&pr_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        if let Some(pattern) = structural::first_unfilled_placeholder_pattern(&content) {
            problems.push(
                Problem::error(
                    pr_path,
                    format!(
                        "PRESS_RELEASE.md has unresolved scaffold/default text (pattern: {})",
                        pattern
                    ),
                )
                .with_check_id(CheckId::EpicPressReleaseIncomplete)
                .with_scope(epic.id()),
            );
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use std::fs;

    #[test]
    fn check_epic_press_release_reports_unresolved_scaffold_as_error() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .build();
        let pr_path = temp.path().join("epics/epic-1/PRESS_RELEASE.md");
        fs::write(&pr_path, "# PRESS RELEASE\n\nTODO: finalize release copy").unwrap();

        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();
        let problems = check_epic_press_release(&board);

        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::EpicPressReleaseIncomplete);
        assert!(problems[0].message.contains("pattern: TODO:"));
    }

    #[test]
    fn check_epic_status_drift_reports_incoherent_derived_state() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic-1"))
            .voyage(TestVoyage::new("v1", "epic-1").status("draft"))
            .build();
        let mut board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        // Simulate stale derived state drift to exercise coherence mapping.
        board
            .epics
            .get_mut("epic-1")
            .unwrap()
            .set_status(crate::domain::model::EpicState::Active);

        let problems = check_epic_status_drift(&board);
        assert_eq!(problems.len(), 1);
        assert_eq!(problems[0].severity, Severity::Error);
        assert_eq!(problems[0].check_id, CheckId::EpicStatusDrift);
    }
}
