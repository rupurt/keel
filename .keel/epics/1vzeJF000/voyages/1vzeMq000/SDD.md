# CLI Commands - Software Design Description

> Full mission lifecycle commands: new, refine, activate, show, list, pause, achieve, verify, abandon

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage adds the `keel mission` CLI subcommand group following the same
patterns as `keel bearing`, `keel epic`, `keel voyage`, and `keel story`.
Each transition command validates preconditions, applies the state machine
transition, persists to disk, and emits guidance.

## Architecture

### New Files

| File | Purpose |
|------|---------|
| `src/cli/commands/management/mission/mod.rs` | Mission subcommand dispatch and transition logic |
| `src/cli/commands/management/mission/show.rs` | Show and list rendering |
| `src/cli/commands/management/mission/refine.rs` | CHARTER.md analysis and question generation |
| `src/cli/commands/management/mission/guidance.rs` | Next-step and recovery guidance |

### Modified Files

| File | Change |
|------|--------|
| `src/cli/commands/management/mod.rs` | Register mission subcommand |
| `src/main.rs` | Wire mission command |

## Components

### Refine Loop

The `refine` command implements a question-generation protocol:

1. Parse CHARTER.md for completeness (Goals table, Constraints, Halting Rules)
2. Identify first missing/incomplete section
3. Return structured question: `{ "status": "question", "field": "goals", "question": "..." }`
4. When all sections are complete: `{ "status": "ready" }`

The `--answer` flag records the response and re-evaluates.

### Achievement Gate

`keel mission achieve` checks board state against CHARTER.md goals:
- For each `MG-XX` with `board:` verification, check that the referenced condition is met
  (e.g., "all epics done" → all mission-scoped epics have Done status)
- For `metric:` or `manual:` goals, skip (these are human-verified in the `verify` step)
- Gate fails if any `board:` goal is unmet

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Refine as single command (not separate ask/answer) | Simpler harness loop — one command, two modes | Reduces round trips |
| Achievement gates only on `board:` goals | `metric:` and `manual:` need human judgment | Avoids false blocking |
| LOG.md digest threshold: entry count | Simpler than age-based; tune from usage | Start with 50 entries |
