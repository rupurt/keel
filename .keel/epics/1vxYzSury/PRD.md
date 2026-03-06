# Deterministic Command Guidance - Product Requirements


## Problem Statement

Management commands have historically mixed informational output with inconsistent guidance phrasing, making automation brittle and operator behavior inconsistent across workflows. Without a canonical guidance contract, harnesses cannot reliably determine the next action or recovery action from command output, and teams must hand-interpret command-specific wording.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish one guidance contract | Actionable commands emit one canonical next or recovery step in text and JSON | 100% of actionable management commands in scope |
| GOAL-02 | Keep informational commands non-prescriptive | Informational commands omit next/recovery directives by design | 0 informational command contract violations |
| GOAL-03 | Reduce command-classification drift | Command capability map and drift tests cover actionable vs informational behavior | 100% drift-test pass rate |
| GOAL-04 | Improve planning visibility | Show surfaces expose authored planning context and evidence status coherently | Epic/voyage/story show commands aligned to contract |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Human planner | Runs queue and governance workflows directly in terminal | Consistent guidance for the next safe action |
| Automation harness | Parses command output for orchestration and prompts | Deterministic machine-readable guidance fields |
| Implementer agent | Executes lifecycle commands while delivering stories | Clear recovery instructions when a transition fails |

## Scope

### In Scope

- [SCOPE-01] Define and enforce a shared guidance output contract with canonical next/recovery fields.
- [SCOPE-02] Apply deterministic guidance rules to story, voyage, governance, decision, and verification command groups in scope.
- [SCOPE-03] Preserve non-prescriptive behavior for informational commands.
- [SCOPE-04] Add command classification and drift tests to prevent contract regression.
- [SCOPE-05] Improve planning read surfaces so authored context and evidence status are visible and actionable.

### Out of Scope

- [SCOPE-06] Replacing command semantics beyond guidance and rendering contract requirements.
- [SCOPE-07] Expanding scope to external API protocols not currently served by CLI surfaces.
- [SCOPE-08] Introducing backward-compatible alternate guidance formats during this hard-cutover phase.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Actionable management commands in scope must emit exactly one canonical next or recovery instruction. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Enables deterministic operator and harness behavior. |
| FR-02 | JSON output for actionable commands must include parity fields for canonical guidance. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Keeps machine consumers aligned with terminal output behavior. |
| FR-03 | Informational commands must remain non-prescriptive and omit canonical action directives. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Prevents misleading or noisy guidance on read-only surfaces. |
| FR-04 | Shared renderer utilities must be used across command groups to prevent formatting drift. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Maintains one rendering contract and reduces duplication. |
| FR-05 | Planning show commands must expose authored context, requirement/evidence status, and lifecycle metadata coherently. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Improves decision quality and acceptance flow. |
| FR-06 | Story creation must default to icebox state for planning-first intake. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Preserves queue discipline and avoids accidental execution starts. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Guidance contract behavior must be deterministic and stable across releases. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Harness reliability depends on contract stability. |
| NFR-02 | Command classification tests must detect drift before merge. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Prevents regression of actionable vs informational semantics. |
| NFR-03 | Rendered output must remain readable in narrow terminal widths without losing key guidance fields. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Ensures practical usability in real operator environments. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add contract tests that validate canonical next/recovery presence, cardinality, and JSON parity for actionable commands.
- Add negative tests ensuring informational commands do not emit prescriptive guidance fields.
- Add drift tests tied to the command classification map so new commands cannot silently violate behavior categories.
- Keep golden-output tests for epic/voyage/story show surfaces to lock rendering coherence for authored planning context.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing command taxonomy can cleanly classify actionable vs informational behavior. | Additional command families may require separate policy treatment. | Classification map review and drift-test enforcement. |
| Shared rendering helpers can satisfy all guidance presentation needs without command-specific forks. | Contract may fragment again across command implementations. | Renderer adoption checks in tests and code review gates. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Future command additions may bypass classification policy if onboarding guidance is weak. | Maintainer | Mitigated with drift tests and AGENTS guidance |
| Overly verbose guidance text could reduce readability in constrained terminal contexts. | Epic owner | Monitoring |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Actionable commands in scope emit one canonical next or recovery step with JSON parity.
- [ ] Informational commands remain non-prescriptive and pass classification drift tests.
- [ ] Shared guidance renderer utilities are adopted across covered command families.
- [ ] Planning show surfaces provide authored context and evidence visibility without contract regressions.
<!-- END SUCCESS_CRITERIA -->

