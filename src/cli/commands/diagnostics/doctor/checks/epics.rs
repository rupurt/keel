use std::path::Path;

use anyhow::Result;

use super::super::types::*;
use crate::domain::model::{Board, EpicState, VoyageState};
use crate::domain::state_machine::invariants::{
    EpicVoyageViolation, validate_epic_voyage_coherence,
};
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

/// Check epic status consistency
/// - Done epics shouldn't have incomplete voyages
/// - Non-done epics with all voyages done should be marked done
pub fn check_epic_status_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for epic in board.epics.values() {
        let voyages = board.voyages_for_epic(epic);

        let epic_state = epic.status();
        let voyage_states: Vec<VoyageState> = voyages.iter().map(|v| v.status()).collect();

        let violations = validate_epic_voyage_coherence(epic.id(), epic_state, &voyage_states);

        for violation in violations {
            let fix = match &violation {
                crate::domain::state_machine::invariants::EpicVoyageViolation::AllVoyagesDoneButEpicNotDone { .. } => {
                    Some(Fix::UpdateEpicStatus {
                        path: epic.path.clone(),
                        new_status: EpicState::Done.to_string(),
                    })
                }
                crate::domain::state_machine::invariants::EpicVoyageViolation::EpicStatusDrift {
                    implicit_status, ..
                } => Some(Fix::UpdateEpicStatus {
                    path: epic.path.clone(),
                    new_status: implicit_status.to_string(),
                }),
                _ => None,
            };

            problems.push(Problem {
                severity: Severity::Warning,
                path: epic.path.clone(),
                message: violation.message(),
                fix,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
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

        if content.contains("TODO:") || content.contains("{{") {
            problems.push(
                Problem::warning(pr_path, "PRESS_RELEASE.md has unfilled placeholders")
                    .with_check_id(CheckId::EpicPressReleaseIncomplete)
                    .with_scope(epic.id()),
            );
        }
    }

    problems
}
