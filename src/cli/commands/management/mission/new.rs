//! New mission command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::infrastructure::duplicate_ids::{self, DuplicateEntity};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::story_id::generate_story_id;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;

/// Create a new mission
pub fn run(title: &str) -> Result<String> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    new_mission(&board_dir, title)
}

/// Create a new mission
fn new_mission(board_dir: &Path, title: &str) -> Result<String> {
    duplicate_ids::ensure_unique_ids(board_dir, DuplicateEntity::Mission, "keel mission new")?;

    // Enforce Title Case
    if !crate::infrastructure::utils::is_title_case(title) {
        return Err(anyhow!(
            "Mission title '{}' must use Title Case (e.g. 'My Mission Title')",
            title
        ));
    }

    let _board = load_board(board_dir)?;
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    // Generate random mission ID
    let mission_id = generate_story_id();

    // Create missions directory if it doesn't exist
    let missions_dir = board_dir.join("missions");
    if !missions_dir.exists() {
        fs::create_dir_all(&missions_dir).with_context(|| {
            format!(
                "Failed to create missions directory: {}",
                missions_dir.display()
            )
        })?;
    }

    // Create mission directory
    let mission_dir = missions_dir.join(&mission_id);
    if mission_dir.exists() {
        return Err(anyhow!("Mission already exists: {}", mission_id));
    }
    fs::create_dir_all(&mission_dir).with_context(|| {
        format!(
            "Failed to create mission directory: {}",
            mission_dir.display()
        )
    })?;

    // Render README.md template
    let readme_content = template_rendering::render(
        templates::mission::README,
        &[
            ("id", &mission_id),
            ("title", title),
            ("created_at", &now),
            ("updated_at", &now),
            ("status", "defining"),
        ],
    );

    // Write README.md
    let readme_path = mission_dir.join("README.md");
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write mission README: {}", readme_path.display()))?;

    // Render CHARTER.md template
    let charter_content = template_rendering::render(
        templates::mission::CHARTER,
        &[("id", &mission_id), ("title", title)],
    );

    // Write CHARTER.md
    let charter_path = mission_dir.join("CHARTER.md");
    fs::write(&charter_path, charter_content).with_context(|| {
        format!(
            "Failed to write mission CHARTER: {}",
            charter_path.display()
        )
    })?;

    // Render LOG.md template
    let log_content = template_rendering::render(
        templates::mission::LOG,
        &[("id", &mission_id), ("title", title)],
    );

    // Write LOG.md
    let log_path = mission_dir.join("LOG.md");
    fs::write(&log_path, log_content)
        .with_context(|| format!("Failed to write mission LOG: {}", log_path.display()))?;

    println!("Created: missions/{}/", mission_id);

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(mission_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn mission_scaffold_creates_all_files() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        let _mission_id = new_mission(board_dir, "My New Mission").unwrap();

        // Find the mission directory
        let missions_dir = board_dir.join("missions");
        let mission_dir = fs::read_dir(missions_dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                let content = fs::read_to_string(p.join("README.md")).unwrap();
                content.contains("title: My New Mission")
            })
            .expect("Mission not found");

        assert!(mission_dir.is_dir());
        assert!(mission_dir.join("README.md").exists());
        assert!(mission_dir.join("CHARTER.md").exists());
        assert!(mission_dir.join("LOG.md").exists());

        let readme = fs::read_to_string(mission_dir.join("README.md")).unwrap();
        assert!(readme.contains("title: My New Mission"));
        assert!(readme.contains("status: defining"));

        let charter = fs::read_to_string(mission_dir.join("CHARTER.md")).unwrap();
        assert!(charter.contains("## Goals"));

        let log = fs::read_to_string(mission_dir.join("LOG.md")).unwrap();
        assert!(log.contains("Decision Log"));
    }
}
