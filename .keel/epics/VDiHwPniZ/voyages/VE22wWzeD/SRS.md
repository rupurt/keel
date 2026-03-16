# Evidence and Risk Carry-Through - SRS

## Summary

Epic: VDiHwPniZ
Goal: Preserve evidence provenance and risks in PRD generation

## Scope

### In Scope

- [SCOPE-01] Include EVIDENCE.md source table in generated PRD as a "Research Provenance" section.
- [SCOPE-02] Populate PRD Open Questions & Risks table from BRIEF.md open questions.

### Out of Scope

- [SCOPE-04] Restructuring existing assessment analysis extraction.
- [SCOPE-05] Generating richer goals/scope from bearing content.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `create_prd_from_bearing` must read EVIDENCE.md and include its source table in the PRD under a "## Research Provenance" heading. | SCOPE-01 | FR-01 | test |
| SRS-02 | `create_prd_from_bearing` must extract bullet items from BRIEF.md `## Open Questions` and emit them as rows in the PRD `## Open Questions & Risks` table. | SCOPE-02 | FR-02 | test |
| SRS-03 | When EVIDENCE.md is absent or contains no source table, the "Research Provenance" section must be omitted (no empty section). | SCOPE-01 | FR-03 | test |
| SRS-04 | When BRIEF.md has no `## Open Questions` section or it is empty, the PRD risks table must fall back to the existing boilerplate row. | SCOPE-02 | FR-03 | test |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Generated PRD must pass `keel doctor` epic structural validation. | SCOPE-01, SCOPE-02 | NFR-01 | test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
