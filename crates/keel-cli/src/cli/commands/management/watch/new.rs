//! New watch command

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};

use keel::infrastructure::loader::load_board;
use keel::infrastructure::story_id::generate_story_id_avoiding_casefold_collisions;
use keel::infrastructure::template_rendering;
use keel::infrastructure::templates;

/// Create a new watch
pub fn run(title: &str, limit: u32) -> Result<String> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    new_watch(&board_dir, title, limit)
}

/// Create a new watch
fn new_watch(board_dir: &Path, title: &str, limit: u32) -> Result<String> {
    // Enforce Title Case
    if !keel::infrastructure::utils::is_title_case(title) {
        return Err(anyhow!(
            "Watch title '{}' must use Title Case (e.g. 'My Watch Title')",
            title
        ));
    }

    let board = load_board(board_dir)?;
    let watch_id = generate_story_id_avoiding_casefold_collisions(board.watches.keys());

    // Create watches directory if it doesn't exist
    let watches_dir = board_dir.join("watches");
    if !watches_dir.exists() {
        fs::create_dir_all(&watches_dir).with_context(|| {
            format!(
                "Failed to create watches directory: {}",
                watches_dir.display()
            )
        })?;
    }

    // Create watch directory
    let watch_dir = watches_dir.join(&watch_id);
    if watch_dir.exists() {
        return Err(anyhow!("Watch already exists: {}", watch_id));
    }
    fs::create_dir_all(&watch_dir)
        .with_context(|| format!("Failed to create watch directory: {}", watch_dir.display()))?;

    // Render README.md template
    let readme_content = template_rendering::render(
        templates::watch::README,
        &[
            ("id", &watch_id),
            ("title", title),
            ("limit", &limit.to_string()),
        ],
    );

    // Write README.md
    let readme_path = watch_dir.join("README.md");
    fs::write(&readme_path, readme_content)
        .with_context(|| format!("Failed to write watch README: {}", readme_path.display()))?;

    println!("Created: watches/{}/", watch_id);

    // Regenerate board
    crate::cli::commands::generate::run(board_dir)?;

    Ok(watch_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::TestBoardBuilder;

    #[test]
    fn watch_scaffold_creates_files() {
        let temp = TestBoardBuilder::new().build();
        let board_dir = temp.path();

        let _watch_id = new_watch(board_dir, "My New Watch", 24).unwrap();

        // Find the watch directory
        let watches_dir = board_dir.join("watches");
        let watch_dir = fs::read_dir(watches_dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                let content = fs::read_to_string(p.join("README.md")).unwrap();
                content.contains("title: My New Watch")
            })
            .expect("Watch not found");

        assert!(watch_dir.is_dir());
        assert!(watch_dir.join("README.md").exists());

        let readme = fs::read_to_string(watch_dir.join("README.md")).unwrap();
        assert!(readme.contains("title: My New Watch"));
        assert!(readme.contains("limit_hours: 24"));
    }
}
