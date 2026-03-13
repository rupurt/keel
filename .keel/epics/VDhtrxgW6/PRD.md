# HEAD-relative Show Selector Resolution - Product Requirements

## Problem Statement

Show commands require exact IDs, which slows navigation and makes it harder to inspect the current or previous entity from the same stable ordering users already rely on in list-style command output.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Let operators and stewards open the current head item or walk backward through showable entities without first copying an ID from a list output. | Supported show commands resolve HEAD-relative selectors from their canonical default ordering. | 100% of supported show commands accept exact IDs and HEAD, HEAD~, HEAD~~, HEAD^ |
| GOAL-02 | Keep selector behavior deterministic and explainable so harnesses and humans see the same entity ordering. | Equivalent board state yields the same HEAD-relative target across repeated runs. | Deterministic regression coverage for each supported entity type |
| GOAL-03 | Reuse existing list-order semantics instead of inventing a second hidden ordering contract for show commands. | Selector resolution shares ordering logic with default list surfaces. | No show command maintains a bespoke HEAD-only sort path |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | The delivery-focused user moving through active work in the terminal. | Open the latest or previous scoped entity quickly without copying IDs. |
| Manager | The planner reviewing missions, epics, voyages, and stories from board surfaces. | Inspect the top of each queue or recent item from a stable command contract. |
| Harness / Agent | Automation invoking show commands from deterministic CLI flows. | Resolve relative selectors locally without ambiguity or hidden state. |

## Scope

### In Scope

- [SCOPE-01] Shared parsing and resolution for exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^.
- [SCOPE-02] Stable default ordering providers for mission, epic, voyage, story, bearing, ADR, and routine show commands.
- [SCOPE-03] Show-command adoption, guidance, and errors for empty lists, out-of-range relative selectors, and invalid syntax.
- [SCOPE-04] Regression and CLI coverage that locks the HEAD-relative contract to canonical list ordering.

### Out of Scope

- [SCOPE-05] Numeric suffix syntaxes such as HEAD~3, reflog-style selectors, or git object semantics beyond repeated `~`/`^`.
- [SCOPE-06] Non-show command adoption for list, next, flow, topology, or lifecycle commands.
- [SCOPE-07] User-configurable sort orders or per-command filter-aware HEAD selection in the first cut.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Add a shared selector parser and resolver that accepts exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ and translates them into entity IDs using stable default ordering per entity type. | GOAL-01, GOAL-02 | must | Centralizing parsing keeps relative selector behavior deterministic and removes command-specific drift. |
| FR-02 | Wire the shared selector path into mission, epic, voyage, story, bearing, ADR, and routine show commands without changing exact-ID behavior. | GOAL-01, GOAL-03 | must | The user-facing value only appears once all show commands accept the same selector family. |
| FR-03 | Reuse the same canonical ordering semantics the default list surfaces expose for each supported entity type, with all default filters enabled and no separate HEAD-only sort path. | GOAL-02, GOAL-03 | must | Relative selectors must point at the same “head” humans already see in list views. |
| FR-04 | Return actionable, deterministic errors when the relevant entity set is empty, the relative selector walks past the end, or the syntax is unsupported. | GOAL-01, GOAL-02 | should | Clear failure guidance keeps harnesses and humans from guessing what HEAD resolved against. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Equivalent board state must produce the same HEAD-relative resolution across repeated runs and test fixtures. | GOAL-02, GOAL-03 | must | Determinism is required for harness-safe CLI behavior. |
| NFR-02 | The selector contract, help text, and regression coverage must stay aligned with the supported show commands. | GOAL-01, GOAL-03 | should | Documentation and tests need to prevent syntax drift as more commands evolve. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Shared resolution logic | Rust unit tests over selector parsing, ordering, and out-of-range behavior | Targeted `cargo test` coverage for the shared resolver |
| Show-command adoption | CLI regression tests for mission/epic/voyage/story/bearing/ADR/routine show surfaces | `cargo test --bin keel` proofs plus story evidence |
| Determinism | Repeated-run fixture tests and repo hygiene (`just quality`, `just test`, `just doctest`, `just keel doctor`) | Story-level manifests and green hygiene output |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Each supported entity type already has, or can expose, a single canonical default list ordering without requiring runtime filters. | HEAD-relative resolution could drift or become ambiguous. | Reuse list/read-model ordering helpers during implementation and surface conflicts in tests. |
| Repeated `~` and `^` tokens are sufficient for the first delivery slice without numeric suffixes. | Users may still want deeper jumps in one token. | Capture follow-on demand after the base selector family lands. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which list-order helper should become the single shared ordering source for each showable entity type? | Epic owner | Open |
| Some entity types may not currently expose a default list projection with no filters. | Epic owner | Open |
| Users may expect `HEAD^` and `HEAD~` to diverge semantically because of git intuition even though this CLI will treat them as equivalent single-step history moves. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Supported show commands resolve exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ from a shared canonical selector path.
- [ ] The resolved head item for each entity type matches the same stable ordering surfaced by the corresponding default list command.
- [ ] Out-of-range, empty-set, and unsupported-selector failures produce deterministic, actionable CLI guidance.
<!-- END SUCCESS_CRITERIA -->
