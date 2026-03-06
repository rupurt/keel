# Planning Lineage and PRD Traceability - Product Requirements


## Problem Statement

Keel's intended planning chain is `PRD -> SRS -> story acceptance criteria -> verification`, but the implementation currently enforces only the tactical half of that contract well. Voyage planning and story execution already depend on SRS traceability, while epic PRDs are treated mostly as authored prose plus structural scaffolds.

That gap shows up in three ways. First, voyage SRS `Source` fields use mixed conventions such as `FR-*`, `NFR-*`, `PRD-*`, and ad hoc labels without one enforced canonical contract. Second, `voyage plan` can validate SRS coverage by stories without validating whether the SRS itself is still faithful to the parent PRD. Third, goals/objectives and scope remain human-readable guidance but are not yet machine-checkable planning inputs.

The result is a weak strategic seam: human planners can approve an epic PRD, but downstream tactical planning can still drift before implementation begins. This epic closes that seam by making epic PRDs parseable, enforceable, and reviewable as the real source of tactical planning truth.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish canonical PRD requirement lineage | Voyage SRS requirements use only valid parent `FR-*` / `NFR-*` IDs | 100% of newly planned voyage requirements |
| GOAL-02 | Block incoherent tactical planning before execution | `voyage plan` rejects missing or invalid parent PRD lineage | 0 invalid lineage transitions to `planned` |
| GOAL-03 | Make epic coverage reviewable across voyages | Every epic requirement is classified by linked voyage coverage in planning surfaces | 100% of epic FR/NFR rows |
| GOAL-04 | Start planning with authored problem context | `epic new` hydrates real problem content into fresh epic scaffolds while goals remain authored in `PRD.md` | 100% of newly scaffolded epics |
| GOAL-05 | Connect strategic intent to tactical scope | Goals and scope can be traced from PRD into voyage planning artifacts | Canonical goal/scope linkage contracts defined and validated |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Planner | Authors epics and voyages before implementation starts | Strategic requirements that remain enforceable downstream |
| Implementer Agent | Pulls stories from planned voyages and executes against tactical specs | Confidence that voyage requirements still reflect approved PRD intent |
| Human Reviewer | Reviews planning artifacts and accepts completed work | Clear lineage from epic intent to tactical scope and later implementation |
| Maintainer | Evolves keel workflows and validation rules | One canonical planning contract without mixed source conventions |

## Scope

### In Scope

- [SCOPE-01] Canonical epic requirement IDs using `FR-*` and `NFR-*` as the only valid PRD requirement lineage tokens.
- [SCOPE-02] Hard PRD-to-SRS lineage checks in planning transitions and doctor diagnostics.
- [SCOPE-03] Epic-wide aggregation of FR/NFR coverage across all voyages in planning read surfaces.
- [SCOPE-04] CLI and template support for authored PRD problem hydration during `epic new`, with goals remaining authored directly in `PRD.md`.
- [SCOPE-05] Canonical goal-to-requirement linkage and diagnostics.
- [SCOPE-06] Canonical PRD-to-SRS scope linkage and scope-drift diagnostics.

### Out of Scope

