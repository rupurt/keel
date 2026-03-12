# Canonical Board Graph and Scoped Regeneration - Decision Log

<!-- Append entries below. Each entry is an H2 with ISO timestamp. -->
<!-- Use `keel mission digest` to compress older entries when this file grows large. -->

## 2026-03-12T08:18:44

Planned epic VDfNdssJL and voyage VDfO1dN84 for the first BoardGraph slice. Decomposed three execution stories covering graph projection, graph-integrity doctor validation, and topology migration to the canonical graph.

## 2026-03-12T08:31:30

Completed story VDfOIc3Ok Introduce BoardGraph Projection. Added the first canonical read_model::board_graph with typed nodes, containment/lineage/governance/dependency edges, routed traceability dependency derivation through it, corrected mixed-type adjacency ordering, and updated next --parallel expectations to treat blocked_by as a real dependency. Verified with just quality, just test, just doctest, cargo run -- doctor, and story submit.
