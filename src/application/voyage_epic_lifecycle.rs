//! Voyage and epic lifecycle application service.
//!
//! This module owns orchestration for voyage and epic lifecycle transitions so
//! CLI command handlers can remain thin interface adapters.

use std::fmt::Write;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use chrono::Local;

use crate::application::domain_events::DomainEvent;
use crate::application::process_manager::DomainProcessManager;
use crate::domain::model::{EpicState, VoyageState};
use crate::domain::state_machine::{
    EnforcementPolicy, TransitionEntity, TransitionIntent, VoyageTransition, classify_findings,
    enforce_transition, format_enforcement_error, format_transition_error,
};
use crate::domain::transitions::{TimestampUpdates, update_frontmatter};
use crate::infrastructure::frontmatter_mutation::{Mutation, apply};
use crate::infrastructure::loader::load_board;

pub struct VoyageEpicLifecycleService;

impl VoyageEpicLifecycleService {
    /// Start a voyage (draft/planned -> in-progress).
    pub fn start_voyage(
        board_dir: &Path,
        id: &str,
        force: bool,
        expect_version: Option<u64>,
    ) -> Result<()> {
        let board = load_board(board_dir)?;

        // Check version if provided (SRS-05: optimistic locking)
        if let Some(expected) = expect_version {
            let actual = board.snapshot_version();
            if actual != expected {
                bail!(
                    "Board version mismatch: expected {}, actual {} - reload and retry",
                    expected,
                    actual
                );
            }
        }

        let voyage = board.require_voyage(id)?;
        let transition = if voyage.status() == VoyageState::Draft {
            VoyageTransition::Plan
        } else {
            VoyageTransition::Start
        };

        let policy = EnforcementPolicy {
            require_requirements_coverage: !force,
            ..EnforcementPolicy::RUNTIME
        };
        let intent = TransitionIntent::Voyage(transition);
        let enforcement =
            enforce_transition(&board, TransitionEntity::Voyage(voyage), intent, policy);
        if !enforcement.allows_transition() {
            bail!(format_enforcement_error(
                &format!("voyage {}", voyage.id()),
                intent,
                &enforcement.blocking_problems
            ));
        }

        let content = fs::read_to_string(&voyage.path)
            .with_context(|| format!("Failed to read voyage: {}", voyage.path.display()))?;

        let mut mutations = vec![Mutation::set("status", VoyageState::InProgress.to_string())];
        if voyage.frontmatter.started_at.is_none() {
            let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
            mutations.push(Mutation::set("started_at", now));
        }

        let updated_content = apply(&content, &mutations);

        fs::write(&voyage.path, updated_content)
            .with_context(|| format!("Failed to write voyage: {}", voyage.path.display()))?;

        println!("Started voyage: {}", voyage.id());

        crate::cli::commands::generate::run(board_dir)?;

        Ok(())
    }

    /// Complete a voyage (in-progress -> done).
    pub fn complete_voyage(
        board_dir: &Path,
        id: &str,
        well: Option<String>,
        hard: Option<String>,
        different: Option<String>,
    ) -> Result<()> {
        let board = load_board(board_dir)?;
        let voyage = board.require_voyage(id)?;

        let intent = TransitionIntent::Voyage(VoyageTransition::Complete);
        let enforcement = enforce_transition(
            &board,
            TransitionEntity::Voyage(voyage),
            intent,
            EnforcementPolicy::RUNTIME,
        );
        if !enforcement.allows_transition() {
            return Err(anyhow!(format_enforcement_error(
                &format!("voyage {}", voyage.id()),
                intent,
                &enforcement.blocking_problems
            )));
        }

        let content = fs::read_to_string(&voyage.path)
            .with_context(|| format!("Failed to read voyage: {}", voyage.path.display()))?;

        let updated_content = update_frontmatter(
            &content,
            VoyageState::Done,
            &TimestampUpdates::voyage_completed(),
        )?;
        let updated_content = add_retrospective(&updated_content, well, hard, different);

        fs::write(&voyage.path, updated_content)
            .with_context(|| format!("Failed to write voyage: {}", voyage.path.display()))?;

        // Reload so follow-on artifacts use the persisted done state and latest story evidence.
        let refreshed_board = load_board(board_dir)?;
        let refreshed_voyage = refreshed_board.require_voyage(id)?;

        if let Err(e) = crate::infrastructure::generate::voyage_report::generate(
            board_dir,
            &refreshed_board,
            refreshed_voyage,
        ) {
            eprintln!("Warning: Failed to generate VOYAGE_REPORT.md: {}", e);
        }

        if let Err(e) = crate::infrastructure::generate::compliance_report::generate(
            board_dir,
            &refreshed_board,
            refreshed_voyage,
        ) {
            eprintln!("Warning: Failed to generate COMPLIANCE_REPORT.md: {}", e);
        }

        if let Err(e) = generate_press_release(&refreshed_board, refreshed_voyage) {
            eprintln!("Warning: Failed to generate PRESS_RELEASE.md: {}", e);
        }

        if let Err(e) =
            crate::infrastructure::generate::knowledge_synthesis::synthesize_voyage_knowledge(
                &refreshed_board,
                refreshed_voyage,
            )
        {
            eprintln!("Warning: Failed to synthesize voyage knowledge: {}", e);
        }

        println!("Completed voyage: {}", voyage.id());

        DomainProcessManager::default().handle(
            board_dir,
            DomainEvent::VoyageCompleted {
                voyage_id: voyage.id().to_string(),
                epic_id: voyage.epic_id.clone(),
            },
        )?;

        Ok(())
    }

