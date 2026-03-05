# Scope Lineage and Drift Detection - Software Design Description

> Connect PRD scope items to voyage SRS scope through canonical IDs and detect scope drift in doctor and planning surfaces.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage makes product scope machine-checkable across the PRD-to-SRS boundary. The design introduces canonical scope IDs in the PRD, requires voyage SRS scope references to cite those IDs, and evaluates the resulting mapping for unknown refs, missing coverage, and direct out-of-scope contradictions.

The initial rollout is diagnostic and read-model focused. It gives planners an objective signal when a voyage's tactical scope has drifted away from approved product scope.

## Context & Boundaries

In scope:
- PRD scope-ID parsing
- SRS scope-reference parsing
- Scope drift/coherence diagnostics
- Planning read-surface summaries

Out of scope:
- Problem/goal CLI hydration
- PRD FR/NFR lineage blocking
- Goal-to-requirement linkage
- Story execution-time scope enforcement

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `templates/epic/[name]/PRD.md` and voyage `SRS.md` | Embedded templates | Need canonical scope-ID shapes in both planning artifacts | current template contract |
| `src/infrastructure/validation/structural.rs` | Internal module | Already parses authored sections and can host scope-structure helpers | current crate API |
| `src/cli/commands/diagnostics/doctor/checks/voyages.rs` / `epics.rs` | Internal module | Emits planning coherence diagnostics | current crate API |
| `src/read_model/planning_show.rs` | Internal module | Surfaces planning summaries and can expose scope drift | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Scope identity | Use canonical IDs for scope bullets rather than relying on prose matching | Prose-only scope cannot be validated objectively |
| Contradiction policy | Treat references to PRD out-of-scope items as explicit drift findings | Makes scope violations concrete and reviewable |
| Rollout location | Start with doctor/read-model checks instead of transition blocking | Keeps the first slice narrower and easier to adopt |
| Authoring ergonomics | Preserve descriptive scope text next to the canonical IDs | Planners still need readable scope statements, not only tokens |

## Architecture

The design adds a scope-lineage pipeline parallel to requirement lineage:

1. Parse PRD `In Scope` and `Out of Scope` bullets into canonical scope entries.
2. Parse voyage SRS scope references into linked tactical scope entries.
3. Compare the two sets to find unknown refs, missing mappings, and contradictions.
4. Emit diagnostic findings and planning summaries from one shared scope model.

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| PRD Scope Parser | Extract canonical scope entries from epic PRDs | Separates in-scope and out-of-scope IDs with descriptive text |
| SRS Scope Reference Parser | Extract parent scope refs from voyage SRS scope sections | Preserves authored prose while collecting canonical IDs |
| Scope Drift Evaluator | Compare PRD and SRS scope models | Finds contradictions, missing mappings, and unknown refs |
| Scope Summary Projection | Render planning-friendly scope lineage status | Shows linked scope items and drift findings deterministically |

## Interfaces

Expected internal interfaces:
- `parse_prd_scope_entries(prd_content) -> ScopeEntries`
- `parse_srs_scope_links(srs_content) -> ScopeLinks`
- `evaluate_scope_drift(epic_or_voyage) -> Vec<Problem>`
- `build_scope_lineage_summary(epic_or_voyage) -> ScopeSummary`

## Data Flow

1. The parser reads scope entries from the epic PRD and voyage SRS.
2. Scope refs are matched against canonical parent scope IDs.
3. The evaluator classifies findings:
   - unknown scope refs
   - missing tactical scope mappings
   - out-of-scope contradictions
4. Doctor and planning read surfaces render those findings for review.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| SRS references unknown parent scope ID | Scope-link parser cannot resolve the ID against the PRD scope set | Emit doctor error | Correct the scope ref or author the missing PRD scope item |
| SRS maps work to PRD out-of-scope item | Drift evaluator detects contradiction against the PRD out-of-scope set | Emit scope-drift finding | Re-scope the voyage or update the PRD intentionally |
| SRS scope has no canonical refs | Parser finds prose but no machine-checkable IDs | Emit diagnostic and fail tests where required | Add canonical parent scope IDs to SRS scope bullets |
| Drift ordering unstable | Deterministic tests fail across equivalent fixtures | Fail tests before merge | Stabilize parser and renderer ordering |
