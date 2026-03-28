# Narrative Contract Tests And Drift Guards - Software Design Description

> Turn the strongest CLI, turn-loop, scene, and routing claims from the docs into executable contract tests and regression guards.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage hardens the public narrative into executable tests. It uses the new command catalog, turn projection, scene contracts, and roles surface to check that the docs and code still describe the same product.

## Context & Boundaries

In scope: targeted docs drift guards and contract tests around the new surfaces. Out of scope: broad snapshot testing or full static-site parsing. The tests should verify the public claims that matter most without turning docs maintenance into a fight.

```
docs fragments + canonical surfaces
            |          | 
            +----+-----+
                 |
          drift / contract tests
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Command catalog | Internal | Authoritative source for CLI family and scene metadata claims. | voyage 1 output |
| Turn and scene projections | Internal | Authoritative source for turn-loop and scene-dependency claims. | voyage 2 output |
| Roles surface and next explanation | Internal | Authoritative source for routing claims. | voyage 3 output |
| MDX docs files | Repo content | Narrative source fragments to compare against canonical metadata. | current docs tree |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Drift-test scope | Read targeted docs fragments and lists, not whole rendered pages | The tests should protect product meaning without becoming brittle to layout changes. |
| Canonical source | Always compare docs to command/turn/scene/roles metadata, never string-to-string between docs pages | The engine should be the executable contract once these voyages land. |
| Coverage | Protect the strongest claims first: command families, turn-loop commands, scene dependencies, and routing examples | Those are the precise seams that motivated the mission. |

## Architecture

Extend `crates/keel-cli/src/drift_tests.rs` or nearby regression modules with targeted checks that read the docs files and compare them to canonical metadata. Update the affected docs pages only where new commands or surfaces need to be acknowledged.

## Components

- CLI atlas drift tests: compare documented family command lists to the command catalog.
- Turn-loop drift tests: compare phase command examples to the turn projection.
- Scene contract guards: compare documented `--scene` surfaces and heartbeat/routing claims to the scene registry.
- Routing drift tests: compare role/lane docs examples to `keel roles` and `next --explain`.

## Interfaces

No new public runtime APIs are required; the voyage consumes canonical metadata and docs files inside tests.

## Data Flow

1. Read canonical metadata from the new command/turn/scene/roles surfaces.
2. Read the targeted docs fragments that describe those surfaces.
3. Compare the two in focused regression tests.
4. Fail fast when a public narrative claim drifts.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Docs fragment becomes too structural for a stable assertion | brittle test failures | narrow the assertion to the semantic list/claim | keep tests focused on meaning |
| Canonical metadata changes without docs updates | drift-test failure | update docs and tests intentionally | preserve synchronized changes |
| Tests only compare strings and miss semantic drift | review/design smell | assert against parsed command lists and canonical enums where possible | improve helper parsers instead of weakening the contract |
