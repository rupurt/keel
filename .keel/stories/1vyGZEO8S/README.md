---
id: 1vyGZEO8S
title: Gate Voyage Planning On PRD Lineage
type: feat
status: backlog
created_at: 2026-03-05T13:49:12
updated_at: 2026-03-05T14:09:05
scope: 1vyFgR2MA/1vyFiQPoH
---

# Gate Voyage Planning On PRD Lineage

## Summary

Enforce the canonical PRD-to-SRS lineage contract during voyage planning so invalid or legacy source references never transition a voyage to `planned`.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Validate every voyage SRS requirement row so the `Source` column contains exactly one existing parent `FR-*` or `NFR-*` from the epic PRD. <!-- verify: cargo test -p keel srs_source_requires_exactly_one_canonical_prd_parent, SRS-02:start -->
- [ ] [SRS-03/AC-01] `voyage plan` hard-blocks when any SRS requirement is missing a parent source, references a non-existent parent, or uses a non-canonical legacy token. <!-- verify: cargo test -p keel voyage_plan_blocks_invalid_prd_lineage, SRS-03:start:end -->
- [ ] [SRS-03/AC-02] [SRS-NFR-02/AC-01] Blocking errors name the artifact path, offending source token, and expected canonical form. <!-- verify: cargo test -p keel prd_lineage_gate_errors_are_actionable, SRS-NFR-02:start:end -->
- [ ] [SRS-03/AC-03] [SRS-NFR-03/AC-01] Legacy `PRD-*` or custom source-token aliases are rejected instead of silently accepted. <!-- verify: cargo test -p keel prd_lineage_rejects_legacy_source_aliases, SRS-NFR-03:start:end -->
