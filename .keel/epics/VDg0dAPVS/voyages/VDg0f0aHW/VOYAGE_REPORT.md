# VOYAGE REPORT: Render Knowledge Graph Command and Drift Surfaces

## Voyage Metadata
- **ID:** VDg0f0aHW
- **Epic:** VDg0dAPVS
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add Static Knowledge Graph Command Surface
- **ID:** VDg1MxQfg
- **Status:** done

#### Summary
Add the first deterministic `keel knowledge graph --static` surface so operators, harnesses, and snapshots can render the whole-project graph from the canonical knowledge-graph projection without relying on an interactive TTY.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] [SRS-NFR-01/AC-01] `keel knowledge graph --static` renders a deterministic whole-project graph summary sourced from the canonical knowledge-graph projection so repeated runs on equivalent inputs stay stable. <!-- verify: cargo test --bin keel knowledge_graph_static_command_renders_deterministic_world_map, SRS-02:start:end, SRS-NFR-01:start:end, proof: ac-1.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDg1MxQfg/EVIDENCE/ac-1.log)

### Add Interactive Knowledge Graph Navigation
- **ID:** VDg1N07jJ
- **Status:** done

#### Summary
Add the interactive `keel knowledge graph` TTY experience so operators can zoom, focus, and explore the canonical whole-project graph without leaving the terminal, while still degrading cleanly when an interactive viewport is unavailable.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] [SRS-NFR-02/AC-01] [SRS-NFR-03/AC-01] The default `keel knowledge graph` command opens an interactive TTY surface with viewport-safe navigation controls and falls back cleanly when no interactive terminal is available. <!-- verify: cargo test --bin keel knowledge_graph_interactive_mode_uses_tty_controls_and_viewport, SRS-01:start:end, SRS-NFR-02:start:end, SRS-NFR-03:start:end, proof: ac-1.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDg1N07jJ/EVIDENCE/ac-1.log)

### Surface Drift Coefficient In Graph-Adjacent Views
- **ID:** VDg1N0Zib
- **Status:** done

#### Summary
Surface the structural drift coefficient and supporting mismatch context through the knowledge-graph experience and graph-adjacent read surfaces so operators can see where board intent, docs, and source-code reality are diverging.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] [SRS-04/AC-01] [SRS-NFR-01/AC-01] The knowledge-graph command, topology, and show-style surfaces render the structural drift coefficient and supporting context from shared graph/drift projections instead of command-local relationship scans. <!-- verify: cargo test --bin keel graph_drift_surfaces_reuse_canonical_projection, SRS-03:start:end, SRS-04:start:end, SRS-NFR-01:start:end, proof: ac-1.log-->
- [x] [SRS-05/AC-01] [SRS-NFR-03/AC-01] Drift surfacing and graph-adjacent rendering remain fully local/offline and structurally correct when semantic cache artifacts are missing or stale. <!-- verify: cargo test --lib graph_drift_surfaces_remain_offline_without_semantic_cache, SRS-05:start:end, SRS-NFR-03:start:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDg1N0Zib/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDg1N0Zib/EVIDENCE/ac-2.log)


