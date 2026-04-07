# Define The Initial Mission Request Command Family - Software Design Description

> Define the first implementation-facing mission request slice covering template, parse, validate, draft, apply, and acknowledge semantics.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines the command contract rather than a provider worker. The
design centers on one canonical mission request envelope and six subcommands
that can be chained by Keeper or any other automation surface. `template`
produces the authoring skeleton, `parse` normalizes inbound content, `validate`
checks structural and semantic correctness, `draft` previews resulting planning
artifacts, `apply` performs the mutation, and `ack` emits a provider-facing
summary payload.

## Context & Boundaries

Keel owns request semantics, planning mutation rules, and the canonical command
surface. Keeper and other programs remain outside this boundary and are
responsible for polling, transport, provider auth, and retry orchestration.

```
┌────────────────────────────────────────────────────┐
│                    Keel Voyage                     │
│                                                    │
│  request envelope -> parse/validate -> draft/apply│
│                                 └──────> ack       │
└────────────────────────────────────────────────────┘
          ↑                              ↑
   provider worker / CLI          provider-facing sink
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `keel` CLI runtime | internal | Hosts the canonical mission request subcommands | current workspace |
| Mission planning model | internal | Supplies mission creation and lineage semantics for `draft` and `apply` | current workspace |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Canonical input shape | One provider-neutral mission request envelope | Prevents providers from forking request semantics |
| Command composition | Separate parse, validate, draft, apply, and ack stages | Keeps automation idempotent and reviewable |
| Output mode | Machine-readable stdout with explicit diagnostics | Makes the surface scriptable by Keeper and other tools |

## Architecture

The command family is a thin orchestration layer over three concerns: envelope
normalization, semantic validation, and planning mutation. Each subcommand
advances the same normalized payload one step further without requiring the
caller to know Keel internals.

## Components

- Envelope normalizer: canonicalizes title, request body fields, provider
  references, and operator intent into one internal representation.
- Validation engine: checks required fields, schema shape, and mission-request
  semantics before any mutation occurs.
- Draft/apply planner: computes the mission-facing artifact changes, then either
  previews or executes them.
- Acknowledgement renderer: produces a stable human-facing summary from the same
  normalized request and planning result.

## Interfaces

- `keel mission request template`: emits the canonical authoring template.
- `keel mission request parse`: accepts raw payload input and returns the
  normalized envelope.
- `keel mission request validate`: returns explicit pass/fail diagnostics over
  the normalized envelope.
- `keel mission request draft`: previews the planning slice and resulting board
  lineage without mutating the repository.
- `keel mission request apply`: creates or updates the mission-level planning
  artifacts from a validated envelope.
- `keel mission request ack`: emits a provider-facing acknowledgement payload
  derived from the validated request and planning result.

## Data Flow

1. External automation collects a provider payload and calls `template` or
   `parse`.
2. `parse` normalizes the request into canonical fields and metadata.
3. `validate` confirms the envelope is structurally and semantically safe.
4. `draft` computes the proposed planning move for human or programmatic review.
5. `apply` materializes the approved planning mutation.
6. `ack` renders the acknowledgement text or payload that external automation can
   post back to the provider.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing required request fields | Validation stage | Return explicit field diagnostics | Caller corrects input and re-runs parse/validate |
| Provider-specific payload leaked into core schema | Parse or validate stage | Reject with boundary error | Normalize at the provider worker before invoking Keel |
| Planning mutation would create ambiguous mission state | Draft or apply stage | Return deterministic refusal with reason | Human reviews request intent and adjusts envelope or board state |
