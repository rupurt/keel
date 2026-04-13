# Foreign Reactor Worktree Execution Lifecycle - Product Requirements

## Problem Statement

Cross-repo execution currently has no formal requirement for managed git worktrees, branch enforcement, or stack-close garbage collection when outside reactors need to work in another member repository.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Require managed git worktrees for foreign reactor execution in another stack member repository. | Foreign stack work only proceeds inside approved worktrees on `stack/<id>` branches. | Foreign execution guardrail shipped |
| GOAL-02 | Define a safe lifecycle for creating, reusing, and garbage-collecting foreign worktrees. | Worktrees can be inspected, reused during an open stack, and cleaned up when the stack closes. | Lifecycle shipped |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Foreign Reactor | A reactor operating on behalf of a stack in a repository it does not primarily own. | A safe checkout boundary that prevents accidental in-place edits and preserves stack provenance. |
| Member Repo Owner | The operator responsible for one participating repository. | Assurance that outside work does not run in the primary checkout or drift off the stack branch. |

## Scope

### In Scope

- [SCOPE-01] Define the managed worktree requirement for foreign execution.
- [SCOPE-02] Define the naming and branch rules for foreign worktrees tied to `stack/<id>`.
- [SCOPE-03] Define how Keel creates, reuses, and validates managed worktrees before foreign work begins.
- [SCOPE-04] Define stack-close garbage collection behavior for foreign worktrees.
- [SCOPE-05] Define command and hook checks that reject foreign execution outside the approved worktree contract.

### Out of Scope

- [SCOPE-90] OS-level or harness-level write sandboxing across the wider workspace.
- [SCOPE-91] Git worktree usage unrelated to Mission Stack foreign execution.
- [SCOPE-92] Rich worktree UI beyond the minimum operator controls needed for stack lifecycle management.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keel SHALL require foreign reactor execution in another member repo to happen inside a managed git worktree rather than the member repo's primary checkout. | GOAL-01 | must | This is the core isolation rule for outside work. |
| FR-02 | Managed foreign worktrees SHALL check out the member repo's `stack/<id>` branch or an allowed stack-specific detached head derived from it. | GOAL-01 | must | Stack provenance depends on a consistent branch contract. |
| FR-03 | Keel SHALL validate worktree ownership, branch, and cleanliness before allowing foreign stack turns to proceed. | GOAL-01, GOAL-02 | must | The protocol needs enforceable preconditions. |
| FR-04 | Keel SHALL garbage-collect managed foreign worktrees when the stack closes. | GOAL-02 | must | The user explicitly wants lifecycle cleanup tied to stack closure. |
| FR-05 | Keel SHOULD expose enough worktree state for hooks and diagnostics to reject unsupported foreign execution paths. | GOAL-02 | should | Enforcement needs shared state between commands and git boundaries. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Managed worktree operations SHALL avoid perturbing the member repo's primary checkout. | GOAL-01 | must | Isolation fails if the primary checkout is mutated as a side effect. |
| NFR-02 | Worktree creation and reuse SHALL be idempotent for the same open stack member when possible. | GOAL-02 | should | Reactors need stable paths during an active stack. |
| NFR-03 | Cleanup behavior SHALL fail safe by reporting leftover worktrees rather than silently deleting uncertain state. | GOAL-02 | should | Stack close cleanup should be trustworthy and auditable. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Worktree guardrail | Command and integration tests | Foreign execution succeeds only inside approved managed worktrees |
| Branch enforcement | CLI proofs and hook tests | Wrong-branch or primary-checkout execution is rejected |
| Lifecycle cleanup | Manual and automated cleanup scenarios | Open-stack reuse works and stack-close garbage collection removes managed worktrees or reports leftovers clearly |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Git worktrees are available in the target environments where foreign reactor execution is expected. | We would need a fallback checkout isolation strategy. | Re-check during voyage planning and environment validation. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Where should managed foreign worktrees live on disk relative to repo-local and cache-local state? | Epic owner | Open |
| Which checks belong in pre-commit versus pre-push when a foreign reactor is operating in a managed worktree? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Foreign reactor execution is formally defined to require a managed git worktree.
- [ ] The `stack/<id>` branch rule is preserved for foreign worktree operation.
- [ ] Stack-close garbage collection is part of the worktree lifecycle contract.
<!-- END SUCCESS_CRITERIA -->
