//! Show story command

use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::presentation::duration::render_completed_with_length;
use crate::cli::presentation::show::{ShowDocument, ShowKeyValues, ShowSection};
use crate::cli::style;
use crate::infrastructure::loader::load_board;
use crate::read_model::planning_show::{self, EvidenceReport};

const NO_EVIDENCE_DIR_PLACEHOLDER: &str = "(EVIDENCE directory not found)";
const NO_SUPPLEMENTARY_PLACEHOLDER: &str = "(no supplementary artifacts)";
const NO_MEDIA_PLACEHOLDER: &str = "(no media artifacts)";
const NO_VERIFY_PLACEHOLDER: &str = "(no verify annotations found)";

struct StoryHeading<'a> {
    level: usize,
    title: &'a str,
}

/// Run the show story command
pub fn run(id: &str) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, id)
}

/// Run the show story command with an explicit board directory
pub fn run_with_dir(board_dir: &Path, id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let story = board.require_story(id)?;
    let projection = planning_show::build_story_show_projection(story)?;

    let width = crate::cli::presentation::terminal::get_terminal_width();
    let mut metadata = ShowKeyValues::new().with_min_label_width(9);
    metadata.push_row("Title:", format!("{}", story.frontmatter.title.bold()));
    metadata.push_row("Type:", style::styled_type(&story.story_type()));
    metadata.push_row("Status:", style::styled_story_status(&story.status));
    metadata.push_optional_row(
        "Scope:",
        story.frontmatter.scope.as_deref().map(styled_story_scope),
    );
    metadata.push_standard_timestamps(
        story
            .frontmatter
            .created_at
            .map(|created_at| format!("{}", created_at.dimmed())),
        story
            .frontmatter
            .started_at
            .map(|started_at| format!("{}", started_at.dimmed())),
        story
            .frontmatter
            .updated_at
            .map(|updated_at| format!("{}", updated_at.dimmed())),
        story.frontmatter.completed_at.map(|completed_at| {
            render_completed_with_length(story.frontmatter.started_at, completed_at)
        }),
    );
    if projection.total_criteria > 0 {
        metadata.push_row(
            "Progress:",
            style::progress_bar(
                projection.checked_criteria,
                projection.total_criteria,
                20,
                None,
            ),
        );
    }
    metadata.push_row("Path:", format!("{}", story.path.display().dimmed()));

    let mut document = ShowDocument::new();
    document.push_header(metadata, None);

    if let Some(body_text) = &projection.body
        && !body_text.trim().is_empty()
    {
        document.push_lines(story_body_lines(body_text, &story.frontmatter.title, width));
    }

    document.push_spacer();
    document.push_section(evidence_section(story.id(), &projection.evidence));
    document.print();

    Ok(())
}

#[cfg(test)]
fn build_evidence_report(story_path: &Path, content: &str) -> EvidenceReport {
    planning_show::build_story_evidence_projection(story_path, content)
}

fn styled_story_scope(scope: &str) -> String {
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

fn story_body_lines(body_text: &str, story_title: &str, width: usize) -> Vec<String> {
    let mut lines = vec![style::rule(width, None)];
    let mut code_block: Option<(String, String)> = None;
    let mut just_rendered_heading = false;

    for line in body_text.trim().lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            if code_block.is_some() {
                flush_story_code_block(&mut lines, &mut code_block);
            } else {
                let lang = trimmed.trim_start_matches('`').trim().to_string();
                code_block = Some((lang, String::new()));
            }
            continue;
        }

        if let Some((_, ref mut code)) = code_block {
            code.push_str(line);
            code.push('\n');
            continue;
        }

        if just_rendered_heading && trimmed.is_empty() {
            just_rendered_heading = false;
            continue;
        }

        if trimmed.starts_with("- [x]")
            || trimmed.starts_with("- [X]")
            || trimmed.starts_with("- [ ]")
        {
            lines.push(style::styled_ac(trimmed));
            just_rendered_heading = false;
        } else if let Some(heading) = parse_story_body_heading(trimmed) {
            if heading.level == 1 && heading.title.eq_ignore_ascii_case(story_title.trim()) {
                // Suppress duplicate title in body when it matches frontmatter title.
                just_rendered_heading = true;
                continue;
            }

            lines.push(heading.title.bold().to_string());
            just_rendered_heading = true;
        } else {
            lines.push(style::styled_inline_markdown(line));
            just_rendered_heading = false;
        }
    }

    flush_story_code_block(&mut lines, &mut code_block);
    lines
}

fn flush_story_code_block(lines: &mut Vec<String>, code_block: &mut Option<(String, String)>) {
    let Some((lang, code)) = code_block.take() else {
        return;
    };

    if let Some(highlighted) = style::highlight_code_block(&code, &lang) {
        lines.extend(highlighted.lines().map(ToString::to_string));
    } else {
        for code_line in code.lines() {
            lines.push(code_line.dimmed().to_string());
        }
    }
}

