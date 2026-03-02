# Schema Hardening and Cleanup - Software Requirements Specification

> Remove legacy migration fixes from doctor and ensure pure canonical schema usage.

**Epic:** [1vv7YWzw2](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

- Remove legacy migration logic and auto-fixes from `keel doctor`.
- Enforce strict datetime parsing for all timestamp fields in entity frontmatter.
- Purge unused compatibility fields and aliases from model structs.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-01 | Remove legacy frontmatter migration fixes from `src/commands/diagnostics/doctor/fixes.rs`. | PRD-04 | manual inspection |
| SRS-02 | Update `deserialize_flexible_datetime` to be strict or remove it in favor of `deserialize_strict_datetime`. | PRD-04 | automated test |
| SRS-03 | Remove deprecated fields like `priority` and `depends` from `StoryFrontmatter`. | PRD-04 | manual inspection |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Source | Verification |
|----|-------------|--------|--------------|
| SRS-NFR-01 | Board loading must fail gracefully or skip entities with legacy frontmatter post-cleanup. | NFR-01 | automated test |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
