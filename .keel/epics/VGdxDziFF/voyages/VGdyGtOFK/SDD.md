# Specify Stack-Aware Turn Next And Doctor Contracts - Software Design Description

> Define stack-aware outputs and gating behavior for turn, next, mission next, and doctor while preserving repo-local heartbeat semantics.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds Mission Stack awareness at the read-model and command-surface
layer. The design keeps stack state as an additional coordination projection
that feeds existing commands without changing the meaning of repo-local
heartbeat or breaking non-stack repos.

## Context & Boundaries

### In Scope

- stack-aware read models for turn, next, mission next, and doctor
- stack-aware blocking and explanation outcomes
- text and JSON rendering expectations

### Out of Scope

- worktree lifecycle management
- dedicated stack dashboards
- stack-global pacemaker replacement

### External Actors

- stack steward and member operators
- automation callers consuming JSON command output
- repo-local heartbeat and diagnostics read models

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Mission Stack domain model | Internal contract | Supplies stack membership, mode, and receipt state | Epic `VGdxE0AFZ` |
| Existing command catalog and turn loop | Internal contract | Reuses current command families rather than inventing a new mandatory surface | `turn`, `next`, `mission next`, `doctor` |
| Diagnostics framework | Internal contract | Hosts stack rule violations as doctor findings | Doctor checks and read models |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Surface layering | Add stack context to existing commands | Operators already orient and pull through these surfaces |
| Heartbeat scope | Keep heartbeat repo-local | Stack coordination should not distort pacemaker semantics |
| Output form | Support both text and JSON | Humans and automation need the same coordination truth |

## Architecture

The design adds a Mission Stack projection layer consumed by:

- turn projection and renderer
- next decision engine
- mission status rendering
- doctor diagnostic checks

Each command asks the stack projection for local member state, current mode,
relevant receipts, and any active checkpoint gate, then renders or enforces the
result in its own native idiom.

## Components

- Stack status projector: resolves whether the current repo is a stack member and
  what the active gating state is.
- Next-decision adapter: converts stack state into actionable decisions such as
  blocked, yield, or continue.
- Mission-status adapter: summarizes linked member missions and pending handoffs.
- Doctor checks: flag wrong-branch, missing-ack, and unsupported foreign-exec
  conditions.

## Interfaces

- Turn interface: stack id, local repo role, branch, mode, checkpoint, and
  whether local action is currently allowed.
- Next interface: stack-aware decision kinds plus explanatory details in text
  and JSON.
- Mission next interface: linked member missions, pending negotiations, and
  waiting receipts.
- Doctor interface: stack-specific findings grouped with existing diagnostics.

## Data Flow

1. A command loads the local board and resolves workflow topology as usual.
2. The stack projection determines whether the repo participates in an active
   Mission Stack and, if so, the current local member state.
3. The command-specific adapter maps stack state into render or gating behavior.
4. The command emits text or JSON without changing repo-local heartbeat meaning.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Repo is not part of a Mission Stack | Stack projection | Fall back to current single-repo behavior | None required |
| Repo is part of a stack but local action is blocked | Next/turn adapter | Emit stack-blocked or yield guidance | Wait for another member or checkpoint release |
| Stack metadata is inconsistent | Doctor checks | Report a stack contract violation | Repair branch, receipt, or acknowledgment state |
