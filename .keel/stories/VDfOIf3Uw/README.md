---
id: VDfOIf3Uw
title: Route Topology Through BoardGraph
type: feat
status: backlog
created_at: 2026-03-12T08:17:31
updated_at: 2026-03-12T08:18:05
operator-signal: 
scope: VDfNdssJL/VDfO1dN84
index: 3
---

# Route Topology Through BoardGraph

## Summary

Migrate the topology world-map relationship path to `BoardGraph` so the new graph becomes a shared runtime primitive rather than a diagnostics-only abstraction.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] The topology/world-map relationship path reads hierarchy and dependency edges from `BoardGraph` instead of rebuilding local relationship scans. <!-- verify: cargo test --lib world_map_uses_board_graph_relationships, SRS-05:start -->
- [ ] [SRS-05/AC-02] Existing topology behavior remains stable for focus, zoom, and dependency rendering after the migration. <!-- verify: cargo test --lib world_map_board_graph_preserves_behavior, SRS-05:end -->
- [ ] [SRS-NFR-03/AC-01] The migrated topology path leaves one canonical relationship implementation in place with no compatibility fallback to the previous local graph builder. <!-- verify: cargo test --lib world_map_board_graph_is_canonical_path, SRS-NFR-03:start:end -->
