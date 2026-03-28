# Make Public Narrative Authoritative In The Engine - Product Requirements

## Problem Statement

The public docs now express a clearer product model than the engine itself encodes. Command taxonomy, turn structure, scene semantics, role routing, and narrative invariants still live in parallel or implied forms instead of one canonical implementation contract.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Make the command taxonomy authoritative in code. | Help output, guidance classification, and scene metadata derive from one catalog. | Catalog cutover landed |
| GOAL-02 | Make turn structure and scene semantics first-class product surfaces. | `keel turn` and scene contracts express canonical state instead of duplicated command-local logic. | Turn and scene cutover landed |
| GOAL-03 | Make role and lane routing inspectable and explainable. | `keel roles` and `keel next --explain` expose the configured topology and routing rationale. | Explainability surfaces landed |
| GOAL-04 | Turn narrative claims into executable drift guards. | CLI, turn-loop, scene, and routing claims from the docs are covered by regression tests. | Contract tests landed |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Operator | Uses the CLI as the primary daily interface for delivery and diagnosis. | A command surface whose grouping, scenes, and routing signals are internally coherent. |
| Keel Maintainer | Evolves the engine, help output, and tests. | One canonical product model instead of duplicated taxonomy and implied contracts. |
| Downstream Adopter | Learns Keel through the docs and applies it in another repository. | Public docs that match actual commands, scenes, and routing behavior. |

## Scope

### In Scope

- [SCOPE-01] Introduce a canonical CLI command catalog with family, capability, turn-phase, docs-slug, and scene metadata.
- [SCOPE-02] Add a first-class turn-loop projection and a `keel turn` surface that makes the documented operating rhythm inspectable.
- [SCOPE-03] Create a shared scene-contract registry so `--scene` surfaces declare their canonical dependencies explicitly.
- [SCOPE-04] Add a `keel roles` surface plus richer `keel next` explanation based on workflow topology and role-context projections.
- [SCOPE-05] Add drift and contract tests that treat the public CLI narrative as an executable product contract.

### Out of Scope

- [SCOPE-06] Rewriting unrelated command implementations whose behavior already matches the public contract.
- [SCOPE-07] Replacing the MDX information architecture or redesigning the docs site visuals again.
- [SCOPE-08] Adding a background service, daemon, or non-CLI UI to represent turns, scenes, or routing.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | Keel must define one canonical command catalog that classifies public commands by family, capability, turn phase, docs slug, and scene support. | GOAL-01 GOAL-04 | must | The docs and CLI should not maintain parallel taxonomies. |
| FR-02 | Help grouping and command-guidance classification must consume the canonical command catalog instead of independent hard-coded maps. | GOAL-01 GOAL-04 | must | The product surface should stay consistent when commands are added or regrouped. |
| FR-03 | Keel must expose a first-class turn-loop projection that represents Orient, Inspect, Pull, Ship, and Close as explicit surfaces rather than prose-only concepts. | GOAL-02 GOAL-04 | must | The docs already teach the turn loop as a core system behavior. |
| FR-04 | Keel must define scene contracts centrally so each `--scene` surface declares the canonical signals it depends on and scenes do not invent state. | GOAL-02 GOAL-04 | must | The visual surfaces should compress engine state, not reinterpret it ad hoc. |
| FR-05 | Keel must expose workflow roles and lanes through a direct inspection surface and route explanations that make `keel next` decisions legible. | GOAL-03 GOAL-04 | must | Role-aware routing is a core part of the product story, not an internal implementation detail. |
| FR-06 | Regression tests must cover the strongest CLI narrative claims so docs drift becomes a failing contract instead of silent divergence. | GOAL-04 | must | The docs are now expressive enough to serve as a product truth source. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | New catalog, turn, scene, and routing surfaces must remain deterministic and machine-readable where appropriate. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | These surfaces should work for humans and harnesses alike. |
| NFR-02 | Existing command names and existing `--scene` outputs must not regress in meaning while the underlying metadata and projections are refactored. | GOAL-01 GOAL-02 | must | The mission is a convergence refactor, not a product reset. |
| NFR-03 | Formatting, linting, docs build, and regression suites covering the new surfaces must pass before the epic closes. | GOAL-01 GOAL-02 GOAL-03 GOAL-04 | must | The changes touch code, docs, and test contracts simultaneously. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Command catalog cutover | Unit tests, drift tests, and CLI help review | Voyage 1 story evidence |
| Turn and scene surfaces | Unit tests, CLI proof, and scene contract tests | Voyage 2 story evidence |
| Role and lane explainability | Command regression tests and JSON contract review | Voyage 3 story evidence |
| Narrative invariants | Drift tests against docs-facing claims plus full repo verification | Voyage 4 story evidence |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The current public docs describe the intended product direction more accurately than the duplicated implementation details. | The refactor could codify the wrong abstraction. | Validate each voyage against the docs pages that motivated it. |
| The existing workflow topology and role-context projections are sufficient foundations for an inspectable roles surface. | The role explainability slice would require deeper topology redesign. | Validate during voyage 3 implementation. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much of the scene contract should live in `keel-core` versus `keel-cli` metadata? | Epic owner | Open |
| Should `keel turn` stay read-only or eventually participate in automation guidance? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] One catalog drives command family grouping, capability classification, and scene metadata.
- [ ] `keel turn` exposes the documented turn loop as a first-class CLI surface.
- [ ] `keel roles` and `keel next --explain` make routing legible from configured topology.
- [ ] Drift tests fail when the public CLI, turn-loop, scene, or routing story diverges from code.
<!-- END SUCCESS_CRITERIA -->
