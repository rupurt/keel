# MissionEntity - Product Requirements

> Autonomous harnesses halt prematurely because keel has no first-class
representation of long-running objectives. The "keep going" contract lives in
AGENTS.md prose which harnesses interpret inconsistently. A Mission entity that
encodes goals, success criteria, and halting rules as machine-readable board
state will let keel reason about completion deterministically and eliminate
false halting.

## Problem Statement

Current autonomous workflows (sift, port) span multiple epics over days.
The stopping condition is implicit — an AGENTS.md policy that says "don't stop
until the objective is done." Problems observed:

1. **False halting**: Harnesses stop when the queue is empty even though the
   objective is incomplete. They treat `keel next` returning no work as a
   signal to stop, when they should be creating the next planning unit.
2. **No traceability**: When a multi-day session ends, there's no single
   artifact showing what was accomplished, what's left, and why decisions
   were made. You have to reconstruct intent from scattered epics/voyages.
3. **No verification boundary**: Real-world success (revenue, user metrics,
   latency targets) has no place on the board. The harness can't distinguish
   "board work done" from "objective achieved."
4. **No lineage root**: Epics, bearings, and ADRs spawned during autonomous
   work are disconnected. There's no parent entity linking them to a common
   objective.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Mission entity as first-class board primitive | Domain model, state machine, loader, Board integration | Missions load and persist on disk |
| GOAL-02 | Full CLI lifecycle for missions | All transition commands functional | new, refine, activate, show, list, pause, achieve, verify, abandon |
| GOAL-03 | Lineage tracing from missions to child entities | mission field on epics, bearings, ADRs with doctor validation | Doctor flags orphaned lineage |
| GOAL-04 | Anti-false-halt doctor checks | Active missions with unmet goals block halting | Zero false halts on board-verifiable criteria |
| GOAL-05 | Mission-aware `keel next` | Recommends work creation when queue empty but mission incomplete | Harnesses never halt with unmet goals |
| GOAL-06 | CHARTER.md artifact with structured goals and halting rules | Parseable goal table with verification types | Machine-readable halting boundary |
| GOAL-07 | Interactive refinement loop | `keel mission refine` drives goal elicitation | Harness loops until CHARTER is complete |
| GOAL-08 | Decision journal with digest | LOG.md append-only with rotation strategy | Readable after multi-day sessions |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Autonomous Harness | AI agent using keel for multi-day delivery | Machine-readable objectives and halting rules |
| Human Operator | Reviews mission outcomes and verifies real-world goals | Traceability from objective to delivered artifacts |

## Scope

### In Scope

- [SCOPE-01] Mission domain model, state machine, and frontmatter schema
- [SCOPE-02] Mission directory structure and loader integration into Board
- [SCOPE-03] CLI commands for mission lifecycle transitions
- [SCOPE-04] `keel mission refine` interactive goal elicitation loop
- [SCOPE-05] CHARTER.md parsing and goal verification type system
- [SCOPE-06] LOG.md decision journal with append and digest
- [SCOPE-07] Optional `mission` lineage field on epics, bearings, and ADRs
- [SCOPE-08] Doctor checks for mission completion, orphaned lineage, stale missions
- [SCOPE-09] `keel next` mission-awareness for anti-false-halt behavior
- [SCOPE-10] `keel flow` mission-level progress summary

### Out of Scope

