//! Canonical epic-topology projection for terminal planning and execution views.

use anyhow::Result;
use chrono::Utc;

use crate::domain::model::{Board, Epic, EpicState, StoryState, VoyageState};
use crate::infrastructure::utils::cmp_optional_index_then_id;
use crate::read_model::capacity;
use crate::read_model::knowledge::{self, DetectionConfig, RankedKnowledge};
use crate::read_model::planning_show::{
    self, EpicShowProjection, StoryShowProjection, VoyageShowProjection,
};
use crate::read_model::traceability;

const RECENT_INSIGHT_LIMIT: usize = 2;
const PENDING_KNOWLEDGE_LIMIT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TopologyBuildOptions {
    pub include_done: bool,
}

impl TopologyBuildOptions {
    pub fn includes_done_for(self, epic: &Epic) -> bool {
        self.include_done || epic.status() == EpicState::Done
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EpicTopologyProjection {
    pub epic: EpicTopologyEpic,
    pub voyages: Vec<VoyageTopologyNode>,
    pub recent_insights: Vec<TopologyKnowledgeAnnotation>,
    pub pending_knowledge: Vec<TopologyKnowledgeAnnotation>,
    pub horizon: Vec<HorizonCommentary>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KnowledgeAnnotationKind {
    RecentInsight,
    PendingKnowledge,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyKnowledgeAnnotation {
    pub kind: KnowledgeAnnotationKind,
    pub id: String,
    pub title: String,
    pub category: String,
    pub scope: Option<String>,
}

impl TopologyKnowledgeAnnotation {
    fn from_ranked(kind: KnowledgeAnnotationKind, ranked: &RankedKnowledge) -> Self {
        Self {
            kind,
            id: ranked.knowledge.id.clone(),
            title: ranked.knowledge.title.clone(),
            category: ranked.knowledge.category.clone(),
            scope: ranked.knowledge.scope.clone(),
        }
    }

    fn from_knowledge(kind: KnowledgeAnnotationKind, knowledge: &knowledge::Knowledge) -> Self {
        Self {
            kind,
            id: knowledge.id.clone(),
            title: knowledge.title.clone(),
            category: knowledge.category.clone(),
            scope: knowledge.scope.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizonCommentaryKind {
    Risk,
    Advisory,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HorizonCommentary {
    pub kind: HorizonCommentaryKind,
    pub signal: String,
    pub message: String,
}

impl HorizonCommentary {
    fn risk(signal: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: HorizonCommentaryKind::Risk,
            signal: signal.into(),
            message: message.into(),
        }
    }

    fn advisory(signal: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            kind: HorizonCommentaryKind::Advisory,
            signal: signal.into(),
            message: message.into(),
        }
    }
}

pub fn build_epic_topology_projection(
    board: &Board,
    epic: &Epic,
    options: TopologyBuildOptions,
) -> Result<EpicTopologyProjection> {
    let include_done = options.includes_done_for(epic);
    let epic_show = planning_show::build_epic_show_projection(board, epic)?;
    let traceability = traceability::build_matrix(board);
    let dependencies = traceability::derive_implementation_dependencies(board);

    let mut voyages = board.voyages_for_epic(epic);
    voyages.sort_by(|left, right| {
        cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
    });

    let voyages = voyages
        .into_iter()
        .filter(|voyage| include_done || voyage.status() != VoyageState::Done)
        .map(|voyage| {
            let voyage_show = planning_show::build_voyage_show_projection(board, voyage)?;

            let mut stories = board.stories_for_voyage(voyage);
            stories.sort_by(|left, right| {
                cmp_optional_index_then_id(left.index(), left.id(), right.index(), right.id())
            });

            let stories = stories
                .into_iter()
                .filter(|story| include_done || story.status() != StoryState::Done)
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

    let all_knowledge = knowledge::scan_all_knowledge(&board.root)?;
    let recent_insights = build_recent_insights(&all_knowledge, epic.id(), RECENT_INSIGHT_LIMIT);
    let pending_ranked = knowledge::rank_relevant_knowledge(
        all_knowledge.clone(),
        Some(epic.id()),
        None,
        PENDING_KNOWLEDGE_LIMIT,
    );
    let pending_knowledge = pending_ranked
        .iter()
        .map(|ranked| {
            TopologyKnowledgeAnnotation::from_ranked(
                KnowledgeAnnotationKind::PendingKnowledge,
                ranked,
            )
        })
        .collect();
    let horizon = build_horizon_commentary(
        board,
        epic,
        &epic_show,
        &voyages,
        &all_knowledge,
        &pending_ranked,
    );

    Ok(EpicTopologyProjection {
        epic: EpicTopologyEpic {
            id: epic.id().to_string(),
            title: epic.title().to_string(),
            status: epic.status(),
            index: epic.index(),
            show: epic_show,
        },
        voyages,
        recent_insights,
        pending_knowledge,
        horizon,
    })
}

fn build_recent_insights(
    all_knowledge: &[knowledge::Knowledge],
    epic_id: &str,
    limit: usize,
) -> Vec<TopologyKnowledgeAnnotation> {
    let mut recent: Vec<_> = all_knowledge
        .iter()
        .filter(|unit| knowledge_matches_epic(unit, epic_id))
        .filter(|unit| unit.is_applied())
        .cloned()
        .collect();

    if recent.is_empty() {
        recent = all_knowledge
            .iter()
            .filter(|unit| knowledge_matches_epic(unit, epic_id))
            .cloned()
            .collect();
    }

    recent.sort_by(compare_recent_knowledge);
    recent.truncate(limit);
    recent
        .iter()
        .map(|knowledge| {
            TopologyKnowledgeAnnotation::from_knowledge(
                KnowledgeAnnotationKind::RecentInsight,
                knowledge,
            )
        })
        .collect()
}

fn build_horizon_commentary(
    board: &Board,
    epic: &Epic,
    epic_show: &EpicShowProjection,
    voyages: &[VoyageTopologyNode],
    all_knowledge: &[knowledge::Knowledge],
    pending_ranked: &[RankedKnowledge],
) -> Vec<HorizonCommentary> {
    let mut horizon = Vec::new();

    let stories: Vec<_> = voyages
        .iter()
        .flat_map(|voyage| voyage.stories.iter())
        .collect();
    let stories_without_verification = stories
        .iter()
        .filter(|story| story.show.evidence.items.is_empty())
        .count();

    if epic_show.verification.missing_linked_proofs > 0 || stories_without_verification > 0 {
        let mut parts = Vec::new();
        if epic_show.verification.missing_linked_proofs > 0 {
            let count = epic_show.verification.missing_linked_proofs;
            let suffix = if count == 1 { "" } else { "s" };
            parts.push(format!("{count} missing linked proof{suffix}"));
        }
        if stories_without_verification > 0 {
            let suffix = if stories_without_verification == 1 {
                "y"
            } else {
                "ies"
            };
            parts.push(format!(
                "{} stor{} without verification coverage",
                stories_without_verification, suffix
            ));
        }

        horizon.push(HorizonCommentary::risk(
            "verification-debt",
            format!("verification debt: {}", parts.join("; ")),
        ));
        horizon.push(HorizonCommentary::advisory(
            "verification-debt",
            "record the missing proof artifacts and close uncovered verification before more stories depend on this flow",
        ));
    }

    let epic_capacity = capacity::project(board)
        .epics
        .into_iter()
        .find(|report| report.id == epic.id());
    let blocked = epic_capacity
        .as_ref()
        .map(|report| report.capacity.blocked)
        .unwrap_or(0);
    let ready = epic_capacity
        .as_ref()
        .map(|report| report.capacity.ready)
        .unwrap_or(0);

    if epic_show.eta.remaining_stories > 0 {
        if epic_show.eta.throughput_stories_per_week <= 0.0 {
            let mut message = format!(
                "ETA risk: {} remaining stor{} but no recent throughput signal; ETA is currently unknown",
                epic_show.eta.remaining_stories,
                if epic_show.eta.remaining_stories == 1 {
                    "y"
                } else {
                    "ies"
                }
            );
            if blocked > 0 {
                message.push_str(&format!(
                    "; {blocked} blocked stor{}",
                    if blocked == 1 { "y" } else { "ies" }
                ));
            }
            horizon.push(HorizonCommentary::risk("eta-risk", message));
            horizon.push(HorizonCommentary::advisory(
                "eta-risk",
                "finish or unblock one story to re-establish a trustworthy throughput signal",
            ));
        } else if let Some(eta_weeks) = epic_show.eta.eta_weeks
            && (eta_weeks >= 2.0 || blocked > 0)
        {
            let mut message = format!(
                "ETA risk: {:.1} weeks for {} remaining stor{} at {:.1} stories/week",
                eta_weeks,
                epic_show.eta.remaining_stories,
                if epic_show.eta.remaining_stories == 1 {
                    "y"
                } else {
                    "ies"
                },
                epic_show.eta.throughput_stories_per_week
            );
            if blocked > 0 {
                message.push_str(&format!(
                    "; {blocked} blocked stor{}",
                    if blocked == 1 { "y" } else { "ies" }
                ));
            }
            if ready == 0 {
                message.push_str("; ready queue is empty");
            }
            horizon.push(HorizonCommentary::risk("eta-risk", message));
            horizon.push(HorizonCommentary::advisory(
                "eta-risk",
                "reduce blockage on the visible chain before adding more work to this epic",
            ));
        }
    }

    let relevant_knowledge: Vec<_> = all_knowledge
        .iter()
        .filter(|unit| knowledge_matches_epic(unit, epic.id()))
        .cloned()
        .collect();
    let signals: Vec<_> = relevant_knowledge
        .iter()
        .filter_map(|unit| unit.to_signal())
        .collect();
    let patterns =
        knowledge::detect_rising_patterns(&signals, Utc::now(), &DetectionConfig::default());
    let mut emitted_focus_pattern = false;
    for pattern in patterns {
        let Some(focus_area) = pattern.focus_area() else {
            continue;
        };
        if !matches!(focus_area, "architecture" | "code" | "process") {
            continue;
        }

        emitted_focus_pattern = true;
        horizon.push(HorizonCommentary::risk(
            format!("pattern:{}", pattern.pattern_id()),
            format!(
                "{focus_area} debt signal: rising {focus_area} pattern (+{:.0}% across {} refs)",
                pattern.trend_delta() * 100.0,
                pattern.evidence_ids().len()
            ),
        ));
        horizon.push(HorizonCommentary::advisory(
            format!("pattern:{}", pattern.pattern_id()),
            format!("codify the recurring {focus_area} pattern in an ADR or bearing"),
        ));
    }

    if !emitted_focus_pattern {
        let pending_debt_ids: Vec<_> = pending_ranked
            .iter()
            .filter(|ranked| {
                matches!(
                    ranked.knowledge.category.as_str(),
                    "architecture" | "code" | "process"
                )
            })
            .map(|ranked| ranked.knowledge.id.clone())
            .collect();
        if !pending_debt_ids.is_empty() {
            horizon.push(HorizonCommentary::risk(
                "pending-tech-process-knowledge",
                format!(
                    "process debt signal: pending scoped knowledge {} is still unapplied",
                    pending_debt_ids.join(", ")
                ),
            ));
        }
    }

    let pending_ids: Vec<_> = pending_ranked
        .iter()
        .map(|ranked| ranked.knowledge.id.clone())
        .collect();
    if !pending_ids.is_empty() {
        horizon.push(HorizonCommentary::advisory(
            "pending-knowledge",
            format!(
                "review pending knowledge {} before continuing the epic flow",
                pending_ids.join(", ")
            ),
        ));
    }

    horizon
}

fn knowledge_matches_epic(unit: &knowledge::Knowledge, epic_id: &str) -> bool {
    unit.scope
        .as_deref()
        .is_some_and(|scope| scope == epic_id || scope.starts_with(&format!("{epic_id}/")))
}

fn compare_recent_knowledge(
    left: &knowledge::Knowledge,
    right: &knowledge::Knowledge,
) -> std::cmp::Ordering {
    let left_observed = left
        .observed_at
        .map(|value| value.timestamp())
        .unwrap_or(i64::MIN);
    let right_observed = right
        .observed_at
        .map(|value| value.timestamp())
        .unwrap_or(i64::MIN);
    let left_created = left
        .created_at
        .map(|value| value.and_utc().timestamp())
        .unwrap_or(i64::MIN);
    let right_created = right
        .created_at
        .map(|value| value.and_utc().timestamp())
        .unwrap_or(i64::MIN);

    right_observed
        .cmp(&left_observed)
        .then_with(|| right_created.cmp(&left_created))
        .then_with(|| left.id.cmp(&right.id))
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

    fn done_epic_fixture_builder() -> (TestBoardBuilder, &'static str) {
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
                .epic(TestEpic::new("e1").title("Completed Epic"))
                .voyage(
                    TestVoyage::new("v1", "e1")
                        .title("Done Voyage")
                        .status("done")
                        .index(1)
                        .srs_content(srs),
                )
                .story(
                    TestStory::new("S1")
                        .title("Done Story")
                        .scope("e1/v1")
                        .index(1)
                        .status(StoryState::Done)
                        .body("## Acceptance Criteria\n\n- [x] [SRS-01/AC-01] shipped node"),
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
    fn topology_projection_keeps_done_descendants_visible_for_done_epic() {
        let (builder, prd) = done_epic_fixture_builder();
        let temp = builder.build();
        write_epic_prd(temp.path(), "e1", prd);
        let board = load_board(temp.path()).unwrap();
        let epic = board.require_epic("e1").unwrap();

        let projection =
            build_epic_topology_projection(&board, epic, TopologyBuildOptions::default()).unwrap();

        assert_eq!(projection.epic.status, EpicState::Done);
        assert_eq!(projection.voyages.len(), 1);
        assert_eq!(projection.voyages[0].id, "v1");
        assert_eq!(projection.voyages[0].status, VoyageState::Done);
        assert_eq!(projection.voyages[0].stories.len(), 1);
        assert_eq!(projection.voyages[0].stories[0].id, "S1");
        assert_eq!(projection.voyages[0].stories[0].status, StoryState::Done);
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
