# Evidence-Backed Bearing Research Workflow - Product Requirements

## Problem Statement

Bearings currently conflate framing, research, and decision-making into loosely separated prose files. In practice, `BRIEF.md` and `SURVEY.md` both become places to restate findings, lifecycle progress is driven by whether documents exist rather than whether evidence is credible, and assessments can recommend work without durable citations or recency signals. This makes bearings feel like model-memory retrieval instead of source-backed research, blocks meaningful use of web, academic, social, and internal signals, and leaves EV scoring too detached from the quality of the underlying evidence.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Separate framing, evidence capture, and decision-making into distinct bearing artifacts. | Bearing docs in scope each have one clear responsibility with no duplicated research-summary burden. | 1 canonical responsibility per artifact |
| GOAL-02 | Make evidence first-class, cited, and inspectable. | Findings and recommendations can point to canonical source IDs with provenance and metadata. | 100% of in-scope synthesized claims cite evidence IDs |
| GOAL-03 | Support richer research inputs than model memory alone. | Bearings can capture web, academic, social, and manual/internal signals through a canonical research workflow. | All four signal classes supported in the first workflow slice |
| GOAL-04 | Make recommendation and EV scoring reflect evidence quality. | Scores change predictably when evidence breadth, freshness, authority, or contradiction changes. | Deterministic score sensitivity across fixture corpora |
| GOAL-05 | Complete a hard cutover to the new bearing contract. | Legacy survey-era workflow paths are removed in scope and replaced with one canonical command and validation model. | 0 dual-path bearing contracts in scope |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Researcher / Planner | Human exploring an ambiguous idea before committing to an epic. | A lightweight but rigorous workflow that separates questions, evidence, and recommendation. |
| Agent Researcher | LLM or automation gathering and organizing research on behalf of a human. | Explicit contracts for what to search, how to capture sources, and how to cite findings. |
| Maintainer | Owns keel command contracts, doctor rules, and scoring behavior. | One canonical bearing workflow that is deterministic, validated, and hard to misuse. |

## Scope

### In Scope

- [SCOPE-01] Redefine bearing artifacts around `BRIEF.md` for framing, `EVIDENCE.md` for cited research capture, and `ASSESSMENT.md` for synthesis and recommendation.
- [SCOPE-02] Hard-cut bearing command and lifecycle language from `survey` semantics to `research` semantics wherever the new workflow applies.
- [SCOPE-03] Add a canonical evidence contract with source IDs, source classes, provenance, dates, and evidence-quality metadata.
- [SCOPE-04] Support research capture for web, academic/prior-art, social/trend, and manual or internal signals.
- [SCOPE-05] Add configuration for research provider enablement and weighting heuristics that influence evidence ranking and downstream scoring.
- [SCOPE-06] Update bearing show, file, doctor, and readiness rules to validate and surface the new evidence-backed workflow.
- [SCOPE-07] Evolve assessment and EV scoring to account for evidence breadth, freshness, authority, and contradiction handling.
- [SCOPE-08] Update templates, docs, tests, and fixture boards to the new canonical contract.

### Out of Scope

