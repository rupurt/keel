//! New voyage command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::infrastructure::loader::load_board;
use crate::infrastructure::story_id::generate_story_id;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;

/// Create a new voyage
pub fn run(name: &str, epic_id: &str, goal: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    new_voyage(&board_dir, name, epic_id, goal)?;
    Ok(())
}

/// Create a new voyage
fn new_voyage(board_dir: &Path, name: &str, epic_id: &str, goal: &str) -> Result<String> {
    let board = load_board(board_dir)?;
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let goal_text = goal.trim();
    if goal_text.is_empty() {
        return Err(anyhow!(
            "goal is required (use --goal when creating voyage)"
        ));
    }

    // Enforce Title Case
    if !crate::infrastructure::utils::is_title_case(name) {
        return Err(anyhow!(
            "Voyage title '{}' must use Title Case (e.g. 'My Voyage Title')",
            name
        ));
    }

    // Verify epic exists
    board.require_epic(epic_id)?;

    // Find next voyage number for this epic
    let next_num = find_next_voyage_num(&board, epic_id);
    let voyage_id = generate_story_id();

    // Create voyage directory
    let voyage_dir = board_dir
        .join("epics")
        .join(epic_id)
        .join("voyages")
        .join(&voyage_id);

    if voyage_dir.exists() {
        return Err(anyhow!("Voyage already exists: {}", voyage_id));
    }

    fs::create_dir_all(&voyage_dir).with_context(|| {
        format!(
            "Failed to create voyage directory: {}",
            voyage_dir.display()
        )
    })?;

    // Render template (including epic placeholder)
    let mut content = template_rendering::render(
        templates::voyage::README,
        &[
            ("id", &voyage_id),
            ("title", name),
            ("created_at", &now),
            ("epic", epic_id),
            ("goal", goal_text),
        ],
    );

    // Insert index
    content = crate::cli::commands::management::story::new::insert_frontmatter_field(
        &content,
        &format!("id: {}", voyage_id),
        &format!("index: {}", next_num),
    );

    // Write README
    let readme_path = voyage_dir.join("README.md");
    fs::write(&readme_path, content)
        .with_context(|| format!("Failed to write voyage README: {}", readme_path.display()))?;

    // Write SRS
    let srs_content = template_rendering::render(
        templates::voyage::SRS,
        &[("title", name), ("epic", epic_id), ("goal", goal_text)],
    );
    let srs_path = voyage_dir.join("SRS.md");
    fs::write(&srs_path, srs_content)
        .with_context(|| format!("Failed to write voyage SRS: {}", srs_path.display()))?;

    // Write SDD
    let sdd_content = template_rendering::render(
        templates::voyage::SDD,
        &[("title", name), ("goal", goal_text)],
    );
    let sdd_path = voyage_dir.join("SDD.md");
    fs::write(&sdd_path, sdd_content)
        .with_context(|| format!("Failed to write voyage SDD: {}", sdd_path.display()))?;

    // Count total voyages for this epic (including the one we just created)
    let total_voyages = board.voyages_for_epic_id(epic_id).len() + 1;

    println!();
    println!("  Created voyage: {}", name);
    println!("  Path: epics/{}/voyages/{}/", epic_id, voyage_id);
    println!("  Voyage {} of {}", next_num, total_voyages);
    println!();
    println!("  Next steps:");
    println!(
        "    1. Fill epics/{}/voyages/{}/SRS.md with requirements",
        epic_id, voyage_id
    );
    println!(
        "    2. Fill epics/{}/voyages/{}/SDD.md with design",
        epic_id, voyage_id
    );
    println!("    3. Decompose into stories: keel story new \"<Title>\" --type feat");
    println!();
    println!("  Epic PRD: epics/{}/PRD.md", epic_id);

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(voyage_id)
}

/// Find the next voyage number for an epic
fn find_next_voyage_num(board: &crate::domain::model::Board, epic_id: &str) -> u32 {
    let voyages = board.voyages_for_epic_id(epic_id);
    let max_num = voyages
        .iter()
        .filter_map(|m| m.frontmatter.index)
        .max()
        .unwrap_or(0);

    max_num + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::validation::{CheckId, structural};
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use regex::Regex;

    #[test]
    fn test_new_voyage_success() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .build();
        let board_dir = temp.path();

        let voyage_id = new_voyage(board_dir, "Voyage 1", "test-epic", "My goal").unwrap();

        let voyage_dir = board_dir.join("epics/test-epic/voyages").join(&voyage_id);
        assert!(voyage_dir.is_dir());
        assert!(voyage_dir.join("README.md").exists());
        assert!(voyage_dir.join("SRS.md").exists());
        assert!(voyage_dir.join("SDD.md").exists());

        let readme = fs::read_to_string(voyage_dir.join("README.md")).unwrap();
        assert!(readme.contains("id: "));
        assert!(readme.contains("title: Voyage 1"));
        assert!(readme.contains("> My goal"));
        let created_at_re = Regex::new(r"created_at: \d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}").unwrap();
        assert!(
            created_at_re.is_match(&readme),
            "created_at should be datetime: {readme}"
        );

        let srs = fs::read_to_string(voyage_dir.join("SRS.md")).unwrap();
        assert!(
            !srs.contains("TODO:") && !srs.contains("{{"),
            "SRS should not contain unfilled placeholders: {srs}"
        );

        let date_problems = structural::check_date_consistency(
            &voyage_dir.join("README.md"),
            CheckId::VoyageDateConsistency,
        );
        assert!(
            date_problems.is_empty(),
            "Voyage README should satisfy datetime checks: {date_problems:?}"
        );

        let readme_problems =
            structural::check_voyage_readme_structure(&voyage_dir.join("README.md"));
        let srs_problems = structural::check_voyage_srs_structure(&voyage_dir.join("SRS.md"));
        let sdd_problems = structural::check_voyage_sdd_structure(&voyage_dir.join("SDD.md"));

        assert!(
            readme_problems.is_empty(),
            "Voyage README should satisfy placeholder checks: {readme_problems:?}"
        );
        assert!(
            srs_problems.is_empty(),
            "Voyage SRS should satisfy placeholder checks: {srs_problems:?}"
        );
        assert!(
            sdd_problems.is_empty(),
            "Voyage SDD should satisfy placeholder checks: {sdd_problems:?}"
        );
    }

    #[test]
    fn test_find_next_voyage_num() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-v1", "test-epic").index(1))
            .voyage(TestVoyage::new("02-v2", "test-epic").index(2))
            .build();
        let board = crate::infrastructure::loader::load_board(temp.path()).unwrap();

        assert_eq!(find_next_voyage_num(&board, "test-epic"), 3);
    }
}
