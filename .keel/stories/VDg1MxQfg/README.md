---
id: VDg1MxQfg
title: Add Static Knowledge Graph Command Surface
type: feat
status: backlog
created_at: 2026-03-12T10:52:42
updated_at: 2026-03-12T10:53:28
operator-signal: 
scope: VDg0dAPVS/VDg0f0aHW
index: 1
---

# Add Static Knowledge Graph Command Surface

## Summary

Add the first deterministic `keel knowledge graph --static` surface so operators, harnesses, and snapshots can render the whole-project graph from the canonical knowledge-graph projection without relying on an interactive TTY.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] [SRS-NFR-01/AC-01] `keel knowledge graph --static` renders a deterministic whole-project graph summary sourced from the canonical knowledge-graph projection so repeated runs on equivalent inputs stay stable. <!-- verify: cargo test --lib knowledge_graph_static_command_renders_deterministic_world_map, SRS-02:start:end, SRS-NFR-01:start:end -->
