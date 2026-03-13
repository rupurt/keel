---
id: VDg1N0Zib
title: Surface Drift Coefficient In Graph-Adjacent Views
type: feat
status: done
created_at: 2026-03-12T10:52:42
updated_at: 2026-03-12T17:43:22
operator-signal: 
scope: VDg0dAPVS/VDg0f0aHW
index: 3
started_at: 2026-03-12T17:29:24
completed_at: 2026-03-12T17:43:22
---

# Surface Drift Coefficient In Graph-Adjacent Views

## Summary

Surface the structural drift coefficient and supporting mismatch context through the knowledge-graph experience and graph-adjacent read surfaces so operators can see where board intent, docs, and source-code reality are diverging.

## Acceptance Criteria

- [x] [SRS-03/AC-01] [SRS-04/AC-01] [SRS-NFR-01/AC-01] The knowledge-graph command, topology, and show-style surfaces render the structural drift coefficient and supporting context from shared graph/drift projections instead of command-local relationship scans. <!-- verify: cargo test --bin keel graph_drift_surfaces_reuse_canonical_projection, SRS-03:start:end, SRS-04:start:end, SRS-NFR-01:start:end, proof: ac-1.log-->
- [x] [SRS-05/AC-01] [SRS-NFR-03/AC-01] Drift surfacing and graph-adjacent rendering remain fully local/offline and structurally correct when semantic cache artifacts are missing or stale. <!-- verify: cargo test --lib graph_drift_surfaces_remain_offline_without_semantic_cache, SRS-05:start:end, SRS-NFR-03:start:end, proof: ac-2.log-->
