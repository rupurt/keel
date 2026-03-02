//! Health checks for voyages

use anyhow::Result;
use std::path::Path;

use super::super::types::{CheckId, Fix, GapCategory, Problem, Severity};
use crate::domain::model::Board;
use crate::infrastructure::validation::structural;

pub struct VoyageScanResult {
    pub problems: Vec<Problem>,
    pub file_count: usize,
}

pub fn scan_voyage_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    structural::scan_voyage_files(board_dir)
}

pub fn check_voyage_status_drift(_board: &Board) -> Vec<Problem> {
    Vec::new()
}

/// Check voyage title case
pub fn check_voyage_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        let title = &voyage.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(Problem {
                severity: Severity::Warning,
                path: voyage.path.clone(),
                message: format!("title '{}' should use Title Case", title),
                fix: Some(Fix::UpdateTitle {
                    path: voyage.path.clone(),
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

/// Check voyage date field consistency
pub fn check_voyage_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        problems.extend(structural::check_date_consistency(
            &voyage.path,
            CheckId::VoyageDateConsistency,
        ));
    }

    problems
}

/// Check for duplicate voyage IDs across all epics
pub fn check_voyage_duplicates(board_dir: &Path) -> Vec<Problem> {
    use crate::domain::model::VoyageFrontmatter;
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

    if let Ok(epic_entries) = fs::read_dir(epics_dir) {
        for epic_entry in epic_entries.flatten() {
            if !epic_entry.path().is_dir() {
                continue;
            }

            let voyages_dir = epic_entry.path().join("voyages");
            if !voyages_dir.exists() {
                continue;
            }

            if let Ok(voyage_entries) = fs::read_dir(voyages_dir) {
                for voyage_entry in voyage_entries.flatten() {
                    let path = voyage_entry.path();
                    if !path.is_dir() {
                        continue;
                    }

                    let readme_path = path.join("README.md");
                    if !readme_path.exists() {
                        continue;
                    }

                    if let Ok(content) = fs::read_to_string(&readme_path)
                        && let Ok((fm, _)) = parse_frontmatter::<VoyageFrontmatter>(&content)
                    {
                        id_to_paths.entry(fm.id).or_default().push(readme_path);
                    }
                }
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
                            "duplicate voyage ID '{}' (also in: {})",
                            id,
                            other_paths
                                .iter()
                                .map(|p| p.display().to_string())
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )
                    .with_check_id(CheckId::Unknown),
                );
            }
        }
    }

    problems
}

/// Check voyage ID-directory consistency
pub fn check_voyage_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for voyage in board.voyages.values() {
        // Extract directory name from path: epics/{epic-id}/voyages/{dir-name}/README.md
        let dir_name = voyage
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str());

        let frontmatter_id = voyage.id();

        if let Some(dir) = dir_name
            && dir != frontmatter_id
        {
            let old_path = voyage.path.parent().unwrap().to_path_buf();
            let new_path = old_path.with_file_name(frontmatter_id);

            problems.push(Problem {
                severity: Severity::Error,
                path: voyage.path.clone(),
                message: format!(
                    "directory name '{}' differs from frontmatter id '{}'",
                    dir, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: Some(voyage.scope_path()),
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}
