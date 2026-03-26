# Two-Pass Speccy Refactor - Software Design Description

> Refactor `speccy` in two passes so its module layout is explicit and Keel depends on a smaller, more stable API.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage refactors `speccy` in two controlled passes. Pass 1 makes the internal architecture explicit by splitting the crate into focused modules while preserving behavior. Pass 2 reduces the public API to a smaller render surface and updates Keel to consume that narrower contract. The result keeps `speccy` reusable while lowering the cost of future changes.

## Context & Boundaries

`speccy` remains a single reusable crate. The voyage does not create sub-crates or introduce new templating semantics. Keel remains the first consumer and proves the reusable boundary by depending only on the reduced public API. Template inventory and board semantics remain host-owned concerns.

```
┌──────────────────────────────────────────────────┐
│                     speccy                       │
│                                                  │
│  catalog.rs  hooks.rs  render.rs  frontmatter.rs│
│                  │            │                  │
│                  └──────┬─────┘                  │
│                         lib.rs                   │
└─────────────────────────┬────────────────────────┘
                          │
                     Keel adapters
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `anyhow` | Workspace crate dependency | Fallible catalog loading and hook execution | workspace |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Module structure | Split `speccy` into `catalog`, `hooks`, `render`, and `frontmatter` modules with `lib.rs` re-exporting the public API. | Separates concerns without fragmenting the crate into multiple packages. |
| Render surface | Replace the current top-level helper matrix with core render entrypoints plus options. | Keeps the public API from growing combinatorially as new render concerns appear. |
| Frontmatter behavior | Keep the current frontmatter mutation algorithm and isolate it behind a dedicated module. | Avoids semantic churn during an architecture-focused refactor while making the complexity visible. |
| Keel integration | Keep Keel wrappers thin and make them consume the reduced `speccy` surface. | Proves the API boundary with the existing first consumer. |

## Architecture

The crate exposes three public concepts:

- Catalog abstraction for loading templates by identifier.
- Rendering primitives with fallible hooks and option-driven transforms.
- Frontmatter mutation utilities for markdown documents.

Internal helpers such as token scanning and frontmatter stripping live inside the relevant modules instead of accumulating in `lib.rs`.

## Components

- `catalog`
  Purpose: host-provided template lookup.
  Interface: `TemplateCatalog`, `MemoryTemplateCatalog`.
- `hooks`
  Purpose: opt-in host callbacks layered around rendering.
  Interface: `RenderHooks` and callback type aliases.
- `render`
  Purpose: token substitution pipeline plus catalog-backed entrypoints.
  Interface: reduced render entrypoints and render options.
- `frontmatter`
  Purpose: generic markdown frontmatter mutation and stripping helpers.
  Interface: `Mutation`, mutation application, frontmatter stripping.

## Interfaces

Pass 2 converges the public rendering contract around two main entrypoints:

- `render(template, replacements, options) -> Result<String>`
- `render_from_catalog(catalog, template_id, replacements, options) -> Result<String>`

`RenderOptions` carries optional hooks, mutation batches, and body-only/frontmatter stripping behavior. Frontmatter mutation remains independently callable for hosts that need it outside the render pipeline.

## Stable Extension Points

- `TemplateCatalog` is the stable host-facing abstraction for template lookup.
- `RenderHooks` and `RenderOptions` are the stable host-facing configuration points for render-time behavior.
- `render` and `render_from_catalog` are the stable render entrypoints the host should build upon.
- `Mutation` and `apply_frontmatter_mutations` remain stable for generic markdown frontmatter edits outside the main render pipeline.

## Host-Owned Responsibilities

- Template inventory and storage strategy remain outside `speccy`.
- Project-specific helper functions, wrapper APIs, and domain-specific frontmatter choices remain outside `speccy`.
- Keel continues to own board-specific scaffolds, command wiring, and any helper names retained for its internal adapter compatibility.

## Data Flow

1. A host provides either a raw template string or a `TemplateCatalog`.
2. The render pipeline performs double-curly token substitution, consulting hooks for unresolved tokens when configured.
3. Optional transforms run in a fixed order: post-processing, frontmatter mutation, then optional frontmatter stripping.
4. Keel adapters expose narrower project-specific helpers on top of the same reduced pipeline.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Template ID missing from catalog | `TemplateCatalog::load` returns an error | Propagate `Result` to the caller | Host selects a valid template or fixes catalog wiring |
| Hook or post-processor fails | Render entrypoint returns an error | Propagate failure without partial silent fallback | Host fixes hook behavior or input |
| Refactor changes supported output unexpectedly | Regression tests fail | Block closure until behavior matches expectations or docs are updated intentionally | Adjust implementation or capture the intended contract change |
