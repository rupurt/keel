//! Centralized structural validation for stories, voyages, and epics.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;

use crate::domain::model::{EpicFrontmatter, StoryFrontmatter};
use crate::infrastructure::parser::parse_frontmatter;
use crate::infrastructure::validation::types::{CheckId, Fix, Problem, Severity};

/// Check for date field naming and type consistency.
///
/// Ensures all date/time fields end in `_at` and are valid `NaiveDateTime` strings.
pub fn check_date_consistency(path: &Path, check_id: CheckId) -> Vec<Problem> {
    let mut problems = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    if !content.starts_with("---") {
        return problems;
    }

    let fm_str = if let Some(end) = content[3..].find("---") {
        &content[3..3 + end]
    } else {
        return problems;
    };

    let fm_val: serde_yaml::Value = match serde_yaml::from_str(fm_str) {
        Ok(v) => v,
        Err(_) => return problems,
    };

    if let Some(mapping) = fm_val.as_mapping() {
        for (key, value) in mapping {
            let key_str = key.as_str().unwrap_or_default();

            // 1. Check for fields that SHOULD end in _at but don't
            let suspected_date_fields = [
                "date",
                "created",
                "completed",
                "updated",
                "started",
                "laid",
                "decided",
            ];
            if suspected_date_fields.contains(&key_str) {
                let severity = if check_id == CheckId::AdrDateConsistency {
                    Severity::Error
                } else {
                    Severity::Warning
                };

                problems.push(Problem {
                    severity,
                    path: path.to_path_buf(),
                    message: format!(
                        "Field '{}' should be renamed to '{}_at' for consistency",
                        key_str, key_str
                    ),
                    fix: None,
                    scope: None,
                    category: None,
                    check_id,
                });
            }

            // 2. Check fields ending in _at for datetime format
            if key_str.ends_with("_at")
                && let Some(val_str) = value.as_str()
            {
                // Check if it matches YYYY-MM-DDTHH:MM:SS
                let dt_format = "%Y-%m-%dT%H:%M:%S";

                if chrono::NaiveDateTime::parse_from_str(val_str, dt_format).is_err() {
                    problems.push(Problem {
                        severity: Severity::Error,
                        path: path.to_path_buf(),
                        message: format!(
                            "Field '{}' has invalid datetime format (expected YYYY-MM-DDTHH:MM:SS): {}",
                            key_str, val_str
                        ),
                        fix: None,
                        scope: None,
                        category: None,
                        check_id,
                    });
                }
            }
        }
    }

    problems
}

/// Scan story files for structural problems.
/// Returns (problems, file_count).
pub fn scan_story_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut id_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut file_count = 0;

    let stories_dir = board_dir.join("stories");
    if stories_dir.exists() {
        for entry in fs::read_dir(&stories_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let readme_path = path.join("README.md");
                if readme_path.exists() {
                    file_count += 1;
                    problems.extend(check_date_consistency(
                        &readme_path,
                        CheckId::StoryDateConsistency,
                    ));
                    if let Some(problem) = check_story_file(&readme_path) {
                        problems.push(problem);
                    } else {
                        // Extract ID for duplicate checking
                        if let Some(id) = extract_story_id_from_file(&readme_path) {
                            id_to_paths.entry(id).or_default().push(readme_path);
                        }
                    }
                }
            }
        }
    }

    // Check for duplicate IDs
    for (id, paths) in id_to_paths {
        if paths.len() > 1 {
            for path in &paths {
                let other_paths: Vec<_> = paths.iter().filter(|p| *p != path).collect();
                let other_bundles: Vec<_> = other_paths
                    .iter()
                    .filter_map(|p| {
                        p.parent()
                            .and_then(|p| p.file_name())
                            .and_then(|n| n.to_str())
                    })
                    .collect();

                problems.push(
                    Problem::error(
                        path.clone(),
                        format!(
                            "duplicate story ID '{}' (also in: {})",
                            id,
                            other_bundles.join(", ")
                        ),
                    )
                    .with_check_id(CheckId::StoryDuplicateId),
                );
            }
        }
    }

    Ok((problems, file_count))
}

