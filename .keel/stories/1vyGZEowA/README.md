---
id: 1vyGZEowA
title: Parse Canonical PRD Requirement Lineage
type: feat
status: done
created_at: 2026-03-05T13:49:12
updated_at: 2026-03-05T16:43:49
scope: 1vyFgR2MA/1vyFiQPoH
started_at: 2026-03-05T15:37:46
completed_at: 2026-03-05T16:43:49
---

# Parse Canonical PRD Requirement Lineage

## Summary

Introduce the canonical parser and lineage model that extracts epic `FR-*` and `NFR-*` requirements from `PRD.md` and makes them reusable across planning gates, diagnostics, and coverage projections.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Parse canonical parent `FR-*` and `NFR-*` rows from an epic `PRD.md` into a reusable lineage model keyed by epic ID. <!-- verify: cargo test -p keel prd_lineage_parser_builds_canonical_parent_map, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-03] The lineage model exposes enough canonical parent metadata for downstream coverage and enforcement paths to reuse one shared parse result per epic. <!-- verify: cargo test -p keel prd_lineage_model_exposes_reusable_parent_metadata, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-01/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD fixtures produce deterministically ordered lineage output. <!-- verify: cargo test -p keel prd_lineage_parser_is_deterministic, SRS-NFR-01:start:end, proof: ac-3.log-->
