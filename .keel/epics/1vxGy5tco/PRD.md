# Template Contract Hard Cutover - Product Requirements


## Problem Statement

Template contracts are currently inconsistent across markdown scaffolds, command inputs, and validation rules.
Token naming drifts (`date` vs `datetime`), CLI exposure does not consistently reflect field ownership, and unresolved scaffold text can survive into terminal artifacts without being blocked. This creates planning noise, weakens trust in generated artifacts, and forces manual cleanup after the fact.

## Goals & Objectives

Establish one canonical planning contract that is explicit, enforceable, and non-legacy.

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Canonical template vocabulary | Non-canonical tokens removed from embedded templates | 100% of active templates |
| GOAL-02 | CLI ownership clarity | Creation commands expose only user-owned fields as flags | 100% of `new` command surfaces |
| GOAL-03 | Coherency enforcement | Unresolved scaffold/default text fails doctor and lifecycle gates | 0 false negatives for covered docs |
| GOAL-04 | Hard cutover | No compatibility aliases for replaced token/validation paths | 0 legacy fallback paths in new behavior |

## Users

Primary users are human planners and implementation agents relying on deterministic scaffolds.

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Planner | Defines epics/voyages/stories and sets requirements | Predictable template structure with explicit review expectations |
| Implementer Agent | Executes stories from generated scaffolds | Clear CLI inputs and coherent acceptance criteria contracts |
| Reviewer | Accepts/rejects story outcomes and checks artifacts | Hard validation that catches unresolved scaffolds before acceptance |
| Maintainer | Evolves keel commands/templates over time | Single canonical path without compatibility branches |

## Scope

### In Scope

- [SCOPE-01] Canonical token normalization in planning templates and creation renderers.
- [SCOPE-02] CLI contract alignment for all creation commands in this epic's scope.
- [SCOPE-03] Hard doctor and transition enforcement for unresolved scaffold/default text.
- [SCOPE-04] Regression tests proving hard-cutover behavior.

### Out of Scope

- [SCOPE-05] Smart command suggestion output improvements across all commands.
- [SCOPE-06] Validation of generated report artifacts (`VOYAGE_REPORT.md`, `COMPLIANCE_REPORT.md`, `KNOWLEDGE.md`).
- [SCOPE-07] Legacy board artifact migration/remediation pass.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | All active planning templates must use canonical schema-mirrored token names and remove deprecated token aliases. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Eliminates ambiguous template vocabulary and drift. |
| FR-02 | Creation command CLI options must expose only user-owned inputs and must not expose system-owned fields. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Preserves clear ownership boundaries between operator intent and system state. |
| FR-03 | `adr new` must support optional `--context` and repeatable `--applies-to` flags and persist them in frontmatter. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Aligns ADR scaffolding inputs with governance model fields. |
| FR-04 | `voyage new` must require `--goal` at CLI parse time. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Prevents invalid scaffolding attempts from reaching runtime. |
| FR-05 | `keel doctor` must report unresolved scaffold/default text as errors for covered planning/coherency docs. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Enforces review quality before work progresses. |
| FR-06 | Story submit/accept transition gates must block unresolved scaffold/default text in story and reflection artifacts. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Stops incoherent artifacts from reaching terminal stages. |
| FR-07 | Story/reflection completeness checks must be stage-aware and only enforced for `needs-human-verification` and `done`. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Matches lifecycle intent while avoiding premature failures. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The implementation must use hard cutover semantics with no legacy compatibility fallbacks in new behavior. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Keeps contracts simple and enforceable. |
| NFR-02 | Validation and gate behavior must be covered by deterministic tests with stable assertions. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | Prevents regressions and validation drift. |
| NFR-03 | New checks must provide actionable error messages that include the failing artifact context. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | should | Reduces turnaround time when fixing coherency failures. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add template-rendering tests that fail when deprecated tokens or scaffold defaults appear in generated planning docs.
- Add CLI argument contract tests for `voyage new --goal` and `adr new --context/--applies-to` parsing behavior.
- Assert unresolved scaffold text fails both doctor and transition gates via shared fixture tests.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Existing command and doctor architecture can absorb stricter checks without redesign. | May require broader refactor and additional voyage scope. | Compile/test pass and command regression coverage. |
| Existing boards can tolerate hard-cutover rollout while migration is handled separately. | Team may need immediate migration tooling. | Run doctor on representative boards and review outcomes. |
| Token ownership can be encoded as a static contract in tests. | Test strategy may need runtime introspection instead. | Add failing tests for unknown and out-of-bucket tokens. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should unresolved instructional comments be enforced at doctor time for all planning docs in future phases? | Epic owner | deferred |
| Will stricter gate checks expose a large backlog of existing incoherent artifacts? | Epic owner | acknowledged |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] All template token replacements in scope use canonical schema-mirrored names with no deprecated aliases.
- [ ] CLI creation surfaces match ownership policy and include required `voyage new --goal`, `adr new --context`, and repeatable `--applies-to`.
- [ ] Doctor reports unresolved scaffold/default text as errors for covered docs and excludes generated report artifacts.
- [ ] Story submit/accept transitions block unresolved scaffold/default text and enforce terminal-stage reflection/story coherency.
- [ ] Regression tests assert hard-cutover behavior and reject legacy compatibility expectations.
<!-- END SUCCESS_CRITERIA -->

