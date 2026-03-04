# Icebox-First Story Intake - Software Design Description

> Make story creation default to icebox so planning work never starts as active execution.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage changes story intake semantics so `story new` always creates planning-stage work in `icebox`, then updates lifecycle guidance and tests so users have an explicit thaw/start path and doctor remains clean.

## Context & Boundaries

In scope:
- `src/cli/commands/management/story/new.rs`
- Story template/frontmatter generation path
- Story creation guidance text
- Regression tests covering doctor coherence after creation/linking

Out of scope:
- Existing story migration
- Stage-machine transition contract changes beyond creation defaults

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Story template rendering | Internal | Initial frontmatter status materialization | current crate API |
| Story lifecycle commands (`thaw`, `start`) | Internal | Explicit progression out of intake stage | current crate API |
| Doctor coherence checks | Internal | Validate policy removes immediate planned-voyage errors | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Default stage | Force `icebox` for all `story new` operations | Keeps newly created stories in planning intake until explicitly activated |
| Backward behavior | No compatibility mode for backlog-by-default | Hard-cutover policy requires one canonical path |
| Transition UX | Surface thaw/start guidance immediately after create | Avoids ambiguity introduced by stricter intake stage |

## Architecture

1. Update story creation logic to write `status: icebox` during initial file generation.
2. Ensure any stage override paths in creation code converge to the same default.
3. Update command guidance text emitted after story creation.
4. Add tests for unscoped/scoped story creation and doctor coherence.

## Components

- Story Creation Adapter (`story/new.rs`):
  - Owns stage default assignment and final persisted frontmatter.

- Lifecycle Guidance Renderer:
  - Emits actionable next-step commands after creation (thaw/start sequence).

- Doctor Regression Fixtures:
  - Board setups proving planned-voyage story intake no longer fails due to creation defaults.

## Interfaces

- `story new <title> --type <type>` persists `status: icebox`.
- Existing commands remain:
  - `story thaw <id>` to move to backlog
  - `story start <id>` after thaw/backlog readiness

## Data Flow

1. User invokes `story new`.
2. Creation pipeline renders template/frontmatter with `icebox` status.
3. Story can be linked/scoped without planned-voyage coherence breakage.
4. User receives thaw/start guidance for execution.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unexpected non-icebox status in created file | Post-create fixture/test assertion fails | Fail tests and block merge | Fix creation path to canonical status assignment |
| User attempts to start directly from icebox | State machine validation error | Show recovery guidance pointing to thaw | Run `keel story thaw <id>` then `keel story start <id>` |
