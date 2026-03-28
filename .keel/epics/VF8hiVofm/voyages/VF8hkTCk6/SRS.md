# Canonical Command Catalog And CLI Taxonomy - SRS

## Summary

Epic: VF8hiVofm
Goal: Create one authoritative command catalog that classifies command families, capabilities, turn phases, scene support, and docs slugs so help text and guidance stop diverging.

## Scope

### In Scope

- [SCOPE-01] Introduce a canonical CLI command catalog for the public command surface, including family, capability, turn-phase, docs-slug, and scene-support metadata.

### Out of Scope

- [SCOPE-05] Adding new user-facing commands beyond metadata needed for later voyages.
- [SCOPE-06] Implementing turn-loop rendering or scene registries beyond the metadata hooks introduced here.
- [SCOPE-07] Changing role-routing behavior itself.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | The catalog must enumerate the public CLI command surfaces with stable metadata for family, capability, turn phase, docs slug, and scene support. | SCOPE-01 | FR-01 | automated tests |
| SRS-02 | Help grouping and command-guidance classification must be rendered from the catalog instead of separate hard-coded maps. | SCOPE-01 | FR-02 | automated tests + CLI review |
| SRS-03 | The catalog must expose scene-capable commands in a machine-readable way so later docs and scene contracts can depend on one source. | SCOPE-01 | FR-01 | automated tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Catalog-driven grouping must preserve the current public command names and user-facing family vocabulary unless a deliberate command-story change updates the docs too. | SCOPE-01 | NFR-02 | code review + tests |
| SRS-NFR-02 | The catalog helpers must remain deterministic and cheap enough to reuse in help output, tests, and future docs tooling. | SCOPE-01 | NFR-01 | code review + tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
