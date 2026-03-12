# Deterministic Voyage Artifact Rendering - Product Requirements

## Problem Statement

Voyage artifact generation still produces churn because proof discovery and board-wide sync ordering are not fully canonical, so repeated generation can touch unrelated files or rewrite reports without semantic changes.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make voyage reports and compliance reports byte-stable across repeated generation on the same board state. | Re-running voyage artifact generation without input changes produces identical output bytes. | 100% stable output in automated regression coverage |
| GOAL-02 | Remove nondeterministic ordering from board-level artifact sync paths that feed voyage artifacts. | Equivalent boards loaded in different insertion orders generate identical voyage artifact content. | Determinism proven in automated regression coverage |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | The agent or human running lifecycle commands that regenerate voyage artifacts. | Re-running generation should not create noisy diffs or unrelated churn. |
| Reviewer | The person reviewing generated reports and board diffs. | Generated artifact changes should reflect real semantic changes only. |
| Mission Steward | The planner coordinating mission progress and board health. | Board updates should stay clean enough that lifecycle state is easy to reason about. |

## Scope

### In Scope

- [SCOPE-01] Deterministic ordering of evidence and proof artifacts inside `VOYAGE_REPORT.md` and `COMPLIANCE_REPORT.md`.
- [SCOPE-02] Canonical iteration order for epics and voyages during board artifact sync where voyage artifacts are generated.
- [SCOPE-03] Regression coverage that proves repeated generation and equivalent board layouts produce identical voyage artifact output.

### Out of Scope

- [SCOPE-04] Frontier-scoped selective regeneration using `BoardGraph`.
- [SCOPE-05] Report schema redesigns, new generated artifact types, or stakeholder-facing content changes beyond stable ordering and normalization.
- [SCOPE-06] Background indexing, persisted graph storage, or non-voyage generation pipelines.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Canonicalize proof and evidence enumeration in voyage-generated reports so filesystem ordering cannot change the rendered markdown. | GOAL-01 | must | `read_dir` order is platform-dependent and currently causes report churn without semantic changes. |
| FR-02 | Canonicalize board sync iteration for epics and voyages before generator execution so equivalent board loads walk the same render order. | GOAL-02 | must | HashMap-backed iteration leaks insertion order into generation flow and makes deterministic output harder to guarantee. |
| FR-03 | Add regression coverage proving repeated generation and equivalent board layouts yield identical voyage artifact output. | GOAL-01, GOAL-02 | must | Determinism needs executable proof, not just code review confidence. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Preserve existing artifact filenames and markdown contracts for voyage reports and compliance reports. | GOAL-01 | must | This slice should eliminate churn without forcing downstream documentation or parser changes. |
| NFR-02 | Unchanged inputs must produce byte-identical generated voyage artifacts on the second sync. | GOAL-01, GOAL-02 | must | Idempotent generation is the concrete operational bar for stability. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Voyage report determinism | Unit tests that vary filesystem or insertion order and compare rendered output | Story-level targeted test proofs plus `just test` |
| Board sync determinism | Unit tests over equivalent boards and repeated sync calls | Story-level targeted test proofs plus `just test` |
| Repo safety | Full hygiene pass on the real board | `just quality`, `just test`, `just doctest`, `just keel doctor` |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Current report consumers care about stable ordering but not about preserving platform-specific enumeration quirks. | If false, tests could pass while downstream expectations drift. | Keep the content contract stable and review rendered output against current docs. |
| Voyage artifact churn is primarily caused by ordering and normalization issues rather than missing business data. | If false, this slice may reduce but not eliminate diff noise. | Verify repeated sync on unchanged boards is clean after the implementation. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Whether additional generated artifacts beyond voyage reports share the same ordering flaws. | Epic owner | Open |
| Whether section rewrite normalization contributes to the trailing-line churn observed in practice. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Re-running voyage artifact generation on unchanged input produces no content diff.
- [ ] Equivalent boards built with different insertion orders yield identical voyage artifact output in regression coverage.
- [ ] The real repo board passes `just keel doctor` and the full hygiene stack after lifecycle-triggered generation.
<!-- END SUCCESS_CRITERIA -->
