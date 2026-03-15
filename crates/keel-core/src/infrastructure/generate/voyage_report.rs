//! Voyage Narrative Report generation

use std::fmt::Write;
use std::fs;

use crate::domain::model::{Board, StoryState, Voyage};
use crate::infrastructure::template_rendering;
use crate::infrastructure::templates;

/// Generate a comprehensive VOYAGE_REPORT.md for the voyage
pub fn generate(board: &Board, voyage: &Voyage) -> anyhow::Result<()> {
    let content = generate_voyage_report(board, voyage);
    let report_path = voyage.path.parent().unwrap().join("VOYAGE_REPORT.md");
    super::artifact_io::write_if_changed(&report_path, &content)?;
    Ok(())
}

/// Generate the content for VOYAGE_REPORT.md
pub fn generate_voyage_report(board: &Board, voyage: &Voyage) -> String {
    let stories = board.stories_for_voyage(voyage);
    let done_count = stories
        .iter()
        .filter(|s| s.status == StoryState::Done)
        .count();
    let total_count = stories.len();

    let mut narrative = String::new();
    let mut sorted_stories = stories;
    sorted_stories.sort_by(|a, b| a.id().cmp(b.id()));

    for story in &sorted_stories {
        writeln!(narrative, "### {}", story.title()).unwrap();
        writeln!(narrative, "- **ID:** {}", story.id()).unwrap();
        writeln!(narrative, "- **Status:** {}", story.status).unwrap();

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
            && let Ok(insights) =
                crate::read_model::knowledge::load_reflection_knowledge(&board.root, &reflect_path)
            && !insights.is_empty()
        {
            writeln!(narrative).unwrap();
            writeln!(narrative, "#### Implementation Insights").unwrap();
            render_knowledge_entries(&mut narrative, &insights);
        }

        // Include Evidence links
        let evidence_dir = story_dir.join("EVIDENCE");
        if let Ok(proofs) = super::artifact_io::read_sorted_file_names(&evidence_dir)
            && !proofs.is_empty()
        {
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
        writeln!(narrative).unwrap();
    }

    let status = voyage.status().to_string();
    let done_count = done_count.to_string();
    let total_count = total_count.to_string();

    template_rendering::render(
        templates::voyage::REPORT,
        &[
            ("title", voyage.title()),
            ("id", voyage.id()),
            ("epic_id", &voyage.epic_id),
            ("status", &status),
            ("goal", voyage.frontmatter.goal.as_deref().unwrap_or("-")),
            ("done_count", &done_count),
            ("total_count", &total_count),
            ("narrative", &narrative),
        ],
    )
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

fn render_knowledge_entries(
    output: &mut String,
    entries: &[crate::read_model::knowledge::Knowledge],
) {
    for entry in entries {
        writeln!(output, "- **{}: {}**", entry.id, entry.title).unwrap();
        writeln!(output, "  - Insight: {}", entry.insight).unwrap();
        writeln!(output, "  - Suggested Action: {}", entry.suggested_action).unwrap();
        if !entry.applies_to.trim().is_empty() {
            writeln!(output, "  - Applies To: {}", entry.applies_to).unwrap();
        }
        if !entry.category.trim().is_empty() && entry.category != "unknown" {
            writeln!(output, "  - Category: {}", entry.category).unwrap();
        }
        writeln!(output).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn voyage_artifacts_render_proofs_deterministically() {
        let srs = r#"# SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Stable reports | SCOPE-01 | FR-01 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("E1"))
            .voyage(
                TestVoyage::new("V1", "E1")
                    .status("done")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("S1")
                    .title("Story One")
                    .scope("E1/V1")
                    .status(StoryState::Done)
                    .body(
                        "# Summary\n\nDeterministic report.\n\n## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] stable\n",
                    ),
            )
            .build();

        let evidence_dir = temp.path().join("stories").join("S1").join("EVIDENCE");
        fs::create_dir_all(&evidence_dir).unwrap();
        fs::write(evidence_dir.join("zeta.log"), "proof").unwrap();
        fs::write(evidence_dir.join("alpha.log"), "proof").unwrap();

        let board = load_board(temp.path()).unwrap();
        let voyage = board.require_voyage("V1").unwrap();
        let report = generate_voyage_report(&board, voyage);

        let alpha_index = report.find("[alpha.log]").unwrap();
        let zeta_index = report.find("[zeta.log]").unwrap();
        assert!(
            alpha_index < zeta_index,
            "proof links should be rendered in filename order"
        );
    }
}