/// Check if a string contains unfilled placeholders (TODO: or unreplaced {{tokens}}).
/// Ignores markers inside HTML comments.
pub fn is_placeholder_unfilled(content: &str) -> bool {
    let mut search_text = content.to_string();
    while let Some(start) = search_text.find("<!--") {
        if let Some(end) = search_text[start..].find("-->") {
            search_text.replace_range(start..start + end + 3, "");
        } else {
            break;
        }
    }

    // 1. Check for literal TODO:
    if search_text.contains("TODO:") {
        return true;
    }

    // 2. Check for unreplaced template tokens {{placeholder}}
    // Use a regex to find {{...}} that weren't substituted
    let token_re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
    token_re.is_match(&search_text)
}

/// Extract story ID from file by parsing frontmatter.
pub fn extract_story_id_from_file(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let result: Result<(StoryFrontmatter, &str), _> = parse_frontmatter(&content);
    result.ok().map(|(fm, _)| fm.id)
}

/// Check a single story file for problems.
pub fn check_story_file(path: &Path) -> Option<Problem> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return Some(
                Problem::error(path.to_path_buf(), format!("cannot read file: {}", e))
                    .with_check_id(CheckId::Unknown),
            );
        }
    };

    // Check for missing frontmatter
    if !content.starts_with("---") {
        return Some(
            Problem::error(
                path.to_path_buf(),
                "missing frontmatter (file doesn't start with ---)",
            )
            .with_check_id(CheckId::StoryMissingFrontmatter),
        );
    }

    // Try to parse frontmatter
    let result: Result<(StoryFrontmatter, &str), _> = parse_frontmatter(&content);

    match result {
        Err(e) => Some(
            Problem::error(path.to_path_buf(), format!("invalid YAML: {}", e))
                .with_check_id(CheckId::StoryInvalidYaml),
        ),
        Ok((fm, _)) => {
            // Check required fields
            if fm.id.is_empty() {
                return Some(
                    Problem::error(path.to_path_buf(), "missing required field: id")
                        .with_check_id(CheckId::StoryMissingId)
                        .with_scope(fm.scope.unwrap_or_default()),
                );
            }
            if fm.title.is_empty() {
                return Some(
                    Problem::error(path.to_path_buf(), "missing required field: title")
                        .with_check_id(CheckId::StoryMissingTitle)
                        .with_scope(fm.scope.unwrap_or_default()),
                );
            }
            None
        }
    }
}

/// Scan voyage files for structural problems.
/// Returns (problems, file_count).
pub fn scan_voyage_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut voyage_count = 0;
    let mut id_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Ok((problems, 0));
    }

    for epic_entry in fs::read_dir(epics_dir)? {
        let epic_entry = epic_entry?;
        if !epic_entry.path().is_dir() {
            continue;
        }

        let voyages_dir = epic_entry.path().join("voyages");
        if !voyages_dir.exists() {
            continue;
        }

        for voyage_entry in fs::read_dir(voyages_dir)? {
            let voyage_entry = voyage_entry?;
            let path = voyage_entry.path();
            if !path.is_dir() {
                continue;
            }

            voyage_count += 1;
            let dir_id = path.file_name().unwrap().to_string_lossy();

            // Extract ID from frontmatter for global uniqueness check
            let readme_path = path.join("README.md");
            if readme_path.exists() {
                problems.extend(check_date_consistency(
                    &readme_path,
                    CheckId::VoyageDateConsistency,
                ));
                if let Some(fm_id) = extract_voyage_id_from_file(&readme_path) {
                    id_to_paths.entry(fm_id).or_default().push(readme_path);
                }
            }

            // Check for required files and their content
            let required = [
                ("README.md", CheckId::VoyagesReadmeStructure),
                ("SRS.md", CheckId::VoyagesSrsExists),
                ("SDD.md", CheckId::VoyagesSddExists),
            ];
            for (file, check_id) in required {
                let file_path = path.join(file);
                if !file_path.exists() {
                    problems.push(
                        Problem::warning(
                            path.clone(),
                            format!("voyage {} missing standard file: {}", dir_id, file),
                        )
                        .with_check_id(check_id),
                    );
                } else {
                    // Call dedicated structure checks if file exists
                    match file {
                        "README.md" => problems.extend(check_voyage_readme_structure(&file_path)),
                        "SRS.md" => problems.extend(check_voyage_srs_structure(&file_path)),
                        "SDD.md" => problems.extend(check_voyage_sdd_structure(&file_path)),
                        _ => {}
                    }
                }
            }
        }
    }

    // Check for duplicate IDs across all epics
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

    Ok((problems, voyage_count))
}

/// Extract voyage ID from file by parsing frontmatter.
pub fn extract_voyage_id_from_file(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let result: Result<(crate::domain::model::VoyageFrontmatter, &str), _> =
        parse_frontmatter(&content);
    result.ok().map(|(fm, _)| fm.id)
}

