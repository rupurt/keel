# Epic Topology Terminal View - Software Design Description

> Render an epic-scoped topology map in the terminal that shows voyages, stories, drift hotspots, knowledge annotations, and forward-looking risks in one operator-friendly view.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds a new informational command, `keel topology --epic <id>`, backed by a canonical epic-topology projection. The command composes existing planning, traceability, throughput, verification, and knowledge read models into one terminal surface rather than re-parsing planning artifacts inside the CLI adapter.

The output is intentionally human-oriented:
- a topology tree rooted at the epic
- grouped voyage and story lanes
- inline drift annotations on relevant nodes
- a horizon panel for approaching risk
- optional done-item visibility for fuller historical context

The design favors one canonical projection plus a thin renderer so drift logic remains shared with existing `show`, `flow`, and knowledge surfaces.

## Context & Boundaries

```text
┌────────────────────────────────────────────────────────────┐
│                  `keel topology --epic`                   │
│                                                            │
│  CLI command ──> topology read model ──> terminal renderer │
│                       │                │                   │
│                       │                └─> width-aware     │
│                       │                    summaries        │
│                       ├─ planning_show                      │
│                       ├─ traceability                       │
│                       ├─ throughput / verification          │
│                       └─ knowledge scanner / navigator      │
└────────────────────────────────────────────────────────────┘
```

In scope:
- Composing one epic-scoped topology projection from existing board artifacts and read models.
- Rendering an operator-friendly terminal surface with drift and horizon overlays.
- Supporting a focused default view plus an include-done option.

Out of scope:
- Whole-board aggregation across epics.
- Export or machine-facing formats.
- Any mutation of story, voyage, or epic lifecycle state from topology output.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `planning_show` projection helpers | internal service | Reuse scope drift, requirement coverage, verification rollups, and epic or voyage summaries | existing crate API |
| `traceability::derive_implementation_dependencies` | internal service | Reuse dependency and blockage derivation already aligned with `keel flow` | existing crate API |
| `knowledge::scanner`, `knowledge::navigator`, `application::knowledge_context` | internal service | Surface scoped knowledge, pending insights, and trend-aware risk inputs | existing crate API |
| throughput and flow read models | internal service | Estimate ETA and blockage pressure for horizon commentary | existing crate API |
| terminal presentation helpers | internal service | Reuse width, style, and snapshot-friendly rendering behavior | existing crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Topology root | Epic-scoped only in this voyage | Matches the primary user request and keeps the first delivery readable |
| Canonical data path | New `read_model::topology` projection composed from existing projections | Prevents command-local drift and keeps the renderer thin |
| Knowledge shape | Knowledge appears as annotations and summary rows, not first-class graph nodes | The user preferred annotations unless a node graph was clearly better; annotations reduce layout noise |
| Commentary style | Deterministic rule inputs with heuristic narrative phrasing | The user allows heuristic commentary, but the underlying signals still need stable tests |
| Done visibility | Hidden by default, optional to include | Planned and in-progress flow is the default operational concern, while done context is still useful on demand |

## Architecture

Proposed module split:
- `src/read_model/topology.rs`
  Produces `EpicTopologyProjection` and related DTOs for nodes, drift annotations, knowledge annotations, and horizon warnings.
- `src/cli/commands/management/topology.rs`
  Parses `--epic` and done-visibility flags, loads the board, calls the read model, and prints rendered output.
- `src/cli/presentation/topology.rs` or `src/cli/presentation/topology/**`
  Owns width-aware terminal rendering, grouping, connectors, legends, and summary sections.

Projection composition:
1. Load epic, voyages, and scoped stories from the board.
2. Reuse `planning_show` to pull epic and voyage coverage, scope drift, verification summaries, and requirement completion.
3. Reuse `traceability` to derive story dependency and blockage relationships.
4. Reuse knowledge scanning, ranking, and navigation helpers to attach recent scoped insights, pending unapplied knowledge, and risk-oriented pattern inputs.
5. Reuse throughput and verification signals to derive forward-looking warnings.
6. Normalize all rows into one ordered projection before the renderer formats anything.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| Topology command | Informational entrypoint for humans | `run(board_dir, epic_id, include_done)` | Thin adapter only |
| Topology projection builder | Canonical source of topology data | `build_epic_topology_projection(board, epic, options)` | Owns ordering and composition |
| Drift annotator | Converts shared board signals into node-level hotspots | internal helper in read model | Reuses planning and traceability data |
| Knowledge annotator | Attaches recent and pending knowledge to relevant scopes | internal helper in read model | Uses scoped ranking and voyage or story provenance |
| Horizon analyzer | Produces warning and recommendation rows from deterministic inputs | internal helper in read model | Advisory language, deterministic thresholds |
| Terminal renderer | Renders tree, summaries, and commentary | `render_topology(projection, width, no_color)` | Human-first, width-aware |

## Interfaces

Primary command contract:
- `keel topology --epic <id>`
- optional flag to include done voyages and stories

Projection contract sketch:

```text
EpicTopologyProjection
  epic summary
  voyages[]
    stories[]
    drift_annotations[]
    knowledge_annotations[]
  epic_drift_annotations[]
  horizon_warnings[]
  recommendation_rows[]
```

No JSON or export interface is introduced in this voyage.

## Data Flow

1. CLI resolves board root and epic ID.
2. Loader fetches the board and target epic.
3. Topology read model gathers:
   - epic and voyage planning summaries
   - story dependency chains
   - verification or proof gap signals
   - scoped knowledge and rising-pattern inputs
   - throughput and ETA indicators
4. Read model normalizes all entities into one ordered projection with node annotations.
5. Renderer prints:
   - epic header and legend
   - voyage and story topology tree
   - drift hotspot callouts
   - knowledge annotations
   - horizon and recommendation section

Heuristic examples:
- Verification debt: stories or requirements with missing proof coverage or pending manual review
- ETA risk: remaining stories with weak throughput signal or high blockage ratio
- Tech or process debt: high-confidence recent or pending knowledge in `architecture`, `code`, or `process` categories attached to the epic scope
- Drift escalation: multiple simultaneous hotspot types in the same voyage or story chain

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Epic ID missing or unknown | CLI parsing or board lookup | Return actionable error with existing epic guidance | Run `keel epic list --status +done` or correct the ID |
| Epic has no planned or in-progress flow | Projection sees no visible voyages or stories in default filter | Render explicit empty-state message and point to the done-visibility option if relevant | Include done items or continue planning |
| Required supporting artifacts are absent | Existing read models return placeholders or empty collections | Render placeholder annotations instead of failing the whole command | Author missing PRD, SRS, reflect, or evidence artifacts |
| Dense topology exceeds readable width | Renderer width heuristics detect overflow | Collapse detail into summaries while preserving deterministic ordering | Re-run in a wider terminal or include fewer done items |
| Heuristic inputs are sparse or inconclusive | Horizon analyzer lacks enough throughput or knowledge signal | Render a low-confidence warning or omit that specific heuristic with explicit reason | Gather more evidence over time; no fallback export path in this slice |
