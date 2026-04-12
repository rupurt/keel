# Plan Reactor-Aware Mission Request Scheduling - SRS

## Summary

Epic: VGYPeZj64
Goal: Define how normalized ingress becomes reactor-visible demand and how a trusted consumer schedules native mission-request application without letting connectors or janitor posture pull board lanes directly.

## Scope

### In Scope

- [SCOPE-01] Define the staged ingress record that carries normalized work, replay identity, trust metadata, and current scheduling state before board mutation.
- [SCOPE-02] Define how Keel-native reactors and read models become aware of staged ingress using communication or application-reactor mechanisms.
- [SCOPE-03] Define the trusted-consumer scheduling and mission-request apply boundary.
- [SCOPE-04] Define the first rollout split across `keel` and `spoke`.

### Out of Scope

- [SCOPE-90] Direct provider or connector mutation of `.keel` planning state.
- [SCOPE-91] Replacing conversational `ping` and `poke` with the planning workflow.
- [SCOPE-92] Non-GitHub providers beyond the provider-neutral core rules.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The voyage SHALL define a staged ingress record that stores the normalized request or work envelope, replay identity, trust metadata, and scheduling state before any planning mutation happens. | SCOPE-01 | FR-01 | manual |
| SRS-02 | The voyage SHALL define how Keel-native reactors and read models become aware of staged ingress through communication or application-reactor mechanisms rather than provider-owned direct mutation. | SCOPE-02 | FR-02 | manual |
| SRS-03 | The voyage SHALL define the trusted-consumer boundary for scheduling and mission-request `apply`, including which actors may observe, schedule, acknowledge, or escalate. | SCOPE-03 | FR-03 | manual |
| SRS-04 | The voyage SHALL define the first rollout split across `keel` and `spoke` needed to land staged ingress, reactor awareness, and trusted-consumer scheduling. | SCOPE-04 | FR-04 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The staged-ingress workflow SHALL remain deterministic and replayable from normalized provider revision through scheduling, `apply`, and acknowledgement. | SCOPE-01, SCOPE-03 | NFR-01 | manual |
| SRS-NFR-02 | The contract SHALL keep conversational comms distinct from structured planning ingress so free-form `ping` and `poke` do not become implicit planning mutations. | SCOPE-02, SCOPE-03 | NFR-02 | manual |
| SRS-NFR-03 | The trusted-consumer core SHALL stay provider-neutral after normalization so GitHub-specific parsing and transport remain outside the scheduling boundary. | SCOPE-01, SCOPE-04 | NFR-03 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