/// Scan epic files for structural problems.
/// Returns (problems, epic_count).
pub fn scan_epic_files(board_dir: &Path) -> Result<(Vec<Problem>, usize)> {
    let mut problems = Vec::new();
    let mut epic_count = 0;
    let mut id_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let epics_dir = board_dir.join("epics");
    if !epics_dir.exists() {
        return Ok((problems, epic_count));
    }

    for entry in fs::read_dir(&epics_dir)? {
        let entry = entry?;
        if !entry.path().is_dir() {
            continue;
        }

        epic_count += 1;
        let readme_path = entry.path().join("README.md");
        if readme_path.exists() {
            problems.extend(check_date_consistency(
                &readme_path,
                CheckId::EpicDateConsistency,
            ));
            if let Some(problem) = check_epic_file(&readme_path) {
                problems.push(problem);
            } else if let Some(fm_id) = extract_epic_id_from_file(&readme_path) {
                id_to_paths
                    .entry(fm_id)
                    .or_default()
                    .push(readme_path.clone());
            }
            problems.extend(check_epic_readme_structure(&readme_path));
        } else {
            problems.push(
                Problem::warning(readme_path, "missing README.md (required epic frontmatter)")
                    .with_check_id(CheckId::EpicMissingReadme),
            );
        }

        let prd_path = entry.path().join("PRD.md");
        if prd_path.exists() {
            problems.extend(check_epic_prd_structure(&prd_path));
        } else {
            problems.push(
                Problem::warning(prd_path, "missing PRD.md (Product Requirements Document)")
                    .with_check_id(CheckId::EpicMissingPrd),
            );
        }
    }

    // Check for duplicate IDs
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
                    .with_check_id(CheckId::Unknown),
                );
            }
        }
    }

    Ok((problems, epic_count))
}

/// Extract epic ID from file by parsing frontmatter.
pub fn extract_epic_id_from_file(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    let result: Result<(EpicFrontmatter, &str), _> = parse_frontmatter(&content);
    result.ok().map(|(fm, _)| fm.id)
}

/// Check a single epic file for problems.
pub fn check_epic_file(path: &Path) -> Option<Problem> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            return Some(
                Problem::error(path.to_path_buf(), format!("cannot read file: {}", e))
                    .with_check_id(CheckId::Unknown),
            );
        }
    };

    if !content.starts_with("---") {
        return Some(
            Problem::error(
                path.to_path_buf(),
                "missing frontmatter (file doesn't start with ---)",
            )
            .with_check_id(CheckId::EpicInvalidFrontmatter),
        );
    }

    let result: Result<(EpicFrontmatter, &str), _> = parse_frontmatter(&content);

    match result {
        Err(e) => Some(
            Problem::error(path.to_path_buf(), format!("invalid YAML: {}", e))
                .with_check_id(CheckId::EpicInvalidFrontmatter),
        ),
        Ok((fm, _)) => {
            if fm.id.is_empty() {
                return Some(
                    Problem::error(path.to_path_buf(), "missing required field: id")
                        .with_check_id(CheckId::EpicInvalidFrontmatter),
                );
            }
            if fm.title.is_empty() {
                return Some(
                    Problem::error(path.to_path_buf(), "missing required field: title")
                        .with_check_id(CheckId::EpicInvalidFrontmatter),
                );
            }
            None
        }
    }
}

/// Check epic README structure for rendering issues.
pub fn check_epic_readme_structure(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    let has_begin_marker = content.contains("<!-- BEGIN GENERATED -->");
    let has_end_marker = content.contains("<!-- END GENERATED -->");

    if !has_begin_marker || !has_end_marker {
        problems.push(
            Problem::warning(path.to_path_buf(), "missing generated section markers")
                .with_check_id(CheckId::Unknown),
        );
        return problems;
    }

    // Check for broken table structure
    if let Some(begin_idx) = content.find("<!-- BEGIN GENERATED -->") {
        let after_marker = &content[begin_idx + "<!-- BEGIN GENERATED -->".len()..];
        let table_header_before = content[..begin_idx].contains("|-----------|");
        let table_header_inside = after_marker.contains("|-----------|");

        if table_header_before
            && !table_header_inside
            && let Some(first_line) = after_marker.trim_start().lines().next()
        {
            let first_line = first_line.trim();
            let is_table_row = first_line.starts_with('|')
                && first_line.contains('[')
                && first_line.ends_with('|');

            if !is_table_row {
                problems.push(
                    Problem::warning(path.to_path_buf(), "broken table structure")
                        .with_check_id(CheckId::Unknown),
                );
            }
        }
    }

    if content.contains("> TODO: Value proposition") {
        problems.push(
            Problem::warning(path.to_path_buf(), "goal is unfilled")
                .with_check_id(CheckId::Unknown)
                .with_fix(Fix::ClearPlaceholder {
                    path: path.to_path_buf(),
                    pattern: "> TODO: Value proposition".to_string(),
                }),
        );
    } else if is_placeholder_unfilled(&content) {
        let token_re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let pattern = token_re
            .find(&content)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "TODO:".to_string());

        problems.push(
            Problem::warning(path.to_path_buf(), "contains unfilled TODO placeholder")
                .with_check_id(CheckId::Unknown)
                .with_fix(Fix::ClearPlaceholder {
                    path: path.to_path_buf(),
                    pattern,
                }),
        );
    }

    problems
}

