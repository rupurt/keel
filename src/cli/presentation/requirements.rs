//! Shared requirement rendering helpers for show commands.

use owo_colors::OwoColorize;

use crate::cli::style;
use crate::read_model::planning_show::{RequirementCompletion, RequirementKind, RequirementRow};

pub fn grouped_requirement_lines(rows: &[RequirementRow], empty_placeholder: &str) -> Vec<String> {
    let mut lines = Vec::new();
    if rows.is_empty() {
        lines.push(format!("  {}", empty_placeholder.dimmed()));
        return lines;
    }

    let functional: Vec<&RequirementRow> = rows
        .iter()
        .filter(|row| row.kind == RequirementKind::Functional)
        .collect();
    let non_functional: Vec<&RequirementRow> = rows
        .iter()
        .filter(|row| row.kind == RequirementKind::NonFunctional)
        .collect();

    push_requirement_group(&mut lines, "Functional Requirements", &functional);
    push_requirement_group(&mut lines, "Non-Functional Requirements", &non_functional);
    lines
}

fn push_requirement_group(lines: &mut Vec<String>, title: &str, rows: &[&RequirementRow]) {
    if rows.is_empty() {
        return;
    }

    if !lines.is_empty() {
        lines.push(String::new());
    }
    lines.push(format!("{}", title.bold()));
    for row in rows {
        lines.extend(requirement_lines(row));
    }
}

pub fn requirement_lines(row: &RequirementRow) -> Vec<String> {
    let linked = if row.linked_stories.is_empty() {
        "none".to_string()
    } else {
        row.linked_stories
            .iter()
            .map(|story| style::styled_story_id(&story.id))
            .collect::<Vec<_>>()
            .join(", ")
    };

    vec![
        format!(
            "  {} {} {}",
            requirement_completion_icon(row.completion),
            style::styled_requirement_id(&row.id),
            style::styled_inline_markdown(&row.description)
        ),
        format!(
            "    Verification: {} | Linked Stories: {linked}",
            style::styled_inline_markdown(&row.verification)
        ),
    ]
}

fn requirement_completion_icon(completion: RequirementCompletion) -> String {
    match completion {
        RequirementCompletion::Done => format!("{}", "[x]".green().bold()),
        RequirementCompletion::InProgress => format!("{}", "[~]".yellow().bold()),
        RequirementCompletion::Queued | RequirementCompletion::Unmapped => {
            format!("{}", "[ ]".bright_blue())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::read_model::planning_show::StoryRef;

    #[test]
    fn grouped_requirements_render_functional_before_non_functional() {
        let rows = vec![
            RequirementRow {
                id: "SRS-NFR-01".to_string(),
                description: "Meet latency budget".to_string(),
                kind: RequirementKind::NonFunctional,
                linked_stories: vec![],
                completion: RequirementCompletion::Queued,
                verification: "manual (1)".to_string(),
            },
            RequirementRow {
                id: "SRS-01".to_string(),
                description: "Render grouped requirement output".to_string(),
                kind: RequirementKind::Functional,
                linked_stories: vec![StoryRef {
                    id: "S1".to_string(),
                    stage: StoryState::Done,
                    index: Some(1),
                }],
                completion: RequirementCompletion::Done,
                verification: "automated (1)".to_string(),
            },
        ];

        let rendered = grouped_requirement_lines(&rows, "(none)").join("\n");
        let functional_idx = rendered.find("Functional Requirements").unwrap();
        let non_functional_idx = rendered.find("Non-Functional Requirements").unwrap();

        assert!(functional_idx < non_functional_idx);
    }

    #[test]
    fn requirement_lines_render_inline_metadata_and_colored_story_ids() {
        let row = RequirementRow {
            id: "SRS-01".to_string(),
            description: "Render grouped requirement output".to_string(),
            kind: RequirementKind::Functional,
            linked_stories: vec![StoryRef {
                id: "S1".to_string(),
                stage: StoryState::Done,
                index: Some(1),
            }],
            completion: RequirementCompletion::Done,
            verification: "automated (1)".to_string(),
        };

        let lines = requirement_lines(&row);
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("Verification:"));
        assert!(lines[1].contains("| Linked Stories:"));
        assert!(lines[1].contains(&style::styled_story_id("S1")));
    }
}
