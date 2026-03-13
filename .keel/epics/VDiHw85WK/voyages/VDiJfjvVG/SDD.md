# RoadmapMVP - Software Design Description

> GOAL-01

**SRS:** [SRS.md](SRS.md)

## Overview

Build a single canonical roadmap projection from existing board state and expose it through management-facing text output. The implementation should rely on current board graph and workflow-state sources (no alternate repository for roadmap state).

## Context & Boundaries

### In scope

- Read model and text rendering for roadmap output.
- No new persistence schema.
- Existing management lane command surface only.

### Out of scope

- New interactive UI or persistence migration.
- Changes to non-management lane execution semantics.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `read_model::topology` | module | Horizon/dependency context, existing commentary logic | local |
| `read_model::board_graph` | module | Dependency and ordering context | local |
| `cli/commands/management` | module | Command and text render entrypoint | local |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Output path | Reuse existing management output surfaces | Keeps command surface stable and testable |
| Posture semantics | Derive from existing board/ bearing status | Avoids duplicate status model |
| Determinism | Stable sort on priority/state/title/id | Enables deterministic snapshots |

## Architecture

- Add a roadmap projection helper to the existing management/read-model flow.
- Project non-terminal entities to rows with posture and blocker context.
- Render via deterministic ordering for CLI output.
- Add/extend tests around roadmap shape and ordering.

## Components

| Component | Purpose | Interface | Behavior |
|-----------|---------|-----------|----------|
| Roadmap projector | Convert board entities into roadmap rows | `render_roadmap(...)` | Outputs ordered rows with posture and blocker metadata |
| Posture mapping | Translate status/dependency state into proceed/park/blocked labels | internal helper | Maps state to canonical posture consistently |
| Management command renderer | Surface roadmap mode in CLI text output | CLI command handler | Prints table or structured output with stable columns |

## Interfaces

Roadmap rows should include:

- `entity_id`
- `entity_type`
- `title`
- `status`
- `posture`
- `priority`
- `blocking_count`
- `blocking_ids`
- `index`

## Data Flow

1. Command handler loads board model.
2. Read model provides dependency graph + workflow states.
3. Roadmap projector builds deterministic rows.
4. Renderer prints roadmap rows to management output.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing dependency target IDs | Lookups by ID fail | Emit row with diagnostic context and continue | Keep command output to preserve operator visibility |
| Duplicate projection entries | Same ID appears in multiple buckets | Deduplicate by ID before sorting/rendering | Keep first canonical row and log duplication in diagnostics |
| Unstable ordering | Non-deterministic sort keys | Unit test catches unstable ordering | Expand comparator to include explicit tie-breakers |
