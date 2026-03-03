//! New epic command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::infrastructure::loader::load_board;
use crate::infrastructure::story_id::generate_story_id;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;

/// Create a new epic
pub fn run(name: &str, goal: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    new_epic(&board_dir, name, goal)
}

/// Create a new epic
fn new_epic(board_dir: &Path, name: &str, goal: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let goal_text = goal.trim();
    if goal_text.is_empty() {
        return Err(anyhow!("goal is required (use --goal when creating epic)"));
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
            ("goal", goal_text),
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
        &[("title", name), ("goal", goal_text)],
    );
    let prd_path = epic_dir.join("PRD.md");
    fs::write(&prd_path, prd_content)
        .with_context(|| format!("Failed to write epic PRD: {}", prd_path.display()))?;

    // Write PRESS_RELEASE
    let pr_content = template_rendering::render(
        templates::epic::PRESS_RELEASE,
        &[("title", name), ("goal", goal_text)],
    );
    let pr_path = epic_dir.join("PRESS_RELEASE.md");
    fs::write(&pr_path, pr_content)
        .with_context(|| format!("Failed to write epic press release: {}", pr_path.display()))?;

    println!("Created: epics/{}/", epic_id);

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands::diagnostics::doctor::checks::epics::check_epic_press_release;
    use crate::infrastructure::loader::load_board;
    use crate::infrastructure::validation::{CheckId, structural};
    use crate::test_helpers::TestBoardBuilder;
    use regex::Regex;

    #[test]
    fn test_new_epic_success() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        new_epic(board_dir, "My New Epic", "A goal").unwrap();

        // Find the epic directory (it's random now)
        let epics_dir = board_dir.join("epics");
        let epic_dir = fs::read_dir(epics_dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                let content = fs::read_to_string(p.join("README.md")).unwrap();
                content.contains("title: My New Epic")
            })
            .expect("Epic not found");

        assert!(epic_dir.is_dir());
        assert!(epic_dir.join("README.md").exists());
        assert!(epic_dir.join("PRD.md").exists());
        assert!(epic_dir.join("PRESS_RELEASE.md").exists());

        let readme = fs::read_to_string(epic_dir.join("README.md")).unwrap();
        assert!(readme.contains("title: My New Epic"));
        assert!(!readme.contains("\nstatus:"));
        assert!(readme.contains("> A goal"));
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

        let pr = fs::read_to_string(epic_dir.join("PRESS_RELEASE.md")).unwrap();
        assert!(pr.contains("# PRESS RELEASE: My New Epic"));
        assert!(pr.contains("Keel introduces My New Epic: A goal"));
        assert!(
            !pr.contains("TODO:") && !pr.contains("{{"),
            "PRESS_RELEASE should not contain unfilled placeholders: {pr}"
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

        let board = load_board(board_dir).unwrap();
        let pr_problems = check_epic_press_release(&board);
        assert!(
            pr_problems.is_empty(),
            "Epic PRESS_RELEASE should satisfy placeholder hygiene checks: {pr_problems:?}"
        );
    }

    #[test]
    fn test_new_epic_name_collision_is_ok() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        // Names can now collide because IDs are random
        new_epic(board_dir, "Duplicate", "goal").unwrap();
        let res = new_epic(board_dir, "Duplicate", "goal");

        assert!(res.is_ok());
    }
}
