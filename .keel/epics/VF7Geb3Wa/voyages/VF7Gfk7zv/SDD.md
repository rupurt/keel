# Introduce Derived Heartbeat Surface And Flow Fallback - Software Design Description

> Add a derived heartbeat projection and command, then cut flow over to it with a temporary compatibility fallback while the file-backed path still exists.

**SRS:** [SRS.md](SRS.md)

## Overview

Pass 1 introduces a derived heartbeat projection in core read-model space and makes that projection the shared source of truth for two operator surfaces: a new `keel heartbeat` command and the energization logic inside `keel flow --scene`. The legacy `.keel/heartbeat` file remains available only as a bounded compatibility fallback so we can prove the derived path before deleting the file-backed model in pass 2.

## Context & Boundaries

```
dirty tracked files + HEAD commit
                |
                v
      derived heartbeat projection
          /                 \
         v                   v
keel heartbeat         keel flow --scene
         \
          v
legacy file fallback (pass 1 only)
```

In scope: deriving heartbeat from repository state, surfacing it in a command, and teaching `flow` to consume it. Out of scope: deleting the legacy file path from board models, hooks, and docs.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Git repository metadata | Local system | Supplies dirty-worktree and commit-activity inputs for the derived heartbeat. | existing repo state |
| Existing Keel repository utilities | Internal | Reuse path-dirty detection and board loading support where possible. | workspace current |
| `chrono` | Library | Normalizes timestamps and age calculations for CLI output and flow decisions. | workspace current |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Canonical source precedence | Dirty tracked files first, HEAD commit time second, legacy file only as pass 1 fallback | This models real active work while still allowing a safe transition. |
| Heartbeat command semantics | Read-only command surface that explains timestamp, age, and source | The new heartbeat model should be inspectable, not another ritual mutation. |
| Fallback boundary | Keep fallback near the projection consumer edge, not embedded through every call site | Pass 2 should be able to delete the fallback cleanly. |

## Architecture

The voyage adds a reusable heartbeat projection in core read-model space, then wires two thin adapters on top:

- `keel-core` projection computes derived activity and exposes structured fields such as `last_activity_at`, `source`, and whether compatibility fallback was used.
- `keel heartbeat` renders that structured data for operators.
- `flow` reads the same projection to decide whether the system should render energized or unplugged.

## Components

- Heartbeat projection: encapsulates repository-state inspection and returns a deterministic heartbeat snapshot.
- Heartbeat CLI command: renders a concise operator-facing summary of the snapshot.
- Flow adapter: replaces direct file mtime reads with projection access and fallback-aware energization checks.

## Interfaces

The projection should expose a small internal contract with fields equivalent to:

- `last_activity_at`
- `age`
- `source`
- `used_legacy_fallback`

## Data Flow

1. Inspect repository state for dirty tracked files and latest commit activity.
2. Build one derived heartbeat snapshot.
3. If repository-derived data is unavailable, consult the legacy file-backed signal for pass 1 compatibility only.
4. Render the snapshot in `keel heartbeat`.
5. Feed the same snapshot into `flow` to decide energized versus unplugged output.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Git metadata cannot be resolved | repository inspection error | return an unavailable derived heartbeat | compatibility fallback may still energize during pass 1 |
| Dirty-file timestamp set is empty | no tracked dirty files | fall back to latest commit activity | normal clean-repo path |
| Legacy fallback path missing | fallback lookup returns none | render idle/unplugged if no derived activity exists | pass 2 will remove this branch entirely |
