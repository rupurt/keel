//! README generation for the board
//!
//! Generates markdown files for the board, epics, and voyages.

use anyhow::{Result, anyhow};

use crate::domain::model::{Board, Epic, Voyage};
use crate::infrastructure::utils::cmp_optional_index_then_id;

pub mod artifact_io;
pub mod board_readme;
pub mod compliance_report;
pub mod epic_readme;
pub mod knowledge_synthesis;
pub mod sections;
pub mod voyage_artifacts;
pub mod voyage_readme;
pub mod voyage_report;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BoardArtifactSyncOptions {
    pub synthesize_knowledge_for_voyage: Option<String>,
}

/// Regenerate all persisted board artifacts from the current board snapshot.
pub fn sync_board_artifacts(board: &Board, options: &BoardArtifactSyncOptions) -> Result<()> {
    board_readme::generate(board)?;

    for epic in ordered_epics_for_sync(board) {
        epic_readme::generate(board, epic)?;
    }

    let mut issues = Vec::new();
    for voyage in ordered_voyages_for_sync(board) {
        let outcome = voyage_artifacts::sync(
            board,
            voyage,
            voyage_artifacts::SyncOptions {
                synthesize_knowledge: options.synthesize_knowledge_for_voyage.as_deref()
                    == Some(voyage.id()),
            },
        );
        issues.extend(outcome.issues.into_iter().map(|issue| {
            format!(
                "voyage {} {}: {}",
                voyage.id(),
                issue.artifact,
                issue.message
            )
        }));
    }

    if !issues.is_empty() {
        return Err(anyhow!(
            "failed to sync generated voyage artifacts:\n{}",
            issues.join("\n")
        ));
    }

    Ok(())
}

fn ordered_epics_for_sync(board: &Board) -> Vec<&Epic> {
    let mut epics: Vec<_> = board.epics.values().collect();
    epics.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    epics
}

fn ordered_voyages_for_sync(board: &Board) -> Vec<&Voyage> {
    let mut voyages: Vec<_> = board.voyages.values().collect();
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });
    voyages
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};

    fn voyage_fixture(order: &[(&str, u32)], story_order: &[(&str, u32)]) -> tempfile::TempDir {
        let srs = r#"# SRS

## Functional Requirements
<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Stable report output | SCOPE-01 | FR-01 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;
        let mut builder = TestBoardBuilder::new()
            .epic(TestEpic::new("E0").index(0))
            .voyage(
                TestVoyage::new("V0", "E0")
                    .status("done")
                    .index(0)
                    .srs_content(srs),
            );

        for (epic_id, index) in order {
            builder = builder.epic(TestEpic::new(epic_id).index(*index)).voyage(
                TestVoyage::new(&format!("{epic_id}-voyage"), epic_id)
                    .status("done")
                    .index(*index)
                    .srs_content(srs),
            );
        }

        for (story_id, index) in story_order {
            builder = builder.story(
                TestStory::new(story_id)
                    .title(&format!("Story {story_id}"))
                    .scope("E0/V0")
                    .index(*index)
                    .status(StoryState::Done)
                    .body(
                        "# Summary\n\nStable report.\n\n## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] stable <!-- verify: manual, SRS-01:start:end -->\n",
                    ),
            );
        }

        let temp = builder.build();
        for (story_id, _) in story_order {
            let evidence_dir = temp.path().join("stories").join(story_id).join("EVIDENCE");
            fs::create_dir_all(&evidence_dir).unwrap();
            fs::write(evidence_dir.join("zeta.log"), "proof").unwrap();
            fs::write(evidence_dir.join("alpha.log"), "proof").unwrap();
        }
        temp
    }

    #[test]
    fn sync_board_artifacts_uses_canonical_order() {
        let board_a =
            load_board(voyage_fixture(&[("E2", 2), ("E1", 1)], &[("S2", 2), ("S1", 1)]).path())
                .unwrap();
        let board_b =
            load_board(voyage_fixture(&[("E1", 1), ("E2", 2)], &[("S1", 1), ("S2", 2)]).path())
                .unwrap();

        let epic_ids_a: Vec<_> = ordered_epics_for_sync(&board_a)
            .iter()
            .map(|epic| epic.id().to_string())
            .collect();
        let epic_ids_b: Vec<_> = ordered_epics_for_sync(&board_b)
            .iter()
            .map(|epic| epic.id().to_string())
            .collect();
        let voyage_ids_a: Vec<_> = ordered_voyages_for_sync(&board_a)
            .iter()
            .map(|voyage| voyage.id().to_string())
            .collect();
        let voyage_ids_b: Vec<_> = ordered_voyages_for_sync(&board_b)
            .iter()
            .map(|voyage| voyage.id().to_string())
            .collect();

        assert_eq!(epic_ids_a, epic_ids_b);
        assert_eq!(voyage_ids_a, voyage_ids_b);
    }

    #[test]
    fn voyage_artifact_generation_is_idempotent() {
        let temp_a = voyage_fixture(&[("E2", 2), ("E1", 1)], &[("S2", 2), ("S1", 1)]);
        let board_a = load_board(temp_a.path()).unwrap();
        sync_board_artifacts(&board_a, &BoardArtifactSyncOptions::default()).unwrap();
        let voyage_report_a = fs::read_to_string(
            temp_a
                .path()
                .join("epics")
                .join("E0")
                .join("voyages")
                .join("V0")
                .join("VOYAGE_REPORT.md"),
        )
        .unwrap();
        let compliance_report_a = fs::read_to_string(
            temp_a
                .path()
                .join("epics")
                .join("E0")
                .join("voyages")
                .join("V0")
                .join("COMPLIANCE_REPORT.md"),
        )
        .unwrap();

        sync_board_artifacts(&board_a, &BoardArtifactSyncOptions::default()).unwrap();
        let voyage_report_a_repeat = fs::read_to_string(
            temp_a
                .path()
                .join("epics")
                .join("E0")
                .join("voyages")
                .join("V0")
                .join("VOYAGE_REPORT.md"),
        )
        .unwrap();
        let compliance_report_a_repeat = fs::read_to_string(
            temp_a
                .path()
                .join("epics")
                .join("E0")
                .join("voyages")
                .join("V0")
                .join("COMPLIANCE_REPORT.md"),
        )
        .unwrap();

        let temp_b = voyage_fixture(&[("E1", 1), ("E2", 2)], &[("S1", 1), ("S2", 2)]);
        let board_b = load_board(temp_b.path()).unwrap();
        sync_board_artifacts(&board_b, &BoardArtifactSyncOptions::default()).unwrap();
        let voyage_report_b = fs::read_to_string(
            temp_b
                .path()
                .join("epics")
                .join("E0")
                .join("voyages")
                .join("V0")
                .join("VOYAGE_REPORT.md"),
        )
        .unwrap();
        let compliance_report_b = fs::read_to_string(
            temp_b
                .path()
                .join("epics")
                .join("E0")
                .join("voyages")
                .join("V0")
                .join("COMPLIANCE_REPORT.md"),
        )
        .unwrap();

        assert_eq!(voyage_report_a, voyage_report_a_repeat);
        assert_eq!(compliance_report_a, compliance_report_a_repeat);
        assert_eq!(voyage_report_a, voyage_report_b);
        assert_eq!(compliance_report_a, compliance_report_b);
    }
}
