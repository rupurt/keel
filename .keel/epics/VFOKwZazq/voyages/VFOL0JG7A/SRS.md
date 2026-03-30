# Frame Accurate Scheduler - SRS

## Summary

Epic: VFOKwZazq
Goal: Ensure playback respects GIF frame delays for accurate timing.

## Scope

### In Scope

- Real-time frame scheduling using `atxt` timestamp metadata.
- Non-blocking terminal IO for interruptible playback.
- Efficient delta-encoding cursor movement.

### Out of Scope

- Real-time audio synchronization.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Playback respects frame delays | SCOPE-01 | FR-01 | board: VFOL0JG7A |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Low terminal flicker via delta updates | SCOPE-01 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
