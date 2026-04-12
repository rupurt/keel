# Plan The Janitor Handoff And GitHub Connector Bridge - SRS

## Summary

Epic: VGYFmJEuH
Goal: Define the first executable contract for Keeper-managed janitor stewardship over Keel and the GitHub connector flow it depends on.

## Scope

### In Scope

- [SCOPE-01] Define the custody context fields Keeper must provide to Keel for janitor actions.
- [SCOPE-02] Define the janitor automation envelope across the Keel turn loop.
- [SCOPE-03] Define the GitHub connector ingress/egress contract used by janitor posture.
- [SCOPE-04] Define the first rollout split between `keel` and `spoke`.

### Out of Scope

- [SCOPE-90] Driver/navigator posture semantics or broader fleet coordination.
- [SCOPE-91] Non-GitHub connector implementations.
- [SCOPE-92] Direct connector mutation of `.keel` state.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Define the custody context Keeper must provide to Keel so janitor posture, selected board role, and reactor/project provenance are all explicit in lifecycle evidence. | SCOPE-01 | FR-01 | manual |
| SRS-02 | Define the janitor automation envelope across Orient/Inspect/Pull/Ship/Close, including the Keel commands janitor may call autonomously and the situations that must escalate to a human. | SCOPE-02 | FR-02 | manual |
| SRS-03 | Define the GitHub connector ingress/egress contract for janitor posture, including inbound event classes, normalized envelopes, and outbound acknowledgement or handoff messages. | SCOPE-03 | FR-03 | manual |
| SRS-04 | Define the first rollout split between `keel` and `spoke`, naming the initial crates, docs, and command surfaces each repo must change. | SCOPE-04 | FR-04 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The janitor handoff remains deterministic and replayable from connector event through Keel mutation and provider acknowledgement. | SCOPE-01, SCOPE-03 | NFR-01 | manual |
| SRS-NFR-02 | The contract preserves distinct semantics for Keeper posture versus Keel board-role routing. | SCOPE-01, SCOPE-02 | NFR-02 | manual |
| SRS-NFR-03 | GitHub-specific details stay isolated to the connector contract so future providers can lower into the same janitor custody model. | SCOPE-03 | NFR-03 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
