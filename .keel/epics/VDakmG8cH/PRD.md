# Pulse Automation Engine - Product Requirements

## Problem Statement

Recurring work will not become operational until the board has a safe,
non-interactive way to turn due routines into real work during cron/systemd
execution. Operators also need `keel flow` to show the scheduled automation lane
so upcoming and due automation demand is visible before work is materialized.
Without both pieces, temporal gating stays informational and system-level
automation remains external and opaque.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Provide a non-interactive `keel pulse` entry point for automation environments. | Pulse runs end-to-end without prompts and reports what it evaluated or triggered. | Cron/systemd can invoke pulse safely. |
| GOAL-02 | Materialize due routine work safely and idempotently. | Eligible work is created once per due window and repeated runs do not duplicate output. | Frequent automation runs remain safe. |
| GOAL-03 | Make scheduled automation visible in `keel flow`. | Flow surfaces a scheduled lane or equivalent scheduled-capacity view. | Operators can review upcoming automation before or after pulse runs. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Automation Runner | Cron/systemd or another non-interactive invoker. | A command that can run unattended and produce actionable status output. |
| Human Operator | Maintainer observing recurring work. | Visibility into what pulse will do and what scheduled work is pending. |

## Scope

### In Scope

- [SCOPE-01] `keel pulse` command surface and non-interactive execution semantics
- [SCOPE-02] Due routine materialization and duplicate-prevention behavior
- [SCOPE-03] Scheduled automation visibility in `keel flow`

### Out of Scope

- [SCOPE-90] Long-running scheduler daemons or hosted automation services
- [SCOPE-91] End-user documentation and narrative guides

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Provide a `keel pulse` command that evaluates due routines and runs an automation cycle without interactive input. | GOAL-01 | must | Automation environments need a stable, unattended entry point. |
| FR-02 | Materialize due routine work exactly once per eligible window and skip already-created work safely on repeated runs. | GOAL-02 | must | Pulse must be safe for cron frequency and avoid duplicate work creation. |
| FR-03 | Extend `keel flow` to surface scheduled automation demand and due/upcoming routine visibility. | GOAL-03 | must | Operators need a planning surface for automation load, not just a fire-and-forget command. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Pulse remains idempotent and safe for frequent cron/systemd execution. | GOAL-02 | must | Repeated automation runs are the normal operating mode. |
| NFR-02 | Pulse and scheduled-lane output remain structured and observable enough for operator review and regression tests. | GOAL-01, GOAL-03 | should | Automation has to be diagnosable when a run skips or materializes work unexpectedly. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Pulse execution | CLI/integration tests | Non-interactive command and cycle summary proofs |
| Idempotent materialization | Integration tests | Duplicate-prevention and repeated-run assertions |
| Scheduled lane visibility | CLI/rendering tests | `keel flow` scheduled-lane or capacity output proofs |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Routine due-state calculation from epic `VDakmCGYi` is available before pulse execution work begins. | Pulse would need to duplicate gating logic and drift from `keel next`. | Sequence delivery after temporal gating lands. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How should pulse report partial failures when some routines materialize successfully and others do not? | Engineering | Open |
| A weak scheduled-lane model could make flow noisy rather than useful. | Product | Mitigate during voyage design |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel pulse` can be run unattended and reports what automation work it evaluated and triggered.
- [ ] Repeated pulse runs do not duplicate materialized work for the same due routine window.
- [ ] `keel flow` gives operators explicit visibility into due and upcoming scheduled automation work.
<!-- END SUCCESS_CRITERIA -->
