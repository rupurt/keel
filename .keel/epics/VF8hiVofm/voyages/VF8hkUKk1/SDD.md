# First-Class Turn Loop And Scene Contracts - Software Design Description

> Make turn structure and scene semantics explicit projections so visual surfaces render canonical engine state instead of distributed command-local interpretation.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage turns two narrative concepts into explicit CLI/read-model surfaces: the turn loop and scene contracts. `keel turn` will expose the turn phases directly, while a shared scene-contract registry will document which scenes exist and what canonical signals they depend on.

## Context & Boundaries

In scope: a read-only turn projection, a new `keel turn` command, and a scene-contract registry. Out of scope: redesigning existing scene art or changing the underlying heartbeat/topology models beyond exposing them through cleaner contracts.

```
board + command catalog + existing projections
                 |
      +----------+-----------+
      |                      |
      v                      v
 turn loop projection   scene contracts
      |                      |
      v                      v
  keel turn          scene-facing commands/tests
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Command catalog | Internal | Supplies phase command metadata and scene-capable command list. | voyage 1 output |
| Heartbeat and health projections | Internal | Scene contracts should reference the canonical signals already used by flow/workshop/health. | current read models |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Turn surface | Add a dedicated `keel turn` command | The turn loop is central enough in the docs to deserve a first-class CLI inspection surface. |
| Projection ownership | Keep turn semantics in a read-model module and scene descriptors in shared CLI/read-model metadata | The turn loop is a product projection; scene contracts sit between projections and renderers. |
| Scene contract detail | Start with ids, commands, scene flag shape, and canonical dependency labels | That is enough to eliminate ad hoc lists without overbuilding a rendering DSL. |

## Architecture

Add a `turn_loop` projection module and a `turn` command adapter. Add a `scene_contracts` module describing scene surfaces and their dependencies. Existing commands such as `flow` and `workshop` continue to render their art, but tests and docs-facing logic consume the central contract data.

## Components

- Turn-loop projection: maps the documented phases to command surfaces and high-level board cues.
- `keel turn` command: renders the projection in text and JSON.
- Scene contract descriptor: describes a scene surface, owning command, and canonical signal labels.
- Scene registry helpers: list scene surfaces for tests and docs drift guards.

## Interfaces

Key interfaces should resemble:

- `project_turn_loop(board_dir) -> TurnLoopProjection`
- `all_scene_contracts() -> &[SceneContract]`
- `scene_contract_for_command(name) -> Option<&SceneContract>`

## Data Flow

1. Read command metadata and existing board/health/heartbeat signals.
2. Build one turn-loop projection.
3. Render that projection in `keel turn`.
4. Define scene contracts for every `--scene` surface and reuse them in tests/drift guards.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Turn projection drifts from docs phases | turn regression test failure | keep phase vocabulary centralized | update projection or docs deliberately |
| Scene contract omits a scene-capable command | catalog/scene test failure | fail during tests | add the missing scene descriptor |
| `keel turn` output becomes ambiguous for harnesses | JSON contract test failure | preserve stable fields | update tests and docs together for deliberate schema changes |
