# BoardGraph Integrity Kernel - Software Design Description

> Introduce the canonical BoardGraph projection and first doctor integrity path so tree-state validation stops depending on repeated ad hoc scans.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces `BoardGraph` as a derived relationship kernel built once from the loaded `Board`. The first slice keeps storage and entity frontmatter unchanged. Instead, it centralizes the relationship derivation currently spread across `Board` helper scans, traceability dependency builders, world-map helpers, and doctor integrity logic. The graph will then drive one new doctor check and one migrated read-model consumer so the abstraction proves real value immediately.

## Context & Boundaries

Planned scope for this voyage:
- build a graph projection module with typed node ids, edge kinds, and indexed traversal helpers
- derive containment and dependency relationships from current board state
- add graph-integrity diagnostics and migrate one existing consumer to the graph

Deferred beyond this voyage:
- persisted graph storage or caching across process runs
- broad migration of every generator and read model
- changing markdown contracts, entity directory structure, or lifecycle semantics

```
┌──────────────────────────────────────────────────────┐
│                    This Voyage                       │
│                                                      │
│  Board -> BoardGraph builder -> graph indexes        │
│                      │                 │              │
│                      │                 ├─ doctor      │
│                      │                 └─ world map   │
└──────────────────────────────────────────────────────┘
           ↑                                   ↑
      entity frontmatter                existing CLI output
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `Board` aggregate | Internal model | Source entity set for graph construction | current crate API |
| `traceability` dependency derivation | Internal read model | Existing inferred story-order dependency source to be folded into the graph | current crate API |
| Doctor diagnostics engine | Internal application/read path | Host for the new graph-integrity check | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Graph location | Build `BoardGraph` as a read-model projection, not a persisted domain aggregate | The graph is derived state over the canonical markdown-backed entities. |
| Relationship ownership | Keep one canonical graph builder and make consumers depend on it | This avoids the current proliferation of local lineage scans. |
| First migrated consumer | Reuse `BoardGraph` in a non-doctor read path as part of the same voyage | The mission needs proof that the abstraction reduces duplication, not just a new doctor layer. |

## Architecture

The voyage adds a graph projection module that accepts `&Board` and emits:
- deterministic node and edge collections
- adjacency indexes keyed by node id and edge kind
- helper methods for direct lineage traversal and subtree/frontier selection

Doctor checks consume the projection rather than reconstructing containment locally. The migrated non-doctor consumer will request dependency or hierarchy data from `BoardGraph` through a thin adapter rather than parsing the board again.

## Components

`BoardGraphBuilder`
- Purpose: derive nodes and edges from the current board snapshot
- Interface: `build_board_graph(&Board) -> BoardGraph`
- Behavior: gather entity nodes, infer parent-child containment, fold in story dependency edges, sort deterministically

`BoardGraph`
- Purpose: canonical relationship/query surface
- Interface: traversal helpers such as parent, children, descendants, ancestors, incoming, outgoing, and edge queries by kind
- Behavior: store indexed adjacency maps so consumers do not re-scan the board

`GraphIntegrityCheck`
- Purpose: validate structural tree coherence quickly
- Interface: doctor check function returning `Problem` values
- Behavior: walk containment graph once, identify orphan nodes, detect containment cycles, and compare terminal-parent expectations against descendant state

`GraphBackedConsumerAdapter`
- Purpose: migrate one existing read-model consumer to the graph
- Interface: consumer-specific helper or replacement function
- Behavior: preserve current user-facing semantics while delegating relationship lookup to `BoardGraph`

## Interfaces

Primary internal interface:
- `build_board_graph(board: &Board) -> BoardGraph`

Representative query surface:
- `children(id, edge_kind)`
- `parent(id, edge_kind)`
- `descendants(id, edge_kind)`
- `ancestors(id, edge_kind)`
- `outgoing(id, edge_kind)`
- `incoming(id, edge_kind)`

## Data Flow

1. Loader builds the canonical `Board`.
2. `BoardGraphBuilder` derives typed nodes and edges from `Board` plus existing dependency signals.
3. Doctor or the migrated read-model consumer asks `BoardGraph` for relationship answers.
4. The caller emits diagnostics or presentation output without rebuilding local lineage maps.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Graph builder finds a missing parent target for a derived containment edge | builder/query validation | record an orphaned-node integrity problem instead of panicking | fix the underlying lineage field or path contract |
| Story dependency source references a missing story id | graph construction or consumer lookup | surface a deterministic structural problem | correct the blocked/dependency source and rerun doctor |
| Migrated consumer output drifts from existing behavior | regression tests and smoke CLI output | fail the story verification slice | tighten adapter semantics before merge |
