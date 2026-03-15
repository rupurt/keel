//! Automated fixes for board health problems

use crate::infrastructure::loader::load_board;
use crate::infrastructure::validation::Fix;
use crate::read_model::diagnostics::DoctorReport;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

/// Run automated fixes from a doctor report
pub fn run_fixes(board_dir: &Path, report: &DoctorReport) -> Result<()> {
    let board = load_board(board_dir)?;

    let all_checks = [
        &report.story_checks,
        &report.voyage_checks,
        &report.epic_checks,
        &report.adr_checks,
        &report.bearing_checks,
        &report.mission_checks,
        &report.routine_checks,
        &report.workflow_checks,
    ];

    for checks in all_checks {
        for check in checks {
            for problem in &check.problems {
                if let Some(fix) = &problem.fix {
                    apply_fix(board_dir, &board, fix)?;
                }
            }
        }
    }

    Ok(())
}

fn apply_fix(_board_dir: &Path, _board: &crate::domain::model::Board, fix: &Fix) -> Result<()> {
    match fix {
        Fix::RenameFile { old_path, new_path } => {
            println!("Renaming {} -> {}", old_path.display(), new_path.display());
            fs::rename(old_path, new_path).context("rename failed")?;
        }
        Fix::SetFrontmatterField { path, field, value } => {
            println!("Setting {} in {}", field, path.display());
            let content = fs::read_to_string(path)?;
            let updated = crate::infrastructure::frontmatter_mutation::apply(
                &content,
                &[crate::infrastructure::frontmatter_mutation::Mutation::set(
                    field,
                    value.clone(),
                )],
            );
            fs::write(path, updated)?;
        }
        _ => {
            // TODO: Implement other fix types as needed
        }
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct TestIndex {
    pub test_names: Vec<String>,
}

impl TestIndex {
    pub fn build(test_names: Vec<String>) -> Self {
        Self { test_names }
    }

    pub fn is_empty(&self) -> bool {
        self.test_names.is_empty()
    }

    pub fn find_match(&self, _criterion: &str) -> Option<String> {
        // Basic match for now
        None
    }
}

pub fn extract_grep_command(text: &str) -> Option<String> {
    if text.contains("grep") {
        Some("grep".to_string())
    } else {
        None
    }
}

pub fn is_date_only_format(s: &str) -> bool {
    let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    re.is_match(s)
}
