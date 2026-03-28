# Canonical Command Catalog And CLI Taxonomy - Software Design Description

> Create one authoritative command catalog that classifies command families, capabilities, turn phases, scene support, and docs slugs so help text and guidance stop diverging.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage introduces a canonical command catalog module inside the CLI layer and moves the existing hard-coded family and capability logic onto that metadata. The catalog becomes the authoritative description of the public command surface for later turn, scene, and docs-drift work.

## Context & Boundaries

In scope: command metadata, help grouping, and actionable-versus-informational guidance classification. Out of scope: new product behavior for turns, scenes, or routing. The main boundary is between CLI-owned command metadata and core-owned board read models; this voyage keeps the catalog in the CLI because it describes command surfaces rather than board entities.

```
command catalog metadata
      |          \
      v           v
  help groups   capability/guidance
      |
      v
 future turn + scene + docs drift consumers
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `clap` | Library | Existing command-tree construction still owns the parser; the catalog feeds descriptive grouping around it. | workspace current |
| Existing guidance renderers | Internal | Continue rendering canonical next/recovery guidance once command capability is catalog-backed. | current CLI modules |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Catalog ownership | Keep the catalog in `keel-cli` alongside the command tree | It describes the CLI product surface, not a core board entity. |
| Family vocabulary | Preserve the existing public family names from the docs and help output | The mission is converging implementation to narrative, not inventing a new taxonomy. |
| Capability source | Derive actionable/informational classification from the same command descriptors used for family grouping | This removes one duplicated map immediately and creates a path for later drift tests. |

## Architecture

Add a module such as `cli::command_catalog` that exports static command descriptors and helper queries. `command_tree.rs` uses those helpers to render the after-help family groups, and `capability_map.rs` becomes a thin adapter or disappears in favor of catalog-backed queries.

## Components

- Command descriptor: static metadata for one public command surface.
- Family renderer: groups descriptors into the public family layout used by help output.
- Capability adapter: answers actionable-versus-informational questions from the same descriptors.
- Scene-support query: returns the subset of descriptors that expose `--scene`.

## Interfaces

The internal interface should support queries equivalent to:

- `all_command_descriptors()`
- `descriptors_by_family()`
- `descriptor_for_command(name)`
- `scene_command_descriptors()`

## Data Flow

1. Define descriptors for the public command surface.
2. Render help-family sections from grouped descriptors.
3. Resolve capability classification from descriptor metadata.
4. Reuse the same metadata in tests and later voyages.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Catalog omits a public command | drift or unit test failure | fail fast in tests | update the descriptor set before merging |
| Help order changes unintentionally | help rendering regression | compare rendered families in tests | fix ordering metadata or renderer |
| Guidance classification diverges | capability-map test failure | keep one adapter layer only | remove duplicated classification branches |
