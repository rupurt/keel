# Implement Mission Stack Context Surfaces - Software Design Description

> Ship repo-local Mission Stack loading, stack-aware turn/next/mission-next output, and first guardrail diagnostics for managed foreign execution.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a repo-local Mission Stack read model that is loaded
alongside the existing board and reused by CLI adapters. The first slice keeps
stack state local, git-native, and optional: if no stack manifest exists, all
surfaces behave exactly as they do today.

## Context & Boundaries

- Mission Stack manifest loading and projection
- stack-aware turn, next, mission-next, and doctor adapters
- git-derived branch and worktree validation for the local checkout
- diagnostics for wrong-branch, checkpoint, and foreign-worktree violations

### Out of Scope

- network synchronization across member repos
- richer receipt or attestation artifacts
- automatic foreign worktree creation/cleanup commands in this slice

### External Actors

- local git repository state
- managed foreign worktree checkout paths
- mission-request protocol documented in ADR `VGdx8jTUm`

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Mission Stack ADR | Internal contract | Defines stack modes, receipt fields, and worktree rule | `VGdx8jTUm` |
| Git branch/worktree state | External contract | Supplies current branch, repo root, and linked-worktree metadata | `git rev-parse`, `git branch --show-current`, `git worktree list --porcelain` |
| Existing board loader | Internal contract | Continues to load repo-local board state independently from stack state | `load_board` |
| Doctor/read-model adapters | Internal contract | Existing CLI surfaces to extend without changing non-stack behavior | `turn`, `next`, `mission next`, `doctor` |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Stack source of truth | `.keel/stacks/<id>/manifest.yaml` | Keeps stack state repo-local, explicit, and optional |
| Board integration | Separate read model instead of adding `Board` fields | Avoids cross-cutting loader churn and keeps stack semantics parallel to the board |
| Receipt model | Git-native fields in the manifest | Matches the ADR and avoids premature artifact complexity |
| Enforcement strategy | Diagnose and gate first; automate worktree lifecycle later | Safer first slice with lower risk of destructive behavior |

## Architecture

The implementation adds three layers:

1. `keel-core::read_model::mission_stack`
   Loads and validates the local manifest, derives current git/worktree state,
   and projects an optional `MissionStackProjection`.
2. CLI adapter integration
   `turn`, `next`, and `mission next --status` query the projection and add
   stack-specific text/JSON state only when a stack is present.
3. Doctor enforcement
   A new Mission Stack doctor section reuses the same projection to report
   branch, checkpoint, foreign-worktree, and close-state violations.

## Components

- Manifest parser: deserializes the stack manifest and canonical mode/member
  metadata.
- Git state resolver: derives repo root, current branch, current checkout path,
  and linked-worktree metadata for the active checkout.
- Stack projection builder: combines manifest and git state into one
  adapter-friendly projection with derived local status.
- Adapter presenters: render stack context in text and JSON for `turn`, `next`,
  and `mission next --status`.
- Doctor checks: report protocol violations using the same projection so the CLI
  and diagnostics share one source of truth.

## Interfaces

- Manifest interface:
  stack id, steward repo, local repo identity, branch, mode, optional
  checkpoint, member table, member mission linkage, receipts, and foreign
  execution metadata.
- Projection interface:
  active stack id, local member role, local actionability, blocking reason,
  branch/worktree status, checkpoint acknowledgment state, and close-state
  leftovers.
- `next` interface:
  new stack-specific machine-readable decisions for blocked, yield, and
  unsupported foreign-worktree execution paths.

## Data Flow

1. A command loads the board as it does today.
2. The command optionally loads the local Mission Stack projection from
   `.keel/stacks`.
3. If no active stack exists, the command returns current single-repo behavior.
4. If a stack exists, the adapter augments output or routing with stack state.
5. Doctor reuses the same projection to emit stack integrity findings.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Manifest is missing required fields | YAML parsing/validation | Treat as stack configuration error in doctor and surface the parse failure | Fix manifest content |
| Local checkout is on the wrong branch | Git branch comparison | Block stack execution and report wrong-branch diagnostics | Switch to `stack/<id>` |
| Foreign execution is attempted in the primary checkout | Worktree metadata and manifest expectation | Emit unsupported foreign-worktree status and diagnostic failure | Re-run inside the managed worktree |
| Closed stack still has managed worktree leftovers | Manifest close-state plus worktree scan | Warn in doctor without deleting paths | Review and clean leftovers explicitly |
