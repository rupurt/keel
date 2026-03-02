//! Logic for applying automated fixes

use super::types::DoctorReport;
use crate::infrastructure::validation::Fix;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};

/// Apply all fixable problems found in the report
pub fn run_fixes(_board_dir: &Path, report: &DoctorReport) -> Result<()> {
    let sections = vec![
        &report.story_checks,
        &report.voyage_checks,
        &report.epic_checks,
        &report.adr_checks,
        &report.bearing_checks,
    ];

    for results in sections {
        for check in results {
            for problem in &check.problems {
                if let Some(fix) = &problem.fix {
                    apply_fix(fix)?;
                }
            }
        }
    }

    Ok(())
}

/// Apply a single fix
fn apply_fix(fix: &Fix) -> Result<()> {
    match fix {
        Fix::UpdateTitle { path, new_title } => {
            update_frontmatter_field(path, "title", new_title)?;
            println!("  FIX: Updated title in {}", path.display());
        }
        Fix::RemoveFile { path } => {
            if path.exists() {
                fs::remove_file(path)?;
                println!("  FIX: Removed {}", path.display());
            }
        }
        Fix::RenameFile { old_path, new_path } => {
            if old_path.exists() {
                fs::rename(old_path, new_path)?;
                println!(
                    "  FIX: Renamed {} to {}",
                    old_path.display(),
                    new_path.display()
                );
            }
        }
        Fix::UpdateFrontmatterId { path, new_id } => {
            update_frontmatter_field(path, "id", new_id)?;
            println!("  FIX: Updated id in {}", path.display());
        }
        Fix::ClearPlaceholder { path, pattern } => {
            let content = fs::read_to_string(path)?;
            let new_content = content.replace(pattern, "");
            fs::write(path, new_content)?;
            println!(
                "  FIX: Cleared placeholder '{}' in {}",
                pattern,
                path.display()
            );
        }
        Fix::UpdateEpicStatus { path, new_status } => {
            update_frontmatter_field(path, "status", new_status)?;
            println!("  FIX: Updated epic status in {}", path.display());
        }
        _ => {
            // Other fixes can be implemented as needed
        }
    }
    Ok(())
}

/// Helper to update a single field in YAML frontmatter
fn update_frontmatter_field(path: &Path, field: &str, value: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    if lines.is_empty() || lines[0] != "---" {
        return Err(anyhow!("File {} does not have frontmatter", path.display()));
    }

    let mut in_frontmatter = false;
    let mut updated = false;
    let mut new_lines = lines.clone();

    for line in new_lines.iter_mut() {
        if line == "---" {
            if in_frontmatter {
                break; // End of frontmatter
            }
            in_frontmatter = true;
            continue;
        }

        if in_frontmatter && line.trim_start().starts_with(&format!("{}:", field)) {
            *line = format!("{}: {}", field, value);
            updated = true;
            break;
        }
    }

    if updated {
        let mut final_content = new_lines.join("\n");
        if content.ends_with('\n') {
            final_content.push('\n');
        }
        fs::write(path, final_content)?;
        Ok(())
    } else {
        Err(anyhow!(
            "Field '{}' not found in frontmatter of {}",
            field,
            path.display()
        ))
    }
}

pub fn is_date_only_format(s: &str) -> bool {
    let parts: Vec<&str> = s.splitn(2, ':').collect();
    if parts.len() < 2 {
        return false;
    }
    let val = parts[1].trim();
    // Simple check: "2024-01-01" vs "2024-01-01T12:00:00"
    val.len() == 10 && val.chars().all(|c| c.is_ascii_digit() || c == '-')
}

pub fn extract_grep_command(_s: &str) -> Option<String> {
    None
}

pub struct TestIndex;

impl TestIndex {
    pub fn build(_names: Vec<String>) -> Self {
        Self
    }

    pub fn is_empty(&self) -> bool {
        true
    }

    pub fn find_match(&self, _criterion: &str) -> Option<String> {
        None
    }
}
