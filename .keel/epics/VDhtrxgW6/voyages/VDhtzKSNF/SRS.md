# Resolve HEAD Show Selectors - SRS

## Summary

Epic: VDhtrxgW6
Goal: Add a shared HEAD-relative selector path and wire it into show commands using the canonical stable list order for each entity type.

## Scope

### In Scope

- [SCOPE-01] Parse exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ for show-command selectors.
- [SCOPE-02] Resolve HEAD-relative selectors from canonical default list ordering for mission, epic, voyage, story, bearing, ADR, and routine entities.
- [SCOPE-03] Reuse shared ordering/resolution logic in the corresponding show commands.
- [SCOPE-04] Return deterministic errors for empty sets, unsupported syntax, and offsets that walk past the available history, and add regression/CLI coverage that locks selector behavior to the canonical order contract.

### Out of Scope

- [SCOPE-06] Numeric suffix forms such as HEAD~3 or reflog-style selectors.
- [SCOPE-07] Extending HEAD-relative selection to non-show commands or user-configurable sorting/filter-aware HEAD resolution.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The voyage must add a shared parser that accepts exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ and normalizes them into a deterministic selector form for show-command resolution. | SCOPE-01 | FR-01 | automated |
| SRS-02 | The voyage must resolve HEAD-relative selectors from the same default ordering that the corresponding list surfaces expose for missions, epics, voyages, stories, bearings, ADRs, and routines. | SCOPE-02 | FR-03 | automated |
| SRS-03 | Mission, epic, voyage, story, bearing, ADR, and routine show commands must all consume the shared selector path instead of maintaining command-local HEAD parsing or sorting. | SCOPE-03 | FR-02 | automated |
| SRS-04 | The command path must emit actionable failures when the candidate entity set is empty, the requested relative selector exceeds the available history, or the selector syntax is unsupported. | SCOPE-04 | FR-04 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Equivalent board state must produce the same HEAD-relative resolution across repeated runs and fixtures. | SCOPE-02, SCOPE-04 | NFR-01 | automated |
| SRS-NFR-02 | Regression coverage and CLI help/docs touched by the voyage must stay aligned with the supported selector forms and entity set. | SCOPE-03, SCOPE-04 | NFR-02 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
