# Specify Managed Foreign Worktree Lifecycle - Software Design Description

> Define how Keel creates, validates, reuses, and garbage-collects managed foreign worktrees on stack branches for outside reactor execution.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines the isolation boundary for foreign stack execution. The
design uses managed git worktrees as the only approved location for outside
reactor work in another member repository, validates branch and ownership before
execution, and ties cleanup to stack closure.

## Context & Boundaries

### In Scope

- managed foreign worktree requirement
- branch and ownership validation
- create, reuse, and cleanup lifecycle
- command and hook enforcement seams

### Out of Scope

- OS-level workspace sandboxing
- general-purpose worktree tooling unrelated to Mission Stack
- non-git foreign execution fallbacks

### External Actors

- foreign reactor
- member repo owner
- git worktree command boundary
- stack-close lifecycle trigger

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Git worktree support | External contract | Create isolated checkouts for foreign execution | `git worktree` |
| Mission Stack protocol | Internal contract | Decide when foreign execution is allowed and when cleanup happens | Epic `VGdxE0AFZ` |
| Hook and diagnostics seams | Internal contract | Reject unsupported foreign execution and surface leftovers | Existing hook and doctor model |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Execution location | Foreign work runs only in managed worktrees | Prevents accidental edits in the member repo's primary checkout |
| Branch contract | Managed worktrees use `stack/<id>` or approved stack-derived heads | Preserves stack provenance and handoff traceability |
| Cleanup timing | Garbage collection happens when the stack closes | Matches the requested lifecycle and avoids premature deletion |
| Cleanup safety | Ambiguous leftovers are reported, not silently deleted | Keeps cleanup auditable and conservative |

## Architecture

The lifecycle has four stages:

1. ensure: create or reuse the managed foreign worktree
2. validate: confirm branch, ownership, and cleanliness before execution
3. execute: allow foreign stack turns only in the approved worktree
4. close: garbage-collect or report leftovers when the stack closes

## Components

- Worktree manager: resolves the expected managed path and performs create or
  reuse operations.
- Validation gate: confirms branch, member ownership, and local cleanliness.
- Hook bridge: shares enough state for pre-commit or pre-push checks to reject
  unsupported foreign execution.
- Cleanup runner: removes managed worktrees on stack close or reports leftovers
  that require human review.

## Interfaces

- Ensure interface: stack id, member repo identity, expected branch, and target
  worktree path.
- Validation interface: current checkout path, branch, ownership marker, and
  cleanliness state.
- Cleanup interface: stack-close event plus known managed worktree registry.

## Data Flow

1. A foreign reactor receives permission to work in another member repo.
2. Keel ensures the managed worktree exists for that repo and stack id.
3. Validation checks confirm the worktree is the approved execution path.
4. Foreign work proceeds inside the managed worktree only.
5. When the stack closes, cleanup removes the managed worktree or reports
   leftovers that need human review.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Foreign reactor tries to run in the primary checkout | Validation gate | Reject execution | Re-run inside the managed worktree |
| Managed worktree exists on the wrong branch | Branch validation | Reject execution | Recreate or retarget the worktree to `stack/<id>` |
| Cleanup finds dirty or ambiguous leftover state | Cleanup runner | Report the leftover and avoid silent deletion | Human review or explicit cleanup command |
