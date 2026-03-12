# Routine Entity and Blueprints - Product Requirements

## Problem Statement

Recurring work in Keel currently lives outside the board or is recreated by
hand as ad hoc stories. There is no first-class artifact that says what work
should recur, where that work belongs, and what blueprint later automation
should instantiate. That leaves the upcoming temporal gating, pulse, and
scheduled-lane work without a canonical source of truth and forces operators to
manage recurring processes through external notes or scripts.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make Routine a first-class board primitive for recurring work blueprints. | Routines load, persist, and render through canonical board surfaces. | A routine bundle under `.keel/routines/` is discoverable without manual path knowledge. |
| GOAL-02 | Give operators a minimal authoring surface for recurring work definitions. | CLI create/list/show flow exists for routines. | Operators can scaffold and inspect routines without hand-creating directories. |
| GOAL-03 | Capture enough blueprint metadata for later scheduling epics without changing story contracts. | Routine schema stores cadence and target scope while story frontmatter remains unchanged. | Temporal gating and pulse can consume Routine as their canonical input. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Autonomous Harness | Agent loop that will later select and trigger recurring work. | A canonical routine definition with cadence and target scope. |
| Human Operator | Planner or maintainer defining recurring board work. | A simple way to author and inspect recurring blueprints inside Keel. |

## Scope

### In Scope

- [SCOPE-01] Routine entity schema and canonical bundle under `.keel/routines/<id>/`
- [SCOPE-02] Board, loader, and filesystem storage integration for routines
- [SCOPE-03] Minimal `keel routine new`, `keel routine list`, and `keel routine show` surfaces
- [SCOPE-04] Routine scaffolding that captures cadence, target scope, and blueprint content in one artifact

### Out of Scope

- [SCOPE-90] Time-based scheduling decisions in `keel next`
- [SCOPE-91] Non-interactive `keel pulse` execution
- [SCOPE-92] `keel flow` scheduled-lane rendering and operator guidance
- [SCOPE-93] Automatic story creation from routines

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Define a Routine entity and bundle that captures identity, cadence metadata, target scope, and authored blueprint content for recurring work. | GOAL-01, GOAL-03 | must | Later automation needs one canonical recurring-work source of truth. |
| FR-02 | Load and persist routines through the board model and filesystem adapter alongside existing entities. | GOAL-01 | must | Routine must be visible to future scheduling logic and CLI read surfaces. |
| FR-03 | Provide `keel routine new`, `keel routine list`, and `keel routine show` commands for authoring and inspection. | GOAL-02 | must | Operators need a supported path to create and review routine definitions. |
| FR-04 | Scaffold routine bundles with human-editable sections that keep cadence, target scope, and blueprint narrative together. | GOAL-02, GOAL-03 | should | The routine contract needs to be editable without introducing a second format. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Routine loading and listing remain deterministic and safe when no routines exist on the board. | GOAL-01 | must | Boards without routines must keep current behavior and ordering guarantees. |
| NFR-02 | Routine authoring must not require backward-compatibility changes to existing story frontmatter or story lifecycle contracts. | GOAL-03 | must | Routine should extend the model without destabilizing current delivery flows. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Routine domain contract | Rust unit tests | Frontmatter, model, and loader coverage |
| CLI authoring/read surfaces | Command integration tests | `routine new/list/show` assertions |
| Board integration | Loader and storage tests | Board/routine persistence and discovery proofs |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| A single markdown bundle is sufficient for the first routine blueprint contract. | We may need a second artifact (for example `BLUEPRINT.md`) sooner than planned. | Validate while implementing `routine show` and later pulse consumption. |
| Cadence can be stored as opaque metadata in this epic and interpreted later by scheduling work. | Temporal gating may need to reshape the routine schema. | Revisit in epic `VDakmCGYi`. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Which cadence syntax should become the long-term canonical routine trigger format? | Architecture | Open |
| Should routines target only voyages, or also epic-scoped stories? | Planning | Open |
| A too-thin blueprint contract could force churn when pulse begins instantiating work. | Engineering | Mitigate with explicit scope/cadence fields in v1 |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Routine bundles under `.keel/routines/` load through the board and storage stack.
- [ ] Operators can create, list, and inspect routines through CLI commands instead of manual directory setup.
- [ ] Routine definitions capture cadence and target scope without changing existing story frontmatter contracts.
<!-- END SUCCESS_CRITERIA -->
