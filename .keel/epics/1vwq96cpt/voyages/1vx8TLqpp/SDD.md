# Normalize Physical DDD Module Layout - Software Design Description

> Physically align src with DDD layers: cli, application, domain, infrastructure, read_model; remove legacy top-level module duplication.

**SRS:** [SRS.md](SRS.md)

## Overview

Normalize the physical module layout to match the DDD layering model already
described in architecture docs. Migration will proceed in thin vertical slices:

1. Move CLI adapters and command handlers under `src/cli`.
2. Move domain core modules under `src/domain`.
3. Move infrastructure services/adapters under `src/infrastructure`.
4. Remove legacy roots and enforce normalized module graph with tests.

## Context & Boundaries

```
┌─────────────────────────────────────────────────────────────────────┐
│                             CLI Layer                              │
│                 clap parsing, command adapters, output             │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────────┐
│                        Application Layer                            │
│                  use-case orchestration/services                    │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────────┐
│                           Domain Layer                              │
│                 entities, policies, transitions, rules              │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────────┐
│                      Infrastructure Layer                           │
│                fs IO, parsing/loading, templates, reports           │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
┌───────────────────────────────▼─────────────────────────────────────┐
│                         Read Model Layer                            │
│                   flow/capacity/status projections                  │
└─────────────────────────────────────────────────────────────────────┘
```

## Dependencies

<!-- External systems, libraries, services this design relies on -->

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Rust module system | language runtime | Maintains normalized compile-time module graph. | stable |
| Existing command regression suite | internal | Guards behavioral parity during file moves. | repo local |
| Keel workflow commands | internal tooling | Tracks story evidence/verification for each migration slice. | repo local |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Migration granularity | Four stories mapped to layer families | Limits blast radius and keeps commits reviewable. |
| Safety strategy | Keep temporary re-exports only within a story and remove by voyage end | Avoid long-lived dual layouts and confusion. |
| Enforcement strategy | Add explicit architecture contract checks for physical placement + forbidden roots | Prevents regression after normalization. |

## Architecture

- `src/cli/**`: command parsing, dispatch, diagnostics presentation, queue/flow UI.
- `src/application/**`: use cases and process managers.
- `src/domain/**`: model, policy, transitions, state/gating/validation core.
- `src/infrastructure/**`: filesystem adapters, parser/loader, templates, report generation, verification engine.
- `src/read_model/**`: projections consumed by CLI/application.

## Components

- CLI module root: exposes adapter modules and route handlers from normalized paths.
- Domain module root: exposes domain concepts by bounded context and policy family.
- Infrastructure module root: exposes ports/adapters for persistence and generation.
- Architecture contract tests: verify location and import rules for normalized layout.

## Interfaces

- Internal Rust module interfaces only.
- No external API or wire protocol changes in this voyage.

## Data Flow

1. CLI adapter receives command.
2. Application use case orchestrates workflow.
3. Domain layer evaluates rules and transitions.
4. Infrastructure performs IO/template/report/verification work.
5. Read model projections render board/flow/capacity views.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Broken imports during file moves | `cargo test` / compile errors | Fix module declarations and use paths in same story | Re-run tests before submit |
| Behavioral regression after relocation | command regression tests fail | Restore intended call path while keeping new file location | Add regression assertion before finalize |
| Architecture drift reintroduced | architecture contract test failure | Update module placement or dependency edges to satisfy rules | Keep contracts mandatory in quality gate |
