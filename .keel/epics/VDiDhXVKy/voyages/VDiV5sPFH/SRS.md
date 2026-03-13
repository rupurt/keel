# AttachMissionBearingCommand - SRS

## Summary

Epic: VDiDhXVKy
Goal: GOAL-01

## Scope

### In Scope

- [SCOPE-01] Implement the explicit command flow for attaching a bearing to a mission.
- [SCOPE-02] Persist and expose mission-bearings lineage so mission readiness/diagnostics consume the linkage.
- [SCOPE-03] Emit explicit operator guidance when attachment preconditions fail.

### Out of Scope

- [SCOPE-04] General mission lifecycle command redesign outside bearing attachment.
- [SCOPE-05] Non-bearing work stream for other strategic entities.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Provide a mission command path to attach a mission-owned bearing idempotently with deterministic updates to both mission and bearing artifacts. | SCOPE-01, SCOPE-02 | FR-02 | automated |
| SRS-02 | Ensure mission readiness and activation checks include mission-owned bearings from the attached lineage. | SCOPE-02 | FR-03 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Provide recovery guidance and fail-fast errors for invalid mission-bearing attachment states. | SCOPE-03 | NFR-02 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