- [SCOPE-90] Multi-mission scheduling and prioritization
- [SCOPE-91] External metrics integration (webhooks, API endpoints)
- [SCOPE-92] Mission-to-mission lineage or dependencies

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Mission entity with YAML frontmatter (id, title, status, timestamps) and directory-based identity | GOAL-01 | must | Follows existing entity conventions for consistency |
| FR-02 | Mission state machine: Defining → Active → Achieved → Verified, with Paused and Abandoned branches | GOAL-01, GOAL-02 | must | Lifecycle for long-running autonomous objectives |
| FR-03 | CHARTER.md artifact with Goals table (MG-XX IDs), Constraints, and Halting Rules sections | GOAL-06 | must | Machine-readable halting boundary for harnesses |
| FR-04 | `keel mission new` creates mission directory with README.md and CHARTER.md scaffold | GOAL-02 | must | Entry point for mission creation |
| FR-05 | `keel mission refine` analyzes CHARTER.md and returns next question or ready signal | GOAL-07 | must | Drives interactive goal elicitation without blocking harnesses |
| FR-06 | `keel mission activate` transitions Defining → Active, gated on CHARTER completeness | GOAL-02 | must | Ensures missions have well-defined goals before starting |
| FR-07 | `keel mission show/list` display mission state, child entities, and goal progress | GOAL-02 | must | Visibility into mission state |
| FR-08 | `keel mission pause/achieve/verify/abandon` transition commands with gating | GOAL-02 | must | Full lifecycle management |
| FR-09 | Optional `mission` field on epic, bearing, and ADR frontmatter with loader support | GOAL-03 | must | Lineage tracing from mission to child entities |
| FR-10 | LOG.md append-only decision journal with structured entries | GOAL-08 | must | Traceability for multi-day autonomous decisions |
| FR-11 | `keel mission digest` compresses older LOG.md entries into summary block | GOAL-08 | should | Keeps LOG.md readable after long sessions |
| FR-12 | Doctor check: MissionGoalAchieved signals when all board-verifiable goals pass | GOAL-04 | must | Halt signal for completed missions |
| FR-13 | Doctor check: MissionActiveNoWork warns when mission active but no in-flight work | GOAL-04 | must | Prompts harness to create next work unit |
| FR-14 | Doctor check: MissionOrphanedLineage errors when entity references nonexistent mission | GOAL-03 | must | Lineage integrity |
| FR-15 | `keel next --agent` checks active mission goals before returning "no work" | GOAL-05 | must | Anti-false-halt behavior |
| FR-16 | `keel flow` includes mission-level progress when missions exist | GOAL-05 | should | High-level autonomous progress visibility |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Mission operations deterministic and reproducible across runs | GOAL-01 | must | Consistent behavior for autonomous harnesses |
| NFR-02 | Mission loading does not regress board load time by more than 10% | GOAL-01 | should | Missions are a lightweight addition to the board |
| NFR-03 | LOG.md digest keeps working file under 500 lines | GOAL-08 | should | Readable after multi-day sessions |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Domain model and state machine verified through unit tests
- CLI commands verified through integration tests with test board builder
- Doctor checks verified through existing doctor test infrastructure
- `keel next` mission-awareness verified through end-to-end flow tests
- CHARTER.md parsing verified through unit tests on goal extraction

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Single active mission per board is sufficient for v1 | May need multi-mission scheduling sooner | Monitor port/sift usage patterns |
| CHARTER.md goals can be adequately parsed from markdown tables | May need structured format (YAML/TOML) | Validate with harness usage |
| Existing entity patterns (frontmatter, state machine, loader) scale to missions | May need architectural changes | First voyage validates this |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| LOG.md digest threshold (line count vs entry count vs age) | Engineering | Open — start simple, tune from usage |
| How does mission-awareness interact with multi-mission boards? | Architecture | Deferred — design for 1:many, build single |
| Should CHARTER.md constraints auto-create ADRs? | Architecture | Deferred — nice-to-have for future |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Mission entity with state machine, frontmatter, loader, and Board integration
- [ ] CLI commands for full mission lifecycle (new, refine, activate, show, list, pause, achieve, verify, abandon)
- [ ] Lineage field (`mission`) on epics, bearings, and ADRs with doctor validation
- [ ] Doctor checks that signal mission completion and prevent false halting
- [ ] `keel next` mission-awareness — recommends creating work when queue empty but mission incomplete
- [ ] CHARTER.md with structured goals, constraints, and halting rules
- [ ] Refinement loop (`keel mission refine`) for interactive goal elicitation
- [ ] LOG.md decision journal with digest/rotation for long-running missions
<!-- END SUCCESS_CRITERIA -->

## Research Analysis

*From bearing assessment:*

## Findings

- Autonomous harnesses halt early despite explicit AGENTS.md policy because the
  stopping condition is prose, not board state [SRC-01] [SRC-02]
- Mission entity fills the gap between "board work done" and "objective
  achieved" by encoding both machine-checkable and human-verifiable goals [SRC-03]
- Design follows existing entity conventions, minimizing new infrastructure [SRC-03]

## Opportunity Cost

Time spent building Mission is time not spent on other keel features. However,
Mission directly enables the core use case (autonomous multi-day delivery) and
the false-halting problem is the highest-friction issue observed in production
harness usage [SRC-01] [SRC-02].

## Dependencies

- Existing entity infrastructure is stable and well-tested [SRC-03]
- Lineage field pattern proven by bearing → epic lineage (just shipped) [SRC-03]

## Alternatives Considered

- **Enhanced AGENTS.md only**: More prescriptive prose rules. Rejected — harnesses
  already ignore the existing rules because they're not machine-checkable [SRC-01]
- **External objective tracker**: Track goals outside keel. Rejected — breaks the
  "board is authoritative" principle and adds integration complexity [SRC-01] [SRC-02]
- **Epic-level goals**: Add goal tracking to epics instead. Rejected — objectives
  span multiple epics and need a dedicated lifecycle [SRC-03]

---

*This PRD was seeded from bearing `1vzeJF000`. See `bearings/1vzeJF000/` for original research.*
