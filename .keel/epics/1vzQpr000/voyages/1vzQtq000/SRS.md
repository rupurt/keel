# Bearing Contract Cutover and Migration - Software Requirements Specification

> Replace the survey-era bearing artifact and lifecycle contract with a framing/evidence/assessment workflow and hard-cutover migration rules.

**Epic:** [1vzQpr000](../../README.md) | **SDD:** [SDD.md](SDD.md)

## Scope

### In Scope
- [SCOPE-01] Redefine bearing artifacts around `BRIEF.md` for framing, `EVIDENCE.md` for cited research capture, and `ASSESSMENT.md` for synthesis and recommendation.
- [SCOPE-02] Hard-cut bearing command and lifecycle language from `survey` semantics to `research` semantics wherever the new workflow applies.
- [SCOPE-06] Update bearing show, file, doctor, and readiness rules to validate and surface the new evidence-backed workflow.
- [SCOPE-08] Update templates, docs, tests, and fixture boards to the new canonical contract.

### Out of Scope
- [SCOPE-03] Add a canonical evidence contract with source IDs, source classes, provenance, dates, and evidence-quality metadata.
- [SCOPE-04] Support research capture for web, academic/prior-art, social/trend, and manual or internal signals.
- [SCOPE-05] Add configuration for research provider enablement and weighting heuristics that influence evidence ranking and downstream scoring.
- [SCOPE-07] Evolve assessment and EV scoring to account for evidence breadth, freshness, authority, and contradiction handling.
- [SCOPE-09] Building authenticated integrations for every possible external provider or paid research API.
- [SCOPE-12] Preserving backward-compatible `SURVEY.md` or `bearing survey` behavior once the new contract lands.

## Assumptions & Dependencies

| Assumption/Dependency | Type | Impact if Invalid |
|-----------------------|------|-------------------|
| Existing bearing loader, doctor, and transition code can absorb a hard rename from survey-era semantics without introducing compatibility aliases. | Internal dependency | The cutover could fragment into dual contracts and violate epic scope. |
| The current active bearing set is small enough that explicit migration guidance or fixture updates are feasible within one planning slice. | Operational assumption | A larger migration burden would need a dedicated migration voyage before cutover. |
| `show`, `file`, `flow`, and documentation guidance surfaces can be updated in the same slice as template and lifecycle changes. | Internal dependency | Users would keep seeing mixed terminology after the cutover. |

## Constraints

- This voyage must enforce one canonical bearing contract; it cannot preserve `SURVEY.md` or `bearing survey` as supported aliases.
- The voyage may introduce placeholder or stub evidence structures only insofar as they unblock the renamed contract; full evidence modeling is deferred.
- Validation and guidance must fail fast with actionable replacement messages when legacy survey-era artifacts remain.
- The cutover must keep existing bearing IDs and core lifecycle intent intact even while document names and terminology change.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-01 | Bearing scaffolds and generated document references MUST replace the survey-era artifact contract so `README.md` points to `BRIEF.md`, `EVIDENCE.md`, and `ASSESSMENT.md`, with `BRIEF.md` scoped to framing only. | SCOPE-01 SCOPE-08 | FR-01 | template tests + doctor validation tests |
| SRS-02 | Bearing lifecycle commands, help text, generated guidance, and docs MUST cut over from survey language to research language, including the canonical command path used to create the evidence document. | SCOPE-02 SCOPE-08 | FR-02 | clap/help tests + docs assertions + CLI regression tests |
| SRS-03 | Bearing doctor checks and readiness rules MUST validate the new framing/evidence/assessment artifact contract and reject legacy survey-era structures with actionable recovery guidance. | SCOPE-06 SCOPE-08 | FR-06 | doctor tests + structural validation tests |
| SRS-04 | Migration-facing errors, fixtures, and board artifacts in scope MUST update to the new contract in the same slice so no supported workflow or test fixture continues to rely on `SURVEY.md` or `bearing survey`. | SCOPE-02 SCOPE-08 | FR-08 | fixture-board tests + hard-cutover regression tests |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Scope | Source | Verification |
|----|-------------|-------|--------|--------------|
| SRS-NFR-01 | The cutover MUST keep a single canonical bearing contract across code, templates, docs, and validation with no compatibility aliases for the removed survey-era path. | SCOPE-01 SCOPE-02 SCOPE-06 SCOPE-08 | NFR-04 | architecture contract tests + command regression tests |
| SRS-NFR-02 | Contract validation and CLI guidance MUST remain deterministic so the same bearing artifacts always produce the same migration errors, doctor results, and next-step suggestions. | SCOPE-02 SCOPE-06 SCOPE-08 | NFR-01 | deterministic doctor tests + snapshot tests |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->
