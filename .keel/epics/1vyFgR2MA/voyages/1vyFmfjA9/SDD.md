# Goal-to-Requirement Lineage - Software Design Description

> Connect canonical PRD goal IDs to FR/NFR requirements and surface orphaned, invalid, or missing goal links in planning diagnostics.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage makes PRD goals machine-checkable. The design extends epic PRDs with canonical `GOAL-*` identifiers, adds goal-link metadata to FR/NFR requirement rows, and reuses one parser path for diagnostics and planning read surfaces.

The result is a direct strategic seam inside the PRD itself: reviewers can see which product goals are unsupported, over-concentrated, or disconnected before looking at voyage-level decomposition.

## Context & Boundaries

In scope:
- Goal ID parsing in PRD goals table
- Goal-link parsing in PRD requirement rows
- Doctor diagnostics for invalid or missing goal links
- Planning read-surface summaries

Out of scope:
- `epic new` scaffolding inputs
- PRD-to-SRS parent requirement enforcement
- Scope linkage between PRD and SRS
- Story-level acceptance criteria

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `templates/epic/[name]/PRD.md` | Embedded template | Must evolve to carry goal IDs and requirement goal-link columns | current template contract |
| `src/infrastructure/validation/structural.rs` | Internal module | Already validates authored PRD content and can host stricter goal-link structure checks | current crate API |
| `src/cli/commands/diagnostics/doctor/checks/epics.rs` | Internal module | Emits epic-level planning coherence diagnostics | current crate API |
| `src/read_model/planning_show.rs` | Internal module | Surfaces PRD planning summaries in `epic show` | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Goal ID format | Use canonical `GOAL-*` identifiers in the PRD goals table | Gives one human-readable and machine-checkable goal namespace |
| Link placement | Store goal refs on PRD requirement rows instead of a separate mapping section | Keeps strategic lineage visible next to the requirement it constrains |
| Rollout path | Start with doctor and read-surface enforcement | Preserves a narrow first implementation slice before considering transition gating |
| Fanout model | Allow one goal to support multiple requirements and one requirement to support multiple goals | Matches real product planning where goals and requirements are not strictly one-to-one |

## Architecture

The design adds a second PRD-only lineage layer:

1. Parse goal rows into canonical goal entries.
2. Parse FR/NFR rows into requirement entries with linked goal IDs.
3. Validate that every linked goal exists and every goal participates in at least one requirement link.
4. Emit diagnostics and read-side summaries from the same parsed model.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| Goal Table Parser | Extract canonical `GOAL-*` rows from the PRD goals table | Returns goal IDs plus authored descriptive text |
| Requirement Goal-Link Parser | Extract goal-link metadata from FR/NFR rows | Returns valid/invalid/missing goal refs per requirement |
| Goal Coherence Evaluator | Produce doctor findings for invalid or orphaned lineage | Flags unknown goals, orphan goals, and goal-less requirements |
| Goal Coverage Summary | Render planning-friendly lineage summaries | Groups goals by linked requirements deterministically |

## Interfaces

Expected internal interfaces:
- `parse_prd_goal_entries(prd_content) -> Vec<GoalEntry>`
- `parse_prd_requirement_goal_links(prd_content) -> Vec<RequirementGoalLink>`
- `evaluate_prd_goal_lineage(epic) -> Vec<Problem>`
- `build_goal_lineage_summary(epic) -> Vec<GoalCoverageRow>`

## Data Flow

1. The PRD parser reads the goals table and requirement tables.
2. Goal-link refs are validated against the canonical `GOAL-*` entry set.
3. Doctor renders structural/coherence problems for missing or invalid links.
4. Epic planning projections render goal coverage summaries for human review.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Requirement references unknown goal ID | Goal-link parser cannot resolve the referenced `GOAL-*` | Emit doctor error | Correct the goal ref or author the missing goal |
| Goal has no linked requirements | Goal coverage summary finds zero inbound requirement links | Emit doctor error/warning per final policy | Link the goal from one or more FR/NFR rows |
| Requirement has no goal linkage | Requirement parser finds empty goal-link cell | Emit doctor error | Add one or more canonical goal refs |
| Goal/link ordering unstable | Deterministic tests fail across equivalent fixtures | Fail tests before merge | Stabilize sort order in parser/projection code |
