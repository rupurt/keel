# Adaptive Resizing - SRS

## Summary

Epic: VFOKwgN0l
Goal: Playback should respond to terminal window resizing without breaking the scene.

## Scope

### In Scope

- [SCOPE-01] Terminal resize signal listener.
- [SCOPE-02] Real-time TheaterScene layout recalculation.
- [SCOPE-03] Integration with atxt dynamic planning.

### Out of Scope

- [SCOPE-04] Frame-buffer caching across resize events.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Re-center theater frame on terminal resize | SCOPE-03 | FR-01 | board: VFOL0XX8w |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Glitch-free resizing (< 50ms latency) | SCOPE-01 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
