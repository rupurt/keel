# Dependency-Aware Bearing Prioritization - Product Requirements

## Problem Statement

Keel's bearing prioritization ranks research by EV score (impact, confidence, effort, risk) but ignores dependencies between bearings. When bearing B depends on findings from bearing A, the system cannot recommend that A be researched first. Operators must manually track inter-bearing dependencies, which breaks down as the board grows.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Surface dependency-aware sequencing recommendations for active bearings. | `keel bearing list` and `keel next` reflect dependency ordering when present. | Bearings with unresolved upstream dependencies sort below their prerequisites. |
| GOAL-02 | Allow operators to declare explicit dependency edges between bearings. | Bearing frontmatter supports a `depends_on` field validated by `keel doctor`. | Doctor flags dangling or cyclic dependency references as errors. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Agent Operator | AI or human executing tactical moves on the board. | Clear next-research recommendation that accounts for sequencing constraints. |
| Manager | Human or AI planning research and prioritizing the bearing backlog. | Confidence that research ordering won't produce orphaned findings. |

## Scope

### In Scope

- [SCOPE-01] Add `depends_on: Vec<String>` to BearingFrontmatter and parse it during board load.
- [SCOPE-02] Validate dependency references in `keel doctor` (existence, no cycles, no self-references).
- [SCOPE-03] Factor dependency state into bearing sort order: bearings whose dependencies are unresolved sort lower.

### Out of Scope

- [SCOPE-04] Automatic inference of dependencies from content analysis.
- [SCOPE-05] Cross-entity dependencies (bearing-to-epic, bearing-to-voyage).
- [SCOPE-06] Visualization of the dependency graph (future voyage).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | BearingFrontmatter must support an optional `depends_on` field containing a list of bearing IDs. | GOAL-02 | must | Provides the structural primitive for dependency edges. |
| FR-02 | `keel doctor` must validate that every ID in `depends_on` references an existing bearing and that the dependency graph is acyclic. | GOAL-02 | must | Prevents dangling references and infinite loops in sequencing. |
| FR-03 | Bearing sort order in `keel bearing list` and `keel next` must demote bearings whose `depends_on` targets are not in a terminal state (laid, declined, parked). | GOAL-01 | must | Ensures recommended sequencing respects dependency constraints. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Dependency validation in doctor must scale linearly with the number of bearings. | GOAL-02 | must | Prevents performance regression as the board grows. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| FR-01 | Unit test | Board loader test with `depends_on` in frontmatter |
| FR-02 | Unit test | Doctor tests for dangling refs, cycles, self-references |
| FR-03 | Unit test + manual | Priority sort tests; manual CLI verification |
| NFR-01 | Unit test | Linear scaling test with N bearings |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Bearing dependencies are sparse (most bearings have 0-2 deps). | Cycle detection cost grows; may need optimized algorithm. | Monitor board state as bearings accumulate. |
| Terminal states (laid, declined, parked) represent resolved dependencies. | A laid bearing whose epic fails would leave dependents unblocked incorrectly. | Acceptable for now; revisit if epic failure loops emerge. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should `depends_on` accept non-bearing IDs (e.g., external signals)? | Epic owner | Resolved: No, keep it bearing-to-bearing only for SCOPE-01. |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] An operator can declare `depends_on: [BRG-A]` on bearing BRG-B and see BRG-A recommended first.
- [ ] `keel doctor` rejects cyclic and dangling dependency references.
<!-- END SUCCESS_CRITERIA -->
