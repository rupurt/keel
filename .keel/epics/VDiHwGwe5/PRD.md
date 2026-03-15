# Strict Deterministic Board Generation - Product Requirements

## Problem Statement

The `keel generate` command currently produces non-deterministic output (e.g., varying whitespace or incidental edits) and does not respect frontier boundaries, leading to unnecessary git churn and potential data loss in manual sections.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Ensure `keel generate` is strictly deterministic. | Zero bytes of diff on repeated runs with unchanged input. | 100% pass rate in regression tests. |
| GOAL-02 | Protect authored content from machine-generated churn. | Authored sections are preserved exactly as written. | Zero regression in authored content integrity. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Developer | A human contributor manually editing board artifacts. | Confidence that machine generation won't undo manual formatting or content. |
| Agent | An autonomous agent performing automated moves. | Predictable, minimal diffs for better git traceability and reduced noise. |

## Scope

### In Scope

- [SCOPE-01] Canonical serialization of all `.keel` entities.
- [SCOPE-02] Scoped regeneration: only update files affected by the change.
- [SCOPE-03] Preserve manually authored sections outside of `<!-- BEGIN/END GENERATED -->` blocks.

### Out of Scope

- [SCOPE-04] Performance optimization beyond basic caching.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Entities must serialize keys in a fixed, predictable order. | GOAL-01 | must | Prevents random key shuffling in frontmatter. |
| FR-02 | Whitespace between frontmatter and body must be standardized. | GOAL-01, GOAL-02 | must | Eliminates incidental newline churn. |
| FR-03 | Regeneration must only touch files where the effective state has changed. | GOAL-01 | should | Reduces git noise and filesystem IO. |
| FR-04 | Authorship boundaries must be respected during regeneration. | GOAL-02 | must | Ensures manual edits outside generated blocks are preserved. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Regression testing for `keel generate` on a fixed fixture set. | GOAL-01 | must | Detects non-deterministic drift during development. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Determinism | Automated CLI tests | Repeated `generate` runs on a fixture set show zero diff. |
| Scoped Updates | Integration tests | Only affected entity files have updated timestamps. |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Alphabetical key ordering is acceptable for all entities. | Frontmatter might be less "human-readable" if certain keys are buried. | Review sample output during implementation. |
| `serde_yaml` supports deterministic mapping serialization. | We might need to manually sort keys or use a different crate. | Verify with unit tests. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should we allow per-entity custom key ordering? | Epic Owner | TBD |
| How do we handle custom YAML tags if they appear in future extensions? | Architect | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel generate` produces 0 bytes of diff on a board with no pending state changes.
- [ ] Frontmatter keys are always in the same order (id, title, status, ...).
<!-- END SUCCESS_CRITERIA -->
