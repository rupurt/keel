# Lineage And Doctor - Software Requirements Specification

> Mission lineage field on child entities, doctor checks for completion and integrity

**Epic:** [1vzeJF000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope

- [SCOPE-01] Optional `mission` field on EpicFrontmatter, BearingFrontmatter, AdrFrontmatter
- [SCOPE-02] Loader support for parsing `mission` field from YAML frontmatter
- [SCOPE-03] CHARTER.md goal table parsing — extract MG-XX goals with verification types
- [SCOPE-04] Doctor check: MissionGoalAchieved — all board-verifiable goals met
- [SCOPE-05] Doctor check: MissionActiveNoWork — active mission with no in-flight work
- [SCOPE-06] Doctor check: MissionOrphanedLineage — entity references nonexistent mission
- [SCOPE-07] Doctor check: MissionStale — active mission with no board changes (warning)
- [SCOPE-08] Transition gating — `keel mission achieve` blocked when board goals unmet

### Out of Scope

- [SCOPE-90] CLI commands beyond gating logic (V2)
- [SCOPE-91] `keel next` and `keel flow` integration (V4)

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| V1 domain foundation complete (Mission in Board) | Dependency | Cannot validate mission references |
| V2 CLI commands complete (transition gating needs command surface) | Dependency | Gating logic has no consumer |
| Existing doctor check infrastructure (configured_check, CheckId, Problem) | Dependency | Need to extend existing patterns |

## Constraints

- Doctor checks must follow existing `configured_check()` pattern
- New CheckId variants must be added to the enum
- Lineage field must be optional — entities can exist without missions

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Add optional `mission: Option<String>` to EpicFrontmatter, BearingFrontmatter, AdrFrontmatter | SCOPE-01 | FR-09 | unit test |
| SRS-02 | Loader parses `mission` field from YAML frontmatter for epics, bearings, and ADRs | SCOPE-02 | FR-09 | unit test |
| SRS-03 | Parse CHARTER.md Goals table extracting MG-XX IDs, descriptions, and verification types (board, metric, manual) | SCOPE-03 | FR-03 | unit test |
| SRS-04 | Doctor check MissionGoalAchieved: for each active mission, evaluate all `board:` goals against board state; flag Info when all pass | SCOPE-04 | FR-12 | unit test |
| SRS-05 | Doctor check MissionActiveNoWork: warn when mission is Active but no mission-scoped epics or bearings are in non-terminal state | SCOPE-05 | FR-13 | unit test |
| SRS-06 | Doctor check MissionOrphanedLineage: error when epic/bearing/ADR has `mission` field referencing nonexistent mission ID | SCOPE-06 | FR-14 | unit test |
| SRS-07 | `keel mission achieve` gate: reject transition when any `board:` goal in CHARTER.md is unmet | SCOPE-08 | FR-08 | integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Doctor checks must be deterministic and reproducible across repeated runs | SCOPE-04 | NFR-01 | unit test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