- [SCOPE-07] Direct story acceptance-criteria references to PRD requirements; SRS remains the tactical boundary.
- [SCOPE-08] Bearing-to-PRD lineage changes or ADR workflow changes outside planning contracts.
- [SCOPE-09] Runtime compatibility aliases for legacy `PRD-*` or custom source tokens.
- [SCOPE-10] Historical board migration in the same slice as new enforcement; migration follows after checks land.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Epic PRDs MUST define canonical parent requirement IDs using only `FR-*` and `NFR-*` rows. | GOAL-01 | must | Establishes one unambiguous strategic source-of-truth contract. |
| FR-02 | Every voyage SRS requirement MUST reference exactly one valid parent `FR-*` or `NFR-*` from its epic PRD. | GOAL-01 GOAL-02 | must | Creates deterministic PRD-to-SRS lineage without ambiguous ownership. |
| FR-03 | Planning transitions and doctor diagnostics MUST hard-fail missing, invalid, or non-canonical PRD source references using the same underlying coherence rules. | GOAL-02 | must | Prevents runtime/reporting drift and stops incoherent tactical planning early. |
| FR-04 | Epic planning surfaces MUST aggregate FR/NFR coverage across all voyages and identify uncovered parent requirements. | GOAL-03 | must | Gives reviewers an epic-wide planning view instead of isolated voyage checks. |
| FR-05 | `keel epic new` MUST accept explicit problem text and hydrate that authored problem into fresh epic scaffolds, while goals/objectives remain authored directly in `PRD.md`. | GOAL-04 | must | Prevents fresh epics from starting as scaffold comments and keeps multi-row goals out of the CLI contract. |
| FR-06 | PRD goals/objectives MUST use canonical IDs and each PRD requirement MUST link to at least one goal ID. | GOAL-05 | should | Connects strategic intent to concrete requirements instead of leaving goals as disconnected prose. |
| FR-07 | PRD scope and voyage SRS scope MUST use canonical linkage so scope drift can be detected objectively. | GOAL-05 | should | Keeps tactical decomposition inside approved product scope. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | New planning-lineage behavior MUST follow hard-cutover policy with no runtime compatibility aliases for legacy source tokens. | GOAL-01 GOAL-02 GOAL-05 | must | Keeps the planning contract simple, strict, and enforceable. |
| NFR-02 | Parsing, diagnostics, and coverage output MUST be deterministic for equivalent board states. | GOAL-03 GOAL-05 | must | Prevents flaky planning reviews and unstable command/test output. |
| NFR-03 | Validation failures MUST identify the artifact, offending token or linkage, and expected canonical form. | GOAL-02 GOAL-03 GOAL-05 | must | Makes planning errors actionable for humans and agents. |
| NFR-04 | Epic aggregation MUST support one-to-many parent-to-child planning relationships without ambiguous ownership or double counting. | GOAL-03 | must | Preserves accurate epic-wide coverage reporting as work fans out into voyages. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| PRD and SRS lineage parsing | Rust unit/integration tests (`cargo test`) | Parser fixtures, invalid-token cases, deterministic ordering tests |
| Planning transition enforcement | Rust lifecycle and gate tests (`cargo test`) | `voyage plan` / transition parity tests for blocking and allowed paths |
| Doctor coherence reporting | Rust doctor regression tests (`cargo test`) | Shared-fixture assertions showing parity between diagnostics and gates |
| Planning read surfaces | Read-model and CLI snapshot tests, optionally VHS when terminal rendering changes materially | Epic/voyage show output snapshots and coverage render fixtures |
| Coverage breadth | Rust coverage run (`cargo llvm-cov --workspace --lcov --output-path coverage/lcov.info`) | Coverage report exercising parser, gating, doctor, and planning-read paths |

`llm-judge` remains optional for qualitative review of rendered planning output, but canonical lineage correctness must be proven primarily by deterministic tests rather than probabilistic review.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Epic PRD requirement IDs can remain stable enough to serve as parent lineage anchors. | Tactical artifacts may need broader rekey/migration behavior. | Parser fixtures plus planning migration follow-up if needed. |
| Each SRS requirement can have exactly one parent FR/NFR while a parent can fan out to multiple SRS rows. | Coverage math and planning ownership become ambiguous. | Coverage model tests and decomposition review. |
| Canonical goal and scope IDs can be introduced without making PRD/SRS authoring too cumbersome. | Human planning ergonomics may degrade and adoption may stall. | Template review plus CLI scaffolding and doctor usability tests. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much migration help should Keel provide when older epics or voyages still use human-only goal and scope prose? | Epic owner | open |
| Scope linkage currently activates only once a voyage authors canonical `[SCOPE-*]` links; decide whether stricter rollout is worth the churn. | Epic owner | open |
| Enforcing canonical lineage will surface existing mixed `PRD-*` and custom source tokens across historical voyages. | Epic owner | acknowledged |
| Some story creation and planning CLI surfaces may need cleanup to match the new planning contract before downstream implementation is smooth. | Epic owner | acknowledged |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Epic PRDs use canonical `FR-*` / `NFR-*` IDs and voyage SRS requirements reference them with exactly one valid parent per SRS row.
- [ ] `voyage plan` and doctor diagnostics block or report PRD-to-SRS lineage issues coherently using the same rules.
- [ ] Epic planning surfaces show FR/NFR coverage across all voyages and identify uncovered parent requirements.
- [ ] `epic new` hydrates authored problem content into fresh epic scaffolds while leaving goals/objectives to be authored directly in `PRD.md`.
- [ ] Goal linkage and scope linkage contracts are defined, validated, and visible in planning artifacts.
<!-- END SUCCESS_CRITERIA -->
