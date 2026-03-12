# Pulse Automation Execution - Software Design Description

> Introduce a non-interactive pulse command that materializes due routine work and surfaces scheduled capacity in flow.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage turns temporal routine state into automation behavior. It adds a
single-shot pulse command, makes materialization idempotent, and projects
scheduled automation pressure into `keel flow`.

## Context & Boundaries

Pulse is an on-demand automation cycle, not a daemon. The system assumes an
external scheduler invokes the command repeatedly and expects stable output.

```
routine due-state -> pulse command -> materialization -> flow scheduled view
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Routine due-state logic | Internal | Determine which routines are eligible in a pulse cycle | Output of voyage `VDcFd5kmn` |
| Story creation/lifecycle services | Internal | Materialize routine work into board artifacts | Existing story application services |
| Flow projection/rendering | Internal | Surface scheduled automation demand | Current `flow` read-model stack |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Pulse execution model | Single command invocation per automation cycle | Matches cron/systemd operating model |
| Duplicate prevention | Record or derive enough routine/run state to skip already-materialized windows | Required for safe repeated runs |
| Flow visibility | Surface scheduled work in the existing flow surface instead of a separate report | Keeps automation review in the normal operator workflow |

## Architecture

The voyage adds a pulse application path that consumes due-state, calls
materialization services, and then projects scheduled pressure into flow.

## Components

| Component | Responsibility |
|-----------|----------------|
| Pulse command handler | Parse options, run one automation cycle, emit summary |
| Materialization service | Create work from due routines and enforce one-window idempotency |
| Scheduled flow projection | Show due/upcoming automation demand in flow |
| Diagnostic output contract | Explain created, skipped, and deferred automation work |

## Interfaces

| Interface | Input | Output |
|-----------|-------|--------|
| `keel pulse` | Current board state + optional execution flags | Cycle summary and materialized work side effects |
| Pulse/materialization API | Eligible routines | Created work + skipped/deferred reasons |
| `keel flow` scheduled view | Board + routine schedule state | Scheduled lane or capacity section |

## Data Flow

1. Pulse loads due routine state from the board.
2. Eligible routines are checked against duplicate-prevention rules.
3. New work is materialized for routines that are due and not yet satisfied for the window.
4. Pulse reports created/skipped work and flow projects scheduled demand from the same state.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Pulse invoked with invalid routine state | Eligibility/materialization validation | Report failed routine with actionable detail | Fix routine or prior board state, then rerun |
| Duplicate materialization attempt | Duplicate-prevention check | Skip creation and record reason | Safe to rerun pulse later |
| Flow scheduled projection drift | Rendering or projection assertion failure | Fail test/doctor before rollout | Update pulse and flow projections together |
