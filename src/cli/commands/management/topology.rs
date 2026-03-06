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
    use crate::infrastructure::loader::load_board;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use chrono::{Duration, Utc};
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

    const KNOWLEDGE_PRD: &str = r#"# PRD

## Problem Statement

Need topology knowledge and horizon review.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Surface execution learning and approaching risk. | Knowledge and horizon shown. | 1 command |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | Reviews epic flow. | See knowledge and risk in one place. |

## Scope

### In Scope

- [SCOPE-01] Render topology flow.
- [SCOPE-02] Surface scoped knowledge and horizon commentary.

### Out of Scope

- [SCOPE-03] Whole-board aggregation.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Render knowledge-aware topology output. | GOAL-01 | must | Operators need execution learning. |
| FR-02 | Render horizon commentary from board signals. | GOAL-01 | must | Operators need early warnings. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Reuse canonical read models. | GOAL-01 | must | Keep commentary deterministic. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
"#;

    const KNOWLEDGE_SRS: &str = r#"# SRS

## Scope
In scope:
- [SCOPE-01] Render topology flow.
- [SCOPE-02] Surface scoped knowledge and horizon commentary.

Out of scope:
- [SCOPE-03] Whole-board aggregation.

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Visible story with proof debt. | SCOPE-01 | FR-01 | automated |
| SRS-02 | Visible story without verification coverage. | SCOPE-01 | FR-01 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->
"#;

    fn write_epic_prd(board_root: &Path, epic_id: &str, content: &str) {
        fs::write(
            board_root.join("epics").join(epic_id).join("PRD.md"),
            content,
        )
        .unwrap();
    }

    fn write_story_reflect(board_root: &Path, story_id: &str, content: &str) {
        fs::write(
            board_root.join("stories").join(story_id).join("REFLECT.md"),
            content,
        )
        .unwrap();
    }

    struct ReflectKnowledgeSpec<'a> {
        title: &'a str,
        knowledge_id: &'a str,
        category: &'a str,
        insight: &'a str,
        suggested_action: &'a str,
        applied: &'a str,
        observed_at: chrono::DateTime<Utc>,
        score: f64,
        confidence: f64,
    }

    fn reflect_with_knowledge(spec: ReflectKnowledgeSpec<'_>) -> String {
        format!(
            "---\ncreated_at: {}\n---\n\n# Reflection - {}\n\n## Knowledge\n\n### {}: {}\n| Field | Value |\n|-------|-------|\n| **Category** | {} |\n| **Context** | topology horizon |\n| **Insight** | {} |\n| **Suggested Action** | {} |\n| **Applies To** | src/cli/presentation/topology.rs |\n| **Linked Knowledge IDs** | |\n| **Observed At** | {} |\n| **Score** | {:.2} |\n| **Confidence** | {:.2} |\n| **Applied** | {} |\n\n## Observations\n\nScoped topology should surface this signal.\n",
            spec.observed_at.format("%Y-%m-%dT%H:%M:%S"),
            spec.title,
            spec.knowledge_id,
            spec.title,
            spec.category,
            spec.insight,
            spec.suggested_action,
            spec.observed_at.to_rfc3339(),
            spec.score,
            spec.confidence,
            spec.applied,
        )
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

    fn knowledge_horizon_fixture() -> tempfile::TempDir {
        let temp = TestBoardBuilder::new()
            .epic(TestEpic::new("e1").title("Epic One With Knowledge"))
            .epic(TestEpic::new("e2").title("Other Epic"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .title("Voyage One With Horizon Signals")
                    .status("in-progress")
                    .index(1)
                    .srs_content(KNOWLEDGE_SRS),
            )
            .voyage(
                TestVoyage::new("v2", "e2")
                    .title("Other Voyage")
                    .status("planned")
                    .index(1)
                    .srs_content(KNOWLEDGE_SRS),
            )
            .story(
                TestStory::new("S1")
                    .title("Story Missing Proof")
                    .scope("e1/v1")
                    .index(1)
                    .status(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] missing proof <!-- verify: cargo test --lib missing_proof_story, SRS-01:start:end, proof: missing.log -->",
                    ),
            )
            .story(
                TestStory::new("S2")
                    .title("Story Missing Verification Coverage")
                    .scope("e1/v1")
                    .index(2)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] missing verify annotation"),
            )
            .story(
                TestStory::new("K1")
                    .title("Applied Verification Insight")
                    .scope("e1/v1")
                    .index(3)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] shipped"),
            )
            .story(
                TestStory::new("K2")
                    .title("Pending Process Insight One")
                    .scope("e1/v1")
                    .index(4)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] shipped"),
            )
            .story(
                TestStory::new("K3")
                    .title("Pending Process Insight Two")
                    .scope("e1/v1")
                    .index(5)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] shipped"),
            )
            .story(
                TestStory::new("K4")
                    .title("Pending Process Insight Three")
                    .scope("e1/v1")
                    .index(6)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] shipped"),
            )
            .story(
                TestStory::new("X1")
                    .title("Unrelated Epic Insight")
                    .scope("e2/v2")
                    .index(1)
                    .status(StoryState::Done)
                    .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] shipped"),
            )
            .build();
        write_epic_prd(temp.path(), "e1", KNOWLEDGE_PRD);
        write_epic_prd(temp.path(), "e2", KNOWLEDGE_PRD);

        let now = Utc::now();
        write_story_reflect(
            temp.path(),
            "K1",
            &reflect_with_knowledge(ReflectKnowledgeSpec {
                title: "Applied Verification Insight",
                knowledge_id: "1AbCdE240",
                category: "testing",
                insight: "Recent applied insights should stay visible in topology.",
                suggested_action: "Keep showing completed verification lessons near active work.",
                applied: "Applied in topology renderer",
                observed_at: now - Duration::days(1),
                score: 0.62,
                confidence: 0.85,
            }),
        );
        write_story_reflect(
            temp.path(),
            "K2",
            &reflect_with_knowledge(ReflectKnowledgeSpec {
                title: "Pending Process Insight One",
                knowledge_id: "1AbCdE241",
                category: "process",
                insight: "Pending process insights should warn operators about review debt.",
                suggested_action: "Review the pending checklist before starting the next story.",
                applied: "",
                observed_at: now - Duration::days(6),
                score: 0.50,
                confidence: 0.82,
            }),
        );
        write_story_reflect(
            temp.path(),
            "K3",
            &reflect_with_knowledge(ReflectKnowledgeSpec {
                title: "Pending Process Insight Two",
                knowledge_id: "1AbCdE242",
                category: "process",
                insight: "Recurring process lessons should escalate into horizon commentary.",
                suggested_action: "Group related process insights into one advisory recommendation.",
                applied: "",
                observed_at: now - Duration::days(3),
                score: 0.74,
                confidence: 0.87,
            }),
        );
        write_story_reflect(
            temp.path(),
            "K4",
            &reflect_with_knowledge(ReflectKnowledgeSpec {
                title: "Pending Process Insight Three",
                knowledge_id: "1AbCdE243",
                category: "process",
                insight: "A rising process pattern should point to accumulating process debt.",
                suggested_action: "Codify the recurring process issue in an ADR or bearing.",
                applied: "",
                observed_at: now - Duration::days(1),
                score: 0.96,
                confidence: 0.93,
            }),
        );
        write_story_reflect(
            temp.path(),
            "X1",
            &reflect_with_knowledge(ReflectKnowledgeSpec {
                title: "Unrelated Epic Insight",
                knowledge_id: "1AbCdE299",
                category: "code",
                insight: "Other-epic knowledge should not leak into scoped topology output.",
                suggested_action: "Ignore this while reviewing epic e1.",
                applied: "",
                observed_at: now - Duration::days(1),
                score: 0.80,
                confidence: 0.90,
            }),
        );

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

    #[test]
    fn topology_annotations_surface_scoped_knowledge() {
        let temp = knowledge_horizon_fixture();

        let output = build_topology_output_with_width(temp.path(), "e1", false, 140).unwrap();

        assert!(output.contains("Knowledge"));
        assert!(output.contains("recent insight"));
        assert!(output.contains("Applied Verification Insight"));
        assert!(output.contains("pending knowledge"));
        assert!(output.contains("1AbCdE241"));
        assert!(output.contains("Pending Process Insight Three"));
        assert!(!output.contains("Unrelated Epic Insight"));
    }

    #[test]
    fn topology_horizon_surfaces_verification_and_eta_risk() {
        let temp = knowledge_horizon_fixture();

        let output = build_topology_output_with_width(temp.path(), "e1", false, 140).unwrap();

        assert!(output.contains("Horizon"));
        assert!(output.contains("verification debt"));
        assert!(output.contains("ETA risk"));
        assert!(output.contains("missing linked proof"));
        assert!(output.contains("no recent throughput"));
        assert!(output.contains("advisory:"));
        assert!(output.contains("process debt"));
    }

    #[test]
    fn topology_horizon_reuses_knowledge_helpers() {
        let temp = knowledge_horizon_fixture();
        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();

        let projection =
            build_epic_topology_projection(&board, epic, TopologyBuildOptions::default()).unwrap();
        let all_knowledge = crate::read_model::knowledge::scan_all_knowledge(temp.path()).unwrap();
        let expected_pending = crate::read_model::knowledge::rank_relevant_knowledge(
            all_knowledge.clone(),
            Some("e1"),
            None,
            3,
        );
        let expected_pending_ids: Vec<_> = expected_pending
            .iter()
            .map(|ranked| ranked.knowledge.id.clone())
            .collect();
        let projected_pending_ids: Vec<_> = projection
            .pending_knowledge
            .iter()
            .map(|annotation| annotation.id.clone())
            .collect();
        assert_eq!(projected_pending_ids, expected_pending_ids);

        let relevant_signals: Vec<_> = all_knowledge
            .iter()
            .filter(|unit| {
                unit.scope
                    .as_deref()
                    .is_some_and(|scope| scope == "e1" || scope.starts_with("e1/"))
            })
            .filter_map(|unit| unit.to_signal())
            .collect();
        let patterns = crate::read_model::knowledge::detect_rising_patterns(
            &relevant_signals,
            Utc::now(),
            &crate::read_model::knowledge::DetectionConfig::default(),
        );

        assert!(
            patterns.iter().any(|pattern| projection
                .horizon
                .iter()
                .any(|entry| { entry.signal == format!("pattern:{}", pattern.pattern_id()) })),
            "expected projection horizon to retain navigator pattern ids"
        );
    }
}
