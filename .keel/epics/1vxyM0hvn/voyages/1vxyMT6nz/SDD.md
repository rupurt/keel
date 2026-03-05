# Semantic Conflict Detection For Parallel Next - Software Design Description

> Select low-conflict parallel stories using semantic code-structure analysis, conservative confidence thresholding, and pairwise blocker explanations.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extends `keel next --parallel` with a semantic conflict engine that evaluates candidate story pairs before recommending concurrent execution. The design combines readiness, pairwise semantic signals, confidence-weighted conflict scoring, and conservative threshold gating. Uncertain pairs are blocked and explained.

## Context & Boundaries

In scope:
- Pairwise risk model for stories already considered ready by queue policy.
- Explanation payloads for blocked and allowed decisions.
- Optional metadata override (`blocked_by`) for explicit constraints.
- Doctor checks for metadata and coherence.

Out of scope:
- Repository-wide static analysis graph indexing.
- User-configured risk profile tuning.
- Historical conflict mining from git.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `read_model::traceability` and queue policy projections | Internal | Candidate and dependency context | Current crate API |
| `cli::commands::management::next_support` | Internal | Decision and rendering integration | Current crate API |
| `doctor` check framework | Internal | Parallel conflict coherence checks | Current crate API |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Risk posture | Conservative-by-default blocking | User preference prioritizes minimum merge-conflict risk |
| Eligibility control | Single global threshold constant | Keeps behavior predictable without config expansion |
| Explanation shape | Pairwise blockers with reasons | Directly actionable for humans and automation |
| Override model | Optional `blocked_by` metadata only | Minimal explicit override with high planning value |

## Architecture

Core flow:
1. Build ready candidate set using existing queue/readiness projection.
2. Generate pairwise semantic feature vectors (code-structure, scope, architectural touchpoint signals).
3. Score each pair with conflict risk + confidence.
4. Apply threshold and conservative default blocks.
5. Apply explicit `blocked_by` overrides.
6. Produce actionable set and pairwise blockers for terminal/JSON output.
7. Validate metadata and coherence through doctor.

## Components

- `ParallelFeatureExtractor`
Purpose: derive deterministic semantic pairwise signals.
Inputs: board stories, scope, structural hints.
Output: normalized feature vectors per story pair.

- `ParallelConflictScorer`
Purpose: compute conflict risk/confidence from features.
Behavior: penalize unresolved architectural context; emit conservative scores.

- `ParallelEligibilityGate`
Purpose: apply global threshold and conservative blocking defaults.
Behavior: allow only high-confidence low-risk pairs.

- `ParallelBlockerRenderer`
Purpose: produce pairwise blocker explanations in human and JSON output.
Behavior: stable ordering and deterministic messages.

- `ParallelOverrideResolver`
Purpose: enforce optional `blocked_by` metadata.
Behavior: explicit overrides take precedence over inferred allow.

- `DoctorParallelConflictCheck`
Purpose: detect invalid `blocked_by` references and contradictory constraints.
Behavior: return actionable diagnostics with fix guidance.

## Interfaces

- Next decision projection gains pairwise blocker contract:
  - `blocked_pairs: [{story_id, blocked_by, reasons, confidence}]`
  - `actionable_parallel: [story_id...]`
- Story frontmatter (optional):
  - `blocked_by: ["<story-id>", ...]`

## Data Flow

1. `next --parallel` builds initial candidates.
2. Pairwise engine evaluates each candidate pair.
3. Threshold gate + overrides produce allowed/blocked matrix.
4. Selector builds final actionable recommendations.
5. Renderer emits pairwise explanations.
6. Doctor independently validates metadata coherence.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing semantic context for pair | Feature extraction lacks required signal | Mark pair blocked with conservative reason | Implement additional signal extraction or add explicit `blocked_by` |
| Invalid `blocked_by` target | Doctor validation and parser checks | Emit doctor error | Fix referenced story IDs |
| Contradictory override graph | Doctor coherence check | Emit doctor error with pair details | Remove contradictory metadata or align constraints |
| Rendering contract drift | Projection tests | Fail tests | Update renderer and JSON contracts together |
