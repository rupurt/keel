# Canonical PRD Requirement Lineage - Software Requirements Specification

> Enforce canonical FR/NFR lineage from epic PRDs into voyage SRS requirements with blocking voyage-plan gates, doctor parity, and epic-wide coverage reporting.

**Epic:** [1vyFgR2MA](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-01] Parse canonical parent `FR-*` / `NFR-*` rows from epic `PRD.md` files.
- [SCOPE-02] Validate voyage SRS `Source` references against the parent epic PRD.
- [SCOPE-02] Hard-block `voyage plan` on missing, invalid, or non-canonical PRD lineage.
- [SCOPE-02] Reuse the same lineage coherence logic in doctor diagnostics.
- [SCOPE-03] Aggregate epic requirement coverage across all voyages for planning read surfaces.

Out of scope:
- [SCOPE-04] Adding new `epic new` CLI inputs or PRD section hydration behavior.
- [SCOPE-05] Goal/objective linkage between PRD goals and PRD requirements.
- [SCOPE-06] Scope linkage and scope-drift enforcement between PRD and SRS.
- [SCOPE-10] Historical board migration for legacy `PRD-*` or ad hoc source tokens.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Parent epic PRDs contain authored functional and non-functional requirement tables. | Content | Lineage checks cannot resolve source IDs reliably. |
| Voyage SRS files keep the canonical requirement table shape with a `Source` column. | Contract | Validation logic would need broader parser redesign. |
| Epic show/read-model surfaces are acceptable first-class consumers of epic-wide coverage data. | UX | Coverage may need a different planning surface before rollout. |

## Constraints

- Hard cutover only: canonical parent requirement IDs are `FR-*` and `NFR-*`; new runtime behavior must not accept `PRD-*` or custom aliases.
- Each SRS requirement maps to exactly one parent FR/NFR; a parent FR/NFR may fan out to multiple SRS requirements across voyages.
- Transition and doctor paths must consume one shared lineage coherence implementation.
- Coverage must aggregate across all voyages in the epic, not only the voyage under review.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Parse canonical parent `FR-*` / `NFR-*` rows from the epic `PRD.md` into a reusable lineage model keyed by epic. | SCOPE-01 | FR-01 | parser unit tests + invalid fixture coverage |
| SRS-02 | Validate each voyage SRS requirement row so its `Source` column contains exactly one existing parent FR/NFR from the epic PRD. | SCOPE-02 | FR-02 | SRS validation tests + invalid lineage fixtures |
| SRS-03 | `voyage plan` MUST hard-block when any SRS requirement is missing a parent source, references a non-existent FR/NFR, or uses a non-canonical legacy token. | SCOPE-02 | FR-03 | voyage plan command tests + enforcement-path tests |
| SRS-04 | Doctor diagnostics MUST report PRD-to-SRS lineage problems using the same coherence rules used by planning transitions. | SCOPE-02 | FR-03 | doctor regression tests + gate/doctor parity tests |
| SRS-05 | Epic planning projections MUST aggregate parent FR/NFR coverage across all voyages and identify uncovered parent requirements with linked-child counts. | SCOPE-03 | FR-04 | read-model tests + epic show snapshot tests |
| SRS-06 | Coverage aggregation MUST preserve exactly-one-parent ownership for each SRS requirement while allowing one-to-many parent FR/NFR fanout without double counting. | SCOPE-03 | FR-04 | coverage model tests + deterministic fixture assertions |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Lineage parsing and epic coverage output MUST be deterministic for equivalent PRD/SRS inputs. | SCOPE-03 | NFR-02 | deterministic parser and projection tests |
| SRS-NFR-02 | Blocking and diagnostic error messages MUST name the artifact path, offending source token, and expected canonical form. | SCOPE-02 | NFR-03 | assertion tests on gate and doctor messages |
| SRS-NFR-03 | New validation behavior MUST not retain compatibility aliases for legacy `PRD-*` or custom source tokens. | SCOPE-02 | NFR-01 | negative tests + hard-cutover regression fixtures |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
