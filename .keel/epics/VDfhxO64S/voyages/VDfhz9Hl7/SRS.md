# Canonicalize Voyage Artifact Ordering - SRS

## Summary

Epic: VDfhxO64S
Goal: Make repeated voyage artifact syncs byte-stable across equivalent board states and repeated runs.

## Scope

### In Scope

- [SCOPE-01] Deterministic ordering of evidence and proof artifacts inside `VOYAGE_REPORT.md` and `COMPLIANCE_REPORT.md`.
- [SCOPE-02] Canonical iteration order for epics and voyages during board artifact sync where voyage artifacts are generated.
- [SCOPE-03] Regression coverage that proves repeated generation and equivalent board layouts produce identical voyage artifact output.

### Out of Scope

- [SCOPE-04] Frontier-scoped selective regeneration using `BoardGraph`.
- [SCOPE-05] Report schema redesigns, new generated artifact types, or stakeholder-facing content changes beyond stable ordering and normalization.
- [SCOPE-06] Background indexing, persisted graph storage, or non-voyage generation pipelines.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `VOYAGE_REPORT.md` renders discovered proof artifacts in deterministic filename order regardless of filesystem enumeration order. | SCOPE-01 | FR-01 | automated |
| SRS-02 | `COMPLIANCE_REPORT.md` renders proof links and story coverage in deterministic order for equivalent board inputs. | SCOPE-02 | FR-01 | automated |
| SRS-03 | Board artifact sync iterates epics and voyages in canonical order before generating voyage artifacts. | SCOPE-03 | FR-02 | automated |
| SRS-04 | Regression coverage proves repeated generation and equivalent board layouts yield identical voyage artifact output. | SCOPE-03 | FR-03 | automated |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The deterministic fix preserves existing voyage artifact filenames and markdown section contracts. | SCOPE-01, SCOPE-02, SCOPE-03 | NFR-01 | automated |
| SRS-NFR-02 | A second sync on unchanged input produces byte-identical voyage artifacts and no additional generated diff. | SCOPE-01, SCOPE-02, SCOPE-03 | NFR-02 | automated |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
