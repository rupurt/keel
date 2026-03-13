# RoadmapMVP - SRS

## Summary

Epic: VDiHw85WK
Goal: GOAL-01

## Scope

### In Scope

- [SCOPE-01] Add a deterministic roadmap view mode to management-facing output that surfaces priority, dependency blockers, and proceed/park posture for relevant board entities.
- [SCOPE-01] Add enough command/surface behavior so operators can move from ambiguous next-step recommendations to a canonical roadmap decision trail.
- [SCOPE-01] Ensure the roadmap view is derivable from existing board graph and workflow state without introducing a separate backlog model.

### Out of Scope

- [SCOPE-02] Creating or migrating existing board state formats.
- [SCOPE-02] Replacing management lane logic outside the roadmap output path.
- [SCOPE-02] Authoring a new interactive UI; textual outputs only.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Add a management command or output mode that renders mission/epic/voyage/story items as a roadmap with explicit proceed-vs-park posture for non-terminal bearings/entities. | SCOPE-01 | FR-01 | CLI test |
| SRS-02 | The roadmap view must include deterministic dependency context (blocking entity IDs + blockers count) for each displayed node and prioritize blocked states consistently. | SCOPE-01 | FR-01 | Unit test |
| SRS-03 | The roadmap output must remain readable in static CLI mode and avoid requiring raw file reads for decision-making. | SCOPE-01 | FR-01 | Manual review |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Roadmap output ordering and rendered text must be deterministic for identical board state across runs. | SCOPE-01 | NFR-01 | Automated CLI snapshot test |
| SRS-NFR-02 | Roadmap mode should not degrade existing `keel flow`/management command latency; regression must remain within current command expectations. | SCOPE-01 | NFR-01 | Benchmarked command test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