- [SCOPE-09] Building authenticated integrations for every possible external provider or paid research API.
- [SCOPE-10] Replacing human judgment with fully autonomous recommendation generation.
- [SCOPE-11] Adding non-bearing research workflows for epics, voyages, or ADRs in the same slice.
- [SCOPE-12] Preserving backward-compatible `SURVEY.md` or `bearing survey` behavior once the new contract lands.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The bearing workflow MUST redefine document responsibilities so `BRIEF.md` captures framing only, `EVIDENCE.md` captures cited research inputs, and `ASSESSMENT.md` captures synthesis plus recommendation. | GOAL-01 GOAL-02 GOAL-05 | must | The current overlap between brief and survey is the root workflow problem. |
| FR-02 | Bearing lifecycle commands and generated guidance MUST cut over from survey language to research language, including canonical scaffolding and next-step instructions. | GOAL-01 GOAL-05 | must | The contract should be visible and consistent at the CLI, not only in templates. |
| FR-03 | `EVIDENCE.md` MUST support canonical source IDs and metadata including source class, origin or URL, publication or observation date, retrieval date, and evidence-quality fields such as authority and freshness. | GOAL-02 GOAL-03 GOAL-04 | must | Evidence quality cannot influence decisions or scoring unless it is modeled explicitly. |
| FR-04 | The research workflow MUST support capturing and organizing evidence from web, academic or prior-art, social or trend, and manual or internal signals, with provider provenance recorded for each source. | GOAL-02 GOAL-03 | must | Bearings need richer inputs than model memory and one undifferentiated research bucket. |
| FR-05 | `keel.toml` MUST allow operators to enable or disable research providers and define weighting or ranking heuristics used for evidence ordering and evaluation. | GOAL-03 GOAL-04 GOAL-05 | should | Different projects will trust and prioritize source classes differently. |
| FR-06 | Bearing read surfaces and doctor checks MUST validate the new artifact contract, including citation completeness, evidence metadata presence, and readiness rules tied to evidence quality rather than file existence alone. | GOAL-01 GOAL-02 GOAL-05 | must | A new workflow only matters if the system enforces it. |
| FR-07 | Assessment scoring and EV computation MUST incorporate evidence quality signals such as breadth, freshness, authority, and contradiction or gap handling in addition to operator-authored impact or effort judgments. | GOAL-02 GOAL-04 | must | The user explicitly wants a more meaningful EV score grounded in research quality. |
| FR-08 | Migration and hard-cutover behavior MUST update scaffolds, docs, fixtures, and CLI errors so legacy survey-era artifacts fail fast with actionable replacement guidance. | GOAL-05 | must | Dual contracts would recreate the same ambiguity the redesign is trying to remove. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The same evidence corpus, config, and provider outputs MUST produce deterministic evidence ordering, readiness evaluation, and EV scoring. | GOAL-02 GOAL-04 GOAL-05 | must | Research-backed workflows still need stable automation and reviewable diffs. |
| NFR-02 | Terminal and file surfaces MUST preserve source provenance clearly enough that a human can inspect where a finding came from without leaving the bearing context. | GOAL-02 GOAL-03 | must | Citations are only useful if they are visible and navigable in practice. |
| NFR-03 | When external providers are unavailable, disabled, or rate-limited, the workflow MUST surface that state explicitly and MUST NOT fabricate evidence from model memory. | GOAL-02 GOAL-03 GOAL-05 | must | The redesign only succeeds if evidence gaps stay visible instead of becoming silent hallucination. |
| NFR-04 | The cutover MUST keep one canonical bearing contract in code, templates, docs, and validation with no backward-compatibility aliases in scope. | GOAL-05 | must | Hard cutover is necessary to remove workflow ambiguity. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Artifact contract | Template, parser, and doctor tests for `BRIEF.md`, `EVIDENCE.md`, and `ASSESSMENT.md` responsibilities | Regression tests showing legacy survey-era artifacts fail and new artifacts validate |
| Research capture | Command and adapter tests using deterministic provider fixtures for web, academic, social, and manual inputs | Fixture-backed CLI proofs showing source IDs, metadata, and provenance are recorded |
| Rendering and guidance | `show`, `file`, and next-step contract tests plus terminal rendering regression coverage | Stable output proofs for evidence-backed bearing surfaces and lifecycle guidance |
| Scoring behavior | Unit and fixture tests for evidence-weighted EV calculations and contradiction handling | Deterministic score deltas across evidence-quality scenarios |
| End-to-end workflow | CLI integration tests covering `bearing new`, research progression, assessment readiness, and hard-cutover migration errors | `just test`, `just quality`, and `just keel doctor` passing on updated fixture boards |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| External research provider support can start with deterministic fixture-backed adapters and still provide meaningful value before deeper integrations. | Scope may expand if the first provider layer is too shallow to prove the model. | Validate during voyage design and initial adapter stories. |
| Not every bearing requires heavy external research, but the workflow can still support manual or internal evidence entry through the same evidence contract. | The workflow could feel too heavy for internal-only research. | Validate with a lightweight manual-evidence path in the first slice. |
| Evidence-aware EV scoring can remain heuristic while still being materially better than the current document-presence-based research flow. | Users may distrust scores if heuristics feel arbitrary. | Add deterministic fixture scenarios and document the scoring rationale. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which provider set is realistic for the first cut without introducing auth, cost, or rate-limit drag? | Epic owner | Open |
| Social and trend signals may overpower more authoritative sources if weighting is naive. | Epic owner | Open |
| Existing bearings may need a dedicated migration pass to preserve active research without blocking the cutover. | Epic owner | Open |
| The evidence contract could become too heavy if source metadata is exhaustive before the workflow proves itself. | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Bearings in scope use a framing/evidence/assessment contract with no duplicated research-summary burden between the brief and evidence surfaces.
- [ ] Every in-scope synthesized finding or recommendation in `ASSESSMENT.md` can cite canonical evidence IDs captured in `EVIDENCE.md`.
- [ ] The research workflow can capture and surface web, academic, social, and manual or internal evidence with provenance and quality metadata.
- [ ] Bearing readiness and doctor validation fail when evidence or citations are incomplete, stale, or structurally missing according to the new contract.
- [ ] EV scoring changes deterministically in fixture scenarios when evidence breadth, freshness, authority, or contradiction changes.
- [ ] Legacy survey-era command and template paths in scope are removed and replaced with actionable hard-cutover guidance.
<!-- END SUCCESS_CRITERIA -->
