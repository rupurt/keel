---
id: VDfOIc3Ok
title: Introduce BoardGraph Projection
type: feat
status: backlog
created_at: 2026-03-12T08:17:31
updated_at: 2026-03-12T08:18:05
operator-signal: 
scope: VDfNdssJL/VDfO1dN84
index: 1
---

# Introduce BoardGraph Projection

## Summary

Introduce the first canonical `BoardGraph` projection and deterministic relationship queries so downstream code can stop reconstructing lineage from ad hoc scans.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add a `BoardGraph` projection that materializes typed nodes and typed containment/lineage edges for the current board entities. <!-- verify: cargo test --lib board_graph_builds_typed_relationships, SRS-01:start -->
- [ ] [SRS-02/AC-01] Expose deterministic graph query helpers for parent, children, ancestors, descendants, and edge lookup. <!-- verify: cargo test --lib board_graph_queries_traverse_relationships, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] Include story dependency edges derived from implementation-order logic and explicit `blocked_by` relationships. <!-- verify: cargo test --lib board_graph_merges_story_dependency_sources, SRS-03:start:end -->
- [ ] [SRS-NFR-01/AC-01] Equivalent board states produce identical node ordering, edge ordering, and query results. <!-- verify: cargo test --lib board_graph_is_deterministic_across_equivalent_boards, SRS-NFR-01:start:end, SRS-01:end -->
