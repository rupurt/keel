# Resolve HEAD Show Selectors - Software Design Description

> Add a shared HEAD-relative selector path and wire it into show commands using the canonical stable list order for each entity type.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces one shared selector-resolution path for show commands. Each show command keeps accepting exact IDs, but it also routes selector strings through a common HEAD-relative parser and resolver. The resolver pulls a canonical default ordering for the target entity type from the same list-order source the matching list command uses, then translates HEAD, HEAD~, HEAD~~, and HEAD^ into a concrete ID before the existing show renderer runs.

## Context & Boundaries

### In Scope

- selector parsing and normalization for show commands
- shared ordering providers for showable entities
- show-command adoption and deterministic failures
- regression coverage and CLI guidance

### Out of Scope

- numeric suffix selectors such as `HEAD~3`
- non-show command adoption
- user-configurable sorting or filter-aware HEAD resolution

```
┌─────────────────────────────────────────┐
│              This Voyage                │
│                                         │
│  ┌───────────────────────────────────┐ │
│  │ HEAD selector parser / resolver   │ │
│  └───────────────────────────────────┘ │
│      ↑ stable order providers          │
│      ↓ concrete IDs                    │
│  show command adapters + renderers     │
└─────────────────────────────────────────┘
            ↑
      Board + read models
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Existing board/read-model loaders | internal | Supply canonical entity collections and ordering inputs | current repo |
| Existing show command adapters | internal | Consume resolved IDs without changing rendering behavior | current repo |
| Clap CLI parsing | internal dependency | Accept selector strings as command arguments | current repo |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Selector semantics | Treat `HEAD^` as the same single-step backward move as `HEAD~` in this CLI. | The command model is linear list history, not git ancestry. |
| Ordering source | Reuse canonical list-order helpers or extract shared ordering functions from the same underlying data paths. | Prevents show/list drift and satisfies the mission goal. |
| Error model | Fail fast with deterministic messages for empty sets, unsupported syntax, and out-of-range offsets. | Harnesses need stable failures and humans need clear recovery guidance. |

## Architecture

The implementation should stay in the CLI/read-model boundary:
- a shared selector module parses and normalizes selector strings
- per-entity ordering providers expose the default ordered IDs for each showable entity type
- show command adapters resolve selector strings into IDs before delegating to existing rendering/report functions
- regression tests cover parser behavior, ordering reuse, and command outputs

## Components

- Selector parser:
  Parses exact IDs and HEAD-relative forms into either literal IDs or relative offsets.
- Ordering providers:
  Return canonical default ordered IDs for missions, epics, voyages, stories, bearings, ADRs, and routines.
- Show command adapters:
  Replace direct `require_*` lookup on raw user input with `resolve_selector(entity_kind, selector) -> id`.
- Regression suite:
  Verifies exact-ID compatibility, HEAD-relative success paths, and deterministic error outputs.

## Interfaces

- `parse_head_selector(input: &str) -> Result<HeadSelector>`
- `resolve_head_selector(board: &Board, entity_kind: ..., selector: &str) -> Result<String>`
- `ordered_<entity>_ids(board: &Board) -> Vec<String>` or equivalent shared projection helpers
- show commands continue to expose the same CLI shape, but the `<id>` argument becomes a selector input

## Data Flow

1. CLI receives a show selector string.
2. Shared parser classifies it as exact ID or HEAD-relative offset.
3. Resolver loads the canonical ordered IDs for the target entity type.
4. Resolver maps the selector to a concrete ID or returns a deterministic error.
5. Existing show command code loads/render the entity by concrete ID.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Empty entity set | Ordered ID list is empty | Return a deterministic “no <entity> found” error for the show command | Create the entity or choose another entity type |
| Selector walks past history | Relative offset exceeds ordered ID count | Return a deterministic out-of-range error with available depth | Use a shorter HEAD-relative form or exact ID |
| Unsupported syntax | Parser sees invalid trailing tokens or mixed forms | Reject before entity lookup with canonical guidance | Use exact ID or supported HEAD forms |
| Ordering drift between list/show paths | Regression tests compare selector head to list-order head | Fail tests and block rollout | Reuse or refactor the ordering source until paths converge |