/// Check voyage README structure.
pub fn check_voyage_readme_structure(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    if is_placeholder_unfilled(&content) {
        let token_re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let pattern = token_re
            .find(&content)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "TODO:".to_string());

        problems.push(
            Problem::warning(
                path.to_path_buf(),
                "README contains unfilled TODO placeholder",
            )
            .with_check_id(CheckId::VoyagesReadmeStructure)
            .with_fix(Fix::ClearPlaceholder {
                path: path.to_path_buf(),
                pattern,
            }),
        );
    }

    if !content.contains("<!-- BEGIN DOCUMENTS -->") || !content.contains("<!-- END DOCUMENTS -->")
    {
        problems.push(
            Problem::warning(
                path.to_path_buf(),
                "README missing standard Documents section markers",
            )
            .with_check_id(CheckId::VoyagesReadmeStructure),
        );
    }

    if !content.contains("<!-- BEGIN GENERATED -->") || !content.contains("<!-- END GENERATED -->")
    {
        problems.push(
            Problem::warning(
                path.to_path_buf(),
                "README missing generated section markers",
            )
            .with_check_id(CheckId::VoyagesReadmeStructure),
        );
    }

    problems
}

/// Check voyage SRS structure.
pub fn check_voyage_srs_structure(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    if is_placeholder_unfilled(&content) {
        let token_re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let pattern = token_re
            .find(&content)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "TODO:".to_string());

        problems.push(
            Problem::warning(path.to_path_buf(), "SRS contains unfilled TODO placeholder")
                .with_check_id(CheckId::VoyagesSrsExists)
                .with_fix(Fix::ClearPlaceholder {
                    path: path.to_path_buf(),
                    pattern,
                }),
        );
    }

    let sections = ["FUNCTIONAL_REQUIREMENTS", "NON_FUNCTIONAL_REQUIREMENTS"];
    for section in sections {
        let begin = format!("<!-- BEGIN {} -->", section);
        let end = format!("<!-- END {} -->", section);
        if !content.contains(&begin) || !content.contains(&end) {
            problems.push(
                Problem::warning(
                    path.to_path_buf(),
                    format!("SRS missing {} markers", section),
                )
                .with_check_id(CheckId::VoyagesSrsExists),
            );
        }
    }

    problems
}

/// Check voyage SDD structure.
pub fn check_voyage_sdd_structure(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    if is_placeholder_unfilled(&content) {
        let token_re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let pattern = token_re
            .find(&content)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "TODO:".to_string());

        problems.push(
            Problem::warning(path.to_path_buf(), "SDD contains unfilled TODO placeholder")
                .with_check_id(CheckId::VoyagesSddExists)
                .with_fix(Fix::ClearPlaceholder {
                    path: path.to_path_buf(),
                    pattern,
                }),
        );
    }

    problems
}

