---
id: VDfuntHAT
title: Add Knowledge Graph Projection and Cache Manifest
type: feat
status: backlog
created_at: 2026-03-12T10:26:37
updated_at: 2026-03-12T10:28:31
operator-signal: 
scope: VDfufyl6w/VDfumi8ha
index: 1
---

# Add Knowledge Graph Projection and Cache Manifest

## Summary

Add the first deterministic knowledge-graph projection and repo-local cache manifest so later graph rendering and drift surfacing can reuse one canonical indexed world model.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The read model emits deterministic nodes for entities, authored artifacts, knowledge units, project docs, and selected source files with stable canonical IDs and ordering. <!-- verify: cargo test --lib knowledge_graph_projection_builds_entity_document_and_code_nodes, SRS-01:start -->
- [ ] [SRS-02/AC-01] The read model emits deterministic structural edges for containment, lineage, provenance, governance, traceability, and attachment relationships. <!-- verify: cargo test --lib knowledge_graph_projection_builds_structural_edges, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] [SRS-04/AC-01] The cache writes deterministic manifests and content-addressed blob metadata under `.keel/cache/knowledge-graph/` and remains structurally correct when semantic blobs are absent or stale. <!-- verify: cargo test --lib knowledge_graph_cache_manifest_is_deterministic, SRS-03:start:end, SRS-04:start:end -->
- [ ] [SRS-05/AC-01] [SRS-NFR-01/AC-01] [SRS-NFR-02/AC-01] The projection exposes deterministic structural drift inputs and cache reuse behavior across repeated offline runs on equivalent board states. <!-- verify: cargo test --lib knowledge_graph_projection_exposes_structural_drift_inputs, SRS-05:start:end, SRS-NFR-01:start:end, SRS-NFR-02:start:end -->
