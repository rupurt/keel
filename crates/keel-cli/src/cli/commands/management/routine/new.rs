//! New routine command

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use chrono::Local;
use serde_yaml::{Mapping, Value};

use keel::domain::model::Board;
use keel::infrastructure::loader::load_board;
use keel::infrastructure::template_rendering;
use keel::infrastructure::templates;
use keel::infrastructure::utils::is_title_case;

/// Create a new routine.
pub fn run(title: &str, target_scope: &str, cadence: &[String]) -> Result<PathBuf> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    new_routine(&board_dir, title, target_scope, cadence)
}

fn new_routine(
    board_dir: &Path,
    title: &str,
    target_scope: &str,
    cadence: &[String],
) -> Result<PathBuf> {
    if !is_title_case(title) {
        return Err(anyhow!(
            "Routine title '{}' must use Title Case (e.g. 'Weekly Review')",
            title
        ));
    }

    let cadence = parse_cadence(cadence)?;
    let board = load_board(board_dir)?;
    validate_target_scope(&board, target_scope)?;

    let routine_id = keel_core::infrastructure::story_id::generate_story_id();

    let routine_dir = board_dir.join("routines").join(&routine_id);
    if routine_dir.exists() {
        return Err(anyhow!("Routine already exists: {}", routine_id));
    }
    fs::create_dir_all(&routine_dir).with_context(|| {
        format!(
            "Failed to create routine directory: {}",
            routine_dir.display()
        )
    })?;

    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let cadence_block = render_indented_yaml(&cadence, 2)?;
    let readme_path = routine_dir.join("README.md");
    let content = template_rendering::render(
        templates::routine::README,
        &[
            ("id", &routine_id),
            ("title", title),
            ("cadence_block", &cadence_block),
            ("target_scope", target_scope),
            ("created_at", &now),
            ("updated_at", &now),
        ],
    );

    fs::write(&readme_path, content)
        .with_context(|| format!("Failed to write routine README: {}", readme_path.display()))?;

    println!("Created: routines/{}/", routine_id);

    Ok(readme_path)
}

fn validate_target_scope(board: &Board, target_scope: &str) -> Result<()> {
    if let Some((epic_id, voyage_id)) = target_scope.split_once('/') {
        board.require_epic(epic_id)?;
        let voyage = board.require_voyage(voyage_id)?;
        if voyage.epic_id != epic_id {
            return Err(anyhow!(
                "Cannot target scope '{}': voyage '{}' belongs to epic '{}', expected '{}'",
                target_scope,
                voyage_id,
                voyage.epic_id,
                epic_id
            ));
        }
        Ok(())
    } else {
        board.require_epic(target_scope)?;
        Ok(())
    }
}

fn parse_cadence(entries: &[String]) -> Result<Value> {
    if entries.is_empty() {
        return Err(anyhow!(
            "Routine cadence requires at least one --cadence key=value entry"
        ));
    }

    let mut mapping = Mapping::new();
    for entry in entries {
        let (key, value) = entry.split_once('=').ok_or_else(|| {
            anyhow!(
                "Invalid cadence entry '{}'; expected key=value for --cadence",
                entry
            )
        })?;
        let key = key.trim();
        let value = value.trim();

        if key.is_empty() || value.is_empty() {
            return Err(anyhow!(
                "Invalid cadence entry '{}'; expected non-empty key=value",
                entry
            ));
        }

        let key_value = Value::String(key.to_string());
        if mapping.contains_key(&key_value) {
            return Err(anyhow!("Duplicate cadence key '{}'", key));
        }

        let parsed_value = serde_yaml::from_str::<Value>(value)
            .unwrap_or_else(|_| Value::String(value.to_string()));
        mapping.insert(key_value, parsed_value);
    }

    Ok(Value::Mapping(mapping))
}

fn render_indented_yaml(value: &Value, indent: usize) -> Result<String> {
    let yaml = serde_yaml::to_string(value).context("serialize cadence metadata")?;
    let padding = " ".repeat(indent);
    Ok(yaml
        .trim_end()
        .lines()
        .map(|line| format!("{padding}{line}"))
        .collect::<Vec<_>>()
        .join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::{TestBoardBuilder, TestEpic, TestVoyage};
    use std::fs;

    #[test]
    fn routine_new_scaffolds_valid_single_bundle_with_opaque_cadence_mapping() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(TestVoyage::new("V1", "E1"))
            .build();

        let path = new_routine(
            temp.path(),
            "Weekly Review",
            "E1/V1",
            &[
                "cron=0 9 * * 1".to_string(),
                "timezone=America/Los_Angeles".to_string(),
            ],
        )
        .unwrap();

        assert!(path.to_string_lossy().contains("/routines/VD"));
        assert!(path.exists());
        assert!(!path.parent().unwrap().join("BLUEPRINT.md").exists());

        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("id: VD"));
        assert!(content.contains("title: Weekly Review"));
        assert!(content.contains("cadence:"));
        assert!(content.contains("cron:"));
        assert!(content.contains("0 9 * * 1"));
        assert!(content.contains("timezone:"));
        assert!(content.contains("America/Los_Angeles"));
        assert!(content.contains("target-scope: E1/V1"));
        assert!(content.contains("# Blueprint"));
    }
}
