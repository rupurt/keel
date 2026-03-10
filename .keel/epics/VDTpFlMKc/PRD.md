# Role Based Architecture - Product Requirements

## Problem Statement

The current system hardcodes 'human' and 'agent' roles into queues, limiting autonomy and extensibility for finer-grained AI or human permissions. As agents become more capable, they should be able to operate in management and planning capacities, not just pure execution.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Rebrand the flow queues to reflect the nature of the work rather than the actor. | Queues are displayed as "Management" and "Execution" | 100% replacement in UI |
| GOAL-02 | Implement a role-based taxonomy for command authorization and work assignment. | `keel next` uses `--role` instead of `--agent`/`--human` | 100% of queue commands |
| GOAL-03 | Integrate personality/context templates based on the active role. | Agents receive specialized system prompts based on role | Implemented for core roles |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Autonomous Harness | An AI script running `keel` commands. | Needs fine-grained roles (e.g. `engineer/software` vs `manager/product`) to access the correct queue. |
| Human Operator | A developer interacting with the board. | Needs to clearly distinguish between management tasks and execution tasks. |

## Scope

### In Scope

- [SCOPE-01] Replacing `--agent` and `--human` flags with `--role <TAXONOMY>`.
- [SCOPE-02] Renaming visual and conceptual queue names from Human/Agent to Management/Execution in `flow` and `next`.
- [SCOPE-03] Adapting the taxonomy parser from the `vibes` repository to authorize transitions (like `accept`).
- [SCOPE-04] Scaffolding personality template injection for harnesses.

### Out of Scope

- [SCOPE-05] Dynamic/runtime creation of new roles outside the basic string parser constraints.
- [SCOPE-06] Changing the underlying 2-queue pull-system structure (it remains a 2-queue system, just renamed).

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `keel next` must accept `--role <TAXONOMY>` and reject legacy `--agent`/`--human` flags with explicit migration guidance. | GOAL-02 | must | Core UX shift for work pull under hard cutover. |
| FR-02 | `keel flow` must display "Management Queue" and "Execution Queue". | GOAL-01 | must | Updates the dashboard terminology. |
| FR-03 | The `accept` command must require a `manager/*` role taxonomy for stories with manual verification criteria. | GOAL-02 | must | Enforces authorization boundaries on subjective work. |
| FR-04 | The system must map `engineer/*` roles to the Execution queue and `manager/*` roles to the Management queue. | GOAL-02 | must | Routes the correct work to the right persona. |
| FR-05 | The system must inject role-specific personality and context templates into actionable harness guidance for core roles. | GOAL-03 | must | Provides necessary context for autonomous completion without a separate prompt-export surface. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Role parsing, queue routing, and template selection must be deterministic for the same taxonomy input. | GOAL-02, GOAL-03 | must | Prevents prompt drift and inconsistent queue assignment for identical role requests. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| CLI Interface | `cargo test` on clap command definitions | Automated tests |
| Queue Routing | Unit tests for `next` algorithm | Automated tests |
| Authorization | Unit tests for `accept` transition gates | Automated tests |
| Role Templates | Unit tests for template resolution and guidance rendering | Automated tests |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The 2-queue system is sufficient for all roles. | We might need a 3rd queue. | Monitor role distribution during implementation. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How do harnesses receive role templates without adding another command surface? | Keel AI | Resolved: extend actionable guidance emitted from role-aware `keel next` responses. |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel flow` displays Management and Execution queues.
- [ ] `keel next --role engineer/software` pulls from the Execution queue.
- [ ] `keel next --role manager/product` pulls from the Management queue.
- [ ] `keel story accept --role manager/product` respects manual-verification authorization.
- [ ] `keel next --role engineer/software --json` and `keel next --role manager/product --json` emit the matching role context templates.
<!-- END SUCCESS_CRITERIA -->
