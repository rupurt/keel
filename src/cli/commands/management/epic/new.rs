//! New epic command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::infrastructure::duplicate_ids::{self, DuplicateEntity};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::story_id::generate_story_id;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;

/// Create a new epic
pub fn run(name: &str, problem: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    new_epic(&board_dir, name, problem)
}

/// Create a new epic
fn new_epic(board_dir: &Path, name: &str, problem: &str) -> Result<()> {
    duplicate_ids::ensure_unique_ids(board_dir, DuplicateEntity::Epic, "keel epic new")?;

    let board = load_board(board_dir)?;
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let problem_text = problem.trim();
    if problem_text.is_empty() {
        return Err(anyhow!(
            "problem is required (use --problem when creating epic)"
        ));
    }

    // Enforce Title Case
    if !crate::infrastructure::utils::is_title_case(name) {
        return Err(anyhow!(
            "Epic title '{}' must use Title Case (e.g. 'My Epic Title')",
            name
        ));
    }

    // Generate random epic ID
    let epic_id = generate_story_id();

    // Calculate next index
    let next_index = board
        .epics
        .values()
        .filter_map(|e| e.frontmatter.index)
        .max()
        .unwrap_or(0)
        + 1;

    // Create epic directory
    let epic_dir = board_dir.join("epics").join(&epic_id);
    if epic_dir.exists() {
        return Err(anyhow!("Epic already exists: {}", epic_id));
    }
    fs::create_dir_all(&epic_dir)
        .with_context(|| format!("Failed to create epic directory: {}", epic_dir.display()))?;

    // Render template
    let mut content = template_rendering::render(
        templates::epic::README,
        &[
            ("id", &epic_id),
            ("title", name),
            ("created_at", &now),
            ("problem", problem_text),
        ],
    );

    // Insert index
    content = crate::cli::commands::management::story::new::insert_frontmatter_field(
        &content,
        &format!("id: {}", epic_id),
        &format!("index: {}", next_index),
    );

    // Write README
    let readme_path = epic_dir.join("README.md");
    fs::write(&readme_path, content)
        .with_context(|| format!("Failed to write epic README: {}", readme_path.display()))?;

    // Write PRD
    let prd_content = template_rendering::render(
        templates::epic::PRD,
        &[("title", name), ("problem", problem_text)],
    );
    let prd_path = epic_dir.join("PRD.md");
    fs::write(&prd_path, prd_content)
        .with_context(|| format!("Failed to write epic PRD: {}", prd_path.display()))?;

    println!("Created: epics/{}/", epic_id);
    println!(
        "  Next: author epics/{}/PRD.md before decomposing voyages",
        epic_id
    );
    println!(
        "  Optional: add epics/{}/PRESS_RELEASE.md only for large user-facing value shifts",
        epic_id
    );

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::validation::{CheckId, structural};
    use crate::test_helpers::TestBoardBuilder;
    use regex::Regex;

    fn find_epic_dir(board_dir: &std::path::Path, title: &str) -> std::path::PathBuf {
        fs::read_dir(board_dir.join("epics"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                fs::read_to_string(path.join("README.md"))
                    .unwrap()
                    .contains(&format!("title: {title}"))
            })
            .expect("Epic not found")
    }

    fn extract_section<'a>(content: &'a str, heading: &str, next_heading: &str) -> &'a str {
        let start = content
            .find(heading)
            .unwrap_or_else(|| panic!("missing heading: {heading}"));
        let start = start + heading.len();
        let tail = &content[start..];
        let end = tail
            .find(next_heading)
            .unwrap_or_else(|| panic!("missing heading: {next_heading}"));
        tail[..end].trim()
    }

    #[test]
    fn test_new_epic_success() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        new_epic(board_dir, "My New Epic", "A problem").unwrap();

        // Find the epic directory (it's random now)
        let epic_dir = find_epic_dir(board_dir, "My New Epic");

        assert!(epic_dir.is_dir());
        assert!(epic_dir.join("README.md").exists());
        assert!(epic_dir.join("PRD.md").exists());
        assert!(!epic_dir.join("PRESS_RELEASE.md").exists());

        let readme = fs::read_to_string(epic_dir.join("README.md")).unwrap();
        assert!(readme.contains("title: My New Epic"));
        assert!(!readme.contains("\nstatus:"));
        assert!(readme.contains("> A problem"));
        assert!(readme.contains("index: 1")); // First epic
        let created_at_re = Regex::new(r"created_at: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
        assert!(
            created_at_re.is_match(&readme),
            "created_at should be datetime: {readme}"
        );

        let prd = fs::read_to_string(epic_dir.join("PRD.md")).unwrap();
        assert!(
            !prd.contains("TODO:") && !prd.contains("{{"),
            "PRD should not contain unfilled placeholders: {prd}"
        );

        let epic_date_problems = structural::check_date_consistency(
            &epic_dir.join("README.md"),
            CheckId::EpicDateConsistency,
        );
        assert!(
            epic_date_problems.is_empty(),
            "Epic README should satisfy datetime checks: {epic_date_problems:?}"
        );

        let prd_problems = structural::check_epic_prd_structure(&epic_dir.join("PRD.md"));
        assert!(
            prd_problems.is_empty(),
            "Epic PRD should satisfy placeholder hygiene checks: {prd_problems:?}"
        );
    }

    #[test]
    fn test_new_epic_name_collision_is_ok() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        // Names can now collide because IDs are random
        new_epic(board_dir, "Duplicate", "problem").unwrap();
        let res = new_epic(board_dir, "Duplicate", "problem");

        assert!(res.is_ok());
    }

    #[test]
    fn epic_new_problem_input_fails_fast_without_defaults() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        let err = new_epic(board_dir, "My New Epic", "   ").unwrap_err();

        assert!(
            err.to_string()
                .contains("problem is required (use --problem when creating epic)")
        );

        let created_epics = fs::read_dir(board_dir.join("epics"))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.path().join("README.md").exists())
            .filter(|entry| {
                fs::read_to_string(entry.path().join("README.md"))
                    .unwrap()
                    .contains("title: My New Epic")
            })
            .count();
        assert_eq!(
            created_epics, 0,
            "should not scaffold a new epic on invalid input"
        );
    }

    #[test]
    fn epic_new_hydrates_problem_into_prd_and_summary_surface() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        new_epic(
            board_dir,
            "My New Epic",
            "Users cannot recover access after sign-in failures.",
        )
        .unwrap();

        let epic_dir = find_epic_dir(board_dir, "My New Epic");
        let readme = fs::read_to_string(epic_dir.join("README.md")).unwrap();
        let prd = fs::read_to_string(epic_dir.join("PRD.md")).unwrap();

        assert!(readme.contains("> Users cannot recover access after sign-in failures."));
        assert!(prd.contains("> Users cannot recover access after sign-in failures."));
        assert!(prd.contains(
            "## Problem Statement\n\nUsers cannot recover access after sign-in failures."
        ));
    }

    #[test]
    fn epic_new_leaves_goal_table_for_direct_prd_authoring() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        new_epic(
            board_dir,
            "My New Epic",
            "Users cannot recover access after sign-in failures.",
        )
        .unwrap();

        let epic_dir = find_epic_dir(board_dir, "My New Epic");
        let prd = fs::read_to_string(epic_dir.join("PRD.md")).unwrap();
        let goals_section = extract_section(&prd, "## Goals & Objectives", "## Users");

        assert!(goals_section.contains("| Goal | Success Metric | Target |"));
        assert!(!goals_section.contains("Users cannot recover access after sign-in failures."));
    }

    #[test]
    fn epic_problem_scaffold_is_deterministic() {
        let board_a = TestBoardBuilder::new().build();
        let board_b = TestBoardBuilder::new().build();
        let title = "My New Epic";
        let problem = "Users cannot recover access after sign-in failures.";

        new_epic(board_a.path(), title, problem).unwrap();
        new_epic(board_b.path(), title, problem).unwrap();

        let epic_dir_a = find_epic_dir(board_a.path(), title);
        let epic_dir_b = find_epic_dir(board_b.path(), title);

        let readme_a = fs::read_to_string(epic_dir_a.join("README.md")).unwrap();
        let readme_b = fs::read_to_string(epic_dir_b.join("README.md")).unwrap();
        let prd_a = fs::read_to_string(epic_dir_a.join("PRD.md")).unwrap();
        let prd_b = fs::read_to_string(epic_dir_b.join("PRD.md")).unwrap();

        let normalize = |content: &str| {
            let content = Regex::new(r"id: [A-Za-z0-9]+")
                .unwrap()
                .replace_all(content, "id: <id>")
                .into_owned();
            Regex::new(r"created_at: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}")
                .unwrap()
                .replace_all(&content, "created_at: <created_at>")
                .into_owned()
        };

        assert_eq!(normalize(&readme_a), normalize(&readme_b));
        assert_eq!(prd_a, prd_b);
    }
}
