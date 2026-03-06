//! `keel topology` command adapter.

use std::path::Path;

use anyhow::Result;

use crate::cli::presentation::terminal::get_terminal_width;
use crate::cli::presentation::topology::render_topology;
use crate::infrastructure::loader::load_board;
use crate::read_model::topology::{TopologyBuildOptions, build_epic_topology_projection};

/// Run the topology command.
pub fn run(epic_id: &str, include_done: bool) -> Result<()> {
    let board_dir = crate::infrastructure::config::find_board_dir()?;
    run_with_dir(&board_dir, epic_id, include_done)
}

/// Run the topology command with an explicit board directory.
pub fn run_with_dir(board_dir: &Path, epic_id: &str, include_done: bool) -> Result<()> {
    let output = build_topology_output(board_dir, epic_id, include_done)?;
    print!("{output}");
    Ok(())
}

fn build_topology_output(board_dir: &Path, epic_id: &str, include_done: bool) -> Result<String> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(epic_id)?;
    let projection =
        build_epic_topology_projection(&board, epic, TopologyBuildOptions { include_done })?;

    Ok(render_topology(
        &projection,
        include_done,
        get_terminal_width(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use std::path::Path;

    const PRD: &str = r#"# PRD

## Problem Statement

Need an epic topology view.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Render topology. | Visible nodes. | 1 command |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | Reviews one epic. | See flow. |

## Scope

### In Scope

- [SCOPE-01] Render topology rows.

### Out of Scope

- [SCOPE-02] Export formats.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Render a topology view. | GOAL-01 | must | Required for epic review. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Keep output deterministic. | GOAL-01 | must | Required for trust. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#;

    const SRS: &str = r#"# SRS

## Scope
In scope:
- [SCOPE-01] Render projection rows.

Out of scope:
- [SCOPE-02] Rendering polish.

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Render projection nodes. | SCOPE-01 | FR-01 | automated |
| SRS-02 | Preserve dependency order. | SCOPE-01 | FR-01 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

    fn write_epic_prd(board_root: &Path, epic_id: &str, content: &str) {
        fs::write(
            board_root.join("epics").join(epic_id).join("PRD.md"),
            content,
        )
        .unwrap();
    }

    fn topology_fixture() -> tempfile::TempDir {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1").title("Epic One"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .title("Voyage One")
                    .status("planned")
                    .index(1)
                    .srs_content(SRS),
            )
            .voyage(
                TestVoyage::new("v2", "e1")
                    .title("Done Voyage")
                    .status("done")
                    .index(2)
                    .srs_content(SRS),
            )
            .story(
                TestStory::new("S1")
                    .title("Story One")
                    .scope("e1/v1")
                    .index(1)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] render node"),
            )
            .story(
                TestStory::new("S2")
                    .title("Done Story")
                    .scope("e1/v1")
                    .index(2)
                    .status(StoryState::Done)
                    .body(
                        "## Acceptance Criteria\n\n- [x] [SRS-02/AC-01] preserve dependency order",
                    ),
            )
            .story(
                TestStory::new("S3")
                    .title("Archived Story")
                    .scope("e1/v2")
                    .index(1)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] archived work"),
            )
            .build();
        write_epic_prd(temp.path(), "e1", PRD);
        temp
    }

    #[test]
    fn topology_command_invokes_epic_projection() {
        let temp = topology_fixture();

        let output = build_topology_output(temp.path(), "e1", false).unwrap();

        assert!(output.contains("Epic One"));
        assert!(output.contains("Topology"));
        assert!(output.contains("Voyage One"));
        assert!(output.contains("Story One"));
        assert!(output.contains("focused (planned + in-progress)"));
    }

    #[test]
    fn topology_command_hides_done_by_default() {
        let temp = topology_fixture();

        let output = build_topology_output(temp.path(), "e1", false).unwrap();

        assert!(output.contains("Voyage One"));
        assert!(output.contains("Story One"));
        assert!(!output.contains("Done Voyage"));
        assert!(!output.contains("Done Story"));
        assert!(!output.contains("Archived Story"));
    }

    #[test]
    fn topology_command_includes_done_when_requested() {
        let temp = topology_fixture();

        let output = build_topology_output(temp.path(), "e1", true).unwrap();

        assert!(output.contains("Voyage One"));
        assert!(output.contains("Story One"));
        assert!(output.contains("Done Voyage"));
        assert!(output.contains("Done Story"));
        assert!(output.contains("Archived Story"));
        assert!(output.contains("all entities (including done)"));
    }
}
