# Narrative Contract Tests And Drift Guards - SRS

## Summary

Epic: VF8hiVofm
Goal: Turn the strongest CLI, turn-loop, scene, and routing claims from the docs into executable contract tests and regression guards.

## Scope

### In Scope

- [SCOPE-05] Add drift and contract tests for command families, turn-loop claims, scene dependency semantics, and routing examples, plus the small docs updates needed to acknowledge the new first-class surfaces.

### Out of Scope

- [SCOPE-05] Broader docs copy rewrites outside the new contract surfaces.
- [SCOPE-06] Visual redesign of the docs site or home page.
- [SCOPE-07] New CLI behavior unrelated to protecting the narrative contract.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Drift tests must fail when the public CLI family lists diverge from the canonical command catalog. | SCOPE-05 | FR-06 | automated tests |
| SRS-02 | Drift tests must fail when the documented turn-loop command examples diverge from the canonical turn-loop projection. | SCOPE-05 | FR-06 | automated tests |
| SRS-03 | Contract tests must fail when scene-capable commands or heartbeat/routing dependency claims diverge from the central scene contracts. | SCOPE-05 | FR-06 | automated tests |
| SRS-04 | Routing drift tests must fail when roles-and-lanes docs and CLI examples diverge from the canonical roles surface and `next --explain` contract. | SCOPE-05 | FR-06 | automated tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Narrative drift guards must read small targeted docs fragments rather than requiring a site build or brittle full-page snapshots. | SCOPE-05 | NFR-01 | code review + tests |
| SRS-NFR-02 | The contract tests must remain understandable enough that maintainers can intentionally update both code and docs when the product contract changes. | SCOPE-05 | NFR-03 | code review + tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
