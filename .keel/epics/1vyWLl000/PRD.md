# Dogfood VHS Acceptance Verification - Product Requirements

## Problem Statement

Keel can recommend and execute verification techniques, but it does not yet verify its own end-to-end CLI workflows through the same acceptance-proof loop it asks teams to use. Without a dogfood suite that records real user flows and evaluates the resulting evidence, regressions in planning, execution, rendering, and proof orchestration can drift past unit and command tests.

Today the repository also lacks an isolated secondary board for those dogfood flows. That makes it hard to exercise real planning and execution transitions without polluting the primary `.keel` board or falling back to synthetic fixtures that do not look like authored work.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish a real dogfood proof loop for keel CLI workflows | Representative CLI flows are captured as authored tapes and replayed through keel verification | Phase 1 epic and bearing flows both pass from a dedicated dogfood board |
| GOAL-02 | Keep dogfood validation isolated from the primary board | Dogfood runs do not mutate the repo's main `.keel` board | 100% of e2e scenarios run against a secondary board/workspace |
| GOAL-03 | Make rendered CLI evidence a first-class verification input | VHS outputs and related artifacts are linked into manifests and acceptance criteria | Every dogfood story in scope emits tape-driven verification artifacts |
| GOAL-04 | Preserve provider choice for semantic judging | Artifact-aware judging is invoked through a provider-agnostic contract | No vendor-specific SDK or prompt contract is hardcoded into keel |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Maintainer | Evolves keel workflows and needs confidence before merge | End-to-end proof that real CLI journeys still work as authored |
| Implementer Agent | Uses `epic show`, `voyage show`, `story show`, `next`, and `flow` to steer work | Stable summarized surfaces that are validated by the same toolchain they depend on |
| Reviewer | Evaluates whether acceptance criteria were actually met | Durable, replayable evidence artifacts tied to specific acceptance criteria |

## Scope

### In Scope

- [SCOPE-01] Create a dedicated secondary keel board/workspace for dogfood e2e scenarios, separate from the repo's primary `.keel` board.
- [SCOPE-02] Add a phase 1 local opt-in VHS suite that exercises epic creation, decomposition, `keel next`, `keel flow`, and execution/verification transitions.
- [SCOPE-03] Add a phase 1 local opt-in VHS suite that exercises bearing creation, survey, assess, lay, and related steering surfaces.
- [SCOPE-04] Persist tape-driven verification artifacts and link them into story verification manifests.
- [SCOPE-05] Upgrade `llm-judge` in phase 2 so it can inspect artifact bundles produced by dogfood runs instead of judging only `git diff`.
- [SCOPE-06] Keep the artifact judging path provider-agnostic so projects can supply their own judge implementation.

### Out of Scope

- [SCOPE-07] Making the new dogfood suite mandatory in CI before it stabilizes.
- [SCOPE-08] Browser or GUI workflow recording beyond terminal-driven CLI journeys.
- [SCOPE-09] Vendor-specific judge SDK integrations baked directly into keel.
- [SCOPE-10] Exhaustive command coverage for every keel subcommand in the first rollout.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The system must provide a dedicated dogfood board/workspace for keel e2e scenarios that is distinct from the repository's primary `.keel` board. | GOAL-01 GOAL-02 | must | Keeps real workflow validation from thrashing the main planning board. |
| FR-02 | The system must provide an opt-in local runner for VHS-based dogfood scenarios. | GOAL-01 GOAL-02 GOAL-03 | must | Makes the suite easy to execute without forcing unstable e2e checks into every development loop. |
| FR-03 | The phase 1 dogfood suite must cover an epic workflow from creation through decomposition and execution steering, including `keel next` and `keel flow`. | GOAL-01 GOAL-03 | must | Validates the primary planning and execution journey users rely on. |
| FR-04 | The phase 1 dogfood suite must cover a bearing workflow from creation through survey, assessment, and lay. | GOAL-01 GOAL-03 | must | Validates the research path separately from implementation planning. |
| FR-05 | Tape-driven runs must emit verification artifacts that are tracked in story manifests and can be referenced by acceptance criteria. | GOAL-03 | must | Makes recordings part of the canonical proof chain instead of side-channel demos. |
| FR-06 | The verification system must support artifact-bundle judging so `llm-judge` can evaluate tape outputs and related artifacts against acceptance criteria. | GOAL-03 GOAL-04 | must | Bridges deterministic recordings and semantic acceptance review. |
| FR-07 | The artifact-judge execution path must invoke a provider-agnostic contract rather than a hardcoded vendor implementation. | GOAL-04 | must | Preserves portability and keeps provider choice outside the core CLI. |
| FR-08 | Dogfood planning artifacts must describe how tapes, transcripts, manifests, and judged outputs prove the in-scope requirements. | GOAL-01 GOAL-03 GOAL-04 | should | Keeps the dogfood suite aligned with keel's lineage and verification model. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Phase 1 dogfood runs must be deterministic enough to pass repeatedly on the same machine and repository state. | GOAL-01 GOAL-03 | must | Prevents flaky evidence from undermining trust in the suite. |
| NFR-02 | Dogfood execution must avoid mutating the primary `.keel` board. | GOAL-02 | must | Protects the real planning board from e2e churn. |
| NFR-03 | The first rollout must remain opt-in and local-only until scenario stability is proven. | GOAL-01 GOAL-02 | must | Keeps developer friction low while the suite matures. |
| NFR-04 | Artifact-judge inputs and outputs must use a documented, provider-agnostic file/command contract. | GOAL-04 | must | Makes future provider swaps or external judge tools practical. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Add fixture-backed tests for the dogfood runner, board isolation rules, and verification-manifest artifact capture.
- Add tape-backed e2e stories for the epic and bearing flows that produce real VHS artifacts from the secondary board/workspace.
- Validate phase 1 with an opt-in local command that runs the dogfood suite and leaves the primary `.keel` board unchanged.
- Validate phase 2 with tests proving `llm-judge` consumes artifact bundles through a provider-agnostic contract rather than the current `git diff` stub.
- Keep `just keel doctor`, `just quality`, and `just test` green for the repository while adding dedicated dogfood proofs.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| VHS rendering is stable enough on the Nix/dev-shell environment to serve as repeatable evidence for CLI flows. | The dogfood suite may become flaky or require heavier normalization. | Repeated local e2e runs across the same fixture state. |
| A dedicated secondary board/workspace can be reset cheaply enough without copying the entire repository for every run. | The runner may become too slow or operationally awkward. | Phase 1 harness design and runtime measurement. |
| A provider-agnostic artifact-judge contract is sufficient for future model integrations. | A specific provider may require richer metadata or transport semantics. | Phase 2 prototype with external command contract and documented artifact bundle shape. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| What is the cheapest reliable reset strategy for the secondary dogfood workspace? | Maintainer | Open |
| VHS timing/rendering could still introduce flakiness if scenario surfaces are not constrained carefully. | Maintainer | Monitoring |
| Artifact bundles may need transcript normalization or frame extraction before semantic judging is robust. | Verification owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Maintainers can run an opt-in local dogfood suite that exercises representative epic and bearing CLI workflows from a secondary board/workspace.
- [ ] Phase 1 dogfood stories produce tape-driven evidence artifacts that are linked through verification manifests.
- [ ] The primary `.keel` board remains untouched by dogfood runs.
- [ ] `llm-judge` gains a provider-agnostic artifact-aware path that can judge dogfood evidence instead of only diff text.
<!-- END SUCCESS_CRITERIA -->
