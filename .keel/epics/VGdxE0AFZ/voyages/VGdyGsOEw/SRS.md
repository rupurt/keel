# Define Mission Stack Stewardship And Handoff Protocol - SRS

## Summary

Epic: VGdxE0AFZ
Goal: Define Mission Stack identity, steward/member coordination, stack modes, and git-backed pushed receipts for cross-reactor handoff.

## Scope

### In Scope

- [SCOPE-01] Define the Mission Stack identity model: stack id, steward reactor, member reactors, and shared `stack/<id>` branch naming.
- [SCOPE-02] Define the coordination modes for `exclusive`, `shared`, and `checkpoint` stack flow.
- [SCOPE-03] Define the pushed-receipt contract and handoff sequence from local seal through remote acknowledgment.
- [SCOPE-04] Define how member reactors materialize local mission lineage after accepting stack work.

### Out of Scope

- [SCOPE-90] Text and JSON rendering for stack-aware CLI surfaces.
- [SCOPE-91] Managed git worktree lifecycle and cleanup mechanics.
- [SCOPE-92] Stronger attestation or non-git receipt artifacts.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The protocol must define Mission Stack as a federation of independent Keel boards with one steward role and one or more member roles. | SCOPE-01 | FR-01 | manual |
| SRS-02 | The protocol must define `exclusive`, `shared`, and `checkpoint` as the canonical stack flow-control modes. | SCOPE-02 | FR-03 | manual |
| SRS-03 | The protocol must define the canonical handoff sequence from local work and commit through push, receipt issuance, negotiation, foreign execution, and acknowledgment. | SCOPE-03 | FR-05 | manual |
| SRS-04 | The protocol must define the minimum git-native receipt fields required for handoff: stack id, repo identity, branch, head sha, and handoff role or checkpoint context. | SCOPE-03 | FR-04 | manual |
| SRS-05 | The protocol must define that target reactors materialize their own local mission lineage after negotiation rather than accepting direct external board mutation. | SCOPE-04 | FR-02 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The protocol must preserve repo-local board authority and repo-local heartbeat semantics. | SCOPE-01 | NFR-01 | manual |
| SRS-NFR-02 | The first receipt contract must stay git-native and avoid introducing stronger artifact requirements unless needed later. | SCOPE-03 | NFR-02 | manual |
| SRS-NFR-03 | The protocol should remain compatible with future stronger audit or attestation layers without changing the base handoff sequence. | SCOPE-03 | NFR-03 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
