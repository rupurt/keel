---
id: 1vyGZfkjV
title: Parse Canonical Scope Lineage
type: feat
status: done
created_at: 2026-03-05T13:49:39
updated_at: 2026-03-05T17:30:27
scope: 1vyFgR2MA/1vyFn0OuN
started_at: 2026-03-05T17:00:15
completed_at: 2026-03-05T17:30:27
---

# Parse Canonical Scope Lineage

## Summary

Parse canonical scope identifiers from PRD and SRS artifacts so planning can reason about in-scope and out-of-scope lineage without stripping away authored descriptive text.

## Acceptance Criteria

- [x] [SRS-01/AC-01] PRD `In Scope` and `Out of Scope` items use canonical identifiers in a parseable form. <!-- verify: cargo test -p keel prd_scope_parser_reads_canonical_scope_ids, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Voyage SRS scope statements reference parent PRD scope IDs for included and excluded scope items. <!-- verify: cargo test -p keel srs_scope_requires_parent_prd_scope_ids, SRS-02:start, proof: ac-2.log-->
- [x] [SRS-02/AC-02] [SRS-NFR-01/AC-01] Equivalent PRD/SRS scope fixtures produce deterministic parsed output. <!-- verify: cargo test -p keel scope_lineage_parser_is_deterministic, SRS-NFR-01:start:end, proof: ac-3.log-->
