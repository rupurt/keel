# Verification Sign-off Gate - SRS

## Summary

Epic: VFOKwnF2e
Goal: Block verification until the human has reviewed and signed off on the artifact playback.

## Scope

### In Scope

- [SCOPE-01] Automatic trigger of Theater Mode during `keel mission verify`.
- [SCOPE-02] Mandatory interactive sign-off prompt after playback.
- [SCOPE-03] Integration with existing mission verification gating rules.

### Out of Scope

- [SCOPE-04] Batch verification review mode.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | verification command triggers playback | SCOPE-01 | FR-01 | board: VFOL0ebAt |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Atomic state transition only on sign-off | SCOPE-02 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
