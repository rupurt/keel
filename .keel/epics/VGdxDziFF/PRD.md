# Stack-Aware Turn And Queue Surfaces - Product Requirements

## Problem Statement

Current turn, next, mission next, and doctor surfaces assume a single local board and cannot explain stack membership, gating modes, checkpoint blocks, or remote handoff state.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make Mission Stack state visible in the canonical operator read surfaces. | `turn`, `next`, `mission next`, and `doctor` can explain stack membership, local authority, and current gating state. | Operator clarity shipped |
| GOAL-02 | Turn stack protocol rules into enforceable command outcomes instead of passive prose. | Pull and closure surfaces can block, redirect, or explain actions based on stack mode and checkpoints. | Stack gating shipped |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Stack Member Operator | A human or agent running Keel inside one member repository. | Clear command output telling them whether this repo may act now and why. |
| Stack Steward Operator | A human or agent coordinating work across multiple member repositories. | A concise view of which members are active, blocked, waiting, or at a checkpoint. |

## Scope

### In Scope

- [SCOPE-01] Add Mission Stack context to `keel turn`.
- [SCOPE-02] Add stack-aware routing and blocking decisions to `keel next`.
- [SCOPE-03] Add linked member mission and negotiation status to `keel mission next --status`.
- [SCOPE-04] Add stack protocol diagnostics to `keel doctor`.
- [SCOPE-05] Define stable text and JSON surfaces for stack-aware operator feedback.

### Out of Scope

- [SCOPE-90] Visual scene redesign for stack state beyond the canonical command surfaces listed above.
- [SCOPE-91] Managed git worktree lifecycle and cleanup mechanics.
- [SCOPE-92] Multi-repo UI coordination outside the Keel CLI.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `keel turn` SHALL surface stack id, local member state, branch, current stack mode, and checkpoint status when the repo participates in a Mission Stack. | GOAL-01 | must | Operators need stack context during orientation. |
| FR-02 | `keel next` SHALL emit stack-aware decisions when the current repo is blocked, gated by checkpoint, or should yield to another member. | GOAL-01, GOAL-02 | must | Pull decisions are where protocol rules become actionable. |
| FR-03 | `keel mission next --status` SHALL show linked member missions, pending negotiations, or waiting handoffs relevant to the current stack. | GOAL-01 | should | Mission inspection should explain cross-repo dependencies, not hide them. |
| FR-04 | `keel doctor` SHALL report Mission Stack violations such as wrong branch, unauthorized active member, missing checkpoint acknowledgment, or unsupported foreign execution state. | GOAL-02 | must | Stack rules need explicit diagnostic enforcement. |
| FR-05 | Stack-aware command output SHALL remain available in JSON as well as human-readable text. | GOAL-01 | should | Automation and humans need the same truth surface. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Stack-aware surfaces SHALL preserve the current single-repo experience when a repo is not part of a Mission Stack. | GOAL-01 | must | Mission Stack support must not regress the default path. |
| NFR-02 | Stack-aware surfaces SHALL keep repo-local heartbeat semantics unchanged. | GOAL-01 | must | Stack readiness must not distort pacemaker meaning. |
| NFR-03 | Blocking and explanation output SHALL be deterministic enough for automated callers to branch on it reliably. | GOAL-02 | must | Coordination logic becomes fragile if outputs drift across surfaces. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Text and JSON surfaces | Command regression tests | Stable `turn`, `next`, `mission next`, and `doctor` outputs under stack and non-stack scenarios |
| Operator guidance | Manual CLI proofs | Example runs showing stack block, checkpoint wait, and active-member explanations |
| Backward compatibility | Existing command coverage | Non-stack repos still receive current single-repo outputs |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The first stack-aware surfaces can be added to existing command families without introducing a separate mandatory stack dashboard. | We may need a distinct stack-specific surface earlier than planned. | Re-check during voyage planning. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should `keel flow` become stack-aware in the first delivery slice or only after the core turn/queue surfaces are stable? | Epic owner | Open |
| Which stack-blocked outcomes need distinct machine-readable decision types versus explanatory text only? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Operators can tell from `turn` and `next` whether the current repo may act, must wait, or should yield to another member.
- [ ] `mission next --status` and `doctor` expose stack-linked dependency and violation state without changing non-stack behavior.
- [ ] Stack-aware outputs are available in both text and JSON forms.
<!-- END SUCCESS_CRITERIA -->
