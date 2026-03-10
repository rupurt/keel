//! Story Audit - Rich evidence and traceability report

use anyhow::Result;
use owo_colors::OwoColorize;
use std::collections::HashMap;
use std::path::Path;

use crate::cli::commands::management::verification_guidance::{
    audit_error_with_recovery, guidance_for_audit_story, print_human,
};
use crate::cli::style;
use crate::domain::model::Board;
use crate::infrastructure::loader::load_board;
use crate::read_model::evidence::{self, EvidenceEntry};
use crate::read_model::planning_show;

/// Run the audit command
pub fn run(board_dir: &Path, id: Option<&str>) -> Result<()> {
    let result: Result<Option<_>> = (|| {
        let board = load_board(board_dir)?;

        if let Some(id) = id {
            // Audit specific entity
            if let Some(story) = board.stories.get(id) {
                println!(
                    "{}",
                    "Story Evidence Audit".bright_blue().bold().underline()
                );
                println!();
                audit_story(story, 0)?;
                Ok(guidance_for_audit_story(story.id(), story.status))
            } else if let Some(voyage) = board.voyages.get(id) {
                println!("{}", "Voyage Evidence Audit".magenta().bold().underline());
                println!();
                audit_voyage(&board, voyage, 0)?;
                Ok(None)
            } else if let Some(epic) = board.epics.get(id) {
                println!("{}", "Epic Evidence Audit".cyan().bold().underline());
                println!();
                audit_epic(&board, epic)?;
                Ok(None)
            } else {
                anyhow::bail!("Entity not found: {}", id);
            }
        } else {
            // Audit entire board
            println!();
            println!(
                "{}",
                "Board Evidence Audit".bright_white().bold().underline()
            );
            println!();

            let mut epics: Vec<_> = board.epics.values().collect();
            epics.sort_by(|a, b| match (a.index(), b.index()) {
                (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
                (Some(_), None) => std::cmp::Ordering::Less,
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (None, None) => a.id().cmp(b.id()),
            });

            for epic in epics {
                audit_epic(&board, epic)?;
                println!();
            }

            Ok(None)
        }
    })();

    match result {
        Ok(guidance) => {
            print_human(guidance.as_ref());
            Ok(())
        }
        Err(error) => Err(audit_error_with_recovery(id, error)),
    }
}

fn audit_story(story: &crate::domain::model::Story, indent_level: usize) -> Result<()> {
    let indent = " ".repeat(indent_level);

    let status_indicator = if story.status == crate::domain::model::StoryState::Done {
        "✓ ".green().bold().to_string()
    } else {
        "".to_string()
    };

    println!(
        "{}{}{} - {} ({})",
        indent,
        status_indicator,
        format!("Story/{}", story.id()).bright_blue().bold(),
        story.frontmatter.title,
        style::styled_story_status(&story.status)
    );

    let content = std::fs::read_to_string(&story.path)?;
    let evidence_report = planning_show::build_story_evidence_projection(&story.path, &content);

    if evidence_report.items.is_empty() {
        println!("{}  {}", indent, "(no verify annotations found)".dimmed());
    } else {
        // Group evidence items by requirement for audit view
        let mut by_req: HashMap<String, Vec<&planning_show::EvidenceItem>> = HashMap::new();
        for item in &evidence_report.items {
            for req_id in &item.requirements {
                by_req
                    .entry(req_id.clone())
                    .or_default()
                    .push(item);
            }
        }

        let mut req_ids: Vec<_> = by_req.keys().collect();
        req_ids.sort();

        for req_id in req_ids {
            println!("{}  Requirement: {}", indent, req_id.cyan());
            for item in &by_req[req_id] {
                let ac_label = item
                    .ac_label
                    .clone()
                    .unwrap_or_else(|| "AC-??".to_string());

                println!(
                    "{}    {}: {}",
                    indent,
                    ac_label.cyan(),
                    style::styled_inline_markdown(&item.criterion)
                );
                println!(
                    "{}      Mode: {}",
                    indent,
                    style::styled_inline_markdown(&item.mode)
                );

                if let Some(proof) = &item.proof_filename {
                    println!(
                        "{}      Proof: {}",
                        indent,
                        style::styled_inline_markdown(proof)
                    );
                    if let Some(recorded_at) = &item.proof_metadata.recorded_at {
                        println!(
                            "{}        recorded_at: {}",
                            indent,
                            style::styled_inline_markdown(recorded_at)
                        );
                    }
                    if let Some(command) = &item.proof_metadata.command {
                        println!(
                            "{}        proof command: {}",
                            indent,
                            style::styled_inline_markdown(command)
                        );
                    }
                    if !item.excerpt_lines.is_empty() {
                        println!(
                            "{}        Excerpt ({} lines):",
                            indent,
                            item.excerpt_lines.len()
                        );
                        for line in &item.excerpt_lines {
                            println!("{}          {}", indent, style::styled_inline_markdown(line));
                        }
                    }
                    if item.missing_proof {
                        println!(
                            "{}        {} linked proof file is missing",
                            indent,
                            "Warning:".yellow()
                        );
                    }
                } else {
                    println!("{}      Proof: (none linked)", indent);
                }
            }
        }
    }

    if evidence_report.evidence_dir_missing {
        println!("{}  {}", indent, "(EVIDENCE directory not found)".dimmed());
    }

    if !evidence_report.supplementary_artifacts.is_empty() {
        println!("{}  Supplementary artifacts:", indent);
        for artifact in &evidence_report.supplementary_artifacts {
            println!("{}    - {}", indent, style::styled_inline_markdown(artifact));
        }
    }

    if !evidence_report.media_artifacts.is_empty() {
        println!("{}  Media artifacts:", indent);
        for media in &evidence_report.media_artifacts {
            println!("{}    - {}", indent, style::styled_inline_markdown(media));
        }
    }

    let bundle_dir = story.path.parent().unwrap();
    let reflect_path = bundle_dir.join("REFLECT.md");
    if reflect_path.exists() {
        let board_dir = bundle_dir
            .parent()
            .and_then(|stories_dir| stories_dir.parent())
            .unwrap_or(bundle_dir);
        if !crate::read_model::knowledge::load_reflection_knowledge(board_dir, &reflect_path)?
            .is_empty()
        {
            println!("{}  {}", indent, "✓ Reflection recorded".green());
        }
    }

    Ok(())
}

fn audit_voyage(
    board: &Board,
    voyage: &crate::domain::model::Voyage,
    indent_level: usize,
) -> Result<()> {
    let indent = " ".repeat(indent_level);

    let stories = board.stories_for_voyage(voyage);
    let done_count = stories
        .iter()
        .filter(|s| s.status == crate::domain::model::StoryState::Done)
        .count();
    let total_count = stories.len();

    let progress = if total_count > 0 {
        style::progress_bar(done_count, total_count, 15, None)
    } else {
        "".to_string()
    };

    println!(
        "{}{} - {} {}",
        indent,
        format!("Voyage/{}", voyage.id()).magenta().bold(),
        voyage.frontmatter.title,
        progress
    );

    let mut stories = stories;
    stories.sort_by(|a, b| match (a.index(), b.index()) {
        (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.id().cmp(b.id()),
    });

    // Collect all unique requirement IDs across all stories in this voyage
    let mut req_ids = std::collections::HashSet::new();
    for story in &stories {
        let content = std::fs::read_to_string(&story.path)?;
        for entry in evidence::collect_story_evidence(story.id(), story.title(), &content) {
            req_ids.insert(entry.requirement_id);
        }
    }

    if req_ids.is_empty() {
        println!(
            "{}  {}",
            indent,
            "No requirements covered in voyage".dimmed()
        );
    } else {
        let mut sorted_reqs: Vec<_> = req_ids.into_iter().collect();
        sorted_reqs.sort();
        println!("{}  Requirements: {}", indent, sorted_reqs.join(", "));
    }

    println!();
    println!("{}  Bundle Audits:", indent);
    for story in stories {
        audit_story(story, indent_level + 4)?;
    }
    Ok(())
}

fn audit_epic(board: &Board, epic: &crate::domain::model::Epic) -> Result<()> {
    let voyages = board.voyages_for_epic_id(epic.id());

    let mut total_stories = 0;
    let mut done_stories = 0;

    for voyage in &voyages {
        let stories = board.stories_for_voyage(voyage);
        total_stories += stories.len();
        done_stories += stories
            .iter()
            .filter(|s| s.status == crate::domain::model::StoryState::Done)
            .count();
    }

    let progress = if total_stories > 0 {
        style::progress_bar(done_stories, total_stories, 20, None)
    } else {
        "".to_string()
    };

    println!(
        "{} - {} {}",
        format!("Epic/{}", epic.id()).cyan().bold(),
        epic.frontmatter.title.cyan(),
        progress
    );

    let mut voyages = voyages;
    voyages.sort_by(|a, b| match (a.index(), b.index()) {
        (Some(idx_a), Some(idx_b)) => idx_a.cmp(&idx_b),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => a.id().cmp(b.id()),
    });

    // Collect all evidence across all voyages in this epic
    let mut epic_evidence = Vec::new();
    for voyage in &voyages {
        let stories = board.stories_for_voyage(voyage);
        for story in stories {
            let content = std::fs::read_to_string(&story.path)?;
            epic_evidence.extend(evidence::collect_story_evidence(
                story.id(),
                story.title(),
                &content,
            ));
        }
    }

    if !epic_evidence.is_empty() {
        println!("  Epic Evidence Chains:");
        render_requirement_groups(board, &epic_evidence, 4);
        println!();
    }

    for voyage in voyages {
        audit_voyage(board, voyage, 2)?;
    }
    Ok(())
}

fn render_requirement_groups(board: &Board, entries: &[EvidenceEntry], indent_level: usize) {
    let indent = " ".repeat(indent_level);

    let mut by_req: HashMap<String, Vec<EvidenceEntry>> = HashMap::new();
    for entry in entries {
        by_req
            .entry(entry.requirement_id.clone())
            .or_default()
            .push(entry.clone());
    }

    let mut req_ids: Vec<_> = by_req.keys().collect();
    req_ids.sort();

    for req_id in req_ids {
        println!("{}Requirement: {}", indent, req_id.cyan());

        // Group by story within requirement
        let mut by_story: HashMap<String, Vec<EvidenceEntry>> = HashMap::new();
        for entry in &by_req[req_id] {
            by_story
                .entry(entry.story_id.clone())
                .or_default()
                .push(entry.clone());
        }

        let mut story_ids: Vec<_> = by_story.keys().collect();
        story_ids.sort();

        for story_id in story_ids {
            let story = if let Some(s) = board.stories.get(story_id) {
                s
            } else {
                continue;
            };

            let content = match std::fs::read_to_string(&story.path) {
                Ok(c) => c,
                Err(_) => continue,
            };

            let evidence_report =
                planning_show::build_story_evidence_projection(&story.path, &content);

            println!(
                "{}  Story: {} - {} ({})",
                indent,
                style::styled_story_id(story.id()),
                story.frontmatter.title,
                style::styled_story_status(&story.status)
            );

            // Filter evidence report items to only those matching this requirement
            for item in &evidence_report.items {
                if item.requirements.contains(req_id) {
                    let ac_label = item.ac_label.clone().unwrap_or_else(|| "AC-??".to_string());

                    println!(
                        "{}    {}: {}",
                        indent,
                        ac_label.cyan(),
                        style::styled_inline_markdown(&item.criterion)
                    );
                    println!(
                        "{}      Mode: {}",
                        indent,
                        style::styled_inline_markdown(&item.mode)
                    );

                    if let Some(proof) = &item.proof_filename {
                        println!(
                            "{}      Proof: {}",
                            indent,
                            style::styled_inline_markdown(proof)
                        );
                        if let Some(recorded_at) = &item.proof_metadata.recorded_at {
                            println!(
                                "{}        recorded_at: {}",
                                indent,
                                style::styled_inline_markdown(recorded_at)
                            );
                        }
                        if let Some(command) = &item.proof_metadata.command {
                            println!(
                                "{}        proof command: {}",
                                indent,
                                style::styled_inline_markdown(command)
                            );
                        }
                        if !item.excerpt_lines.is_empty() {
                            println!(
                                "{}        Excerpt ({} lines):",
                                indent,
                                item.excerpt_lines.len()
                            );
                            for line in &item.excerpt_lines {
                                println!(
                                    "{}          {}",
                                    indent,
                                    style::styled_inline_markdown(line)
                                );
                            }
                        }
                        if item.missing_proof {
                            println!(
                                "{}        {} linked proof file is missing",
                                indent,
                                "Warning:".yellow()
                            );
                        }
                    } else {
                        println!("{}      Proof: (none linked)", indent);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    #[test]
    fn test_audit_run_all() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .story(TestStory::new("S1").scope("epic1/v1"))
            .build();

        let result = run(temp.path(), None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_run_story() {
        let temp = TestBoardBuilder::new().story(TestStory::new("S1")).build();

        let result = run(temp.path(), Some("S1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_run_voyage() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .story(TestStory::new("S1").scope("epic1/v1"))
            .build();

        let result = run(temp.path(), Some("v1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_run_epic() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("epic1"))
            .voyage(TestVoyage::new("v1", "epic1"))
            .story(TestStory::new("S1").scope("epic1/v1"))
            .build();

        let result = run(temp.path(), Some("epic1"));
        assert!(result.is_ok());
    }

    #[test]
    fn test_audit_run_not_found() {
        let temp = TestBoardBuilder::new().build();

        let result = run(temp.path(), Some("NONEXISTENT"));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Entity not found: NONEXISTENT"));
        assert!(err.contains("Recovery step:"));
        assert!(err.contains("keel story list"));
    }

    #[test]
    fn test_audit_story_with_evidence_and_reflect() {
        let temp = TestBoardBuilder::new()
            .story(TestStory::new("S1").body(
                "Evidence: [key](val) <!-- verify: cargo test proof: artifact.txt SRS-01:start -->",
            ))
            .build();

        let story_dir = temp.path().join("stories/S1");

        // Add evidence artifact
        let evidence_dir = story_dir.join("EVIDENCE");
        if !evidence_dir.exists() {
            fs::create_dir(&evidence_dir).unwrap();
        }
        fs::write(evidence_dir.join("artifact.txt"), "some artifact").unwrap();

        // Add reflection
        fs::write(story_dir.join("REFLECT.md"), "### L001: Some reflection").unwrap();

        let result = run(temp.path(), Some("S1"));
        assert!(result.is_ok());
    }
}
