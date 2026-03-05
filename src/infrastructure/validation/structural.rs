//! Centralized structural validation for stories, voyages, and epics.

use std::fs;
use std::path::Path;
use std::sync::LazyLock;

use anyhow::Result;

use crate::domain::model::{EpicFrontmatter, StoryFrontmatter};
use crate::infrastructure::parser::parse_frontmatter;
use crate::infrastructure::validation::types::{CheckId, Fix, Problem, Severity};

static TEMPLATE_TOKEN_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"\{\{([^}]+)\}\}").unwrap());
static LEGACY_SCAFFOLD_MARKERS: &[&str] = &["Define acceptance criteria for this slice"];
static EPIC_PRD_DEFAULT_ROW_MARKERS: &[&str] = &[
    "Deliver the primary user workflow for this epic end-to-end.",
    "Maintain reliability and observability for all new workflow paths introduced by this epic.",
    "Users can complete the primary workflow described in this PRD without manual intervention.",
];

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
                    }
                }
            }
        }
    }

    problems.extend(crate::infrastructure::duplicate_ids::duplicate_id_problems(
        board_dir,
        crate::infrastructure::duplicate_ids::DuplicateEntity::Story,
    ));

    Ok((problems, file_count))
}

/// Check if a string contains unfilled placeholders (TODO: or unreplaced {{tokens}}).
/// Ignores markers inside HTML comments.
pub fn is_placeholder_unfilled(content: &str) -> bool {
    first_unfilled_placeholder_pattern(content).is_some()
}

fn strip_html_comments(content: &str) -> String {
    let mut search_text = content.to_string();
    while let Some(start) = search_text.find("<!--") {
        if let Some(end) = search_text[start..].find("-->") {
            search_text.replace_range(start..start + end + 3, "");
        } else {
            break;
        }
    }

    search_text
}

/// Return the first unresolved scaffold/default marker outside comments.
///
/// Markers are either:
/// - literal `TODO:`
/// - known scaffold defaults from older templates
/// - unresolved `{{token}}` placeholders
pub fn first_unfilled_placeholder_pattern(content: &str) -> Option<String> {
    let search_text = strip_html_comments(content);

    if search_text.contains("TODO:") {
        return Some("TODO:".to_string());
    }

    for marker in LEGACY_SCAFFOLD_MARKERS {
        if search_text.contains(marker) {
            return Some((*marker).to_string());
        }
    }

    TEMPLATE_TOKEN_RE
        .find(&search_text)
        .map(|marker| marker.as_str().to_string())
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
            problems.extend(check_epic_prd_authored_content(&prd_path));
        } else {
            problems.push(
                Problem::warning(prd_path, "missing PRD.md (Product Requirements Document)")
                    .with_check_id(CheckId::EpicMissingPrd),
            );
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
            Problem::error(
                path.to_path_buf(),
                "goal is unfilled (pattern: > TODO: Value proposition)",
            )
            .with_check_id(CheckId::Unknown)
            .with_fix(Fix::ClearPlaceholder {
                path: path.to_path_buf(),
                pattern: "> TODO: Value proposition".to_string(),
            }),
        );
    } else if let Some(pattern) = first_unfilled_placeholder_pattern(&content) {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                format!(
                    "contains unresolved scaffold/default text (pattern: {})",
                    pattern
                ),
            )
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

    if let Some(pattern) = first_unfilled_placeholder_pattern(&content) {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                format!(
                    "README contains unresolved scaffold/default text (pattern: {})",
                    pattern
                ),
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

    if let Some(pattern) = first_unfilled_placeholder_pattern(&content) {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                format!(
                    "SRS contains unresolved scaffold/default text (pattern: {})",
                    pattern
                ),
            )
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

    if let Some(pattern) = first_unfilled_placeholder_pattern(&content) {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                format!(
                    "SDD contains unresolved scaffold/default text (pattern: {})",
                    pattern
                ),
            )
            .with_check_id(CheckId::VoyagesSddExists)
            .with_fix(Fix::ClearPlaceholder {
                path: path.to_path_buf(),
                pattern,
            }),
        );
    }

    problems
}

/// Check epic PRD.md structure for scaffold markers and required marker pairs.
pub fn check_epic_prd_structure(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    if let Some(pattern) = first_unfilled_placeholder_pattern(&content) {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                format!(
                    "PRD contains unresolved scaffold/default text (pattern: {})",
                    pattern
                ),
            )
            .with_check_id(CheckId::Unknown)
            .with_fix(Fix::ClearPlaceholder {
                path: path.to_path_buf(),
                pattern,
            }),
        );
    }

    let sections = [
        "FUNCTIONAL_REQUIREMENTS",
        "NON_FUNCTIONAL_REQUIREMENTS",
        "SUCCESS_CRITERIA",
    ];

    for section_name in sections {
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
        }
    }

    problems
}

