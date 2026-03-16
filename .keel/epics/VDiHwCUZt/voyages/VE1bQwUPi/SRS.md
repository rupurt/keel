# Contract Enforcement - SRS

## Summary

Epic: VDiHwCUZt
Goal: Align bearing research documents and diagnostics on a strict section contract

## Scope

### In Scope
- [SCOPE-01] Formalize the markdown section contract for bearing documents (BRIEF, EVIDENCE, ASSESSMENT).
- [SCOPE-02] Update `keel doctor` to enforce existence of required sections.
- [SCOPE-03] Update `keel bearing show` to project the new sections.

### Out of Scope
- [SCOPE-04] Follow-on improvements or adjacent work that is not required for the first outcome.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Bearing documents must use ## headings for all key sections to allow consistent parsing. | SCOPE-01 | FR-01 | manual |
| SRS-02 | The diagnostic engine must flag any bearing missing required contract sections as a FAILURE. | SCOPE-02 | FR-01 | manual |
| SRS-03 | CLI projections for bearings must surface authored content from ## Feasibility and ## Findings. | SCOPE-03 | FR-01 | manual |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Visual integrity of the Med-Bay must be maintained while reporting high-load pressure. | SCOPE-02 | NFR-01 | manual |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
