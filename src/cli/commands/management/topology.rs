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
    build_topology_output_with_width(board_dir, epic_id, include_done, get_terminal_width())
}

fn build_topology_output_with_width(
    board_dir: &Path,
    epic_id: &str,
    include_done: bool,
    width: usize,
) -> Result<String> {
    let board = load_board(board_dir)?;
    let epic = board.require_epic(epic_id)?;
    let projection =
        build_epic_topology_projection(&board, epic, TopologyBuildOptions { include_done })?;

    Ok(render_topology(&projection, include_done, width))
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

    const HOTSPOT_PRD: &str = r#"# PRD

## Problem Statement

Need topology hotspot review.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Surface topology drift. | Visible hotspots. | 1 command |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | Reviews epic flow. | See drift quickly. |

## Scope

### In Scope

- [SCOPE-01] Render active scope.
- [SCOPE-02] Track unmapped epic scope.

### Out of Scope

- [SCOPE-03] Pull out-of-scope work into the active slice.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Render hotspot-aware topology output. | GOAL-01 | must | Review active flow. |
| FR-02 | Surface uncovered parent requirements. | GOAL-01 | should | Reveal planning drift. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Keep hotspot ordering deterministic. | GOAL-01 | must | Stabilize reviews. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#;

    const HOTSPOT_SRS: &str = r#"# SRS

## Scope
In scope:
- [SCOPE-01] Render active scope.
- [SCOPE-99] Reference an unknown parent scope item.

Out of scope:
- [SCOPE-03] Pull a deferred item into this slice.

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Covered topology node. | SCOPE-01 | FR-01 | automated |
| SRS-02 | Uncovered voyage requirement. | SCOPE-01 | FR-01 | automated |
| SRS-03 | Story with missing proof coverage. | SCOPE-01 | FR-01 | automated |
| SRS-04 | Story with missing verification coverage. | SCOPE-01 | FR-01 | automated |
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

    fn hotspot_fixture() -> tempfile::TempDir {
        let temp = TestBoardBuilder::new()
            .epic(
                TestEpic::new("e1")
                    .title("Epic One With A Longer Title For Hotspot Summaries"),
            )
            .voyage(
                TestVoyage::new("v1", "e1")
                    .title("Voyage One With Drift Signals")
                    .status("planned")
                    .index(1)
                    .srs_content(HOTSPOT_SRS),
            )
            .story(
                TestStory::new("S1")
                    .title("Covered Story")
                    .scope("e1/v1")
                    .index(1)
                    .status(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] covered flow <!-- verify: cargo test --lib covered_story, SRS-01:start:end -->",
                    ),
            )
            .story(
                TestStory::new("S2")
                    .title("Story With Missing Proof")
                    .scope("e1/v1")
                    .index(2)
                    .status(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-03/AC-01] missing proof <!-- verify: cargo test --lib missing_proof_story, SRS-03:start:end, proof: missing.log -->",
                    ),
            )
            .story(
                TestStory::new("S3")
                    .title("Story Missing Verification Coverage")
                    .scope("e1/v1")
                    .index(3)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-04/AC-01] missing verify annotation"),
            )
            .build();
        write_epic_prd(temp.path(), "e1", HOTSPOT_PRD);
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

    #[test]
    fn topology_renderer_surfaces_scope_and_coverage_hotspots() {
        let temp = hotspot_fixture();

        let output = build_topology_output_with_width(temp.path(), "e1", false, 140).unwrap();

        assert!(output.contains("scope drift"));
        assert!(output.contains("SCOPE-02"));
        assert!(output.contains("uncovered PRD"));
        assert!(output.contains("FR-02"));
        assert!(output.contains("SCOPE-99"));
        assert!(output.contains("uncovered SRS"));
        assert!(output.contains("SRS-02"));
    }

    #[test]
    fn topology_renderer_surfaces_dependency_and_proof_gaps() {
        let temp = hotspot_fixture();

        let output = build_topology_output_with_width(temp.path(), "e1", false, 140).unwrap();

        assert!(output.contains("dependency block:"));
        assert!(output.contains("blocked by"));
        assert!(output.contains("S1"));
        assert!(output.contains("verification gap:"));
        assert!(output.contains("missing.log"));
        assert!(output.contains("no verification coverage"));
    }

    #[test]
    fn topology_renderer_summarizes_annotations_at_narrow_widths() {
        let temp = hotspot_fixture();

        let output = build_topology_output_with_width(temp.path(), "e1", false, 80).unwrap();

        assert!(output.contains("hotspot(s): scope drift; coverage gap"));
        assert!(output.contains("hotspot(s): dependency block; verification gap"));
        assert!(!output.contains("missing voyage scope mapping"));
    }
}
