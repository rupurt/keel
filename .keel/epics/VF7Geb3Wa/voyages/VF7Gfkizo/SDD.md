# Remove File Heartbeat And Align Pacemaker Operations - Software Design Description

> Remove the .keel/heartbeat dependency and realign hooks, diagnostics, poke behavior, and documentation around the Git-derived pacemaker model.

**SRS:** [SRS.md](SRS.md)

## Overview

Pass 2 deletes the transitional file-backed heartbeat path after pass 1 proves the derived model. The voyage removes `.keel/heartbeat` from board/core plumbing, rewrites hook and `poke` behavior so they no longer mutate heartbeat state, and updates operator messaging plus public documentation to describe the Git/worktree-derived pacemaker contract.

## Context & Boundaries

```
derived heartbeat projection
          |
          +--> flow
          +--> doctor messaging
          +--> hook/operator contract
          +--> docs and downstream guidance

deleted: file-backed heartbeat loader, fallback path, auto-stage behavior
```

In scope: deleting legacy heartbeat plumbing and updating the operational contract. Out of scope: adding new activity sources beyond the derived projection defined in pass 1.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Pass 1 heartbeat projection | Internal | Provides the canonical heartbeat contract that replaces file-backed behavior. | same epic |
| Hook installation surface | Internal | Needs a cutover from auto-poke/stage behavior to pure quality/commit governance. | workspace current |
| Foundational and MDX docs | Internal | Must be updated in lockstep with the code cutover. | workspace current |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Remove fallback entirely | Delete file-backed heartbeat reads instead of keeping a silent compatibility branch | The mission is specifically to stop teaching and depending on the file. |
| `poke` responsibility | Keep comms/self-heal behavior but remove heartbeat mutation | `poke` should not remain a hidden pacemaker write path. |
| Governor wording | Describe hook + commit lifecycle as the stabilizing controls after the cutover | This matches the actual implementation and user mental model. |

## Architecture

The voyage removes the `Heartbeat` file model from core board state and rewires all consumers to depend on the derived projection from pass 1. The remaining architecture is simpler:

- repository-derived heartbeat projection
- thin CLI consumers (`heartbeat`, `flow`, doctor messaging)
- hook/poke surfaces that no longer participate in heartbeat mutation

## Components

- Loader and board graph cleanup: remove file-backed heartbeat loading and synthetic node behavior.
- Hook/poke cleanup: remove auto-poke and `git add .keel/heartbeat` behavior.
- Diagnostics and docs cleanup: align pacemaker wording with the derived model and documented governor controls.

## Interfaces

The public-facing interface changes are operational rather than structural:

- `keel heartbeat` remains the canonical inspection command.
- `keel flow --scene` consumes the derived heartbeat with no legacy fallback.
- Hooks no longer stage or depend on `.keel/heartbeat`.

## Data Flow

1. Repository activity updates the derived heartbeat signal.
2. CLI consumers read that signal directly.
3. Commit hooks run quality checks without writing heartbeat state.
4. Docs and downstream instructions teach the same model the code implements.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Residual file-backed dependency remains | tests or code review find `.keel/heartbeat` control-path usage | block closure | remove the remaining dependency and extend regression coverage |
| Hook guidance drifts from code behavior | docs review or manual CLI proof shows stale wording | update docs and operator strings together | verify with docs build and command output |
| Downstream upgrade path is unclear | artifact review of upgrade docs finds missing sync steps | expand migration guidance before closure | re-run docs verification |
