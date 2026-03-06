# Scope Lineage and Drift Detection - Software Requirements Specification

> Connect PRD scope items to voyage SRS scope through canonical IDs and detect scope drift in doctor and planning surfaces.

**Epic:** [1vyFgR2MA](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-06] Introduce canonical IDs for PRD in-scope and out-of-scope items.
- [SCOPE-06] Require voyage SRS scope statements to reference parent PRD scope IDs.
- [SCOPE-06] Detect scope drift and contradictions between PRD scope and SRS scope.
- [SCOPE-06] Surface scope-linkage and drift results in doctor and planning views.

Out of scope:
- [SCOPE-04] CLI hydration of problem/goal seed content.
- [SCOPE-02] PRD FR/NFR parent lineage enforcement.
- [SCOPE-05] Goal-to-requirement linkage.
- [SCOPE-07] Story-level scope enforcement during implementation.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| PRD scope can use canonical IDs while remaining readable in bullet-list form. | Content model | Scope linkage may need a different markdown encoding. |
| Voyage SRS scope can carry parent scope references without forcing full prose rewrites. | Authoring workflow | Rollout cost may grow if existing style must change entirely. |
| Scope drift should be reported in planning diagnostics before it becomes runtime work drift. | Governance model | Enforcement may need to move later in the lifecycle. |

## Constraints

- The scope-ID contract must distinguish approved in-scope and out-of-scope items clearly.
- SRS scope references must remain legible to humans while being parseable by diagnostics.
- Drift checks must flag direct contradictions, missing mappings, and references to unknown scope items.
- No alternate legacy scope syntax should be added for the new contract.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | PRD scope items MUST use canonical identifiers for both `In Scope` and `Out of Scope` entries. | SCOPE-06 | FR-07 | scope parser tests + PRD fixture assertions |
| SRS-02 | Voyage SRS scope statements MUST reference parent PRD scope IDs for included and excluded scope items. | SCOPE-06 | FR-07 | SRS scope parsing tests + invalid fixture cases |
| SRS-03 | Doctor diagnostics MUST report unknown scope refs, missing scope mappings, and direct contradictions with PRD out-of-scope definitions. | SCOPE-06 | FR-07 | doctor regression tests + contradiction fixtures |
| SRS-04 | Planning read surfaces MUST summarize linked scope items and highlight scope drift findings for the voyage and epic. | SCOPE-06 | FR-07 | read-model tests + planning output snapshots |
| SRS-05 | Scope parsing MUST preserve authored descriptive text while requiring canonical IDs for machine checks. | SCOPE-06 | FR-07 | mixed prose/ID parser tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Scope-lineage parsing and drift rendering MUST be deterministic for equivalent PRD/SRS artifacts. | SCOPE-06 | NFR-02 | deterministic fixture tests |
| SRS-NFR-02 | Scope drift failures MUST identify the artifact, offending scope ID, and contradiction type. | SCOPE-06 | NFR-03 | diagnostic message assertion tests |
| SRS-NFR-03 | The new scope-lineage contract MUST not retain legacy untagged compatibility paths in the validation logic. | SCOPE-06 | NFR-01 | hard-cutover negative tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
