# Explain Roles Lanes And Next Routing - SRS

## Summary

Epic: VF8hiVofm
Goal: Expose the configured workflow topology as an inspectable product surface and make role-scoped routing legible to humans and agents.

## Scope

### In Scope

- [SCOPE-04] Add a `keel roles` surface and `keel next --explain` so configured workflow topology and routing decisions become directly inspectable.

### Out of Scope

- [SCOPE-05] Changing the configured topology model or role taxonomy syntax.
- [SCOPE-06] Replacing `keel next` decision algorithms beyond explanation output.
- [SCOPE-07] Introducing authorization or policy changes beyond exposing current routing rules.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel roles` must expose configured role families, default lanes, operational contracts, and lane behavior in a form humans can inspect quickly. | SCOPE-04 | FR-05 | command regression + automated tests |
| SRS-02 | `keel roles --json` must return stable machine-readable role and lane data for harnesses and downstream tooling. | SCOPE-04 | FR-05 | automated tests |
| SRS-03 | `keel next --explain` must surface the resolved lane, queue type, parallel/manual-accept posture, and role-context contract for the selected role. | SCOPE-04 | FR-05 | command regression + automated tests |
| SRS-04 | Routing explanations must be derived from canonical workflow-topology and role-context projections rather than duplicated command-local heuristics. | SCOPE-04 | FR-05 | code review + tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The new roles and explain surfaces must not alter existing `keel next` decisions; they may only expose why the current routing resolved the way it did. | SCOPE-04 | NFR-02 | code review + tests |
| SRS-NFR-02 | Plain-text and JSON role explanations must remain deterministic across the same config and board state. | SCOPE-04 | NFR-01 | automated tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
