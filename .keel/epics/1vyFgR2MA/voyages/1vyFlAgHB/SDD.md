# Epic Problem Hydration - Software Design Description

> Seed authored problem context from `epic new --problem` and keep goals as PRD-authored content so new epics start with real strategic context instead of scaffold comments.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage upgrades epic scaffolding from a mostly-empty PRD shell to an authored strategic starting point. The design adds explicit CLI support for problem input, removes goals from the CLI contract, and hydrates authored problem content into the epic scaffold during `epic new`.

The goal is not to fully solve goal lineage yet. This slice only ensures that new epics start with authored problem context while leaving multi-row goals to be filled directly in `PRD.md` for later canonical `GOAL-*` work.

## Context & Boundaries

In scope:
- `epic new` CLI inputs
- Epic README/PRD template rendering
- Template token inventory
- Doctor cleanliness for freshly scaffolded epics

Out of scope:
- Canonical `GOAL-*` linkage
- Scope IDs and scope drift
- PRD-to-SRS parent requirement validation
- Historical epic migration

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `src/cli/command_tree.rs` and runtime wiring | Internal module | Exposes and routes the revised CLI argument contract | current crate API |
| `src/cli/commands/management/epic/new.rs` | Internal module | Owns epic scaffolding orchestration | current crate API |
| `templates/epic/[name]/README.md` and `templates/epic/[name]/PRD.md` | Embedded templates | Receive hydrated problem content and revised goal scaffolding behavior | current template contract |
| `src/infrastructure/templates.rs` tests | Internal module | Guards CLI-owned token inventory and placeholder hygiene | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Problem input | Add explicit `--problem` instead of synthesizing filler text | Strategic context should come from the planner, not a template default |
| Goal ownership | Remove CLI-owned goal hydration and leave `Goals & Objectives` authored in `PRD.md` | A single CLI string does not fit a multi-row goals table and would create the wrong ownership boundary |
| Failure mode | Missing or empty strategic input fails fast | Preserves hard-cutover discipline and avoids fake authored content |
| Template scope | Add only the minimal new token(s) needed for problem-only hydration and remove obsolete goal token usage where required | Keeps template contracts small and reviewable |

## Architecture

The scaffolding path remains single-step:

1. CLI collects `name` and `problem`.
2. Epic creation adapter validates and trims strategic inputs.
3. Template renderer injects authored problem content into fresh epic scaffold surfaces.
4. Fresh artifacts are validated by existing structural/doctor hygiene checks.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| Epic New CLI Contract | Accept strategic input at creation time | Adds required `--problem` and removes CLI-owned goal hydration |
| Epic Scaffolding Adapter | Validates and passes author-owned values to templates | Rejects empty input and renders one canonical scaffold path |
| Epic Scaffold Hydration | Places authored text into `Problem Statement` and any summary surface that remains CLI-owned | Produces editable markdown, not placeholder-only sections |
| Token Contract Tests | Guard template ownership and placeholder hygiene | Ensure `problem` is the only CLI-owned strategic token and no placeholder drift appears |

## Interfaces

Expected internal interfaces:
- `epic new --problem <PROBLEM> <NAME>`
- `render(epic_template, [("title", ...), ("problem", ...)])`

The exact epic README summary rendering can vary, but the command must no longer depend on a CLI-owned goal string.

## Data Flow

1. User invokes `epic new` with title and problem.
2. The command validates title case and required authored inputs.
3. The epic adapter renders README and PRD with hydrated problem content and goal sections left for direct authoring.
4. Doctor/template tests verify the output is still canonical and placeholder-clean.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing `--problem` input | CLI parse/runtime validation | Fail epic creation | Re-run with authored problem text |
| Legacy `--goal` usage after contract cutover | CLI parse/runtime validation | Fail fast with actionable message | Re-run with `--problem` and author goals in `PRD.md` |
| Empty problem after trimming | Command validation | Fail fast with actionable message | Provide non-empty strategic input |
| Unknown template token for problem hydration | Template inventory tests | Fail tests before merge | Register token in the CLI-owned bucket and render path |
| Fresh epic scaffold fails doctor checks | Doctor regression fixtures | Block rollout | Fix template hydration until new epics are doctor-clean |
