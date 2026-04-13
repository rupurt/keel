# Mission Stack Context Engine And Enforcement - Product Requirements

## Problem Statement

Keel documents Mission Stack but does not yet load stack state or enforce protocol rules in read surfaces and command gates.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Load Mission Stack state as a first-class repo-local projection without collapsing multiple boards into one shared board. | A repo with stack metadata can be evaluated deterministically for local member role, mode, receipts, and foreign-worktree state. | Stack projection shipped |
| GOAL-02 | Surface Mission Stack context and gating in the canonical operator read surfaces. | `turn`, `next`, `mission next --status`, and `doctor` report actionable stack state and preserve current output when no stack is present. | Stack-aware surfaces shipped |
| GOAL-03 | Enforce the first managed-worktree and branch guardrails for foreign stack execution. | Unsupported primary-checkout execution, wrong-branch work, or checkpoint mismatches are rejected or diagnosed. | Protocol guardrails shipped |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Stack Member Operator | A human or agent working inside one participating repo. | A precise answer about whether this repo may act now and why. |
| Stack Steward Operator | The human or agent coordinating a cross-repo integration outcome. | Deterministic local stack state, receipt visibility, and clear yield/checkpoint feedback. |
| Foreign Reactor | A reactor operating in another member repo. | Guardrails that force managed worktree execution on the correct stack branch. |

## Scope

### In Scope

- [SCOPE-01] Add a repo-local Mission Stack manifest contract under `.keel/stacks/<id>/manifest.yaml`.
- [SCOPE-02] Load Mission Stack membership, mode, receipts, checkpoint, and foreign-worktree state as a reusable read model.
- [SCOPE-03] Add stack-aware rendering and machine-readable fields to `keel turn`.
- [SCOPE-04] Add stack-aware blocking, yield, and checkpoint decisions to `keel next`.
- [SCOPE-05] Add linked member mission and receipt visibility to `keel mission next --status`.
- [SCOPE-06] Add Mission Stack diagnostics for wrong branch, unsupported foreign execution location, missing checkpoint acknowledgments, and stale closed-stack worktrees.

### Out of Scope

- [SCOPE-90] Network transport or hub-backed synchronization between repositories.
- [SCOPE-91] Stronger receipt artifacts beyond the first git-native contract.
- [SCOPE-92] OS-level workspace sandboxing outside Keel command and hook boundaries.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keel SHALL load an optional Mission Stack manifest and derive local stack state from repo metadata plus current git/worktree state. | GOAL-01 | must | The protocol needs a concrete local source of truth before adapters can act on it. |
| FR-02 | `keel turn` SHALL surface stack id, member role, branch, mode, checkpoint, and foreign-worktree execution state when the repo participates in a Mission Stack. | GOAL-02 | must | Operators orient through `turn` first and need stack context there. |
| FR-03 | `keel next` SHALL emit stack-aware decisions when the current repo must wait, yield to another member, or move into an approved foreign worktree. | GOAL-02, GOAL-03 | must | Pull-time routing is where protocol rules become enforceable. |
| FR-04 | `keel mission next --status` SHALL show linked member missions, pending negotiations, and waiting receipts for the local stack when present. | GOAL-02 | should | Mission inspection should reveal cross-repo coordination dependencies. |
| FR-05 | `keel doctor` SHALL diagnose Mission Stack violations including wrong branch, unsupported foreign execution location, missing checkpoint acknowledgments, and unmanaged leftovers for closed stacks. | GOAL-02, GOAL-03 | must | Doctor is the canonical integrity surface and must enforce the new contract. |
| FR-06 | Stack-aware surfaces SHALL expose deterministic JSON fields alongside human-readable text. | GOAL-02 | should | Automation needs the same contract that humans see. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Non-stack repos SHALL retain their current single-repo behavior across all touched commands. | GOAL-01, GOAL-02 | must | Mission Stack cannot regress the default path. |
| NFR-02 | Repo-local heartbeat semantics SHALL remain unchanged. | GOAL-01, GOAL-02 | must | The ADR explicitly keeps pacemaker semantics local to one repo. |
| NFR-03 | The initial stack manifest and receipt model SHALL remain git-native and repo-local. | GOAL-01 | should | The first slice should stay implementable without cross-repo services. |
| NFR-04 | Foreign-worktree checks SHALL fail safe by reporting unsupported execution state instead of mutating or deleting uncertain checkouts automatically. | GOAL-03 | should | Guardrails should be conservative on day one. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Stack projection | Unit tests | Manifest parsing and git/worktree-derived state projections |
| Stack-aware surfaces | Command and adapter tests | Stable text and JSON output for `turn`, `next`, and `mission next --status` |
| Guardrails | Doctor and integration tests | Wrong-branch, checkpoint, and foreign-worktree violations reported deterministically |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A single repo-local manifest is sufficient for the first stack-aware slice even though the full protocol is federated. | We may need steward/member replication sooner than expected. | Re-check after the first working surfaces ship. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much of the foreign-worktree lifecycle should be command-enforced immediately versus diagnosed first? | Epic owner | Open |
| Should `keel flow` become stack-aware in a follow-on slice once the core adapters are stable? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] A repo can declare a Mission Stack locally and Keel can derive actionable stack state from it.
- [ ] `turn`, `next`, `mission next --status`, and `doctor` expose stack context without changing non-stack output.
- [ ] Wrong-branch or unsupported foreign-execution paths are diagnosed deterministically.
<!-- END SUCCESS_CRITERIA -->
