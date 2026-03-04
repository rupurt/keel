//! Show story command

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use anyhow::Result;
use owo_colors::OwoColorize;

use crate::cli::style;
use crate::infrastructure::loader::load_board;
use crate::infrastructure::verification::parser::{Comparison, parse_verify_annotations};

const NO_EVIDENCE_DIR_PLACEHOLDER: &str = "(EVIDENCE directory not found)";
const NO_LINKED_PROOFS_PLACEHOLDER: &str = "(no annotation-linked proof artifacts)";
const NO_SUPPLEMENTARY_PLACEHOLDER: &str = "(no supplementary artifacts)";
const NO_MEDIA_PLACEHOLDER: &str = "(no media artifacts)";
const NO_VERIFY_PLACEHOLDER: &str = "(no verify annotations found)";

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct ProofMetadata {
    recorded_at: Option<String>,
    mode: Option<String>,
    command: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EvidenceItem {
    criterion: String,
    requirement: Option<String>,
    mode: String,
    command: Option<String>,
    proof_filename: Option<String>,
    proof_metadata: ProofMetadata,
    excerpt_lines: Vec<String>,
    missing_proof: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct EvidenceReport {
    items: Vec<EvidenceItem>,
    evidence_dir_missing: bool,
    linked_proofs: Vec<String>,
    supplementary_artifacts: Vec<String>,
    media_artifacts: Vec<String>,
    missing_proofs: Vec<String>,
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

    let content = fs::read_to_string(&story.path)?;
    let body = extract_body(&content);
    let (checked, total) = body.map(count_acs).unwrap_or((0, 0));

    let width = crate::cli::presentation::terminal::get_terminal_width();
    println!("{}", style::heavy_rule(width, None));
    println!(
        "{}",
        style::header(story.id(), &story.frontmatter.title, style::styled_story_id)
    );
    println!("{}", style::heavy_rule(width, None));
    println!();

    println!("Type:     {}", style::styled_type(&story.story_type()));
    println!("Status:   {}", style::styled_stage(&story.stage));

    if let Some(scope) = &story.frontmatter.scope {
        let styled_scope = if scope.contains('/') {
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
        };
        println!("Scope:    {}", styled_scope);
    }

    if let Some(created_at) = story.frontmatter.created_at {
        println!("Created:  {}", created_at.dimmed());
    }
    if let Some(updated_at) = story.frontmatter.updated_at {
        println!("Updated:  {}", updated_at.dimmed());
    }

    if total > 0 {
        println!();
        println!(
            "Progress: {}",
            style::progress_bar(checked, total, 20, None)
        );
    }

    println!();
    println!("Path: {}", story.path.display().dimmed());

    if let Some(body_text) = body
        && !body_text.trim().is_empty()
    {
        println!();
        println!("{}", style::rule(width, None));

        let mut code_block: Option<(String, String)> = None;
        for line in body_text.trim().lines() {
            let trimmed = line.trim();

            if trimmed.starts_with("```") {
                if let Some((lang, code)) = code_block.take() {
                    if let Some(highlighted) = style::highlight_code_block(&code, &lang) {
                        print!("{}", highlighted);
                    } else {
                        for code_line in code.lines() {
                            println!("{}", code_line.dimmed());
                        }
                    }
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

            if trimmed.starts_with("- [x]")
                || trimmed.starts_with("- [X]")
                || trimmed.starts_with("- [ ]")
            {
                println!("{}", style::styled_ac(trimmed));
            } else if trimmed.starts_with("# ") || trimmed.starts_with("## ") {
                println!("{}", line.bold());
            } else {
                println!("{}", line);
            }
        }

        if let Some((lang, code)) = code_block {
            if let Some(highlighted) = style::highlight_code_block(&code, &lang) {
                print!("{}", highlighted);
            } else {
                for code_line in code.lines() {
                    println!("{}", code_line.dimmed());
                }
            }
        }
    }

    let evidence = build_evidence_report(&story.path, &content);
    render_evidence_report(story.id(), &evidence);

    Ok(())
}

fn build_evidence_report(story_path: &Path, content: &str) -> EvidenceReport {
    let evidence_dir = story_path.parent().unwrap().join("EVIDENCE");
    let evidence_dir_missing = !evidence_dir.exists();

    let mut all_artifacts = Vec::new();
    if !evidence_dir_missing
        && let Ok(entries) = fs::read_dir(&evidence_dir)
    {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                all_artifacts.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    all_artifacts.sort();

    let mut linked = BTreeSet::new();
    let mut missing = BTreeSet::new();
    let mut items = Vec::new();

    for ann in parse_verify_annotations(content) {
        let proof_filename = ann.proof.clone();
        let requirement = ann
            .requirement
            .as_ref()
            .map(|req| req.id.clone())
            .or_else(|| ann.ac_ref.as_ref().map(|ac| ac.srs_id.clone()));

        let mut proof_metadata = ProofMetadata::default();
        let mut excerpt_lines = Vec::new();
        let mut missing_proof = false;

        if let Some(proof) = &proof_filename {
            let proof_path = evidence_dir.join(proof);
            if proof_path.exists() {
                linked.insert(proof.clone());
                if is_text_artifact(proof) {
                    let (metadata, excerpt) = parse_proof_metadata_and_excerpt(&proof_path);
                    proof_metadata = metadata;
                    excerpt_lines = excerpt;
                } else {
                    proof_metadata = parse_proof_metadata_only(&proof_path);
                }
            } else {
                missing_proof = true;
                missing.insert(proof.clone());
            }
        }

        let mode = if ann.comparison == Comparison::Manual {
            "manual".to_string()
        } else {
            "command".to_string()
        };

        items.push(EvidenceItem {
            criterion: ann.criterion,
            requirement,
            mode,
            command: ann.command,
            proof_filename,
            proof_metadata,
            excerpt_lines,
            missing_proof,
        });
    }

    let linked_proofs: Vec<String> = linked.iter().cloned().collect();
    let media_artifacts: Vec<String> = all_artifacts
        .iter()
        .filter(|name| is_media_artifact(name))
        .cloned()
        .collect();
    let supplementary_artifacts: Vec<String> = all_artifacts
        .iter()
        .filter(|name| !linked.contains(*name))
        .filter(|name| !is_media_artifact(name))
        .cloned()
        .collect();

    EvidenceReport {
        items,
        evidence_dir_missing,
        linked_proofs,
        supplementary_artifacts,
        media_artifacts,
        missing_proofs: missing.iter().cloned().collect(),
    }
}

fn parse_proof_metadata_and_excerpt(path: &Path) -> (ProofMetadata, Vec<String>) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return (ProofMetadata::default(), Vec::new()),
    };
    let (meta, body) = split_frontmatter(&content);
    let excerpt = body
        .lines()
        .take(10)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    (meta, excerpt)
}

fn parse_proof_metadata_only(path: &Path) -> ProofMetadata {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => return ProofMetadata::default(),
    };
    let (meta, _) = split_frontmatter(&content);
    meta
}

fn split_frontmatter(content: &str) -> (ProofMetadata, String) {
    let mut metadata = ProofMetadata::default();
    if !content.starts_with("---\n") {
        return (metadata, content.to_string());
    }

    let mut lines = content.lines();
    let _ = lines.next();
    let mut header = Vec::new();
    for line in lines.by_ref() {
        if line.trim() == "---" {
            break;
        }
        header.push(line.to_string());
    }

    for line in header {
        let mut parts = line.splitn(2, ':');
        let Some(key) = parts.next() else {
            continue;
        };
        let Some(value) = parts.next() else {
            continue;
        };
        let value = value.trim().to_string();
        match key.trim() {
            "recorded_at" => metadata.recorded_at = Some(value),
            "mode" => metadata.mode = Some(value),
            "command" => metadata.command = Some(value),
            _ => {}
        }
    }

    let body = lines.collect::<Vec<_>>().join("\n");
    (metadata, body)
}

fn evidence_lines(story_id: &str, report: &EvidenceReport) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("{}", "Evidence".bold()));

    if report.items.is_empty() {
        lines.push(format!("  {}", NO_VERIFY_PLACEHOLDER.dimmed()));
    } else {
        for (idx, item) in report.items.iter().enumerate() {
            lines.push(format!("  AC {}: {}", idx + 1, item.criterion));
            lines.push(format!("    Mode: {}", item.mode));

            if let Some(command) = &item.command {
                lines.push(format!("    Command: {}", command));
            }
            if let Some(requirement) = &item.requirement {
                lines.push(format!("    Requirement: {}", requirement));
            }
            if let Some(proof) = &item.proof_filename {
                lines.push(format!("    Proof: {}", proof));
            } else {
                lines.push("    Proof: (none linked)".to_string());
            }
            if let Some(recorded_at) = &item.proof_metadata.recorded_at {
                lines.push(format!("    recorded_at: {}", recorded_at));
            }
            if let Some(mode) = &item.proof_metadata.mode {
                lines.push(format!("    proof mode: {}", mode));
            }
            if let Some(command) = &item.proof_metadata.command {
                lines.push(format!("    proof command: {}", command));
            }
            if !item.excerpt_lines.is_empty() {
                lines.push(format!("    Excerpt ({} lines):", item.excerpt_lines.len()));
                for line in &item.excerpt_lines {
                    lines.push(format!("      {}", line));
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

    lines.push("  Linked proofs:".to_string());
    if report.linked_proofs.is_empty() {
        lines.push(format!("    {}", NO_LINKED_PROOFS_PLACEHOLDER.dimmed()));
    } else {
        for proof in &report.linked_proofs {
            lines.push(format!("    - {}", proof));
        }
    }

    lines.push("  Supplementary artifacts:".to_string());
    if report.supplementary_artifacts.is_empty() {
        lines.push(format!("    {}", NO_SUPPLEMENTARY_PLACEHOLDER.dimmed()));
    } else {
        for artifact in &report.supplementary_artifacts {
            lines.push(format!("    - {}", artifact));
        }
    }

    lines.push("  Media artifacts:".to_string());
    if report.media_artifacts.is_empty() {
        lines.push(format!("    {}", NO_MEDIA_PLACEHOLDER.dimmed()));
    } else {
        for media in &report.media_artifacts {
            let rel = format!("stories/{}/EVIDENCE/{}", story_id, media);
            lines.push(format!("    - {}", media));
            lines.push(format!("      Playback: ffplay -autoexit {}", rel));
        }
    }

    if !report.missing_proofs.is_empty() {
        lines.push(format!(
            "  Missing proofs: {}",
            report.missing_proofs.join(", ")
        ));
    }

    lines
}

fn render_evidence_report(story_id: &str, report: &EvidenceReport) {
    println!();
    for line in evidence_lines(story_id, report) {
        println!("{}", line);
    }
}

fn is_text_artifact(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".log", ".txt", ".md", ".json", ".yaml", ".yml", ".toml"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

fn is_media_artifact(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    [".gif", ".png", ".jpg", ".jpeg", ".webm", ".mp4", ".mov"]
        .iter()
        .any(|ext| lower.ends_with(ext))
}

/// Extract body content after frontmatter
fn extract_body(content: &str) -> Option<&str> {
    let mut delimiter_count = 0;

    for (idx, line) in content.lines().enumerate() {
        if line == "---" {
            delimiter_count += 1;
            if delimiter_count == 2 {
                let lines: Vec<&str> = content.lines().collect();
                if idx + 1 < lines.len() {
                    let prefix_len: usize = lines[..=idx].iter().map(|l| l.len() + 1).sum();
                    return Some(&content[prefix_len..]);
                }
            }
        }
    }

    None
}

/// Count checked and total acceptance criteria in body text
fn count_acs(body: &str) -> (usize, usize) {
    let mut checked = 0;
    let mut total = 0;

    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            checked += 1;
            total += 1;
        } else if trimmed.starts_with("- [ ]") {
            total += 1;
        }
    }

    (checked, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn show_displays_story() {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(TestVoyage::new("01-first", "test-epic").status("planned"))
            .story(
                TestStory::new("FEAT0001")
                    .title("Test Story")
                    .stage(StoryState::Backlog)
                    .scope("test-epic/01-first")
                    .body("\n# Test Story\n\n## Acceptance Criteria\n\n- [ ] First criterion\n"),
            )
            .build();

        run_with_dir(temp.path(), "FEAT0001").unwrap();
    }

    #[test]
    fn extract_body_works() {
        let content = "---\nid: test\n---\n\n# Body\n\nContent here.";
        let body = extract_body(content).unwrap();
        assert!(body.contains("# Body"));
        assert!(body.contains("Content here."));
    }

    #[test]
    fn count_acs_counts_correctly() {
        let body = "## Acceptance Criteria\n\n- [x] Done\n- [ ] Not done\n- [x] Also done\n";
        let (checked, total) = count_acs(body);
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
        assert_eq!(report.items[0].proof_metadata.mode.as_deref(), Some("command"));
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
        assert!(report.supplementary_artifacts.contains(&"notes.txt".to_string()));
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
        assert!(lines.iter().any(|line| line.contains(NO_EVIDENCE_DIR_PLACEHOLDER)));
        assert!(lines.iter().any(|line| line.contains(NO_LINKED_PROOFS_PLACEHOLDER)));
        assert!(lines.iter().any(|line| line.contains(NO_SUPPLEMENTARY_PLACEHOLDER)));
        assert!(lines.iter().any(|line| line.contains(NO_MEDIA_PLACEHOLDER)));
    }
}
