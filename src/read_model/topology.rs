//! Canonical epic-topology projection for terminal planning and execution views.

use anyhow::Result;

use crate::domain::model::{Board, Epic, EpicState, StoryState, VoyageState};
use crate::infrastructure::utils::cmp_optional_index_then_id;
use crate::read_model::planning_show::{
    self, EpicShowProjection, StoryShowProjection, VoyageShowProjection,
};
use crate::read_model::traceability;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TopologyBuildOptions {
    pub include_done: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpicTopologyProjection {
    pub epic: EpicTopologyEpic,
    pub voyages: Vec<VoyageTopologyNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpicTopologyEpic {
    pub id: String,
    pub title: String,
    pub status: EpicState,
    pub index: Option<u32>,
    pub show: EpicShowProjection,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VoyageTopologyNode {
    pub id: String,
    pub title: String,
    pub status: VoyageState,
    pub index: Option<u32>,
    pub show: VoyageShowProjection,
    pub stories: Vec<StoryTopologyNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoryTopologyNode {
    pub id: String,
    pub title: String,
    pub status: StoryState,
    pub index: Option<u32>,
    pub show: StoryShowProjection,
    pub requirement_refs: Vec<String>,
    pub dependencies: Vec<String>,
    pub unmet_dependencies: Vec<String>,
}

pub fn build_epic_topology_projection(
    board: &Board,
    epic: &Epic,
    options: TopologyBuildOptions,
) -> Result<EpicTopologyProjection> {
    let epic_show = planning_show::build_epic_show_projection(board, epic)?;
    let traceability = traceability::build_matrix(board);
    let dependencies = traceability::derive_implementation_dependencies(board);

    let mut voyages = board.voyages_for_epic(epic);
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });

    let voyages = voyages
        .into_iter()
        .filter(|voyage| options.include_done || voyage.status() != VoyageState::Done)
        .map(|voyage| {
            let voyage_show = planning_show::build_voyage_show_projection(board, voyage)?;

            let mut stories = board.stories_for_voyage(voyage);
            stories.sort_by(|left, right| {
                cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
            });

            let stories = stories
                .into_iter()
                .filter(|story| options.include_done || story.status() != StoryState::Done)
                .map(|story| {
                    let show = planning_show::build_story_show_projection(story)?;
                    let requirement_refs = traceability
                        .story_to_requirements
                        .get(story.id())
                        .cloned()
                        .unwrap_or_default();
                    let story_dependencies =
                        dependencies.get(story.id()).cloned().unwrap_or_default();
                    let unmet_dependencies = story_dependencies
                        .iter()
                        .filter_map(|dependency_id| {
                            let dependency = board.require_story(dependency_id).ok()?;
                            (dependency.status() != StoryState::Done).then(|| dependency_id.clone())
                        })
                        .collect();

                    Ok(StoryTopologyNode {
                        id: story.id().to_string(),
                        title: story.title().to_string(),
                        status: story.status(),
                        index: story.index(),
                        show,
                        requirement_refs,
                        dependencies: story_dependencies,
                        unmet_dependencies,
                    })
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(VoyageTopologyNode {
                id: voyage.id().to_string(),
                title: voyage.title().to_string(),
                status: voyage.status(),
                index: voyage.index(),
                show: voyage_show,
                stories,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(EpicTopologyProjection {
        epic: EpicTopologyEpic {
            id: epic.id().to_string(),
            title: epic.title().to_string(),
            status: epic.status(),
            index: epic.index(),
            show: epic_show,
        },
        voyages,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::StoryState;
    use crate::infrastructure::loader::load_board;
    use crate::read_model::traceability;
    use crate::test_helpers::{TestBoardBuilder, TestEpic, TestStory, TestVoyage};
    use std::fs;
    use std::path::Path;

    fn topology_fixture_builder() -> (TestBoardBuilder, &'static str) {
        let srs = r#"# SRS
> Ship a topology projection.

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

        let prd = r#"# PRD

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

        (
            TestBoardBuilder::new()
            .epic(TestEpic::new("e1").title("Epic One"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .title("Voyage One")
                    .status("planned")
                    .index(1)
                    .srs_content(srs),
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
                    .title("Story Two")
                    .scope("e1/v1")
                    .index(2)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] preserve dependency order"),
            ),
            prd,
        )
    }

    fn write_epic_prd(board_root: &Path, epic_id: &str, content: &str) {
        fs::write(
            board_root.join("epics").join(epic_id).join("PRD.md"),
            content,
        )
        .unwrap();
    }

    #[test]
    fn topology_projection_builds_epic_voyage_story_graph() {
        let (builder, prd) = topology_fixture_builder();
        let temp = builder.build();
        write_epic_prd(temp.path(), "e1", prd);
        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();

        let projection =
            build_epic_topology_projection(&board, epic, TopologyBuildOptions::default()).unwrap();

        assert_eq!(projection.epic.id, "e1");
        assert_eq!(projection.epic.title, "Epic One");
        assert_eq!(projection.voyages.len(), 1);

        let voyage = &projection.voyages[0];
        assert_eq!(voyage.id, "v1");
        assert_eq!(voyage.title, "Voyage One");
        assert_eq!(voyage.stories.len(), 2);
        assert_eq!(voyage.stories[0].id, "S1");
        assert_eq!(voyage.stories[0].requirement_refs, vec!["SRS-01"]);
        assert_eq!(voyage.stories[1].id, "S2");
        assert_eq!(voyage.stories[1].requirement_refs, vec!["SRS-02"]);
        assert_eq!(voyage.stories[1].dependencies, vec!["S1"]);
        assert_eq!(voyage.stories[1].unmet_dependencies, vec!["S1"]);
    }

    #[test]
    fn topology_projection_is_deterministic_across_board_loads() {
        let srs = r#"# SRS

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

        let prd = r#"# PRD

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

        let board_a = TestBoardBuilder::new()
            .epic(TestEpic::new("e1").title("Epic One"))
            .voyage(
                TestVoyage::new("v1", "e1")
                    .title("Voyage One")
                    .status("planned")
                    .index(1)
                    .srs_content(srs),
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
                    .title("Story Two")
                    .scope("e1/v1")
                    .index(2)
                    .status(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] preserve dependency order",
                    ),
            )
            .build();
        write_epic_prd(board_a.path(), "e1", prd);
        let board_b = TestBoardBuilder::new()
            .story(
                TestStory::new("S2")
                    .title("Story Two")
                    .scope("e1/v1")
                    .index(2)
                    .status(StoryState::Backlog)
                    .body(
                        "## Acceptance Criteria\n\n- [ ] [SRS-02/AC-01] preserve dependency order",
                    ),
            )
            .epic(TestEpic::new("e1").title("Epic One"))
            .story(
                TestStory::new("S1")
                    .title("Story One")
                    .scope("e1/v1")
                    .index(1)
                    .status(StoryState::Backlog)
                    .body("## Acceptance Criteria\n\n- [ ] [SRS-01/AC-01] render node"),
            )
            .voyage(
                TestVoyage::new("v1", "e1")
                    .title("Voyage One")
                    .status("planned")
                    .index(1)
                    .srs_content(srs),
            )
            .build();
        write_epic_prd(board_b.path(), "e1", prd);

        let loaded_a = load_board(board_a.path()).unwrap();
        let loaded_b = load_board(board_b.path()).unwrap();

        let projection_a = build_epic_topology_projection(
            &loaded_a,
            loaded_a.require_epic("e1").unwrap(),
            TopologyBuildOptions::default(),
        )
        .unwrap();
        let projection_b = build_epic_topology_projection(
            &loaded_b,
            loaded_b.require_epic("e1").unwrap(),
            TopologyBuildOptions::default(),
        )
        .unwrap();

        assert_eq!(projection_a, projection_b);
    }

    #[test]
    fn topology_projection_reuses_canonical_read_models() {
        let (builder, prd) = topology_fixture_builder();
        let temp = builder.build();
        write_epic_prd(temp.path(), "e1", prd);
        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();
        let voyage = board.require_voyage("v1").unwrap();
        let story = board.require_story("S1").unwrap();

        let projection =
            build_epic_topology_projection(&board, epic, TopologyBuildOptions::default()).unwrap();
        let expected_epic = planning_show::build_epic_show_projection(&board, epic).unwrap();
        let expected_voyage = planning_show::build_voyage_show_projection(&board, voyage).unwrap();
        let expected_story = planning_show::build_story_show_projection(story).unwrap();
        let expected_matrix = traceability::build_matrix(&board);
        let expected_dependencies = traceability::derive_implementation_dependencies(&board);

        assert_eq!(projection.epic.show, expected_epic);
        assert_eq!(projection.voyages[0].show, expected_voyage);
        assert_eq!(projection.voyages[0].stories[0].show, expected_story);
        assert_eq!(
            projection.voyages[0].stories[0].requirement_refs,
            expected_matrix.story_to_requirements["S1"]
        );
        assert_eq!(
            projection.voyages[0].stories[1].dependencies,
            expected_dependencies["S2"]
        );
    }
}
