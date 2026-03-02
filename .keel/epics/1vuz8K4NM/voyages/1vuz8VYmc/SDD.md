# Policy Module and Queue Semantics - Software Design Description

> Create one queue-policy source for thresholds, flow derivation, and human/agent next prioritization.

**SRS:** [SRS.md](SRS.md)

## Overview

Introduce a dedicated `queue_policy` domain module that owns threshold constants and queue decision semantics. Existing consumers (`next` algorithm and flow health/bottleneck rendering) call this module for derivation and priority logic. This removes duplicated constants and guarantees human/agent queue boundaries are enforced uniformly.

## Context & Boundaries

```
┌────────────────────────────────────────────────────┐
│                   queue_policy                     │
│                                                    │
│  thresholds  actor-boundary  priority-derivation   │
└───────────────┬────────────────────┬───────────────┘
                │                    │
         src/next/algorithm.rs   src/flow/{metrics,bottleneck}
```

In scope:
- Policy extraction and integration in `next` and `flow`.
- Human-mode queue boundary enforcement.
- Tests and docs alignment.

Out of scope:
- Schema migrations and compatibility removal.
- Transition-engine redesign (handled in voyage `1vuz8dYT5`).

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `crate::flow::metrics` | Internal module | Supplies queue counts consumed by policy derivation. | current |
| `crate::next::algorithm` | Internal module | Uses policy decisions for action selection. | current |
| `ARCHITECTURE.md` | Documentation artifact | Must reflect canonical thresholds and ordering. | current |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Policy location | New `src/policy/queue.rs` (or equivalent) | Single source of truth for thresholds/orderings. |
| Consumer integration | Keep existing `next` and `flow` entry points; replace internals only | Minimize command-surface churn. |
| Human-mode behavior | Explicitly disallow `Work` decisions in human mode | Enforces the 2-queue actor contract from user decision. |
| Threshold semantics | One constant set reused by both next and flow | Prevents behavioral drift. |

## Architecture

- `queue_policy` exposes:
  - Threshold constants.
  - Flow-state derivation helpers.
  - Actor decision eligibility helpers.
- `next` imports policy helpers to evaluate blocked/accept/research/planning/work decisions.
- `flow` imports policy helpers for bottleneck and action summaries.
- Tests assert that flow and next classify the same queue states consistently.

## Components

- `QueuePolicy` (new):
  - Purpose: canonical queue rules.
  - Interface: pure functions + constants.
  - Behavior: deterministic classification and ordering.
- `next` adapter layer:
  - Purpose: translate policy outputs to `NextDecision`.
  - Interface: existing `calculate_next`.
  - Behavior: human and agent mode decision paths share policy.
- `flow` adapter layer:
  - Purpose: convert metrics to health/bottleneck display text.
  - Interface: existing render/analyze functions.
  - Behavior: no threshold literals outside policy module.

## Interfaces

- `derive_flow_state(queue_depths) -> FlowStateLike`
- `is_verification_blocked(count) -> bool`
- `human_mode_allows(decision_kind) -> bool`
- `priority_order(actor_mode, metrics) -> Vec<DecisionKind>`

All interfaces are internal Rust APIs.

## Data Flow

1. Load board.
2. Compute metrics.
3. Policy module derives state/eligibility.
4. `next` selects decision based on policy order and actor mode.
5. `flow` renders assessment using policy-derived state.
6. Tests validate equivalence between paths for shared conditions.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Policy threshold mismatch regression | Unit/integration test failures | Block merge | Update policy constants and affected assertions together. |
| Human mode emits implementation work | Human-mode integration tests fail | Block merge | Tighten decision filtering via policy guard. |
| Documentation drift from policy | Doc consistency check/review finds mismatch | Treat as release blocker for this voyage | Update docs and tests in same change set. |
