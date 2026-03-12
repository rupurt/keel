//! List routines command

use anyhow::Result;
use serde_yaml::Value;

use crate::cli::style;
use crate::cli::table::Table;
use keel::domain::model::{Board, Routine};
use keel::infrastructure::loader::load_board;

/// List routines.
pub fn run() -> Result<()> {
    let board_dir = keel::infrastructure::config::find_board_dir()?;
    let board = load_board(&board_dir)?;
    let routines = collect_routines(&board);
    print!("{}", render_routine_list(&routines));
    Ok(())
}

fn collect_routines(board: &Board) -> Vec<&Routine> {
    let mut routines: Vec<_> = board.routines.values().collect();
    routines.sort_by(|left, right| left.id().cmp(right.id()));
    routines
}

fn render_routine_list(routines: &[&Routine]) -> String {
    if routines.is_empty() {
        return "No routines found on this board.\n".to_string();
    }

    let mut table = Table::new(&["ID", "TITLE", "CADENCE", "TARGET SCOPE"]);
    for routine in routines {
        let cadence = cadence_summary(routine.cadence());
        let target_scope = styled_target_scope(routine.target_scope());
        table.row(&[
            &style::styled_id(routine.id()),
            routine.title(),
            &cadence,
            &target_scope,
        ]);
    }
    table.render()
}

fn cadence_summary(cadence: &Value) -> String {
    let Some(mapping) = cadence.as_mapping() else {
        return scalar_summary(cadence);
    };

    let mut entries: Vec<(String, String)> = mapping
        .iter()
        .map(|(key, value)| (scalar_summary(key), scalar_summary(value)))
        .collect();
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
        .into_iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn scalar_summary(value: &Value) -> String {
    match value {
        Value::Bool(inner) => inner.to_string(),
        Value::Number(inner) => inner.to_string(),
        Value::String(inner) => inner.clone(),
        Value::Null => "null".to_string(),
        _ => serde_yaml::to_string(value)
            .unwrap_or_default()
            .trim()
            .replace('\n', " "),
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use keel::test_helpers::TestBoardBuilder;
    use std::fs;
    use std::path::Path;

    fn write_routine(root: &Path, id: &str, title: &str, target_scope: &str, cadence: &str) {
        let routine_dir = root.join("routines").join(id);
        fs::create_dir_all(&routine_dir).unwrap();
        fs::write(
            routine_dir.join("README.md"),
            format!(
                r#"---
id: {id}
title: {title}
cadence:
  {cadence}
target-scope: {target_scope}
created_at: 2026-03-11T09:00:00
updated_at: 2026-03-11T09:30:00
---

# Blueprint

- Review the open backlog.
"#
            ),
        )
        .unwrap();
    }

    #[test]
    fn routine_list_renders_discoverable_sorted_summaries() {
        let temp = TestBoardBuilder::new().build();
        write_routine(
            temp.path(),
            "routine-zeta",
            "Zeta Review",
            "E2/V2",
            "cron: 0 9 * * 5",
        );
        write_routine(
            temp.path(),
            "routine-alpha",
            "Alpha Review",
            "E1/V1",
            "timezone: America/Los_Angeles",
        );

        let board = load_board(temp.path()).unwrap();
        let routines = collect_routines(&board);
        let rendered = render_routine_list(&routines);

        assert_eq!(routines.len(), 2);
        assert_eq!(routines[0].id(), "routine-alpha");
        assert_eq!(routines[1].id(), "routine-zeta");
        assert!(rendered.contains("routine-alpha"));
        assert!(rendered.contains("Alpha Review"));
        assert!(rendered.contains("E1"));
        assert!(rendered.contains("V1"));
        assert!(rendered.contains("routine-zeta"));
        assert!(rendered.contains("Zeta Review"));
        assert!(rendered.contains("E2"));
        assert!(rendered.contains("V2"));
    }
}
