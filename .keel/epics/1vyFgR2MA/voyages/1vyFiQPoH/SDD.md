# Canonical PRD Requirement Lineage - Software Design Description

> Enforce canonical FR/NFR lineage from epic PRDs into voyage SRS requirements with blocking voyage-plan gates, doctor parity, and epic-wide coverage reporting.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extends keel's existing SRS/story traceability seam upward into the parent epic PRD. The design introduces a reusable parent-requirement lineage model for epic `FR-*` / `NFR-*` rows, validates voyage SRS `Source` references against that model, and reuses the same coherence results in three places:

1. planning transitions such as `voyage plan`
2. doctor diagnostics for epic/voyage planning coherence
3. epic planning read surfaces that summarize parent requirement coverage across voyages

## Context & Boundaries

In scope:
- Epic PRD requirement parsing
- Voyage SRS parent-source validation
- Transition and doctor coherence parity
- Epic-wide coverage aggregation for FR/NFR lineage

Out of scope:
- Goal/objective IDs
- Scope IDs and scope drift
- CLI scaffolding changes for `epic new`
- Legacy board migration

External actors:
- `src/cli/commands/management/voyage/plan.rs`
- `src/cli/commands/diagnostics/doctor/**`
- `src/read_model/planning_show.rs`

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `src/domain/state_machine/invariants.rs` | Internal module | Hosts existing SRS requirement parsing and current uncovered-requirements logic that this voyage extends | current crate API |
| `src/domain/state_machine/gating.rs` / `enforcement.rs` | Internal module | Enforces voyage transition legality and blocking problems | current crate API |
| `src/cli/commands/diagnostics/doctor/**` | Internal module | Renders planning coherence diagnostics and must stay aligned with gates | current crate API |
| `src/read_model/planning_show.rs` | Internal module | Surfaces epic planning summaries and coverage projections | current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Parent lineage source | Parse PRD requirement rows from the parent epic PRD rather than introducing a second metadata file | Keeps epic PRD as the canonical planning document |
| Enforcement location | Block in `voyage plan` using shared coherence helpers | Stops drift before stories become active backlog work |
| Legacy handling | Reject `PRD-*` and custom aliases with no compatibility path | Matches hard-cutover policy and user direction |
| Coverage ownership | Keep coverage aggregation epic-wide and query-side | Reviewers need a full-epic planning view, not voyage-local fragments |

## Architecture

The design adds one shared lineage pipeline:

1. Parse parent PRD requirement rows into structured parent requirement entries.
2. Parse voyage SRS requirement rows into structured child requirement entries.
3. Validate child `Source` references against the parent entry set.
4. Emit:
   - blocking `Problem`s for transition paths
   - diagnostic `Problem`s for doctor paths
   - aggregate coverage rows for epic planning read models

## Components

| Component | Purpose | Behavior |
|-----------|---------|----------|
| PRD Requirement Parser | Extract parent `FR-*` / `NFR-*` entries from epic PRDs | Returns canonical IDs and authored requirement metadata |
| SRS Source Validator | Validate each SRS row's `Source` cell | Ensures exactly one valid parent FR/NFR per SRS requirement |
| Shared Lineage Coherence Evaluator | Produce reusable coherence findings | Feeds both transition blocking and doctor diagnostics |
| Epic Coverage Projection | Aggregate parent coverage across voyages | Marks uncovered parents and counts linked SRS rows |

## Interfaces

Expected internal interfaces:
- `parse_prd_requirement_entries(prd_path) -> Vec<ParentRequirementEntry>`
- `parse_srs_requirement_entries(srs_path) -> Vec<SrsRequirementEntry>`
- `evaluate_prd_srs_lineage(voyage, board) -> Vec<Problem>`
- `build_epic_requirement_coverage(board, epic) -> Vec<CoverageRow>`

Final function names can vary, but the contract must remain single-path and reusable across gate, doctor, and read-model consumers.

## Data Flow

1. `voyage plan` loads the board and selected voyage.
2. The validator loads the parent epic PRD and voyage SRS, parses both requirement sets, and validates the `Source` mappings.
3. Transition paths convert lineage failures into blocking `Problem`s.
4. Doctor paths reuse the same validation result and render planning coherence findings.
5. Epic read surfaces reuse the parsed lineage model to aggregate coverage across every voyage in the epic.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| SRS requirement missing parent source | Parsed SRS row has empty or absent `Source` cell | Block `voyage plan`; emit doctor error | Add one canonical parent FR/NFR ID |
| SRS source uses non-canonical token | Source token does not match `FR-*` / `NFR-*` | Block transition; emit hard-cutover diagnostic | Replace with canonical parent ID |
| SRS source references unknown parent | Source token not found in parent PRD requirement set | Block transition; emit doctor error | Author or correct the parent PRD requirement |
| Epic coverage double counts child rows | Aggregation detects duplicate child ownership | Fail tests and report deterministic coverage bug | Fix lineage ownership logic before rollout |
