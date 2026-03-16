# Operational Routine Reliability - Software Design Description

> Routines materialize stories with 100% reliability

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage hardens the routine materialization pipeline by adding scope validation before story creation and doctor checks for routine-to-scope coherence. The pulse command already materializes routines into stories; this work ensures it fails explicitly rather than creating orphaned or misplaced stories.

## Context & Boundaries

```
┌─────────────────────────────────────────┐
│           Pulse Cycle                   │
│                                         │
│  ┌───────────┐  ┌───────────┐          │
│  │ Scope     │→ │ Materialize│          │
│  │ Validator │  │ Story     │          │
│  └───────────┘  └───────────┘          │
│       ↑                                 │
│  ┌───────────┐                          │
│  │ Routine   │                          │
│  │ Projector │                          │
│  └───────────┘                          │
└─────────────────────────────────────────┘
        ↑               ↑
   [Board Model]   [Doctor Checks]
```

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Scope validation location | Inside `validate_target_scope()` in routine_materialization.rs | Single validation path already exists; extend rather than duplicate |
| Doctor check placement | New check in `checks/routines.rs` | Follows existing pattern of per-entity doctor checks |

## Components

### Scope Validator (SRS-01)

Extend `validate_target_scope()` to check voyage terminal state. If the voyage referenced by `target-scope` is `done`, reject materialization with a clear error in the pulse outcome.

### Routine Scope Doctor Check (SRS-02)

Add `check_routine_scope_coherence()` to `checks/routines.rs`. For each routine, verify its `target-scope` references an existing epic and (if specified) a non-terminal voyage. Emit a warning for missing scope and an error for terminal scope.

### Structured Pulse Output (SRS-03)

The pulse command already tracks materialization outcomes internally. Ensure the non-scene, non-json output prints one line per routine with its outcome (Created, Skipped, Rejected) and reason.
