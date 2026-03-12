# VOYAGE REPORT: BoardGraph Integrity Kernel

## Voyage Metadata
- **ID:** VDfO1dN84
- **Epic:** VDfNdssJL
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Introduce BoardGraph Projection
- **ID:** VDfOIc3Ok
- **Status:** done

#### Summary
Introduce the first canonical `BoardGraph` projection and deterministic relationship queries so downstream code can stop reconstructing lineage from ad hoc scans.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Add a `BoardGraph` projection that materializes typed nodes and typed containment/lineage edges for the current board entities. <!-- verify: cargo test --lib board_graph_builds_typed_relationships, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Expose deterministic graph query helpers for parent, children, ancestors, descendants, and edge lookup. <!-- verify: cargo test --lib board_graph_queries_traverse_relationships, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-03/AC-01] Include story dependency edges derived from implementation-order logic and explicit `blocked_by` relationships. <!-- verify: cargo test --lib board_graph_merges_story_dependency_sources, SRS-03:start:end, proof: ac-3.log-->
- [x] [SRS-NFR-01/AC-01] Equivalent board states produce identical node ordering, edge ordering, and query results. <!-- verify: cargo test --lib board_graph_is_deterministic_across_equivalent_boards, SRS-NFR-01:start:end, SRS-01:end, proof: ac-4.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDfOIc3Ok/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDfOIc3Ok/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VDfOIc3Ok/EVIDENCE/ac-3.log)
- [ac-4.log](../../../../stories/VDfOIc3Ok/EVIDENCE/ac-4.log)

### Add Graph Integrity Doctor Check
- **ID:** VDfOIcWP1
- **Status:** done

#### Summary
Add the first graph-level doctor check so Keel can validate structural tree integrity from `BoardGraph` instead of repeated local relationship scans.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] Introduce a doctor check that reports orphaned nodes and containment cycles from the canonical `BoardGraph`. <!-- verify: cargo test --lib doctor_graph_integrity_reports_orphans_and_cycles, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] The graph-integrity path reports terminal-parent violations when descendants remain non-terminal beneath a terminal strategic node. <!-- verify: cargo test --lib doctor_graph_integrity_reports_terminal_parent_violations, SRS-04:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The integrity check builds and reuses one graph per validation path instead of rebuilding whole-board relationship scans inside the check. <!-- verify: cargo test --lib doctor_graph_integrity_uses_single_graph_build, SRS-NFR-02:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDfOIcWP1/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDfOIcWP1/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VDfOIcWP1/EVIDENCE/ac-3.log)

### Route Topology Through BoardGraph
- **ID:** VDfOIf3Uw
- **Status:** done

#### Summary
Migrate the topology world-map relationship path to `BoardGraph` so the new graph becomes a shared runtime primitive rather than a diagnostics-only abstraction.

#### Acceptance Criteria
- [x] [SRS-05/AC-01] The topology/world-map relationship path reads hierarchy and dependency edges from `BoardGraph` instead of rebuilding local relationship scans. <!-- verify: cargo test --lib world_map_uses_board_graph_relationships, SRS-05:start, proof: ac-1.log-->
- [x] [SRS-05/AC-02] Existing topology behavior remains stable for focus, zoom, and dependency rendering after the migration. <!-- verify: cargo test --lib world_map_board_graph_preserves_behavior, SRS-05:end, proof: ac-2.log-->
- [x] [SRS-NFR-03/AC-01] The migrated topology path leaves one canonical relationship implementation in place with no compatibility fallback to the previous local graph builder. <!-- verify: cargo test --lib world_map_board_graph_is_canonical_path, SRS-NFR-03:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDfOIf3Uw/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDfOIf3Uw/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VDfOIf3Uw/EVIDENCE/ac-3.log)