/// Check epic PRD.md structure for section markers and content.
pub fn check_epic_prd_structure(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    if is_placeholder_unfilled(&content) {
        let token_re = regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap();
        let pattern = token_re
            .find(&content)
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "TODO:".to_string());

        problems.push(
            Problem::warning(path.to_path_buf(), "PRD contains unfilled TODO placeholder")
                .with_check_id(CheckId::Unknown)
                .with_fix(Fix::ClearPlaceholder {
                    path: path.to_path_buf(),
                    pattern,
                }),
        );
    }

    let sections = [
        ("FUNCTIONAL_REQUIREMENTS", true),
        ("NON_FUNCTIONAL_REQUIREMENTS", true),
        ("SUCCESS_CRITERIA", false),
    ];

    for (section_name, is_table) in sections {
        let begin_marker = format!("<!-- BEGIN {} -->", section_name);
        let end_marker = format!("<!-- END {} -->", section_name);

        if !content.contains(&begin_marker) || !content.contains(&end_marker) {
            problems.push(
                Problem::warning(
                    path.to_path_buf(),
                    format!("PRD missing {} markers", section_name),
                )
                .with_check_id(CheckId::Unknown),
            );
            continue;
        }

        if let Some(begin_idx) = content.find(&begin_marker) {
            let after_begin = &content[begin_idx + begin_marker.len()..];
            if let Some(end_idx) = after_begin.find(&end_marker) {
                let section_content = after_begin[..end_idx].trim();

                if is_table {
                    let has_data_row = section_content.lines().any(|line| {
                        let trimmed = line.trim();
                        trimmed.starts_with("| FR-")
                            || trimmed.starts_with("|FR-")
                            || trimmed.starts_with("| NFR-")
                            || trimmed.starts_with("|NFR-")
                    });

                    if !has_data_row {
                        problems.push(
                            Problem::warning(
                                path.to_path_buf(),
                                format!("PRD {} section is empty", section_name),
                            )
                            .with_check_id(CheckId::Unknown),
                        );
                    }
                } else {
                    let has_checkbox = section_content
                        .lines()
                        .any(|line| line.trim().starts_with("- ["));

                    if !has_checkbox {
                        problems.push(
                            Problem::warning(
                                path.to_path_buf(),
                                format!("PRD {} section is empty", section_name),
                            )
                            .with_check_id(CheckId::Unknown),
                        );
                    }
                }
            }
        }
    }

    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_is_placeholder_unfilled() {
        assert!(is_placeholder_unfilled("TODO: Fix this"));
        assert!(is_placeholder_unfilled("Check this {{placeholder}}"));
        assert!(!is_placeholder_unfilled("No markers here"));
        assert!(!is_placeholder_unfilled("<!-- TODO: in comment -->"));
        assert!(!is_placeholder_unfilled("<!-- {{token}} in comment -->"));
        assert!(is_placeholder_unfilled("TODO: and <!-- comment -->"));
    }

    #[test]
    fn test_is_placeholder_unfilled_mixed() {
        let content = "Actual content <!-- comment --> and a TODO: here";
        assert!(is_placeholder_unfilled(content));

        let content2 = "Actual content <!-- TODO: comment --> and no more";
        assert!(!is_placeholder_unfilled(content2));
    }

    #[test]
    fn test_scan_story_files_empty() {
        let temp = TempDir::new().unwrap();
        let (problems, count) = scan_story_files(temp.path()).unwrap();
        assert_eq!(count, 0);
        assert!(problems.is_empty());
    }

    #[test]
    fn test_check_story_file_missing_frontmatter() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("README.md");
        fs::write(&path, "# Just Markdown").unwrap();

        let problem = check_story_file(&path).unwrap();
        assert_eq!(problem.check_id, CheckId::StoryMissingFrontmatter);
    }

    #[test]
    fn test_check_story_file_invalid_yaml() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("README.md");
        fs::write(&path, "---\ninvalid: yaml: : extra\n---").unwrap();

        let problem = check_story_file(&path).unwrap();
        assert_eq!(problem.check_id, CheckId::StoryInvalidYaml);
    }

    #[test]
    fn test_check_voyage_readme_structure() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("README.md");
        fs::write(
            &path,
            "---\nid: V1\ntitle: T1\n---\n# Voyage\nTODO: fill this",
        )
        .unwrap();

        let problems = check_voyage_readme_structure(&path);
        assert!(!problems.is_empty());
        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("TODO placeholder"))
        );
    }

    #[test]
    fn test_check_epic_prd_structure_empty_sections() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("PRD.md");
        fs::write(
            &path,
            r#"# PRD
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
<!-- END FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN SUCCESS_CRITERIA -->
<!-- END SUCCESS_CRITERIA -->
"#,
        )
        .unwrap();

        let problems = check_epic_prd_structure(&path);
        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("section is empty"))
        );
    }

    #[test]
    fn test_check_epic_prd_structure_missing_markers() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("PRD.md");
        fs::write(&path, "# PRD\nNo markers here").unwrap();

        let problems = check_epic_prd_structure(&path);
        assert!(problems.iter().any(|p| p.message.contains("missing")));
    }
}