/// Check epic PRD.md authored content completeness.
///
/// This is stricter than `check_epic_prd_structure` and is used for transition
/// gates and doctor checks where scaffold-only sections must fail.
pub fn check_epic_prd_authored_content(path: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();

    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return problems,
    };

    let has_problem_statement = extract_markdown_section(&content, "## Problem Statement")
        .as_deref()
        .is_some_and(section_has_authored_paragraph);
    if !has_problem_statement {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Problem Statement' must include authored narrative content",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let has_goals = extract_markdown_section(&content, "## Goals & Objectives")
        .as_deref()
        .is_some_and(section_has_authored_table_or_bullets);
    if !has_goals {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Goals & Objectives' must include at least one authored goal",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let has_users = extract_markdown_section(&content, "## Users")
        .as_deref()
        .is_some_and(section_has_authored_table_row);
    if !has_users {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Users' must include at least one authored persona row",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let scope = extract_markdown_section(&content, "## Scope");
    let (in_scope_count, out_scope_count) =
        scope.as_deref().map(parse_scope_bullets).unwrap_or((0, 0));
    if in_scope_count == 0 {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Scope' must include at least one 'In Scope' bullet",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }
    if out_scope_count == 0 {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Scope' must include at least one 'Out of Scope' bullet",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let functional_block = extract_marker_block(&content, "FUNCTIONAL_REQUIREMENTS");
    if functional_block
        .as_deref()
        .is_none_or(|section| !section_has_authored_table_row(section))
    {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Functional Requirements' must include at least one authored requirement row",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let non_functional_block = extract_marker_block(&content, "NON_FUNCTIONAL_REQUIREMENTS");
    if non_functional_block
        .as_deref()
        .is_none_or(|section| !section_has_authored_table_row(section))
    {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Non-Functional Requirements' must include at least one authored requirement row",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let has_verification_strategy = extract_markdown_section(&content, "## Verification Strategy")
        .as_deref()
        .is_some_and(section_has_authored_text_or_list_or_table);
    if !has_verification_strategy {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Verification Strategy' must include authored verification approach",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let has_assumptions = extract_markdown_section(&content, "## Assumptions")
        .as_deref()
        .is_some_and(section_has_authored_table_row);
    if !has_assumptions {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Assumptions' must include at least one authored assumption row",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let has_open_questions = extract_markdown_section(&content, "## Open Questions & Risks")
        .as_deref()
        .is_some_and(section_has_authored_table_row);
    if !has_open_questions {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Open Questions & Risks' must include at least one authored row",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    let success_criteria_block = extract_marker_block(&content, "SUCCESS_CRITERIA");
    if success_criteria_block
        .as_deref()
        .is_none_or(|section| !section_has_authored_checkbox(section))
    {
        problems.push(
            Problem::error(
                path.to_path_buf(),
                "PRD section 'Success Criteria' must include at least one authored checkbox item",
            )
            .with_check_id(CheckId::EpicPrdAuthoredContent),
        );
    }

    problems
}

fn extract_markdown_section(content: &str, heading: &str) -> Option<String> {
    let mut in_section = false;
    let mut result = String::new();
    let heading_level = heading.chars().take_while(|ch| *ch == '#').count();

    for line in content.lines() {
        if line.trim() == heading {
            in_section = true;
            continue;
        }
        if in_section {
            if line.starts_with('#') {
                let level = line.chars().take_while(|ch| *ch == '#').count();
                if level <= heading_level {
                    break;
                }
            }
            result.push_str(line);
            result.push('\n');
        }
    }

    if result.trim().is_empty() {
        None
    } else {
        Some(result)
    }
}

fn extract_marker_block(content: &str, marker_name: &str) -> Option<String> {
    let begin_marker = format!("<!-- BEGIN {marker_name} -->");
    let end_marker = format!("<!-- END {marker_name} -->");

    let begin_idx = content.find(&begin_marker)?;
    let after_begin = &content[begin_idx + begin_marker.len()..];
    let end_idx = after_begin.find(&end_marker)?;
    Some(after_begin[..end_idx].to_string())
}

fn section_has_authored_paragraph(section: &str) -> bool {
    section.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.is_empty()
            && !trimmed.starts_with("<!--")
            && !trimmed.starts_with('#')
            && !trimmed.starts_with('|')
            && !trimmed.starts_with("- ")
    })
}

fn section_has_authored_table_or_bullets(section: &str) -> bool {
    section_has_authored_table_row(section) || section_has_authored_bullets(section)
}

fn section_has_authored_text_or_list_or_table(section: &str) -> bool {
    section_has_authored_paragraph(section)
        || section_has_authored_bullets(section)
        || section_has_authored_table_row(section)
}

fn section_has_authored_bullets(section: &str) -> bool {
    section.lines().any(|line| {
        let trimmed = line.trim();
        let Some(item) = trimmed.strip_prefix("- ") else {
            return false;
        };
        let item = item.trim();
        !item.is_empty() && !is_default_prd_scaffold_row(item)
    })
}

fn section_has_authored_table_row(section: &str) -> bool {
    section.lines().any(|line| {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            return false;
        }

        let cells: Vec<&str> = trimmed
            .split('|')
            .map(str::trim)
            .filter(|cell| !cell.is_empty())
            .collect();
        if cells.is_empty() || is_table_separator_row(&cells) || is_table_header_row(&cells) {
            return false;
        }

        let row_text = cells.join(" ");
        !is_default_prd_scaffold_row(&row_text)
    })
}

fn section_has_authored_checkbox(section: &str) -> bool {
    section.lines().any(|line| {
        let trimmed = line.trim();
        let item = if let Some(item) = trimmed.strip_prefix("- [ ]") {
            item.trim()
        } else if let Some(item) = trimmed.strip_prefix("- [x]") {
            item.trim()
        } else if let Some(item) = trimmed.strip_prefix("- [X]") {
            item.trim()
        } else {
            return false;
        };

        !item.is_empty() && !is_default_prd_scaffold_row(item)
    })
}

fn parse_scope_bullets(scope_section: &str) -> (usize, usize) {
    enum Mode {
        None,
        InScope,
        OutScope,
    }

    let mut mode = Mode::None;
    let mut in_scope_count = 0;
    let mut out_scope_count = 0;

    for line in scope_section.lines() {
        let trimmed = line.trim();
        if trimmed.eq_ignore_ascii_case("### In Scope") {
            mode = Mode::InScope;
            continue;
        }
        if trimmed.eq_ignore_ascii_case("### Out of Scope") {
            mode = Mode::OutScope;
            continue;
        }

        let Some(item) = trimmed.strip_prefix("- ") else {
            continue;
        };
        let item = item.trim();
        if item.is_empty() || is_default_prd_scaffold_row(item) {
            continue;
        }

        match mode {
            Mode::InScope => in_scope_count += 1,
            Mode::OutScope => out_scope_count += 1,
            Mode::None => {}
        }
    }

    (in_scope_count, out_scope_count)
}

fn is_table_separator_row(cells: &[&str]) -> bool {
    !cells.is_empty()
        && cells
            .iter()
            .all(|cell| cell.chars().all(|ch| ch == '-' || ch == ':' || ch == ' '))
}

fn is_table_header_row(cells: &[&str]) -> bool {
    cells.iter().all(|cell| {
        matches!(
            cell.to_ascii_lowercase().as_str(),
            "goal"
                | "success metric"
                | "target"
                | "persona"
                | "description"
                | "primary need"
                | "id"
                | "requirement"
                | "priority"
                | "rationale"
                | "area"
                | "method"
                | "evidence"
                | "assumption"
                | "impact if wrong"
                | "validation"
                | "question/risk"
                | "owner"
                | "status"
        )
    })
}

fn is_default_prd_scaffold_row(value: &str) -> bool {
    EPIC_PRD_DEFAULT_ROW_MARKERS
        .iter()
        .any(|marker| value.contains(marker))
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
    fn test_first_unfilled_placeholder_pattern_prefers_todo_and_ignores_comments() {
        assert_eq!(
            first_unfilled_placeholder_pattern("<!-- TODO: ignored --> {{token}}"),
            Some("{{token}}".to_string())
        );
        assert_eq!(
            first_unfilled_placeholder_pattern("Real TODO: remains"),
            Some("TODO:".to_string())
        );
    }

    #[test]
    fn test_first_unfilled_placeholder_pattern_detects_legacy_story_scaffold() {
        assert_eq!(
            first_unfilled_placeholder_pattern(
                "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] Define acceptance criteria for this slice"
            ),
            Some("Define acceptance criteria for this slice".to_string())
        );
        assert_eq!(
            first_unfilled_placeholder_pattern(
                "<!-- Define acceptance criteria for this slice -->\nReady content"
            ),
            None
        );
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
        assert!(problems.iter().any(|p| p.severity == Severity::Error));
        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("pattern: TODO:"))
        );
    }

    #[test]
    fn test_check_epic_prd_authored_content_flags_unfilled_sections() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("PRD.md");
        fs::write(
            &path,
            r#"# PRD
## Problem Statement

## Goals & Objectives
| Goal | Success Metric | Target |
|------|----------------|--------|

## Users
| Persona | Description | Primary Need |
|---------|-------------|--------------|

## Scope
### In Scope

### Out of Scope

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
<!-- END FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

## Assumptions
| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|

## Open Questions & Risks
| Question/Risk | Owner | Status |
|---------------|-------|--------|

<!-- BEGIN SUCCESS_CRITERIA -->
<!-- END SUCCESS_CRITERIA -->
"#,
        )
        .unwrap();

        let problems = check_epic_prd_authored_content(&path);
        assert!(!problems.is_empty());
        assert!(
            problems
                .iter()
                .all(|p| p.check_id == CheckId::EpicPrdAuthoredContent)
        );
        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("Problem Statement"))
        );
        assert!(problems.iter().any(|p| p.message.contains("In Scope")));
        assert!(
            problems
                .iter()
                .any(|p| p.message.contains("Success Criteria"))
        );
    }

    #[test]
    fn test_check_epic_prd_authored_content_accepts_filled_sections() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("PRD.md");
        fs::write(
            &path,
            r#"# PRD

## Problem Statement
Operators cannot reliably inspect deployment readiness across surfaces.

## Goals & Objectives
| Goal | Success Metric | Target |
|------|----------------|--------|
| Reduce decision latency | Median review time | < 5 minutes |

## Users
| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Delivery Lead | Owns release quality | Fast confidence checks |

## Scope
### In Scope
- Unified readiness reporting across core flows.

### Out of Scope
- Replacing external observability providers.

## Requirements
### Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Render readiness summaries in CLI show commands. | must | Enables rapid review decisions. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Keep rendering deterministic across runs. | must | Prevents noisy diffs and confusion. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy
- Validate rendering behavior with unit tests and deterministic fixture snapshots.

## Assumptions
| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Operators prefer CLI-first workflows | Adoption risk | Weekly usability review |

## Open Questions & Risks
| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should summaries include trend deltas? | Product | Open |

## Success Criteria
<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Reviews consistently complete within target latency.
<!-- END SUCCESS_CRITERIA -->
"#,
        )
        .unwrap();

        let problems = check_epic_prd_authored_content(&path);
        assert!(problems.is_empty(), "{problems:?}");
    }

    #[test]
    fn test_check_epic_prd_structure_missing_markers() {
        let temp = TempDir::new().unwrap();
        let path = temp.path().join("PRD.md");
        fs::write(&path, "# PRD\nNo markers here").unwrap();

        let problems = check_epic_prd_structure(&path);
        assert!(problems.iter().any(|p| p.message.contains("missing")));
    }

    #[test]
    fn test_unresolved_scaffold_patterns_are_errors_in_srs_sdd_prd() {
        let temp = TempDir::new().unwrap();

        let srs_path = temp.path().join("SRS.md");
        fs::write(
            &srs_path,
            r#"# SRS
TODO: define requirement details
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
<!-- END FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#,
        )
        .unwrap();
        let srs_problems = check_voyage_srs_structure(&srs_path);
        assert!(srs_problems.iter().any(|p| p.severity == Severity::Error));
        assert!(
            srs_problems
                .iter()
                .any(|p| p.message.contains("pattern: TODO:"))
        );

        let sdd_path = temp.path().join("SDD.md");
        fs::write(&sdd_path, "# SDD\n\nTODO: fill architecture decisions").unwrap();
        let sdd_problems = check_voyage_sdd_structure(&sdd_path);
        assert!(sdd_problems.iter().any(|p| p.severity == Severity::Error));
        assert!(
            sdd_problems
                .iter()
                .any(|p| p.message.contains("pattern: TODO:"))
        );

        let prd_path = temp.path().join("PRD.md");
        fs::write(
            &prd_path,
            r#"# PRD
TODO: fill product requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| FR-01 | requirement | must |
<!-- END FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| NFR-01 | requirement | should |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] complete
<!-- END SUCCESS_CRITERIA -->
"#,
        )
        .unwrap();
        let prd_problems = check_epic_prd_structure(&prd_path);
        assert!(prd_problems.iter().any(|p| p.severity == Severity::Error));
        assert!(
            prd_problems
                .iter()
                .any(|p| p.message.contains("pattern: TODO:"))
        );
    }
}
