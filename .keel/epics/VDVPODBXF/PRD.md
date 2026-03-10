# Config-Driven Role and Lane Topology - Product Requirements

## Problem Statement

The current role and queue model still hardcodes manager/engineer families and fixed management/execution routing in core command logic. That makes keel software-shaped, forces subtype-heavy examples like `engineer/software`, and prevents teams from defining their own role-to-lane topology for management, operations, advertising, writing, design, or other non-software delivery contexts.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make role families config-defined instead of hardcoded. | `keel next --role <family>` accepts configured base roles without requiring a subtype | 100% of role routing comes from config |
| GOAL-02 | Make lane topology config-defined with seeded defaults. | Effective lanes in `keel config show` and `keel flow` come from config or seeded defaults | 0 hardcoded role-to-lane mappings in pull surfaces |
| GOAL-03 | Add selector-based lane sourcing with validation. | Lane include/exclude selectors are validated and invalid/overlapping topologies fail doctor | 100% topology validation coverage |
| GOAL-04 | Generalize guidance, template selection, and manual acceptance to configured topology. | Role-sensitive guidance and authorization no longer depend on literal `manager/*` or `engineer/*` families | 100% of role-aware CLI surfaces migrated |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Autonomous Harness | A role-driven agent using `keel next`, lifecycle commands, and guidance output. | Needs queue routing and guidance to follow project-defined roles instead of software-only defaults. |
| Board Maintainer | The person editing `keel.toml` and shaping workflow rules for a board. | Needs to define roles, lanes, defaults, and selector rules without touching code. |
| Non-Software Operator | A user working in ads, operations, writing, research, or another delivery domain. | Needs first-class execution roles like `operator`, `copywriter`, or `advertiser` without pretending to be an engineer. |

## Scope

### In Scope

- [SCOPE-01] Config schema and seeded defaults for workflow roles, lanes, and exact role overrides.
- [SCOPE-02] Config-driven lane resolution for `next`, `accept`, role-context selection, and other role-aware command guidance.
- [SCOPE-03] Lane selector globs, include/exclude semantics, and topology validation.
- [SCOPE-04] Dynamic rendering of effective topology in `keel config show` and `keel flow`.

### Out of Scope

- [SCOPE-05] User-authored template body files or arbitrary prompt text loaded from config.
- [SCOPE-06] Shared multi-lane visibility policies beyond hard-fail overlap detection.
- [SCOPE-07] Changes to the underlying story/voyage/bearing lifecycle state machines.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `keel.toml` must support workflow defaults plus configurable `roles`, `lanes`, and exact `role_overrides`, with seeded defaults for `manager`/`operator` and `management`/`delivery` when omitted. | GOAL-01, GOAL-02, GOAL-04 | must | Establishes a configurable topology without requiring every board to author boilerplate. |
| FR-02 | `keel next --role <taxonomy>` must resolve lane access from the configured base role family and allow subtype-free roles such as `operator`. | GOAL-01, GOAL-02 | must | Removes the software-shaped assumption that execution must be `engineer/*`. |
| FR-03 | `keel flow` and `keel config show` must render the effective configured topology, including seeded defaults, lane ordering, and role-to-lane mappings. | GOAL-02, GOAL-03 | must | Operators need visibility into the topology the board is actually using. |
| FR-04 | Lane definitions must use canonical include/exclude selector globs over board state sources and reject unknown selectors or invalid references. | GOAL-03 | must | Keeps unbounded lane names safe and deterministic. |
| FR-05 | Manual story acceptance authorization must derive from configured lane capabilities rather than literal `manager/*` matching. | GOAL-02, GOAL-04 | must | Acceptance is a workflow behavior, not a hardcoded family name. |
| FR-06 | Role context and guidance selection must resolve from configured role families with optional exact-taxonomy overrides for finer-grained template selection. | GOAL-01, GOAL-04 | must | Supports domain-specific roles and subtype-specific guidance without making subtype mandatory. |
| FR-07 | `keel doctor` must fail on missing defaults, missing role/lane references, unknown selectors, and cross-lane overlap. | GOAL-03 | must | Topology errors must be caught before routing or rendering drifts. |
| FR-08 | `keel next --parallel` must follow configured lane capability (`parallel = true`) rather than a hardcoded execution family. | GOAL-02, GOAL-04 | must | Parallel pull should be driven by lane behavior, not literal role names. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Topology resolution must be deterministic across `config show`, `next`, `accept`, `flow`, and `doctor` for the same config and taxonomy input. | GOAL-01, GOAL-02, GOAL-03, GOAL-04 | must | Prevents command surfaces from disagreeing about lane membership or capabilities. |
| NFR-02 | A board with no topology overrides must preserve a sensible zero-config experience using `manager`/`operator` roles and `management`/`delivery` lanes. | GOAL-01, GOAL-02 | must | Existing boards need immediate usability without extra config authoring. |
| NFR-03 | Selector evaluation and topology projection must come from a single source of truth shared by validation, rendering, and routing. | GOAL-03, GOAL-04 | must | Avoids divergent behavior across queue-sensitive surfaces. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Config contract | Rust unit and integration tests | `cargo test` coverage for config parsing, seeded defaults, and topology resolution |
| CLI rendering | Integration tests and VHS snapshots | `keel config show` / `keel flow` proofs using configured and zero-config topologies |
| Role-driven routing | Unit and integration tests | `keel next` / `keel story accept` proofs for configured roles, capabilities, and failures |
| Validation | Doctor integration tests | Hard-fail proofs for bad selector and overlap topologies |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| One default lane per role family is sufficient for the first rollout. | We may need multi-lane memberships or explicit lane overrides sooner. | Reassess after v1 adoption. |
| Existing taxonomy matching can continue enforcing subtype requirements on individual stories and entities. | We may need a second pass on actor/story compatibility semantics. | Cover subtype cases in routing tests. |
| Seeded defaults are acceptable for boards that do not opt into custom topology. | Zero-config behavior may still feel too opinionated. | Exercise default renderings in CLI proofs. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How should `flow` stay readable when a board defines more than two lanes? | Epic owner | Open |
| Do we ever want intentional cross-lane overlap instead of hard-fail validation? | Epic owner | Deferred |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel config show` exposes seeded `manager`/`operator` roles and `management`/`delivery` lanes when topology config is omitted.
- [ ] `keel next --role operator` pulls delivery work without requiring `/software`.
- [ ] Boards can add custom role families such as `copywriter` or `director` by config alone.
- [ ] `keel story accept --role <family>` authorizes manual verification through lane capability rather than a literal `manager/*` check.
- [ ] `keel flow` renders configured lanes dynamically from the effective topology.
- [ ] Invalid selectors, missing references, or cross-lane overlap fail `keel doctor`.
<!-- END SUCCESS_CRITERIA -->
