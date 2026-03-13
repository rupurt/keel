//! Show routine command

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use keel::domain::model::Routine;
use keel::infrastructure::loader::load_board;
use keel::read_model::show_selector::{ShowEntityKind, resolve_show_selector};

/// Show routine details.
pub fn run(id: &str) -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Show routine details with an explicit board directory.
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let resolved_id = resolve_show_selector(board_dir, &board, ShowEntityKind::Routine, id)?;
    let routine = board.require_routine(&resolved_id)?;
    print!("{}", render_routine_show(routine)?);
    Ok(())
}

fn render_routine_show(routine: &Routine) -> Result<String> {
    let width = crate::cli::presentation::terminal::get_terminal_width();
    let mut metadata = ShowKeyValues::new().with_min_label_width(13);
    metadata.push_row("Title:", format!("{}", routine.title().bold()));
    metadata.push_row("ID:", style::styled_id(routine.id()));
    metadata.push_row("Target Scope:", styled_target_scope(routine.target_scope()));
    metadata.push_optional_row(
        "Created:",
        routine
            .frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
    );
    metadata.push_optional_row(
        "Updated:",
        routine
            .frontmatter
            .updated_at
            .map(|updated_at| format!("{}", updated_at.dimmed())),
    );
    metadata.push_row("Path:", format!("{}", routine.path.display().dimmed()));

    let mut cadence_section = ShowSection::new("Cadence");
    cadence_section.push_lines(render_yaml_block(routine.cadence())?);

    let mut blueprint_section = ShowSection::new("Blueprint");
    blueprint_section.push_lines(render_blueprint_block(routine.blueprint_markdown()));

    let mut document = ShowDocument::new();
    document.push_header(metadata, Some(width));
    document.push_sections_spaced([cadence_section, blueprint_section]);
    Ok(document.render())
}

fn styled_target_scope(scope: &str) -> String {
    if scope.contains('/') {
        let parts: Vec<_> = scope.split('/').collect();
        if parts.len() >= 2 {
            format!(
                "{}/{}",
                style::styled_epic_id(parts[0]),
                style::styled_voyage_id(parts[1])
            )
        } else {
            style::styled_id(scope)
        }
    } else {
        style::styled_epic_id(scope)
    }
}

fn render_yaml_block(value: &serde_yaml::Value) -> Result<Vec<String>> {
    Ok(serde_yaml::to_string(value)?
        .trim_end()
        .lines()
        .map(|line| format!("  {line}"))
        .collect())
}

fn render_blueprint_block(blueprint: &str) -> Vec<String> {
    blueprint
        .trim()
        .lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("  {line}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::TestBoardBuilder;
    use std::fs;
    use std::path::Path;

    fn write_routine(root: &Path, id: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: Weekly Review
cadence:
  cron: 0 9 * * 1
  timezone: America/Los_Angeles
target-scope: E1/V1
created_at: 2026-03-11T09:00:00
updated_at: 2026-03-11T09:30:00
---

# Blueprint

- Review the open backlog.
- Triage aging items.
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn routine_show_renders_cadence_scope_and_blueprint_from_canonical_storage() {
        let temp = TestBoardBuilder::new().build();
        write_routine(temp.path(), "routine-weekly-review");

        let board = load_board(temp.path()).unwrap();
        let routine = board.require_routine("routine-weekly-review").unwrap();
        let rendered = render_routine_show(routine).unwrap();

        assert!(rendered.contains("Weekly Review"));
        assert!(rendered.contains("routine-weekly-review"));
        assert!(rendered.contains("cron"));
        assert!(rendered.contains("0 9 * * 1"));
        assert!(rendered.contains("timezone"));
        assert!(rendered.contains("America/Los_Angeles"));
        assert!(rendered.contains("E1"));
        assert!(rendered.contains("V1"));
        assert!(rendered.contains("# Blueprint"));
        assert!(rendered.contains("Review the open backlog."));
    }
}
