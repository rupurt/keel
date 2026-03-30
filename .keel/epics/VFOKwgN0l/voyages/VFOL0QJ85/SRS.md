# Txt-scene Framing - SRS

## Summary

Epic: VFOKwgN0l
Goal: Center and frame playback in the terminal using the txt-scene library.

## Scope

### In Scope

- [SCOPE-01] Double-line scene borders using Unicode box-drawing characters.
- [SCOPE-02] Centered title bar with Mission ID and Name.
- [SCOPE-03] Adaptive padding to keep content centered.

### Out of Scope

- [SCOPE-04] Interactive playback controls (pause/seek).

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Render a double-line border around playback | SCOPE-01 | FR-01 | board: VFOL0QJ85 |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Minimal layout overhead (< 5 cells) | SCOPE-02 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
