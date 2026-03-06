---
id: 1vyWNb000
title: Build Epic Topology Projection
type: feat
status: backlog
created_at: 2026-03-06T06:42:15
updated_at: 2026-03-06T06:44:10
scope: 1vyWIF000/1vyWIM000
---

# Build Epic Topology Projection

## Summary

Create the canonical epic-topology projection that composes epic, voyage, story, drift, and dependency signals into one deterministic read model for the new command surface.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add a canonical epic-topology projection that normalizes epic, voyage, and story nodes into deterministic order using existing board and planning read models. <!-- verify: cargo test --lib topology_projection_builds_epic_voyage_story_graph, SRS-01:start, proof: ac-1.log -->
- [ ] [SRS-01/AC-03] [SRS-NFR-01/AC-01] Equivalent board states produce the same node ordering, grouping, and annotation order in the topology projection. <!-- verify: cargo test --lib topology_projection_is_deterministic_across_board_loads, SRS-NFR-01:start:end, SRS-01:continues, proof: ac-2.log -->
- [ ] [SRS-01/AC-04] [SRS-NFR-03/AC-01] The projection sources lineage and dependency inputs through canonical planning and traceability helpers instead of duplicate parsing logic. <!-- verify: cargo test --lib topology_projection_reuses_canonical_read_models, SRS-NFR-03:start:end, SRS-01:end, proof: ac-3.log -->
