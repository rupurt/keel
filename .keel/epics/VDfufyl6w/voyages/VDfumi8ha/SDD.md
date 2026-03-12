# Build Knowledge Graph Kernel and Cache - Software Design Description

> Create the first deterministic knowledge-graph projection and cache substrate so higher-level graph rendering and drift analysis can reuse one canonical indexed world model.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds the canonical substrate underneath the future graph UI. It does not render the world map yet; instead it builds one deterministic graph read model that merges board structure, authored artifact files, knowledge units, project documents, and selected source-code files. It also defines a repo-local cache layout under `.keel/cache/knowledge-graph/` so later interactive and semantic features can reuse stable inputs instead of rescanning the whole repo each run.

## Context & Boundaries

### In Scope

- canonical node inventory for entities, artifacts, docs, knowledge, and source files
- deterministic structural edge derivation and drift-input projection
- deterministic cache manifest and content-addressed blob layout
- local embedding metadata/versioning hooks for later semantic edges

### Out of Scope

- interactive `txtplot` rendering
- topology/show command integration
- doctor gating on drift or semantic neighborhoods

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  board + docs + code + knowledge       │
│             ↓                           │
│   canonical graph projection            │
│             ↓                           │
│   cache manifest + graph blobs          │
└─────────────────────────────────────────┘
        ↑               ↑
   later UI      later doctor/show
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `BoardGraph` and existing board loaders | internal | Seed canonical entity relationships instead of rebuilding board lineage from scratch | current repo |
| Knowledge scanner | internal | Surface canonical knowledge-unit nodes and their explicit links | current repo |
| Filesystem scan helpers | internal/std | Enumerate docs, authored artifacts, and source files deterministically | stable |
| Candle-backed embedding path | local library path | Reserve deterministic cache metadata for later semantic reuse | local/offline |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Canonical root | Extend the current graph stack instead of creating a second unrelated graph subsystem | Reuses recent `BoardGraph` work and keeps doctor/topology surfaces convergent |
| Structural vs semantic | Structural edges are authoritative; semantic metadata is cached separately and advisory only | Keeps doctor-grade invariants deterministic and explainable |
| Cache strategy | Use `.keel/cache/knowledge-graph/` with manifest fingerprints plus content-addressed blobs | Matches the user’s zig-build-inspired cache preference and supports incremental reuse |
| Source granularity | Start with selected source files, not symbol-level nodes | Useful drift visibility without exploding the first graph projection |

## Architecture

The voyage introduces one new read-model family for the knowledge graph and one infrastructure cache surface. The read model owns deterministic node/edge derivation and drift-input computation. The cache layer owns manifest hashing, content-addressed blob storage, and embedding metadata persistence. Later UI and doctor work will depend on those layers rather than scanning the repo directly.

## Components

- `knowledge_graph` read model
  Purpose: derive deterministic nodes, edges, and drift-input summaries from the repo.
  Behavior: merges `BoardGraph`, authored artifact discovery, knowledge scanning, project-doc discovery, and selected source-file inventory.
- `graph cache` infrastructure layer
  Purpose: persist reusable manifests and blobs under `.keel/cache/knowledge-graph/`.
  Behavior: records input fingerprints, graph blob references, and embedding metadata/version stamps.
- `drift input` projection
  Purpose: summarize mismatches between board entities and document/source attachment coverage.
  Behavior: exposes structural counts/signals without turning them into gates yet.

## Interfaces

This voyage should expose pure Rust builders/read APIs, not CLI behavior:
- `build_knowledge_graph_projection(board_dir, options) -> Projection`
- `load_or_refresh_knowledge_graph_cache(board_dir, options) -> CacheSnapshot`
- `project_structural_drift_inputs(projection) -> DriftInputs`

## Data Flow

1. Load the board and canonical `BoardGraph`.
2. Deterministically scan authored artifacts, project docs, knowledge units, and selected source files.
3. Normalize them into graph nodes and structural edges.
4. Compute cache fingerprints and write graph/cache blobs only when changed.
5. Derive structural drift inputs from graph relationships for later UI and doctor consumption.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Cache manifest drift | Equivalent inputs produce different manifests in tests | Treat as regression and fail verification | Canonicalize ordering or fingerprint inputs before submission |
| Graph scan misses required document/source classes | Targeted projection tests fail | Block the story and surface the missing class | Extend deterministic discovery rules |
| Embedding metadata becomes unavailable | Cache load detects missing semantic blobs | Continue with structural graph only | Recompute semantic blobs later without affecting structural correctness |
| Drift inputs depend on semantic edges | Tests show structural-only invariants change with semantic cache presence | Fail the slice | Separate structural and semantic pipelines more strictly |