fn evidence_section(story_id: &str, report: &EvidenceReport) -> ShowSection {
    let mut section = ShowSection::new("Evidence");
    section.push_lines(evidence_lines(story_id, report));
    section
}

fn evidence_lines(story_id: &str, report: &EvidenceReport) -> Vec<String> {
    let mut lines = Vec::new();

    if report.items.is_empty() {
        lines.push(format!("  {}", NO_VERIFY_PLACEHOLDER.dimmed()));
    } else {
        for (idx, item) in report.items.iter().enumerate() {
            let ac_label = item
                .ac_label
                .clone()
                .unwrap_or_else(|| format!("AC-{:02}", idx + 1));
            lines.push(format!(
                "  {}: {}",
                ac_label.cyan(),
                style::styled_inline_markdown(&item.criterion)
            ));
            lines.push(format!(
                "    Mode: {}",
                style::styled_inline_markdown(&item.mode)
            ));

            if let Some(command) = &item.command {
                lines.push(format!(
                    "    Command: {}",
                    style::styled_inline_markdown(command)
                ));
            }
            if let Some(requirement) = &item.requirement {
                lines.push(format!(
                    "    Requirement: {}",
                    style::styled_inline_markdown(requirement)
                ));
            }
            if let Some(proof) = &item.proof_filename {
                lines.push(format!(
                    "    Proof: {}",
                    style::styled_inline_markdown(proof)
                ));
            } else {
                lines.push("    Proof: (none linked)".to_string());
            }
            if let Some(recorded_at) = &item.proof_metadata.recorded_at {
                lines.push(format!(
                    "    recorded_at: {}",
                    style::styled_inline_markdown(recorded_at)
                ));
            }
            if let Some(mode) = &item.proof_metadata.mode {
                lines.push(format!(
                    "    proof mode: {}",
                    style::styled_inline_markdown(mode)
                ));
            }
            if let Some(command) = &item.proof_metadata.command {
                lines.push(format!(
                    "    proof command: {}",
                    style::styled_inline_markdown(command)
                ));
            }
            if !item.excerpt_lines.is_empty() {
                lines.push(format!("    Excerpt ({} lines):", item.excerpt_lines.len()));
                for line in &item.excerpt_lines {
                    lines.push(format!("      {}", style::styled_inline_markdown(line)));
                }
            }
            if item.missing_proof {
                lines.push("    Warning: linked proof file is missing".to_string());
            }
        }
    }

    if report.evidence_dir_missing {
        lines.push(format!("  {}", NO_EVIDENCE_DIR_PLACEHOLDER.dimmed()));
    }

    lines.push("  Supplementary artifacts:".to_string());
    if report.supplementary_artifacts.is_empty() {
        lines.push(format!("    {}", NO_SUPPLEMENTARY_PLACEHOLDER.dimmed()));
    } else {
        for artifact in &report.supplementary_artifacts {
            lines.push(format!("    - {}", style::styled_inline_markdown(artifact)));
        }
    }

    lines.push("  Media artifacts:".to_string());
    if report.media_artifacts.is_empty() {
        lines.push(format!("    {}", NO_MEDIA_PLACEHOLDER.dimmed()));
    } else {
        for media in &report.media_artifacts {
            let rel = format!("stories/{}/EVIDENCE/{}", story_id, media);
            lines.push(format!("    - {}", style::styled_inline_markdown(media)));
            lines.push(format!(
                "      Playback: {}",
                style::styled_inline_markdown(&format!("ffplay -autoexit {}", rel))
            ));
        }
    }

    if !report.missing_proofs.is_empty() {
        lines.push(format!(
            "  Missing proofs: {}",
            style::styled_inline_markdown(&report.missing_proofs.join(", "))
        ));
    }

    lines
}

fn parse_story_body_heading(trimmed: &str) -> Option<StoryHeading<'_>> {
    let mut level = 0usize;
    let mut rest = trimmed;
    while let Some(stripped) = rest.strip_prefix('#') {
        level += 1;
        rest = stripped;
    }

    if !(1..=6).contains(&level) {
        return None;
    }

    let title = rest.strip_prefix(' ')?.trim();
    if title.is_empty() {
        return None;
    }

    Some(StoryHeading { level, title })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    #[test]
    fn show_displays_story() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("planned"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Test Story")
                    .status(StoryState::Backlog)
                    .scope("test-epic/01-first")
                    .body("\n# Test Story\n\n## Acceptance Criteria\n\n- [ ] First criterion\n"),
            )
            .build();

        run_with_dir(temp.path(), "FEAT0001").unwrap();
    }

    #[test]
    fn extract_body_works() {
        let content = "---\nid: test\n---\n\n# Body\n\nContent here.";
        let body = planning_show::extract_story_body(content).unwrap();
        assert!(body.contains("# Body"));
        assert!(body.contains("Content here."));
    }

    #[test]
    fn count_acs_counts_correctly() {
        let body = "## Acceptance Criteria\n\n- [x] Done\n- [ ] Not done\n- [x] Also done\n";
        let (checked, total) = planning_show::count_story_acceptance_criteria(body);
        assert_eq!(checked, 2);
        assert_eq!(total, 3);
    }

    #[test]
    fn story_show_proof_metadata() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").body(
                r#"## Acceptance Criteria