    /// Keep epic status synchronized after a voyage reaches done.
    pub(crate) fn sync_epic_after_voyage_completion(board_dir: &Path, epic_id: &str) -> Result<()> {
        auto_tactical_epic_if_needed(board_dir, epic_id)?;
        let completed_epic = auto_complete_epic_if_needed(board_dir, epic_id)
            .context("Failed to sync epic status after completing voyage")?;

        // complete_epic() regenerates the board when it runs.
        if !completed_epic {
            crate::cli::commands::generate::run(board_dir)?;
        }

        Ok(())
    }

    /// Complete an epic (strategic/tactical -> done).
    pub fn complete_epic(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let epic = board.require_epic(id)?;

        if epic.status() == EpicState::Done {
            return Err(anyhow!("Epic {} is already done", epic.id()));
        }

        let gate_problems = crate::domain::state_machine::evaluate_epic_done(&board, epic);
        let blocking = classify_findings(EnforcementPolicy::STRICT, &gate_problems);
        if !blocking.is_empty() {
            return Err(anyhow!(format_transition_error(
                &format!("epic {}", epic.id()),
                "complete",
                &blocking
            )));
        }

        let content = fs::read_to_string(&epic.path)
            .with_context(|| format!("Failed to read epic: {}", epic.path.display()))?;
        let updated_content = update_epic_done_frontmatter(&content)?;

        fs::write(&epic.path, updated_content)
            .with_context(|| format!("Failed to write epic: {}", epic.path.display()))?;

        println!("Completed epic: {}", epic.id());

        crate::cli::commands::generate::run(board_dir)?;

        Ok(())
    }

    /// Reopen an epic (done -> strategic).
    pub fn reopen_epic(board_dir: &Path, id: &str) -> Result<()> {
        let board = load_board(board_dir)?;
        let epic = board.require_epic(id)?;

        if epic.status() != EpicState::Done {
            return Err(anyhow!(
                "Epic {} is not done (status: {})",
                epic.id(),
                epic.status()
            ));
        }

        let content = fs::read_to_string(&epic.path)
            .with_context(|| format!("Failed to read epic: {}", epic.path.display()))?;
        let updated_content = update_epic_reopen_frontmatter(&content)?;

        fs::write(&epic.path, updated_content)
            .with_context(|| format!("Failed to write epic: {}", epic.path.display()))?;

        println!("Reopened epic: {}", epic.id());

        crate::cli::commands::generate::run(board_dir)?;

        Ok(())
    }
}

/// If all voyages in the epic are done, auto-complete the epic.
fn auto_complete_epic_if_needed(board_dir: &Path, epic_id: &str) -> Result<bool> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(epic_id)?;

    if epic.status() == EpicState::Done {
        return Ok(false);
    }

    let voyages = board.voyages_for_epic(epic);
    if voyages.is_empty() {
        return Ok(false);
    }

    let all_done = voyages.iter().all(|v| v.status() == VoyageState::Done);
    if !all_done {
        return Ok(false);
    }

    VoyageEpicLifecycleService::complete_epic(board_dir, epic_id)?;
    Ok(true)
}

/// If epic is strategic but should be tactical, update it.
fn auto_tactical_epic_if_needed(board_dir: &Path, epic_id: &str) -> Result<()> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(epic_id)?;

    if epic.status() != EpicState::Strategic {
        return Ok(());
    }

    let voyages = board.voyages_for_epic(epic);
    let has_tactical_voyage = voyages.iter().any(|v| v.status() != VoyageState::Draft);

    if has_tactical_voyage {
        let content = fs::read_to_string(&epic.path)?;
        let updated = apply(&content, &[Mutation::set("status", "tactical")]);
        fs::write(&epic.path, updated)?;
    }

    Ok(())
}

