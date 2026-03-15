//! Bearing transition execution engine.
//!
//! Handles bearing state transitions with side effects (file creation).

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};
use chrono::Local;

use crate::domain::model::{Bearing, BearingStatus};
use crate::infrastructure::loader::load_board;
use crate::infrastructure::template_rendering;

use super::bearing_spec::{BearingTransitionSpec, TransitionSideEffect};

/// Result of a successful bearing transition.
#[derive(Debug)]
pub struct BearingTransitionResult {
    /// The bearing ID
    #[allow(dead_code)] // Available for logging/debugging
    pub bearing_id: String,
    /// Status before transition
    pub from: BearingStatus,
    /// Status after transition
    pub to: BearingStatus,
    /// File created as side effect (if any)
    pub file_created: Option<String>,
}

/// Execute a bearing state transition.
///
/// This handles:
/// 1. Loading the board and finding the bearing
/// 2. Validating the transition is valid from current status
/// 3. Executing side effects (file creation)
/// 4. Updating the bearing's BRIEF.md with new status
pub fn execute(
    board_dir: &Path,
    bearing_pattern: &str,
    spec: &BearingTransitionSpec,
) -> Result<BearingTransitionResult> {
    let board = load_board(board_dir)?;

    // Find the bearing
    let bearing = board.require_bearing(bearing_pattern)?;

    // Validate transition
    validate_transition(spec, bearing)?;

    let from_status = bearing.frontmatter.status;
    let bearing_id = bearing.id().to_string();
    let bearing_dir = board_dir.join("bearings").join(&bearing_id);

    // Execute side effect (file creation) BEFORE status update
    let file_created = execute_side_effect(&bearing_dir, bearing, &spec.side_effect)?;

    // Update status in BRIEF.md
    update_bearing_status(board_dir, bearing, spec.to)?;

    Ok(BearingTransitionResult {
        bearing_id,
        from: from_status,
        to: spec.to,
        file_created,
    })
}

/// Validate that a transition is allowed from the bearing's current status.
fn validate_transition(spec: &BearingTransitionSpec, bearing: &Bearing) -> Result<()> {
    if !spec.is_valid_source(bearing.frontmatter.status) {
        let valid_sources: Vec<_> = spec.from.iter().map(|s| s.to_string()).collect();
        return Err(anyhow!(
            "Cannot {} bearing '{}' from '{}' state (must be {})",
            spec.name,
            bearing.id(),
            bearing.frontmatter.status,
            valid_sources.join(" or ")
        ));
    }

    if matches!(spec.to, BearingStatus::Evaluating | BearingStatus::Ready) {
        let problems =
            crate::infrastructure::validation::bearings::check_bearing_content_sections_for_bearing(
                bearing,
            );
        if !problems.is_empty() {
            return Err(anyhow!(
                "{}\nRepair `bearings/{}/BRIEF.md` before rerunning `keel bearing {}`.",
                problems
                    .into_iter()
                    .map(|problem| problem.message)
                    .collect::<Vec<_>>()
                    .join("\n"),
                bearing.id(),
                spec.name
            ));
        }
    }

    Ok(())
}

/// Execute the side effect for a transition.
fn execute_side_effect(
    bearing_dir: &Path,
    bearing: &Bearing,
    side_effect: &TransitionSideEffect,
) -> Result<Option<String>> {
    match side_effect {
        TransitionSideEffect::None => Ok(None),
        TransitionSideEffect::CreateFile { template, filename } => {
            let file_path = bearing_dir.join(filename);

            // New bearing scaffolds already materialize the document bundle, so
            // lifecycle transitions should be idempotent if the target file exists.
            if file_path.exists() {
                return Ok(None);
            }

            // Render template
            let content = template_rendering::render(
                template,
                &[
                    ("id", bearing.id()),
                    ("title", &bearing.frontmatter.title),
                    (
                        "created_at",
                        &Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
                    ),
                ],
            );

            // Write file
            fs::write(&file_path, content).with_context(|| {
                format!("Failed to write {}: {}", filename, file_path.display())
            })?;

            Ok(Some(file_path.to_string_lossy().to_string()))
        }
    }
}

