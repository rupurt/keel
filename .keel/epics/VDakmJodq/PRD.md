# Automation Documentation and Guides - Product Requirements

## Problem Statement

The mission introduces routines, temporal gating, pulse automation, and a
scheduled automation lane, but there is no cohesive operator guide that explains
how those pieces fit together. Without a canonical `GUIDE.md`, users will have
to reverse-engineer the intended workflow from CLI help, scattered mission
artifacts, and source code, which makes business automation adoption brittle and
hard to review.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Publish one canonical guide for recurring-work automation in Keel. | Operators can follow one document from routine authoring through scheduled execution. | `GUIDE.md` becomes the primary onboarding/reference artifact for this mission. |
| GOAL-02 | Document the operational boundaries for pulse and scheduled automation. | The guide explains what is automated, what remains manual, and how idempotent runs are expected to behave. | Operators can review cron/systemd usage without inferring unsupported workflows. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Human Operator | Planner or maintainer setting up recurring automation. | An end-to-end playbook for routine authoring, review, and execution. |
| Reviewer | Teammate validating the automation design. | A concise artifact that matches supported CLI behavior and boundaries. |

## Scope

### In Scope

- [SCOPE-01] `GUIDE.md` covering routine authoring, temporal gating, pulse execution, and scheduled-lane review
- [SCOPE-02] End-to-end examples showing how recurring work moves through the system
- [SCOPE-03] Operational safety guidance for cron/systemd, idempotency, and unsupported workflows

### Out of Scope

- [SCOPE-90] Implementation of the underlying routine, gating, or pulse features
- [SCOPE-91] Marketing or press-release style launch collateral

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Author `GUIDE.md` that explains the routine entity, cadence metadata, target scope, and blueprint authoring flow. | GOAL-01 | must | Users need a canonical explanation of the new recurring-work contract. |
| FR-02 | Document the end-to-end automation workflow from routine definition through `keel next`, `keel flow`, and `keel pulse`. | GOAL-01 | must | The mission spans multiple surfaces and needs one narrative that ties them together. |
| FR-03 | Include operational guidance for cron/systemd execution, idempotency expectations, and unsupported automation boundaries. | GOAL-02 | must | Operators need guardrails, not just feature descriptions. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Guide content must align with supported CLI names and hard-cutover workflow semantics. | GOAL-01, GOAL-02 | must | Documentation should not reintroduce legacy names or unsupported side paths. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Narrative completeness | Manual review or llm-judge | Checklist against routine/gating/pulse/flow coverage |
| Workflow correctness | CLI proof review | Guide steps checked against supported commands |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The routine, gating, and pulse command names remain stable while this guide is authored. | The guide may drift before delivery completes. | Review against current CLI before publishing. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which business automation example best demonstrates value without overcommitting to one scheduler integration? | Docs owner | Open |
| Documentation may lag implementation if command semantics shift during execution. | Engineering | Mitigate with story-level proof review before mission closure |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Operators can use `GUIDE.md` as the primary reference for routine authoring, scheduled review, and pulse execution.
- [ ] The guide includes an end-to-end example that maps recurring work from definition to execution surfaces.
- [ ] The guide explicitly documents safety boundaries and unsupported automation assumptions.
<!-- END SUCCESS_CRITERIA -->
