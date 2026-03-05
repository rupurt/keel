# System Coherence and Intelligent Flow - Product Requirements

> Unify enforcement automate knowledge synthesis and enhance visibility of governance and dependencies

## Problem Statement

Keel's execution model accumulated policy and validation behavior across multiple command paths, which made outcomes hard to predict and difficult to reason about. Teams also lacked first-class synthesis of reflection knowledge and clear visibility into dependency and ADR blockers in operational views. Without explicit cleanup of schema compatibility paths, governance and workflow guidance drifted from canonical contracts.

## Goals & Objectives

| Goal | Success Metric | Target |
|------|----------------|--------|
| Unify transition enforcement | Runtime transition behavior and doctor diagnostics use the same enforcement origin | 100% of story/voyage lifecycle paths |
| Synthesize implementation knowledge | Voyage-level knowledge artifacts are generated from story reflections at completion time | 100% of completed voyages |
| Improve governance and dependency visibility | Flow/next surfaces include actionable blocker and dependency context | No hidden blockers in active work queues |
| Harden canonical schema usage | Legacy compatibility paths are removed from enforcement and validation | Zero legacy schema fallbacks in scope |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Planner | Owns queue health and acceptance decisions | Predictable transition outcomes and visible blockers |
| Implementer Agent | Executes stories across concurrent tracks | Deterministic lifecycle guidance and dependency clarity |
| Maintainer | Owns command behavior and validation policy | One enforcement path with minimal drift and lower maintenance overhead |

## Scope

### In Scope

- Route lifecycle checks through unified enforcement wiring for story and voyage paths.
- Generate voyage knowledge artifacts from authored story reflections during completion.
- Add richer dependency and governance visibility for flow and blocking diagnostics.
- Remove remaining schema compatibility behavior in covered enforcement/doctor paths.

### Out of Scope

- Net-new planning workflows outside existing epic/voyage/story lifecycle boundaries.
- Changes to external integrations or third-party reporting destinations.
- UI redesign work beyond terminal/markdown command rendering already in scope.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| FR-01 | Story and voyage lifecycle commands must delegate transition checks to the shared enforcement service. | must | Prevents command-specific policy drift and contradictory outcomes. |
| FR-02 | Voyage completion must synthesize reflection knowledge into canonical voyage artifacts. | must | Preserves institutional learning as a first-class output. |
| FR-03 | Flow and related diagnostics must surface dependency and governance blockers with actionable context. | must | Reduces hidden blockers and queue ambiguity. |
| FR-04 | Schema hardening must remove legacy compatibility paths in covered doctor and transition checks. | must | Enforces a single canonical contract. |
| FR-05 | Updated architecture documentation must describe enforcement and knowledge flow after refactor. | should | Keeps contributor guidance aligned with implementation. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Priority | Rationale |
|----|-------------|----------|-----------|
| NFR-01 | Enforcement behavior must be deterministic across doctor and runtime transitions for equivalent inputs. | must | Prevents policy ambiguity and acceptance churn. |
| NFR-02 | Knowledge synthesis paths must remain auditable with clear source attribution from reflections. | must | Ensures generated artifacts are trustworthy. |
| NFR-03 | Regression tests must guard dependency-visibility output and blocker semantics. | should | Prevents silent degradation of operational visibility. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add transition parity tests that assert unified enforcement outcomes match doctor findings for representative lifecycle fixtures.
- Add integration tests for voyage completion to confirm reflection synthesis populates knowledge artifacts deterministically.
- Add regression tests for flow/dependency visibility and ADR blocker messaging to ensure actionable diagnostics remain stable.
- Gate completion on `just keel doctor` and `just test` passing without warnings or failures.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing reflection content is sufficient for useful synthesis artifacts. | Generated knowledge may be noisy or low signal. | Review voyage artifacts in completed fixtures and adjust synthesis rules. |
| Unified enforcement can cover all transition paths touched in scope without command-specific exceptions. | Additional rule extraction work may be required. | Transition matrix tests across story/voyage commands. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much structured reflection normalization is needed to keep synthesis quality high? | Epic owner | Monitoring |
| Dependency visualization complexity may increase output noise for large boards. | Maintainer | Mitigated with concise rendering rules |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Story and voyage transitions in scope use unified enforcement and show parity with doctor outcomes.
- [ ] Completed voyages include synthesized knowledge derived from authored story reflections.
- [ ] Flow/governance outputs include dependency and blocker context that matches fixture expectations.
- [ ] Legacy compatibility behavior targeted by schema hardening is removed and covered by regression tests.
<!-- END SUCCESS_CRITERIA -->

## Voyages

<!-- Implementation breakdown (auto-generated by keel) -->

<!-- BEGIN VOYAGES -->
| Voyage | Status | Description |
|--------|--------|-------------|
| [1vv7YYY0y](voyages/1vv7YYY0y/) | done | Replace fragmented command checks with the unified enforcement service. |
| [1vv7Yags9](voyages/1vv7Yags9/) | done | Synthesize story reflections into voyage knowledge artifacts. |
| [1vv7YcwBg](voyages/1vv7YcwBg/) | done | Improve governance blocking feedback and dependency visibility. |
| [1vv7YeGDR](voyages/1vv7YeGDR/) | done | Remove legacy schema compatibility behavior from covered paths. |
<!-- END VOYAGES -->
