# Doctor And Transition Hard Enforcement - Software Design Description

> Enforce unresolved scaffold/default text as hard failures in doctor and lifecycle transitions for planning coherence.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage unifies coherency enforcement across diagnostics and lifecycle transitions.
Doctor checks become hard-failure oriented for covered unresolved scaffold/default patterns, and story submit/accept gates enforce the same coherency rules for terminal progression.
Generated report artifacts are intentionally excluded in this iteration.

## Context & Boundaries

In scope:
- Doctor check modules and structural validation helpers for scaffold/default text classification.
- Story-stage-aware coherency checks for story and reflection artifacts.
- Transition gate enforcement for submit and accept.

Out of scope:
- Generated report artifact placeholder enforcement.
- Migration/fixing legacy board artifacts.
- Additional lifecycle stages.

```
┌─────────────────────────────────────────┐
│ Coherency Enforcement Layer             │
│                                         │
│  Doctor     Shared Rules  Transition   │
│  │         │  │         │  │         │ │
│  └─────────┘  └─────────┘  └─────────┘ │
└─────────────────────────────────────────┘
        ↑               ↑
   [Story docs]    [Lifecycle actions]
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `structural::is_placeholder_unfilled` and related helpers | internal service | Base unresolved marker detection | existing + extended |
| Doctor check orchestration | internal service | Category severity/reporting | existing |
| State-machine gating | domain service | Transition allow/block decisions | existing |
| Story lifecycle command adapters | interface adapters | Submit/accept command behavior | existing |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Severity policy | Treat unresolved scaffold/default text as errors for covered checks | Matches hard-cutover coherence goals |
| Stage policy | Enforce story/reflection completion only at `needs-human-verification` and `done`, plus submit/accept transitions | Aligns validation with lifecycle semantics |
| Scope policy | Exclude generated report artifacts for this voyage | Keeps first enforcement slice focused and low-risk |
| Rule sharing | Reuse shared helpers between doctor and transition gates | Prevents behavior drift between diagnostics and runtime |
| Compatibility strategy | Remove warning-oriented legacy expectations | Ensures hard-cutover behavior remains explicit |

## Architecture

1. Extend/centralize unresolved scaffold/default detection helpers.
2. Update doctor checks to emit error-level problems for covered artifacts.
3. Add stage-aware story/reflection coherency checks for terminal stages.
4. Integrate submit/accept gating checks with shared coherency rules.
5. Add parity and regression tests to prevent diagnostic/runtime divergence.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| Unresolved scaffold detector | Identify TODO/default scaffold signatures | Returns deterministic matches and patterns |
| Doctor enforcement checks | Surface hard failures in diagnostics | Emits errors with actionable paths/patterns |
| Terminal stage coherency check | Validate story/reflection completion at terminal stages | Skips non-terminal states |
| Submit/accept gate integration | Prevent incoherent promotions | Blocks transitions with explicit failure reasons |
| Regression parity tests | Keep doctor and gate behavior synchronized | Fails when severity or scope diverges |

## Interfaces

Validation interface:
- Input: markdown artifact content + story stage + command intent
- Output: violation list with severity, message, path, and offending pattern

Gate interface:
- Story submit/accept calls gate evaluator
- Any unresolved scaffold/default violation blocks transition

## Data Flow

1. Doctor scans board artifacts and invokes structural/coherency checks.
2. Terminal story/reflection checks run only for target lifecycle states.
3. Story submit/accept commands call state-machine enforcement.
4. Shared coherency rules evaluate story and reflection artifacts.
5. Violations return blocking errors; no warning fallback path is allowed.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Terminal story contains default summary/AC scaffold | doctor or gate check detects known patterns | block and report error | replace scaffold with reviewed content |
| Reflection contains default TODO scaffold | doctor or gate check detects reflection markers | block and report error | replace scaffold with concrete knowledge/observations |
| Doctor/gate behavior diverges | parity/regression test failure | block merge/build | align both paths to shared helper |
| Excluded generated artifacts accidentally scanned | scope negative test failure | block merge/build | tighten file-scope filters |
