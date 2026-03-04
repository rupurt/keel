# Technique Catalog Configuration And Autodetection - Software Requirements Specification

> Design and implement a verification-technique bank with keel.toml configuration, project autodetection, and recommendation output surfaces.

**Epic:** [1vxqMtskC](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- Define a canonical bank of automated verification techniques with metadata and command templates.
- Support project-level configuration/overrides in `keel.toml`.
- Autodetect project signals (for example Rust CLI, browser test stacks) and generate ranked technique recommendations.
- Surface recommendations in planning read commands so underused techniques like `vhs` and `llm-judge` become visible and adoptable.

Out of scope:
- Automatically executing recommended techniques without explicit user action.
- Replacing existing `verify`/`record` execution semantics in this voyage.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| `keel.toml` is available as the canonical local config source for feature overrides. | Config contract | Technique customization cannot be persisted consistently. |
| Project-stack signals can be inferred from common files (`Cargo.toml`, `package.json`, Playwright config, etc.). | Detection | Recommendations may be weak or noisy on non-standard repos. |
| Existing verification primitives (`vhs`, `llm-judge`, command annotations) remain stable. | Runtime capability | Technique bank entries may drift from executable behavior. |

## Constraints

- Hard cutover: one canonical technique-catalog path (no legacy parallel registries).
- Recommendation output must be deterministic and explainable.
- Safety-first: recommendations are advisory until explicitly executed by the user.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Provide a canonical verification-technique catalog model with built-in entries (including `vhs` and `llm-judge`) and metadata required for recommendation/rendering. | FR-01 | model unit tests + catalog fixtures |
| SRS-02 | Load and merge `keel.toml` technique overrides (enable/disable/customize) on top of the built-in catalog with validation and deterministic precedence. | FR-01 | config parser tests + merge behavior tests |
| SRS-03 | Autodetect project signals and generate ranked, rationale-backed automated verification recommendations from the merged catalog. | FR-01 | autodetection tests + recommendation ranking tests |
| SRS-04 | Surface technique recommendations and adoption guidance in planning read commands (`epic show`, `voyage show`, `story show`). | FR-01 | CLI snapshot tests + read-model projection tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Recommendation ordering and rendering MUST be deterministic across equivalent repository states. | NFR-01 | determinism regression tests |
| SRS-NFR-02 | Recommendation output MUST be safe-by-default (advisory only) and never auto-execute techniques. | NFR-01 | behavior tests + command-path audit tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
