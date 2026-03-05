//! Voyage and epic lifecycle application service.
//!
//! This module owns orchestration for voyage and epic lifecycle transitions so
//! CLI command handlers can remain thin interface adapters.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow, bail};
use chrono::Local;

use crate::domain::model::VoyageState;
use crate::domain::state_machine::{
    EnforcementPolicy, TransitionEntity, TransitionIntent, VoyageTransition, enforce_transition,
    format_enforcement_error,
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

        if let Err(e) =
            crate::infrastructure::generate::knowledge_synthesis::synthesize_voyage_knowledge(
                &refreshed_board,
                refreshed_voyage,
            )
        {
            eprintln!("Warning: Failed to synthesize voyage knowledge: {}", e);
        }

        println!("Completed voyage: {}", voyage.id());

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::VoyageEpicLifecycleService;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;

    fn write_prd(temp: &tempfile::TempDir, epic_id: &str, content: &str) {
        fs::write(temp.path().join(format!("epics/{epic_id}/PRD.md")), content).unwrap();
    }

    #[test]
    fn start_voyage_enforces_requirements_coverage_when_not_forced() {
        let srs = r#"# Test SRS

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First requirement | FR-01 | test |
| SRS-02 | Second requirement | FR-02 | test |
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
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->",
                    ),
            )
            .build();

        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | First requirement | must | test |
| FR-02 | Second requirement | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

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
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | First requirement | FR-01 | test |
| SRS-02 | Second requirement | FR-02 | test |
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
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] First criteria <!-- verify: manual -->",
                    ),
            )
            .build();

        write_prd(
            &temp,
            "test-epic",
            r#"# PRD

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | First requirement | must | test |
| FR-02 | Second requirement | must | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#,
        );

        let result = VoyageEpicLifecycleService::start_voyage(temp.path(), "01-draft", true, None);
        assert!(
            result.is_ok(),
            "forced start should bypass requirements coverage enforcement: {result:?}"
        );
    }
}
