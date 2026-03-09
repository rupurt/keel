# Flow Integration - Software Requirements Specification

> keel next mission-awareness, keel flow mission progress, CHARTER.md goal parsing

**Epic:** [1vzeJF000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] `keel next --agent` mission-awareness — check active mission goals before returning "no work"
- [SCOPE-02] `keel next --agent` recommends creating bearings/epics when queue empty but mission incomplete
- [SCOPE-03] `keel flow` mission-level progress summary when missions exist
- [SCOPE-04] `keel mission refine` CHARTER.md completeness analysis (question generation)
- [SCOPE-05] AGENTS.md template updates — document mission workflow for harnesses

### Out of Scope

- [SCOPE-90] Multi-mission priority scheduling
- [SCOPE-91] External metrics API integration for goal verification

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| V1 domain foundation complete (Mission in Board) | Dependency | Cannot query mission state |
| V2 CLI commands complete (mission lifecycle) | Dependency | No mission to be aware of |
| V3 lineage and doctor checks complete (goal evaluation) | Dependency | Cannot evaluate mission completeness |

## Constraints

- `keel next --agent` must remain fast — mission check adds minimal overhead
- Mission-awareness must not change behavior when no missions exist on the board
- AGENTS.md template must be backward compatible with existing harness workflows

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel next --agent` checks for active missions when no stories are ready; if mission goals are unmet, returns recommendation to create next work unit instead of empty result | SCOPE-01, SCOPE-02 | FR-15 | integration test |
| SRS-02 | `keel next --agent` recommendation includes mission context: which goals are unmet, what type of work to create (bearing vs epic vs voyage) | SCOPE-02 | FR-15 | integration test |
| SRS-03 | `keel flow` includes mission progress section when active missions exist: title, status, goal summary (N/M board goals met), child entity counts | SCOPE-03 | FR-16 | integration test |
| SRS-04 | `keel flow` mission section omitted when no missions exist (backward compatible) | SCOPE-03 | FR-16 | integration test |
| SRS-05 | `keel mission refine` analyzes CHARTER.md Goals, Constraints, and Halting Rules sections for completeness and generates contextual questions | SCOPE-04 | FR-05 | unit test |
| SRS-06 | AGENTS.md template includes Mission workflow section documenting `keel mission new/refine/activate` and the autonomous delivery loop | SCOPE-05 | FR-07 | manual inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | `keel next --agent` latency must not increase by more than 50ms when missions exist | SCOPE-01 | NFR-02 | benchmark |
| SRS-NFR-02 | Mission-aware output is deterministic across repeated invocations | SCOPE-01 | NFR-01 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
