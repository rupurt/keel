# Two-Pass Speccy Refactor - SRS

## Summary

Epic: VF0XAFqlF
Goal: Refactor `speccy` in two passes so its module layout is explicit and Keel depends on a smaller, more stable API.

## Scope

### In Scope

- [SCOPE-01] Split `speccy` into explicit modules without changing the currently supported rendering and mutation behavior.
- [SCOPE-02] Replace the public helper matrix with a smaller options-driven rendering surface.
- [SCOPE-03] Update Keel adapters and call sites that depend on `speccy` so the reduced surface is the canonical path.
- [SCOPE-04] Document the intended stable module boundaries and extension points.

### Out of Scope

- [SCOPE-05] New template language features such as loops or conditionals.
- [SCOPE-06] A full YAML-aware frontmatter rewrite.
- [SCOPE-07] Publishing `speccy` outside the workspace or adding another production consumer in this voyage.

## Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | `speccy` must be split into focused source modules for catalog loading, hooks, rendering, and frontmatter behavior while preserving current supported semantics. | SCOPE-01 | FR-01 | automated tests + code review |
| SRS-02 | `speccy` must expose a reduced render API built around core render entrypoints plus option-driven transforms instead of a matrix of specialized top-level helpers. | SCOPE-02 | FR-02 | automated tests + compile verification |
| SRS-03 | Keel must consume the reduced `speccy` API for its current rendering and frontmatter mutation flows. | SCOPE-03 | FR-03 | automated tests |
| SRS-04 | The voyage artifacts must explain the stable extension points for host projects and the responsibilities that remain in Keel adapters. | SCOPE-04 | FR-04 | artifact review |
<!-- END FUNCTIONAL_REQUIREMENTS -->

## Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Current supported render and frontmatter mutation workflows must remain deterministic after both passes. | SCOPE-01 SCOPE-02 SCOPE-03 | NFR-01 | automated tests |
| SRS-NFR-02 | The reduced public API must not depend on Keel-specific crates, board entities, or `.keel` filesystem assumptions. | SCOPE-02 SCOPE-03 | NFR-02 | code review + compile verification |
| SRS-NFR-03 | Formatting, linting, and regression suites relevant to `speccy` and Keel render flows must pass before closure. | SCOPE-01 SCOPE-02 SCOPE-03 SCOPE-04 | NFR-03 | fmt + clippy + tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
