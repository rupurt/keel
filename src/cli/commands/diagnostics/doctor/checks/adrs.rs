use std::fs;
use std::path::Path;

use anyhow::Result;

use super::super::types::*;
use crate::domain::model::{Adr, AdrStatus, Board, VoyageState};
use crate::infrastructure::parser::parse_frontmatter;

/// Scan ADR files for structural problems
/// Returns (problems, file_count)
pub fn scan_adr_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut adr_count = 0;

    let adrs_dir = board_dir.join("adrs");
    if !adrs_dir.exists() {
        return Ok((problems, adr_count));
    }

    // Find all ADR markdown files
    for entry in fs::read_dir(&adrs_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "md") {
            adr_count += 1;
            if let Some(problem) = check_adr_file(&path) {
                problems.push(problem);
            }
        }
    }

    Ok((problems, adr_count))
}

/// Check a single ADR file for problems
pub fn check_adr_file(path: &Path) -> Option<Problem> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return Some(Problem {
                severity: Severity::Error,
                path: path.to_path_buf(),
                message: format!("cannot read file: {}", e),
                fix: None,
                scope: None,
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    };

    // Check for missing frontmatter
    if !content.starts_with("---") {
        return Some(Problem {
            severity: Severity::Error,
            path: path.to_path_buf(),
            message: "missing frontmatter (file doesn't start with ---)".to_string(),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        });
    }

    // Try to parse frontmatter
    let result: Result<(crate::domain::model::AdrFrontmatter, &str), _> =
        parse_frontmatter(&content);

    match result {
        Err(e) => Some(Problem {
            severity: Severity::Error,
            path: path.to_path_buf(),
            message: format!("invalid YAML: {}", e),
            fix: None,
            scope: None,
            category: None,
            check_id: CheckId::Unknown,
        }),
        Ok((fm, _)) => {
            // Check required fields
            if fm.id.is_empty() {
                return Some(Problem {
                    severity: Severity::Error,
                    path: path.to_path_buf(),
                    message: "missing required field: id".to_string(),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
            if fm.title.is_empty() {
                return Some(Problem {
                    severity: Severity::Error,
                    path: path.to_path_buf(),
                    message: "missing required field: title".to_string(),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
            None
        }
    }
}

/// Check ADR date field consistency
pub fn check_adr_dates(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for adr in board.adrs.values() {
        problems.extend(
            crate::infrastructure::validation::structural::check_date_consistency(
                &adr.path,
                CheckId::AdrDateConsistency,
            ),
        );
    }

    problems
}

/// Check ADR status values are known
/// Check for proposed ADRs governing active contexts (SRS-05)
/// Warns when a proposed ADR exists that governs an active voyage's context
pub fn check_proposed_adr_warning(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    // Find all proposed ADRs
    let proposed_adrs: Vec<&Adr> = board
        .adrs
        .values()
        .filter(|adr| adr.frontmatter.status == AdrStatus::Proposed)
        .collect();

    if proposed_adrs.is_empty() {
        return problems;
    }

    // Find active contexts (in-progress voyages)
    let active_contexts: Vec<&str> = board
        .voyages
        .values()
        .filter(|v| v.frontmatter.status == VoyageState::InProgress)
        .map(|v| v.epic_id.as_str())
        .collect();

    // Check if any proposed ADR governs an active context
    for adr in proposed_adrs {
        let adr_context = adr.frontmatter.context.as_deref();
        let applies_to = &adr.frontmatter.applies_to;

        // Check if ADR context matches any active epic or applies to "all"
        let governs_active = adr_context
            .map(|ctx| active_contexts.contains(&ctx))
            .unwrap_or(false)
            || applies_to.iter().any(|scope| {
                scope == "all" || active_contexts.iter().any(|ctx| ctx.contains(scope))
            });

        if governs_active {
            problems.push(Problem {
                severity: Severity::Warning,
                path: adr.frontmatter.id.clone().into(),
                message: format!(
                    "proposed ADR '{}' ({}) governs active context: {}",
                    adr.frontmatter.id,
                    adr.frontmatter.title,
                    adr_context.unwrap_or("all")
                ),
                fix: None,
                scope: adr_context.map(|s| s.to_string()),
                category: None,
                check_id: CheckId::Unknown,
            });
        }
    }

    problems
}

/// Check ADR title case
pub fn check_adr_title_case(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for adr in board.adrs.values() {
        let title = &adr.frontmatter.title;
        if !crate::infrastructure::utils::is_title_case(title) {
            let new_title = crate::infrastructure::utils::to_title_case(title);
            problems.push(Problem {
                severity: Severity::Warning,
                path: adr.path.clone(),
                message: format!("title '{}' should use Title Case", title),
                fix: Some(Fix::UpdateTitle {
                    path: adr.path.clone(),
                    new_title,
                }),
                scope: adr.frontmatter.context.clone(),
                category: Some(GapCategory::Convention),
                check_id: CheckId::TitleCaseViolation,
            });
        }
    }

    problems
}

/// Required sections in ADR markdown
pub const ADR_REQUIRED_SECTIONS: &[&str] = &[
    "## Status",
    "## Context",
    "## Decision",
    "## Constraints",
    "## Consequences",
    "## Verification",
];

/// Check ADR content sections
pub fn check_adr_content_sections(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for adr in board.adrs.values() {
        let content = match fs::read_to_string(&adr.path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for section in ADR_REQUIRED_SECTIONS {
            if !content.contains(section) {
                let section_name = section.trim_start_matches("## ");
                problems.push(Problem {
                    severity: Severity::Warning,
                    path: adr.path.clone(),
                    message: format!(
                        "ADR '{}' is missing required section: {}",
                        adr.id(),
                        section_name
                    ),
                    fix: None,
                    scope: adr.frontmatter.context.clone(),
                    category: None,
                    check_id: CheckId::Unknown,
                });
            }
        }
    }

    problems
}

/// Check ADR ID-filename consistency
pub fn check_adr_id_consistency(board: &Board) -> Vec<Problem> {
    let mut problems = Vec::new();

    for adr in board.adrs.values() {
        let filename = adr
            .path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        let frontmatter_id = adr.id();

        if !filename.starts_with(frontmatter_id) {
            let old_path = adr.path.clone();
            let mut new_filename = frontmatter_id.to_string();
            // Try to keep the slug if present
            if let Some(rest) = filename.strip_prefix(frontmatter_id) {
                new_filename.push_str(rest);
            } else if let Some(first_dash) = filename.find('-') {
                new_filename.push_str(&filename[first_dash..]);
            } else {
                // If it's totally different, just use ID + slugified title
                new_filename.push('-');
                new_filename.push_str(&crate::infrastructure::utils::slugify(adr.title()));
                new_filename.push_str(".md");
            }

            let new_path = old_path.with_file_name(new_filename);

            problems.push(Problem {
                severity: Severity::Error,
                path: adr.path.clone(),
                message: format!(
                    "filename '{}' does not start with frontmatter id '{}'",
                    filename, frontmatter_id
                ),
                fix: Some(Fix::RenameFile { old_path, new_path }),
                scope: adr.frontmatter.context.clone(),
                category: None,
                check_id: CheckId::IdInconsistency,
            });
        }
    }

    problems
}

/// Check for duplicate ADR IDs
pub fn check_adr_duplicates(board_dir: &Path) -> Vec<Problem> {
    use crate::domain::model::AdrFrontmatter;
    use crate::infrastructure::parser::parse_frontmatter;
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;

    let mut problems = Vec::new();
    let mut id_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let adrs_dir = board_dir.join("adrs");
    if !adrs_dir.exists() {
        return problems;
    }

    if let Ok(entries) = fs::read_dir(adrs_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|e| e == "md")
                && let Ok(content) = fs::read_to_string(&path)
                && let Ok((fm, _)) = parse_frontmatter::<AdrFrontmatter>(&content)
            {
                id_to_paths.entry(fm.id).or_default().push(path);
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
                            "duplicate ADR ID '{}' (also in: {})",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestAdr, TestBoardBuilder};

    #[test]
    fn test_scan_adr_files_empty() {
        let temp = TestBoardBuilder::new().build();
        let (problems, count) = scan_adr_files(temp.path()).unwrap();
        assert_eq!(count, 0);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_scan_adr_files_valid() {
        let temp = TestBoardBuilder::new().adr(TestAdr::new("ADR-001")).build();
        let (problems, count) = scan_adr_files(temp.path()).unwrap();
        assert_eq!(count, 1);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_check_adr_file_valid() {
        let temp = TestBoardBuilder::new().adr(TestAdr::new("ADR-001")).build();
        let path = temp.path().join("adrs/ADR-001-test-adr.md");
        let problem = check_adr_file(&path);
        assert!(problem.is_none());
    }

    #[test]
    fn test_check_proposed_adr_warning() {
        let temp = TestBoardBuilder::new()
            .epic(crate::test_helpers::TestEpic::new("e1"))
            .voyage(crate::test_helpers::TestVoyage::new("v1", "e1").status("in-progress"))
            .adr(TestAdr::new("ADR-001").status("proposed").context("e1"))
            .build();

        let board = load_board(temp.path()).unwrap();
        let problems = check_proposed_adr_warning(&board);
        assert!(!problems.is_empty());
        assert!(problems[0].message.contains("governs active context"));
    }
}
