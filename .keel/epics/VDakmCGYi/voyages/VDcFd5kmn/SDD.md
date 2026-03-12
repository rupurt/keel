# Temporal Routine Gating - Software Design Description

> Teach keel next to evaluate cadence windows, countdowns, and due routine eligibility from routine metadata.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a routine timing evaluator and threads its results into
the `next` projection so scheduled work becomes visible only when it is due.

## Context & Boundaries

Routine timing is read-only in this voyage. It computes due-state and updates
pull visibility, but it does not create stories or modify routine schedules.

```
Routine metadata -> due-state evaluator -> next projection -> human/JSON render
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Routine entity contract | Internal | Source cadence metadata and target scope | Output of voyage `VDcFd11nc` |
| Next read model / projection layer | Internal | Surface scheduled and due state in pull results | Existing next projection APIs |
| Clock abstraction or injectable time source | Internal | Make temporal evaluation deterministic in tests | Current time utilities |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Due-state engine | Centralize cadence evaluation in one helper/service | Avoid duplicated time logic between renderers and selection paths |
| Queue policy | Gate non-due routine work before actionable ranking | Preserve current pull semantics until time conditions are met |
| Output contract | Show both due-now and next-run context | Operators need “why now” and “when later” in the same surface |

## Architecture

The design adds a routine timing evaluator to the read-model path and augments
the `next` decision/render contract with scheduled metadata.

## Components

| Component | Responsibility |
|-----------|----------------|
| Due-state evaluator | Translate cadence metadata into due/not-due state and next eligible time |
| Next projection adapter | Attach scheduled state to routine-derived work items |
| Human/JSON renderers | Expose countdowns and gating rationale consistently |

## Interfaces

| Interface | Input | Output |
|-----------|-------|--------|
| Due-state API | Routine metadata + reference time | Due-state, due-now flag, next eligible time |
| `keel next` projection | Board + reference time | Actionable work plus scheduled routine entries |
| `keel next --json` | Same | Stable machine-readable gating metadata |

## Data Flow

1. Load routines from the board.
2. Evaluate each routine against the reference clock.
3. Filter non-due routine work out of actionable selection.
4. Render due/upcoming state with countdown context in `next`.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Invalid cadence metadata | Parser/evaluator validation | Mark routine as invalid with actionable error | Fix routine bundle before relying on schedule |
| Missing reference time in tests | Injection/setup failure | Fail fast in test harness | Supply deterministic clock input |
| Renderer/projection drift | Snapshot or integration assertion failure | Block release of output contract changes | Update projection and rendering together |
