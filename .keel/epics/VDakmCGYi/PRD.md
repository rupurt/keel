# Temporal Gating in Next Algorithm - Product Requirements

## Problem Statement

The current `keel next` pull logic only reasons about static backlog and
in-progress work. Once routines become the canonical source for recurring work,
operators still will not know whether a routine is due now, when it becomes due
next, or why a scheduled process is being held back. That makes recurring work
opaque and prevents the mission from surfacing time-driven automation through
the normal pull loop.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Teach `keel next` to evaluate routine cadence and due-state deterministically. | Due routines and next eligible timestamps are derived from canonical routine metadata. | Operators can explain why a routine is due or not due from board state alone. |
| GOAL-02 | Surface countdown and gating context in `keel next` so scheduled work is reviewable. | Human and JSON next surfaces expose due-now and next-run context. | Scheduled work no longer requires external notes or manual date arithmetic. |
| GOAL-03 | Preserve existing pull prioritization while keeping non-due routine work out of the actionable queue. | Non-due routines do not appear as ready work and due routines integrate into existing ranking rules. | Temporal gating changes the queue only when time conditions are actually satisfied. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Autonomous Harness | Agent loop evaluating what should happen next on the board. | Deterministic due-state and gating signals for recurring work. |
| Human Operator | Planner or maintainer reviewing scheduled work. | Clear countdowns and “why now” context in the normal pull surface. |

## Scope

### In Scope

- [SCOPE-01] Cadence evaluation and due-state derivation for routines
- [SCOPE-02] `keel next` surface changes that show due/scheduled routine context
- [SCOPE-03] Queue gating rules that keep future routine work out of actionable results until due

### Out of Scope

- [SCOPE-90] Materializing work from due routines (`keel pulse`)
- [SCOPE-91] Scheduled lane rendering in `keel flow`

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Evaluate routine cadence metadata into a due/not-due state and a next eligible time using canonical routine data. | GOAL-01, GOAL-03 | must | Recurring work needs one deterministic temporal decision path before it can participate in pull logic. |
| FR-02 | Extend `keel next` human and JSON output to expose due-now and upcoming routine work with countdown or next-run context. | GOAL-02 | must | Operators need to understand why scheduled work is visible and when future work becomes actionable. |
| FR-03 | Ensure non-due routine work is excluded from actionable pull decisions while due routines integrate with existing prioritization semantics. | GOAL-03 | must | Temporal gating should not destabilize current queue behavior or leak future work into the active queue. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Temporal evaluation remains deterministic for identical board state and injected clock input. | GOAL-01 | must | Time-aware queue behavior must be testable and reproducible. |
| NFR-02 | Countdown and gating explanations remain reviewable in CLI output and stable enough for regression assertions. | GOAL-02 | should | Operators and tests both need predictable presentation of scheduled work state. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Temporal evaluation | Rust unit tests | Due-state and deterministic clock coverage |
| Pull surface behavior | CLI/integration tests | `keel next` human and JSON assertions |
| Countdown rendering | Snapshot-style tests or CLI proofs | Stable countdown and gating output examples |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Routine cadence metadata from epic `VDakm8eVW` is available before this epic reaches execution. | Temporal gating would need to invent a temporary contract and create churn. | Sequence implementation so routine foundation lands first. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which cadence syntax becomes the long-term canonical trigger format? | Architecture | Open |
| Countdown wording may confuse users if due-state categories are underspecified. | Product | Mitigate during CLI design review |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Operators can see whether a routine is due now, when it becomes due next, and why it is gated in `keel next`.
- [ ] Non-due routine work no longer appears as actionable pull work.
- [ ] Due-state behavior is deterministic under test with injected clock input.
<!-- END SUCCESS_CRITERIA -->
