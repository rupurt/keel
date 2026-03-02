//! Voyage Narrative Report generation

use std::fmt::Write;
use std::fs;
use std::path::Path;

use crate::domain::model::{Board, StoryState, Voyage};
use crate::infrastructure::templates;

/// Generate a comprehensive VOYAGE_REPORT.md for the voyage
pub fn generate(_board_dir: &Path, board: &Board, voyage: &Voyage) -> anyhow::Result<()> {
    let content = generate_voyage_report(board, voyage);
    let report_path = voyage.path.parent().unwrap().join("VOYAGE_REPORT.md");
    fs::write(report_path, content)?;
    Ok(())
}

/// Generate the content for VOYAGE_REPORT.md
pub fn generate_voyage_report(board: &Board, voyage: &Voyage) -> String {
    let stories = board.stories_for_voyage(voyage);
    let done_count = stories
        .iter()
        .filter(|s| s.stage == StoryState::Done)
        .count();
    let total_count = stories.len();

    let mut narrative = String::new();
    let mut sorted_stories = stories;
    sorted_stories.sort_by(|a, b| a.id().cmp(b.id()));

    for story in &sorted_stories {
        writeln!(narrative, "### {}", story.title()).unwrap();
        writeln!(narrative, "- **ID:** {}", story.id()).unwrap();
        writeln!(narrative, "- **Status:** {}", story.stage).unwrap();

        if let Ok(content) = fs::read_to_string(&story.path) {
            // Include summary if available
            if let Some(summary) = extract_summary(&content) {
                writeln!(narrative).unwrap();
                writeln!(narrative, "#### Summary").unwrap();
                writeln!(narrative, "{}", summary).unwrap();
            }

            // Include ACs
            writeln!(narrative).unwrap();
            writeln!(narrative, "#### Acceptance Criteria").unwrap();
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("- [") {
                    writeln!(narrative, "{}", trimmed).unwrap();
                }
            }
        }

        // Include Insights from REFLECT.md
        let story_dir = story.path.parent().unwrap();
        let reflect_path = story_dir.join("REFLECT.md");
        if reflect_path.exists()
            && let Ok(reflect_content) = fs::read_to_string(reflect_path)
        {
            writeln!(narrative).unwrap();
            writeln!(narrative, "#### Implementation Insights").unwrap();
            writeln!(narrative, "{}", reflect_content.trim()).unwrap();
        }

        // Include Evidence links
        let evidence_dir = story_dir.join("EVIDENCE");
        if evidence_dir.exists()
            && let Ok(entries) = fs::read_dir(evidence_dir)
        {
            let mut proofs = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    proofs.push(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }

            if !proofs.is_empty() {
                writeln!(narrative).unwrap();
                writeln!(narrative, "#### Verified Evidence").unwrap();
                for proof in proofs {
                    let rel_path = format!("../../../../stories/{}/EVIDENCE/{}", story.id(), proof);
                    if proof.ends_with(".gif") {
                        writeln!(narrative, "![{}]({})", proof, rel_path).unwrap();
                    } else {
                        writeln!(narrative, "- [{}]({})", proof, rel_path).unwrap();
                    }
                }
            }
        }
        writeln!(narrative).unwrap();
    }

    templates::voyage::REPORT
        .replace("{{title}}", voyage.title())
        .replace("{{id}}", voyage.id())
        .replace("{{epic_id}}", &voyage.epic_id)
        .replace("{{status}}", &voyage.status().to_string())
        .replace(
            "{{goal}}",
            voyage.frontmatter.goal.as_deref().unwrap_or("-"),
        )
        .replace("{{done_count}}", &done_count.to_string())
        .replace("{{total_count}}", &total_count.to_string())
        .replace("{{narrative}}", &narrative)
}

fn extract_summary(content: &str) -> Option<String> {
    let mut in_summary = false;
    let mut summary = String::new();

    for line in content.lines() {
        if line.starts_with("# Summary") || line.starts_with("## Summary") {
            in_summary = true;
            continue;
        }
        if in_summary {
            if line.starts_with('#') {
                break;
            }
            summary.push_str(line);
            summary.push('\n');
        }
    }

    let trimmed = summary.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}
