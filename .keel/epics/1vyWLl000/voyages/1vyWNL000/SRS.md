# Tape-Driven Dogfood Workflow Suite - Software Requirements Specification

> Add a local opt-in VHS dogfood suite on a dedicated secondary board that proves representative epic and bearing workflows and records manifest-linked artifacts.

**Epic:** [1vyWLl000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

In scope:
- [SCOPE-01] Stand up a dedicated secondary workspace with its own `.keel` board for dogfood e2e runs.
- [SCOPE-02] Add tape-driven epic workflow coverage, including epic creation, voyage/story decomposition, `keel next`, `keel flow`, and execution steering.
- [SCOPE-03] Add tape-driven bearing workflow coverage, including create, survey, assess, and lay transitions.
- [SCOPE-04] Persist rendered tape outputs and companion artifacts in story evidence/manifests.

Out of scope:
- [SCOPE-05] Artifact-aware `llm-judge` contract design and execution.
- [SCOPE-06] Provider-agnostic semantic judge integration.
- [SCOPE-07] Mandatory CI enforcement for the new suite.
- [SCOPE-08] Browser or GUI recording coverage.
- [SCOPE-10] Exhaustive command coverage beyond the representative flows.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| `vhs` and `ffmpeg` remain available in the local dev environment. | Tooling | Tape capture and playback evidence cannot run reliably. |
| Keel can operate from a nested secondary workspace by resolving the nearest `.keel` board from the current working directory. | Runtime contract | The dogfood harness would need invasive board-resolution changes. |
| Companion text artifacts can be captured alongside rendered tapes to keep phase 1 validation deterministic. | Evidence model | Tape-only validation may be too flaky to trust initially. |

## Constraints

- Phase 1 must remain opt-in and local-only.
- Dogfood execution must not mutate the repository's primary `.keel` board.
- One canonical harness path should orchestrate reset, tape execution, and evidence capture.
- Tapes should target stable CLI text surfaces even though real rendered output is still required.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Provide a checked-in secondary workspace with its own `.keel` board and a deterministic reset path for dogfood scenarios. | SCOPE-01 | FR-01 | fixture workspace tests + reset-path integration tests |
| SRS-02 | Provide an opt-in local runner that executes tape-backed dogfood scenarios from the secondary workspace without touching the primary board. | SCOPE-01 | FR-02 | runner command tests + board-isolation assertions |
| SRS-03 | Record epic workflow tapes that cover epic creation, voyage/story decomposition, `keel next`, `keel flow`, and execution steering against the secondary workspace. | SCOPE-02 | FR-03 | tape runner integration tests + artifact assertions |
| SRS-04 | Record bearing workflow tapes that cover `bearing new`, `bearing survey`, `bearing assess`, and `bearing lay` against the secondary workspace. | SCOPE-03 | FR-04 | tape runner integration tests + artifact assertions |
| SRS-05 | Persist VHS outputs plus companion transcript/log artifacts under story `EVIDENCE/` and include them in verification manifests. | SCOPE-04 | FR-05 | manifest generation tests + evidence inventory assertions |
| SRS-06 | Dogfood planning artifacts and story annotations MUST document how tapes, transcripts, and manifests prove the in-scope requirements. | SCOPE-04 | FR-08 | voyage-plan coverage checks + doctor assertions |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | Phase 1 dogfood runs MUST be deterministic enough to pass repeatedly on the same repository state. | SCOPE-02 | NFR-01 | repeated-run integration tests |
| SRS-NFR-02 | The dogfood runner MUST leave the primary `.keel` board unchanged. | SCOPE-01 | NFR-02 | filesystem guard tests + workspace diff assertions |
| SRS-NFR-03 | The first rollout MUST stay opt-in and must not be wired into default CI or pre-commit paths. | SCOPE-02 | NFR-03 | CI/justfile contract tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
