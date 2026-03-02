# Unified Transition Enforcement - Software Design Description

> Unify runtime and doctor transition validation behind one gate-driven enforcement path.

**SRS:** [SRS.md](SRS.md)

## Overview

Create an enforcement service layer that wraps transition specs plus gate evaluators and exposes a common result model for runtime commands and doctor reporting. Runtime callers enforce strict blocking semantics; doctor callers use reporting policy and consume the same findings as non-blocking visibility.

## Context & Boundaries

```
┌─────────────────────────────────────────────────┐
│           transition_enforcer service           │
├───────────────────┬─────────────────────────────┤
│ Runtime Commands  │ Doctor Checks               │
│ (strict policy)   │ (reporting policy)          │
└───────────────┬───┴───────────────┬─────────────┘
                │                   │
         transitions/spec     state_machine/gating
```

In scope:
- Shared enforcement orchestration.
- Command migration to enforcer.
- Doctor migration to enforcer-compatible gate outputs.

Out of scope:
- Queue policy unification (voyage `1vuz8VYmc`).
- Schema migrations (voyage `1vuz8jNo3`).

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `src/transitions/spec.rs` | Internal module | Canonical transition legality and target state. | current |
| `src/state_machine/gating.rs` | Internal module | Canonical gate problems and severity. | current |
| `src/commands/diagnostics/doctor` | Internal module | Reporting consumer of validation outputs. | current |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Service shape | A typed enforcer API returning structured findings | Keeps runtime/reporting callers simple and aligned. |
| Policy separation | Runtime and reporting policies are explicit inputs | Prevents accidental blocking in doctor paths. |
| Formatting | Central formatting helper for gate failures | Removes copy-paste and message drift. |
| Rollout | Migrate command handlers incrementally behind tests | Safer than single large replacement. |

## Architecture

- `TransitionEnforcer` core:
  - Validates source state via transition spec.
  - Evaluates gate problems for transition/completion.
  - Applies caller policy (`runtime` vs `reporting`) to classify blocking vs informational findings.
- Runtime adapters:
  - Story/voyage commands call enforcer and execute transition only when no blocking problems.
- Reporting adapters:
  - Doctor checks invoke enforcer/gates in reporting mode and map findings to report sections.

## Components

- `enforcer::evaluate_story_transition(...)`
  - Purpose: one path for story transition checks.
  - Behavior: return normalized findings and formatted failure text support.
- `enforcer::evaluate_voyage_transition(...)`
  - Purpose: one path for voyage plan/start/done checks.
  - Behavior: composes transition gate and completion gate where needed.
- `enforcer::classify_findings(policy, problems)`
  - Purpose: decide runtime-blocking vs reporting-visible.
  - Behavior: deterministic severity classification.

## Interfaces

- `enforce_story(board, story, transition, policy) -> EnforcementResult`
- `enforce_voyage(board, voyage, transition, policy) -> EnforcementResult`
- `format_enforcement_error(entity, transition, findings) -> String`

`EnforcementResult` contains raw problems, blocking subset, and caller-ready summary text.

## Data Flow

1. Command/doctor loads board and target entity.
2. Caller constructs transition intent + policy.
3. Enforcer validates transition legality and runs gate evaluators.
4. Enforcer classifies problems by policy.
5. Runtime caller blocks or executes transition; doctor caller records findings.
6. Shared formatting renders coherent diagnostics.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Command bypasses enforcer path | Tests detect inconsistent behavior/messages | Fail CI | Route handler to enforcer and remove duplicate checks. |
| Policy misclassification (warning vs error) | Runtime/reporting parity tests fail | Fail CI | Correct policy mapping and update fixtures. |
| Missing transition coverage in enforcer | Coverage tests reveal unhandled transition | Fail CI | Add enforcer branch and tests for that transition. |