/// Generate a high-fidelity PRESS_RELEASE.md for the voyage.
fn generate_press_release(
    board: &crate::domain::model::Board,
    voyage: &crate::domain::model::Voyage,
) -> Result<()> {
    let mut output = String::new();
    writeln!(output, "# PRESS RELEASE: {}", voyage.title()).unwrap();
    writeln!(output).unwrap();
    writeln!(output, "## Overview").unwrap();
    if let Some(goal) = voyage.frontmatter.goal.as_ref() {
        writeln!(output, "> {}", goal).unwrap();
    }
    writeln!(output).unwrap();

    let stories = board.stories_for_voyage(voyage);

    // 1. Executive Summary (Synthesized from stories)
    writeln!(output, "## Narrative Summary").unwrap();
    for story in &stories {
        let content = fs::read_to_string(&story.path)?;
        if let Some(summary) = extract_story_summary(&content) {
            writeln!(output, "### {}", story.title()).unwrap();
            writeln!(output, "{}", summary).unwrap();
            writeln!(output).unwrap();
        }
    }

    // 2. Insights & Lessons (From REFLECT.md)
    writeln!(output, "## Key Insights").unwrap();
    for story in &stories {
        let story_dir = story.path.parent().unwrap();
        let reflect_path = story_dir.join("REFLECT.md");
        if reflect_path.exists() {
            let reflect_content = fs::read_to_string(reflect_path)?;
            writeln!(output, "### Insights from {}", story.title()).unwrap();
            writeln!(output, "{}", reflect_content.trim()).unwrap();
            writeln!(output).unwrap();
        }
    }

    // 3. Evidence & Proof (vhs recordings, LLM transcripts)
    writeln!(output, "## Verification Proof").unwrap();
    for story in &stories {
        let story_dir = story.path.parent().unwrap();
        let evidence_dir = story_dir.join("EVIDENCE");
        if evidence_dir.exists() {
            let entries = fs::read_dir(evidence_dir)?;
            let mut story_evidence = Vec::new();
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    story_evidence.push(filename);
                }
            }

            if !story_evidence.is_empty() {
                writeln!(output, "### Proof for {}", story.title()).unwrap();
                for filename in story_evidence {
                    let rel_path =
                        format!("../../../../stories/{}/EVIDENCE/{}", story.id(), filename);
                    if filename.ends_with(".gif") {
                        writeln!(output, "![{}]({})", filename, rel_path).unwrap();
                    } else {
                        writeln!(output, "- [{}]({})", filename, rel_path).unwrap();
                    }
                }
                writeln!(output).unwrap();
            }
        }
    }

    let release_path = voyage.path.parent().unwrap().join("PRESS_RELEASE.md");
    fs::write(release_path, output)?;

    Ok(())
}

/// Extract summary section from story README.
fn extract_story_summary(content: &str) -> Option<String> {
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

/// Add retrospective section to content if any fields are provided.
fn add_retrospective(
    content: &str,
    well: Option<String>,
    hard: Option<String>,
    different: Option<String>,
) -> String {
    if well.is_none() && hard.is_none() && different.is_none() {
        return content.to_string();
    }

    let mut result = content.to_string();

    while result.ends_with("\n\n") {
        result.pop();
    }
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push('\n');

    result.push_str("## Retrospective\n\n");

    if let Some(w) = well {
        result.push_str(&format!("**What went well:** {}\n\n", w));
    }
    if let Some(h) = hard {
        result.push_str(&format!("**What was harder than expected:** {}\n\n", h));
    }
    if let Some(d) = different {
        result.push_str(&format!("**What would you do differently:** {}\n\n", d));
    }

    result
}

/// Update epic frontmatter status to done.
fn update_epic_done_frontmatter(content: &str) -> Result<String> {
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    Ok(apply(
        content,
        &[
            Mutation::set("status", "done"),
            Mutation::set("completed_at", now),
        ],
    ))
}

/// Update epic frontmatter status to strategic and remove completed timestamp.
fn update_epic_reopen_frontmatter(content: &str) -> Result<String> {
    Ok(apply(
        content,
        &[
            Mutation::set("status", "strategic"),
            Mutation::remove("completed_at"),
            Mutation::remove("completed"),
        ],
    ))
}

#[cfg(test)]
mod tests {
    use super::VoyageEpicLifecycleService;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    #[test]
    fn start_voyage_enforces_requirements_coverage_when_not_forced() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | First requirement | test |
| SRS-02 | Second requirement | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-draft")
                    .body("- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->"),
            )
            .build();

        let err = VoyageEpicLifecycleService::start_voyage(temp.path(), "01-draft", false, None)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("SRS-02"),
            "non-forced start should fail on uncovered requirements: {err}"
        );
    }

    #[test]
    fn start_voyage_force_bypasses_requirements_coverage_enforcement() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Verification |
|----|-------------|--------------|
| SRS-01 | First requirement | test |
| SRS-02 | Second requirement | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("test-epic"))
            .voyage(
                TestVoyage::new("01-draft", "test-epic")
                    .status("draft")
                    .srs_content(srs),
            )
            .story(
                TestStory::new("STORY01")
                    .scope("test-epic/01-draft")
                    .body("- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->"),
            )
            .build();

        let result = VoyageEpicLifecycleService::start_voyage(temp.path(), "01-draft", true, None);
        assert!(
            result.is_ok(),
            "forced start should bypass requirements coverage enforcement: {result:?}"
        );
    }
}
