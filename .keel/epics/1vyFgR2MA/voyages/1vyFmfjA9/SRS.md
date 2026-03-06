# Goal-to-Requirement Lineage - Software Requirements Specification

> Connect canonical PRD goal IDs to FR/NFR requirements and surface orphaned, invalid, or missing goal links in planning diagnostics.

**Epic:** [1vyFgR2MA](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-05] Introduce canonical `GOAL-*` identifiers in PRD Goals & Objectives rows.
- [SCOPE-05] Link PRD FR/NFR requirement rows to one or more goal IDs.
- [SCOPE-05] Detect orphaned goals, invalid goal refs, and requirements with missing goal linkage.
- [SCOPE-05] Surface goal-to-requirement lineage in epic planning diagnostics and read views.

Out of scope:
- [SCOPE-04] CLI hydration of problem/goal seed content during `epic new`.
- [SCOPE-02] PRD-to-SRS FR/NFR lineage blocking.
- [SCOPE-06] Scope linkage and scope-drift rules.
- [SCOPE-07] Story-level lineage beyond PRD requirements.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| The PRD goals table can accept an explicit goal-ID column without harming readability. | Content model | Goal linkage may need a different markdown encoding. |
| PRD requirements can reference more than one goal ID where necessary. | Product model | One-to-many strategic intent would be underrepresented. |
| Goal linkage initially needs doctor/read-surface enforcement before any transition blocking. | Rollout | Broader runtime gating might need to move into this voyage. |

## Constraints

- Canonical goal IDs must use one parseable format across all epic PRDs.
- Requirement rows must remain human-readable while adding machine-checkable goal refs.
- Goal linkage diagnostics must stay deterministic and actionable.
- No legacy goal-link compatibility syntax should be added for the new contract.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | PRD Goals & Objectives entries MUST use canonical `GOAL-*` identifiers in a parseable table form. | FR-06 | parser tests + PRD fixture assertions |
| SRS-02 | PRD FR/NFR requirement rows MUST reference one or more valid `GOAL-*` identifiers. | FR-06 | requirement parser tests + invalid linkage fixtures |
| SRS-03 | Doctor diagnostics MUST report invalid goal references, orphaned goals, and PRD requirements with no goal linkage. | FR-06 | doctor regression tests + message assertions |
| SRS-04 | Epic planning read surfaces MUST summarize goal-to-requirement lineage so planners can review objective coverage directly from the PRD. | FR-06 | read-model tests + epic show snapshot tests |
| SRS-05 | Goal lineage parsing and reporting MUST preserve one-to-many goal fanout to requirements without unstable ordering. | FR-06 | fanout tests + deterministic projection fixtures |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Goal-lineage parsing and rendering MUST be deterministic for equivalent PRDs. | NFR-02 | deterministic parser and projection tests |
| SRS-NFR-02 | Goal-lineage failures MUST identify the offending goal ID, requirement ID, and artifact path. | NFR-03 | assertion tests on diagnostic messages |
| SRS-NFR-03 | The new goal-lineage contract MUST not introduce compatibility aliases for non-canonical goal tokens. | NFR-01 | negative tests for legacy/invalid goal tokens |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
