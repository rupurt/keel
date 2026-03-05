//! New bearing command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::infrastructure::duplicate_ids::{self, DuplicateEntity};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::story_id::generate_story_id;
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;

/// Create a new bearing
pub fn run(name: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    new_bearing(&board_dir, name)
}

/// Create a new bearing
fn new_bearing(board_dir: &Path, name: &str) -> Result<()> {
    duplicate_ids::ensure_unique_ids(board_dir, DuplicateEntity::Bearing, "keel bearing new")?;

    // Enforce Title Case
    if !crate::infrastructure::utils::is_title_case(name) {
        return Err(anyhow!(
            "Bearing title '{}' must use Title Case (e.g. 'My Bearing Title')",
            name
        ));
    }

    let board = load_board(board_dir)?;
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    // Generate random bearing ID
    let bearing_id = generate_story_id();

    // Calculate next index
    let next_index = board
        .bearings
        .values()
        .filter_map(|b| b.frontmatter.index)
        .max()
        .unwrap_or(0)
        + 1;

    // Create bearings directory if it doesn't exist
    let bearings_dir = board_dir.join("bearings");
    if !bearings_dir.exists() {
        fs::create_dir_all(&bearings_dir).with_context(|| {
            format!(
                "Failed to create bearings directory: {}",
                bearings_dir.display()
            )
        })?;
    }

    // Create bearing directory
    let bearing_dir = bearings_dir.join(&bearing_id);
    if bearing_dir.exists() {
        return Err(anyhow!("Bearing already exists: {}", bearing_id));
    }
    fs::create_dir_all(&bearing_dir).with_context(|| {
        format!(
            "Failed to create bearing directory: {}",
            bearing_dir.display()
        )
    })?;

    // Render README.md template
    let readme_content = template_rendering::render(
        templates::bearing::README,
        &[
            ("id", &bearing_id),
            ("title", name),
            ("created_at", &now),
            ("status", "exploring"),
        ],
    );

    // Insert index into README frontmatter
    let readme_content = crate::cli::commands::management::story::new::insert_frontmatter_field(
        &readme_content,
        &format!("id: {}", bearing_id),
        &format!("index: {}", next_index),
    );

    // Write README.md
    let readme_path = bearing_dir.join("README.md");
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write bearing README: {}", readme_path.display()))?;

    // Render BRIEF.md template
    let brief_content = template_rendering::render(templates::bearing::BRIEF, &[("title", name)]);

    // Strip frontmatter if the template still includes it (templates usually have it)
    // For now, I will just write it as is, but if BRIEF.md template is updated, this is clean.
    // Actually, I should probably ensure BRIEF.md doesn't have frontmatter if I want to be strict.
    let brief_body = if brief_content.starts_with("---") {
        let parts: Vec<&str> = brief_content.splitn(3, "---").collect();
        if parts.len() == 3 {
            parts[2].trim_start().to_string()
        } else {
            brief_content
        }
    } else {
        brief_content
    };

    // Write BRIEF.md (content only)
    let brief_path = bearing_dir.join("BRIEF.md");
    fs::write(&brief_path, brief_body)
        .with_context(|| format!("Failed to write bearing BRIEF: {}", brief_path.display()))?;

    println!("Created: bearings/{}/", bearing_id);

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::TestBoardBuilder;

    #[test]
    fn test_new_bearing_success() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        new_bearing(board_dir, "My New Research").unwrap();

        // Find the bearing directory (it's random now)
        let bearings_dir = board_dir.join("bearings");
        let bearing_dir = fs::read_dir(bearings_dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                let content = fs::read_to_string(p.join("README.md")).unwrap();
                content.contains("title: My New Research")
            })
            .expect("Bearing not found");

        assert!(bearing_dir.is_dir());
        assert!(bearing_dir.join("README.md").exists());
        assert!(bearing_dir.join("BRIEF.md").exists());

        let readme = fs::read_to_string(bearing_dir.join("README.md")).unwrap();
        assert!(readme.contains("title: My New Research"));
        assert!(readme.contains("index: 1"));
    }

    #[test]
    fn test_new_bearing_name_collision_is_ok() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        // Names can now collide because IDs are random
        new_bearing(board_dir, "Duplicate").unwrap();
        let res = new_bearing(board_dir, "Duplicate");

        assert!(res.is_ok());
    }
}
