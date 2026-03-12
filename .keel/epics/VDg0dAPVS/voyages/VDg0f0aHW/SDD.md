# Render Knowledge Graph Command and Drift Surfaces - Software Design Description

> Ship the first interactive/static keel knowledge graph experience and thread the structural drift coefficient into graph-adjacent read surfaces.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage turns the new knowledge-graph kernel into an actual user-facing experience. It adds a dedicated `keel knowledge graph` command with interactive and static rendering modes, then threads the structural drift signal into graph-adjacent read surfaces so topology/show paths stop treating drift as an invisible implementation detail. The voyage reuses the canonical graph and drift projections rather than adding more relationship scanners.

## Context & Boundaries

### In Scope

- `keel knowledge graph` command wiring and presentation
- deterministic static rendering for harnesses and snapshots
- interactive terminal navigation built on the canonical graph projection
- drift coefficient and supporting context reused in knowledge-graph, topology, and show-style surfaces

### Out of Scope

- semantic-neighborhood layouts that depend on embeddings
- doctor gating on drift or semantic similarity
- symbol-level code nodes or remote graph enrichment

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│ canonical graph/drift projections      │
│              ↓                         │
│ knowledge graph renderer + command     │
│              ↓                         │
│ topology/show drift reuse              │
└─────────────────────────────────────────┘
        ↑               ↑
   terminal I/O     existing CLI views
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Knowledge graph read model | internal | Supplies canonical nodes, edges, and drift inputs without re-scanning the repo in each command | current repo |
| Topology presentation stack | internal | Reuses existing interactive/static terminal patterns and `txtplot`-style world-map concepts | current repo |
| Terminal sizing helpers | internal | Keeps interactive rendering viewport-aware and safe on real TTYs | current repo |
| `txtplot` | crate | Supplies the text plotting primitives already used by topology world maps | current repo dependency |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Renderer foundation | Reuse the new knowledge-graph projection and existing topology rendering conventions | Keeps graph-facing commands convergent and avoids introducing a second visualization stack |
| Drift sourcing | Render drift only from structural graph inputs | Preserves deterministic, explainable output even before semantic work lands |
| Static vs interactive | Default to interactive on TTY, preserve deterministic `--static` snapshots everywhere else | Supports both human exploration and harness-safe proofs |
| Surface reuse | Inject drift summaries into topology/show from shared read-model helpers | Prevents more command-local relationship drift |

## Architecture

The voyage adds one command-facing adapter and one presentation family on top of the graph kernel:

- a management command adapter for `keel knowledge graph`
- a presentation module that can render both interactive and static world-map views from the canonical graph projection
- a shared drift-summary projection consumed by the knowledge-graph command and graph-adjacent read surfaces such as topology/show

The existing topology/show commands should depend on these shared projections instead of reconstructing drift or relationship summaries themselves.

## Components

- `knowledge graph command`
  Purpose: expose interactive/static entry points for whole-project graph exploration.
  Behavior: selects interactive vs static mode, loads the canonical projection/cache, and forwards render options.
- `knowledge graph presentation`
  Purpose: render a world-map style graph summary from canonical nodes, edges, and drift inputs.
  Behavior: supports zoom/focus-style exploration on TTYs and deterministic snapshot output in `--static`.
- `drift surface projection`
  Purpose: package the structural drift coefficient and supporting counters for graph-adjacent command reuse.
  Behavior: powers the knowledge-graph command directly and feeds topology/show surfaces without duplicate scanners.

## Interfaces

This voyage should expose:

- `project_knowledge_graph_surface(board, options) -> KnowledgeGraphSurface`
- `render_knowledge_graph_static(surface, width) -> String`
- `render_knowledge_graph_interactive(surface, viewport) -> Frame`
- `project_structural_drift_summary(board) -> DriftSurfaceSummary`

Topology/show read paths should consume `DriftSurfaceSummary` or the underlying knowledge-graph projection instead of ad hoc drift computations.

## Data Flow

1. Load the board and canonical knowledge-graph projection/cache snapshot.
2. Derive a render-ready surface model from structural nodes, edges, and drift inputs.
3. Render either an interactive frame or a deterministic static graph view.
4. Reuse the same drift summary in topology/show-style commands so the metric stays aligned everywhere.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| No TTY available for interactive mode | Terminal capability check | Fall back to static rendering or require `--static` for harness use | Preserve a deterministic static path |
| Static output drifts across equivalent inputs | Snapshot/read-model tests fail | Treat as regression and block submission | Canonicalize ordering or summarize unstable detail |
| Drift summary diverges between knowledge graph and topology/show | Shared-surface tests fail | Block the slice | Route both surfaces through the same projection helper |
| Semantic cache data is missing or stale | Cache inspection detects absent semantic blobs | Continue with structural rendering only | Recompute semantic data later without changing structural output |
