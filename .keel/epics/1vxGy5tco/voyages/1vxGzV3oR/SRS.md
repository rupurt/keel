# Template And CLI Contract Canonicalization - Software Requirements Specification

> Adopt canonical schema-mirrored template tokens and align creation commands so only user-owned fields are CLI-settable.

**Epic:** [1vxGy5tco](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

This voyage defines the template token and CLI contract layer.
It covers token vocabulary normalization, template rendering input alignment, and creation command flag ownership for `epic`, `voyage`, `story`, `bearing`, and `adr` creation paths.
Doctor/gate severity changes are intentionally handled in a separate voyage.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Embedded template constants in `src/infrastructure/templates.rs` remain the single source for scaffold content. | architecture | Contract checks would need to inspect additional template sources. |
| Clap command definitions remain centralized in `src/cli/command_tree.rs`. | code | CLI ownership enforcement becomes fragmented across modules. |
| Template rendering service remains `template_rendering::render`. | code | Token replacement consistency requires broader refactor first. |

## Constraints

- No system-owned fields (`id`, `index`, `status`, `*_at`) may gain creation CLI flags.
- Token names in templates must mirror canonical schema/frontmatter field names when tokenized.
- Hard cutover applies: no token aliases, dual-path parsing, or compatibility shims.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Replace non-canonical template token names with canonical schema/frontmatter-mirrored names across active planning templates and render callsites. | FR-01 | template token inventory tests + targeted command tests |
| SRS-02 | Ensure creation CLI options expose only user-owned inputs for `epic new`, `voyage new`, `story new`, `bearing new`, and `adr new`. | FR-02 | command tree tests + behavior tests |
| SRS-03 | Add `adr new --context` and repeatable `adr new --applies-to` and persist both values in ADR frontmatter. | FR-03 | adr command tests + frontmatter assertions |
| SRS-04 | Make `voyage new --goal` required at clap parse level and keep runtime behavior coherent. | FR-04 | CLI parsing tests + voyage creation tests |
| SRS-05 | Introduce contract tests that fail on unknown tokens and on token ownership bucket violations. | FR-01, FR-02 | drift tests + template module tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Implementation must remove superseded token paths in the same change slice (hard cutover). | NFR-01 | code review + compile/test verification |
| SRS-NFR-02 | Contract and CLI checks must be deterministic and not depend on board runtime state. | NFR-02 | unit tests with fixture inputs |
| SRS-NFR-03 | Validation failures must identify offending token or flag surface clearly. | NFR-03 | assertion text checks in tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