- [x] [SRS-04/AC-01] evidence <!-- verify: cargo test --lib story_show_proof_metadata, SRS-04:start:end, proof: ac-1.log -->
"#,
            ))
            .build();

        let evidence_file = temp.path().join("stories/S1/EVIDENCE/ac-1.log");
        fs::write(
            &evidence_file,
            r#"---
recorded_at: 2026-03-04T18:00:00Z
mode: command
command: cargo test --lib story_show_proof_metadata
---
ok
"#,
        )
        .unwrap();

        let story_path = temp.path().join("stories/S1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        let report = build_evidence_report(&story_path, &content);

        assert_eq!(report.items.len(), 1);
        assert_eq!(
            report.items[0].proof_metadata.recorded_at.as_deref(),
            Some("2026-03-04T18:00:00Z")
        );
        assert_eq!(
            report.items[0].proof_metadata.mode.as_deref(),
            Some("command")
        );
        assert_eq!(
            report.items[0].proof_metadata.command.as_deref(),
            Some("cargo test --lib story_show_proof_metadata")
        );
    }

    #[test]
    fn story_show_artifact_inventory() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").body(
                r#"## Acceptance Criteria
- [x] [SRS-04/AC-01] log proof <!-- verify: cargo test --lib story_show_artifact_inventory, SRS-04:start, proof: ac-1.log -->
- [x] [SRS-04/AC-02] gif proof <!-- verify: manual, SRS-04:end, proof: demo.gif -->
"#,
            ))
            .build();

        let evidence_dir = temp.path().join("stories/S1/EVIDENCE");
        fs::write(evidence_dir.join("ac-1.log"), "ok").unwrap();
        fs::write(evidence_dir.join("demo.gif"), "gif").unwrap();
        fs::write(evidence_dir.join("notes.txt"), "notes").unwrap();
        fs::write(evidence_dir.join("capture.mp4"), "mp4").unwrap();

        let story_path = temp.path().join("stories/S1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        let report = build_evidence_report(&story_path, &content);

        assert!(report.linked_proofs.contains(&"ac-1.log".to_string()));
        assert!(report.linked_proofs.contains(&"demo.gif".to_string()));
        assert!(
            report
                .supplementary_artifacts
                .contains(&"notes.txt".to_string())
        );
        assert!(report.media_artifacts.contains(&"demo.gif".to_string()));
        assert!(report.media_artifacts.contains(&"capture.mp4".to_string()));
    }

    #[test]
    fn story_show_proof_excerpt_10_lines_and_warnings() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").body(
                r#"## Acceptance Criteria
- [x] [SRS-04/AC-01] long proof <!-- verify: cargo test --lib story_show_proof_excerpt_10_lines_and_warnings, SRS-04:start, proof: long.log -->
- [x] [SRS-04/AC-02] missing proof <!-- verify: manual, SRS-04:end, proof: missing.log -->
"#,
            ))
            .build();

        let evidence_dir = temp.path().join("stories/S1/EVIDENCE");
        let body = (1..=15)
            .map(|idx| format!("line-{idx}"))
            .collect::<Vec<_>>()
            .join("\n");
        fs::write(
            evidence_dir.join("long.log"),
            format!("---\nrecorded_at: now\nmode: command\n---\n{body}\n"),
        )
        .unwrap();

        let story_path = temp.path().join("stories/S1/README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        let report = build_evidence_report(&story_path, &content);

        let first = &report.items[0];
        assert_eq!(first.excerpt_lines.len(), 10);
        assert_eq!(first.excerpt_lines[0], "line-1");
        assert_eq!(first.excerpt_lines[9], "line-10");
        assert!(report.missing_proofs.contains(&"missing.log".to_string()));
    }

    #[test]
    fn story_show_missing_evidence_placeholders() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").body(
                r#"## Acceptance Criteria
- [x] [SRS-NFR-02/AC-02] proof missing <!-- verify: manual, SRS-NFR-02:start:end, proof: nofile.log -->
"#,
            ))
            .build();

        let story_dir = temp.path().join("stories/S1");
        fs::remove_dir_all(story_dir.join("EVIDENCE")).unwrap();

        let story_path = story_dir.join("README.md");
        let content = fs::read_to_string(&story_path).unwrap();
        let report = build_evidence_report(&story_path, &content);
        let lines = evidence_lines("S1", &report);

        assert!(report.evidence_dir_missing);
        assert!(
            lines
                .iter()
                .any(|line| line.contains(NO_EVIDENCE_DIR_PLACEHOLDER))
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains(NO_SUPPLEMENTARY_PLACEHOLDER))
        );
        assert!(lines.iter().any(|line| line.contains(NO_MEDIA_PLACEHOLDER)));
    }
}
