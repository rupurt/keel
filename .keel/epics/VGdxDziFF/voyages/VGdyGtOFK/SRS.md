# Specify Stack-Aware Turn Next And Doctor Contracts - SRS

## Summary

Epic: VGdxDziFF
Goal: Define stack-aware outputs and gating behavior for turn, next, mission next, and doctor while preserving repo-local heartbeat semantics.

## Scope

### In Scope

- [SCOPE-01] Add Mission Stack context to `keel turn`.
- [SCOPE-02] Add stack-aware routing and blocking decisions to `keel next`.
- [SCOPE-03] Add linked member mission and negotiation visibility to `keel mission next --status`.
- [SCOPE-04] Add Mission Stack diagnostics to `keel doctor`.
- [SCOPE-05] Define stable text and JSON output expectations for these surfaces.

### Out of Scope

- [SCOPE-90] Managed worktree mechanics and cleanup.
- [SCOPE-91] Visual stack dashboards beyond the existing command surfaces.
- [SCOPE-92] Replacing repo-local `keel heartbeat` semantics with stack-global state.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `keel turn` must expose stack id, local member state, branch, active mode, and checkpoint status when the repo is part of a Mission Stack. | SCOPE-01 | FR-01 | manual |
| SRS-02 | `keel next` must produce stack-aware blocking or redirect decisions when the current repo cannot act. | SCOPE-02 | FR-02 | manual |
| SRS-03 | `keel mission next --status` must describe linked member missions, pending negotiations, or waiting receipts relevant to the current stack. | SCOPE-03 | FR-03 | manual |
| SRS-04 | `keel doctor` must report stack violations such as wrong branch, unsupported active member state, or missing checkpoint acknowledgment. | SCOPE-04 | FR-04 | manual |
| SRS-05 | The stack-aware surfaces must define both human-readable and JSON-compatible output contracts. | SCOPE-05 | FR-05 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Non-stack repos must retain the current single-repo command behavior. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | Stack-aware surfaces must preserve repo-local heartbeat semantics. | SCOPE-05 | NFR-02 | manual |
| SRS-NFR-03 | Machine-readable blocking and explanation output must remain deterministic across repeated evaluations of the same board state. | SCOPE-05 | NFR-03 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
