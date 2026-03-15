# Canonical Serialization - Software Design Description

> Ensure all entities serialize to a canonical, deterministic format.

**SRS:** [SRS.md](SRS.md)

## Overview

We will standardize the serialization logic for all board entities. This involves using `serde` with deterministic settings for YAML frontmatter and ensuring consistent whitespace management when combining frontmatter with the authored markdown body.

## Context & Boundaries

- **In Scope:** `serde_yaml` serialization settings, `Entity` trait implementations for serialization, and the `template_rendering` logic.
- **Out Scope:** Changing the actual file structure or folder layout.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `serde_yaml` | Library | YAML serialization | 0.9 |
| `serde` | Library | Data modeling | 1.0 |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| YAML Key Order | Alphabetical | Simplest deterministic rule for YAML mapping. |
| Whitespace | Exact `\n\n` separator | Standardizes the visual gap between frontmatter and body. |

## Architecture

Standardization will be enforced at the `infrastructure/storage` and `domain/model` layers.

1.  **Serialization Trait**: Enhance or introduce a trait that guarantees deterministic output for all entities.
2.  **File Writer**: Update the filesystem adapter to ensure terminal newlines and proper spacing.

## Data Flow

`Board State` -> `Entity Objects` -> `Canonical YAML` + `Authored Body` -> `Filesystem`