/// Update the status in a bearing's README.md
fn update_bearing_status(
    board_dir: &Path,
    bearing: &Bearing,
    new_status: BearingStatus,
) -> Result<()> {
    let readme_path = board_dir
        .join("bearings")
        .join(bearing.id())
        .join("README.md");

    let content = fs::read_to_string(&readme_path)
        .with_context(|| format!("Failed to read README.md: {}", readme_path.display()))?;

    // Replace the status line in frontmatter
    let old_status = format!("status: {}", bearing.frontmatter.status);
    let new_status_line = format!("status: {}", new_status);
    let updated = content.replace(&old_status, &new_status_line);

    fs::write(&readme_path, updated)
        .with_context(|| format!("Failed to write README.md: {}", readme_path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::transitions::bearing_spec::bearing_transitions;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_bearing_with_status(temp: &TempDir, status: &str) -> PathBuf {
        let root = temp.path();
        let bearing_dir = root.join("bearings/test-research");
        fs::create_dir_all(&bearing_dir).unwrap();

        fs::write(
            bearing_dir.join("README.md"),
            format!(
                r#"---
id: test-research
title: Test Research
status: {}
---
"#,
                status
            ),
        )
        .unwrap();

        fs::write(
            bearing_dir.join("BRIEF.md"),
            r#"# Test Research — Brief

## Hypothesis

The research thread is worth pursuing because it targets a repeated operator pain point.

## Problem Space

The team needs a concrete problem statement before a bearing can advance cleanly.

## Success Criteria

- [ ] The bearing has enough framing to justify deeper research.

## Open Questions

- What evidence would materially change the recommendation?
"#,
        )
        .unwrap();

        root.to_path_buf()
    }

    #[test]
    fn research_creates_file_and_updates_status() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "exploring");

        let result = execute(&board_dir, "test-research", &bearing_transitions::RESEARCH).unwrap();

        assert_eq!(result.from, BearingStatus::Exploring);
        assert_eq!(result.to, BearingStatus::Evaluating);
        assert!(result.file_created.is_some());

        // Verify file was created
        let evidence_path = board_dir.join("bearings/test-research/EVIDENCE.md");
        assert!(evidence_path.exists());

        // Verify status was updated
        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: evaluating"));
    }

    #[test]
    fn research_fails_if_wrong_state() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "evaluating");

        let result = execute(&board_dir, "test-research", &bearing_transitions::RESEARCH);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must be exploring")
        );
    }

    #[test]
    fn research_allows_existing_file() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "exploring");

        // Create file first
        fs::write(
            board_dir.join("bearings/test-research/EVIDENCE.md"),
            "existing",
        )
        .unwrap();

        let result = execute(&board_dir, "test-research", &bearing_transitions::RESEARCH).unwrap();
        assert_eq!(result.to, BearingStatus::Evaluating);
        assert!(result.file_created.is_none());
    }

    #[test]
    fn assess_creates_file_and_updates_status() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "evaluating");

        let result = execute(&board_dir, "test-research", &bearing_transitions::ASSESS).unwrap();

        assert_eq!(result.from, BearingStatus::Evaluating);
        assert_eq!(result.to, BearingStatus::Ready);
        assert!(result.file_created.is_some());

        // Verify file was created
        let assessment_path = board_dir.join("bearings/test-research/ASSESSMENT.md");
        assert!(assessment_path.exists());

        // Verify status was updated
        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: ready"));
    }

    #[test]
    fn assess_fails_if_wrong_state() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "exploring");

        let result = execute(&board_dir, "test-research", &bearing_transitions::ASSESS);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("must be evaluating")
        );
    }

    #[test]
    fn research_rejects_scaffold_brief_sections() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "exploring");
        fs::write(
            board_dir.join("bearings/test-research/BRIEF.md"),
            r#"# Test Research — Brief

## Hypothesis

What we believe and why it might matter.

## Problem Space

What problem or opportunity are we investigating?

## Success Criteria

How will we know if this research was valuable?

- [ ] Criterion 1
- [ ] Criterion 2

## Open Questions

- Question we need to answer
"#,
        )
        .unwrap();

        let result = execute(&board_dir, "test-research", &bearing_transitions::RESEARCH);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("BRIEF.md Hypothesis scaffold"));
        assert!(error.contains("BRIEF.md Problem Space scaffold"));
    }

    #[test]
    fn park_updates_status_only() {
        let temp = TempDir::new().unwrap();
        let board_dir = create_test_bearing_with_status(&temp, "exploring");

        let result = execute(&board_dir, "test-research", &bearing_transitions::PARK).unwrap();

        assert_eq!(result.from, BearingStatus::Exploring);
        assert_eq!(result.to, BearingStatus::Parked);
        assert!(result.file_created.is_none());

        // Verify status was updated
        let readme =
            fs::read_to_string(board_dir.join("bearings/test-research/README.md")).unwrap();
        assert!(readme.contains("status: parked"));
    }
}
