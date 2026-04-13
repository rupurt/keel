# Implement Mission Stack Context Surfaces - SRS

## Summary

Epic: VGe7mCcFW
Goal: Ship repo-local Mission Stack loading, stack-aware turn/next/mission-next output, and first guardrail diagnostics for managed foreign execution.

## Scope

### In Scope

- [SCOPE-01] Define the repo-local Mission Stack manifest contract and reusable stack projection.
- [SCOPE-02] Add stack context to `keel turn`.
- [SCOPE-03] Add stack-aware routing decisions to `keel next`.
- [SCOPE-04] Add linked member mission, negotiation, and receipt visibility to `keel mission next --status`.
- [SCOPE-05] Add Mission Stack diagnostics for branch, checkpoint, foreign-worktree, and close-state violations.
- [SCOPE-06] Expose stable text and JSON contracts for stack-aware operator surfaces.

### Out of Scope

- [SCOPE-90] Hub-backed or network-synchronized stack state.
- [SCOPE-91] Stronger receipt artifacts than the initial git-native contract.
- [SCOPE-92] Full automated creation and cleanup of foreign worktrees.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Keel must parse `.keel/stacks/<id>/manifest.yaml` and derive a deterministic local Mission Stack projection that includes stack identity, local member role, mode, checkpoint, member missions, and receipt state. | SCOPE-01 | FR-01 | automated |
| SRS-02 | `keel turn` must expose stack id, branch, local member role, current mode, checkpoint state, and foreign-worktree execution state when a local stack is active. | SCOPE-02 | FR-02 | automated |
| SRS-03 | `keel next` must produce stack-aware blocking or yield decisions when local execution is forbidden by stack mode, checkpoint requirements, or unsupported foreign-worktree state. | SCOPE-03 | FR-03 | automated |
| SRS-04 | `keel mission next --status` must describe linked member missions, pending negotiations, and waiting receipts relevant to the local stack when present. | SCOPE-04 | FR-04 | automated |
| SRS-05 | `keel doctor` must report stack violations such as wrong branch, missing checkpoint acknowledgment, unsupported foreign execution location, and leftovers after stack close. | SCOPE-05 | FR-05 | automated |
| SRS-06 | Stack-aware command surfaces must expose deterministic JSON fields in addition to human-readable output. | SCOPE-06 | FR-06 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Non-stack repos must retain current behavior across all touched commands. | SCOPE-02 | NFR-01 | automated |
| SRS-NFR-02 | Stack-aware surfaces must preserve repo-local heartbeat semantics and not redefine pacemaker state. | SCOPE-06 | NFR-02 | automated |
| SRS-NFR-03 | Foreign-worktree enforcement must fail safe by diagnosing unsupported execution rather than mutating or deleting uncertain checkouts automatically. | SCOPE-05 | NFR-04 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
