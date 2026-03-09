# Bearing - Product Requirements

## Problem Statement

Bearing entities currently have no explicit, durable lineage to the strategic epic they are intended to address. This breaks traceability for newly laid bearings: a created epic and its parent bearing can diverge, and planner-facing checks cannot assert that brief success criteria were intentionally tied back to epic goals.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Persist a durable bearing-to-epic lineage token during `keel bearing lay`. | Every laid bearing has a first-party lineage reference to an epic. | 100% of future lay operations |
| GOAL-02 | Link bearing success criteria to epic goals at lay time in a machine-readable form. | Every laid bearing records valid goal references from its `BRIEF.md` Success Criteria to the target epic goals. | 100% of eligible laid-bearing transitions |
| GOAL-03 | Make missing/invalid lineage actionable during doctor checks. | `keel doctor` reports specific lineage failures with clear remediation guidance before work can proceed. | 100% of malformed lineage states |
| GOAL-04 | Keep lineage strict and explicit. | Legacy inference paths are removed from critical transitions and checks are hard failures when lineage contracts are unmet. | No silent fallback behavior |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Researcher | Creates and advances bearings through brief/assessment/lay. | A reliable path from bearing intent to strategic execution. |
| Planner | Decomposes epics and runs execution with strict lineage contracts. | Verifiable proof that bearing work maps to epic goals. |
| Implementer | Executes stories generated from voyages and ships code. | Confidence that a strategic lineage boundary exists at lay time. |

## Scope

### In Scope

- [SCOPE-01] Add explicit `epic` lineage field(s) on layed bearings and persist them in bearing frontmatter.
- [SCOPE-02] Capture machine-readable goal references from bearing brief success criteria during `keel bearing lay`.
- [SCOPE-03] Add strict doctor checks for missing lineage and invalid goal references.
- [SCOPE-04] Provide clear user-facing remediation for lineage failures and stale/legacy values.
- [SCOPE-05] Add migration tooling/tests for already-laid or scaffolded historical bearings.

### Out of Scope

- [SCOPE-06] Change the bearing evidence/documents contract outside lineages.
- [SCOPE-07] Add automatic scoring or recommender behavior for success-criteria mapping.
- [SCOPE-08] Update ADR workflows or ADR-to-bearing contracts.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `keel bearing lay` MUST write a durable `epic` lineage reference on the bearing entity using the created/selected epic ID. | GOAL-01 | must | Makes lineage explicit at the moment the transition from research to strategy occurs. |
| FR-02 | `keel bearing lay` MUST parse `BRIEF.md` Success Criteria and persist validated links to epic goals in a machine-readable frontmatter field. | GOAL-02 | must | Keeps strategic intent and discovery output connected for downstream checks. |
| FR-03 | Laid bearings with missing or invalid goal references MUST fail in a strict validation path, with explicit remediation in diagnostics. | GOAL-03 | must | Prevents silent data drift and untraceable lineage after transitions. |
| FR-04 | Existing `keel bearing lay` outputs and existing laid bearings must be auditable after migration, with one explicit correction flow for historical data. | GOAL-03 GOAL-04 | should | Preserves operational continuity while removing implicit assumptions from lineage checks. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Validation failures for lineage must be deterministic, testable, and fail fast without best-effort fallbacks. | GOAL-03 GOAL-04 | must | Keeps lineage checks reliable and non-ambiguous. |
| NFR-02 | `keel bearing lay` lineage behavior must remain idempotent for repeated reads and repeated show/render paths. | GOAL-01 GOAL-02 | should | Prevents UI and evidence surfaces from oscillating due to derived state differences. |
| NFR-03 | Error messaging for lineage failures must include artifact path, offending token, and correction command. | GOAL-03 | should | Improves operator feedback and reduces debug time. |
| NFR-04 | Migration and doctor checks should scale to existing board sizes without introducing non-linear regressions in the command path. | GOAL-04 | should | Keeps day-2 behavior stable for existing historical projects. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Command path | `cargo test` on bearing lay transition and migration helpers | Story-level acceptance evidence |
| Structural diagnostics | Doctor + transition gate tests across valid/invalid scenarios | Deterministic regression tests |
| Template/readability | PR templates and show surfaces | Smoke render tests + doc assertions |
| Migration | Board fixture tests for pre-existing bearings | Fixture migration tests and evidence logs |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| `epic` creation flow can still be changed by other commands. | Lineage references could be outdated if not reconciled. | Keep lineage validation in doctor and block inconsistent transitions. |
| Brief success-criteria IDs are not present or are not formatted consistently. | Lineage capture may fail for some bearings. | Require explicit format during lay path and provide clear remediation. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How should legacy laid bearings without clean goal references be repaired without manual editing? | Epic owner | Open |
| Should lineage capture be required for all bearings or only research-phase bearings that become strategic? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Every `keel bearing lay` operation writes a stable `epic` lineage and records valid goal references from `BRIEF.md`.
- [ ] `keel doctor` hard-fails and gives actionable remediation when laid-bearing lineage is missing or invalid.
- [ ] Migration path is covered for existing bearings that were laid before this contract and legacy behavior is observable.
<!-- END SUCCESS_CRITERIA -->
