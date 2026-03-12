# Temporal Routine Gating - SRS

## Summary

This voyage adds the first temporal behavior to recurring work by evaluating
routine cadence into due-state and surfacing that state through `keel next`.

## Scope

### In Scope

- [SCOPE-01] Due-state evaluation from routine cadence metadata
- [SCOPE-02] `keel next` countdown and scheduled-work rendering
- [SCOPE-03] Queue gating that suppresses non-due routine work

### Out of Scope

- [SCOPE-90] Pulse-based work materialization
- [SCOPE-91] Scheduled lane rendering in `keel flow`

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Evaluate routine cadence metadata into a due/not-due state plus next eligible time. | SCOPE-01 | FR-01 | unit test |
| SRS-02 | Extend `keel next` human and JSON surfaces to show due and upcoming routine work with countdown context. | SCOPE-02 | FR-02 | integration test |
| SRS-03 | Exclude non-due routines from actionable pull results while allowing due routines to participate in existing prioritization rules. | SCOPE-03 | FR-03 | integration test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Temporal evaluation remains deterministic for identical board state and injected clock input. | SCOPE-01 | NFR-01 | unit test |
| SRS-NFR-02 | Countdown and gating explanations remain stable enough for CLI review and regression assertions. | SCOPE-02 | NFR-02 | integration test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
