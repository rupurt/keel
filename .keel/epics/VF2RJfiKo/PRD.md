# Teach Downstream Keel Adoption And Instruction Sync - Product Requirements

## Problem Statement

Keel's public docs explain the board model and persona workflows, but they do not yet teach downstream repos how to adopt AGENTS.md/INSTRUCTIONS.md as a project-management engine contract or how to upgrade Keel and sync upstream guidance safely.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Help downstream maintainers understand how `AGENTS.md` and `INSTRUCTIONS.md` make Keel the active project-management engine inside their repo. | Maintainers can explain and adapt the upstream agent contract for a project repo using the public docs. | A dedicated workflow page lands in the docs site. |
| GOAL-02 | Help downstream maintainers upgrade Keel and sync upstream instruction changes without erasing repo-specific customizations. | Maintainers can follow a documented upgrade checklist and validation loop. | A dedicated upgrade workflow page lands in the docs site. |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Downstream Maintainer | A project owner adopting Keel inside their own repository. | Understand which upstream instruction rules stay canonical and which repo surfaces must be adapted locally. |
| Harness Operator | A person configuring AI agents against a downstream repo. | Know where the project-level operating contract lives and how to keep it synchronized during upgrades. |

## Scope

### In Scope

- [SCOPE-01] Add public docs explaining the role of `AGENTS.md` and `INSTRUCTIONS.md` when a downstream repo uses Keel as its project-management engine.
- [SCOPE-02] Use the `port` repository as a concrete downstream example of upstream instruction adaptation.
- [SCOPE-03] Add public docs covering how to upgrade Keel and sync upstream instruction changes while preserving repo-specific commands, proof contracts, and operational expectations.
- [SCOPE-04] Integrate the new material into the current public docs IA and cross-link it from relevant onboarding or workflow pages.

### Out of Scope

- [SCOPE-05] Automated sync tooling, code generation, or CLI commands that rewrite downstream instruction files.
- [SCOPE-06] Exhaustive reference documentation for every downstream harness extension file such as `CLAUDE.md` or `GEMINI.md`.
- [SCOPE-07] Changes to Keel's runtime lifecycle behavior, doctor rules, or project scaffolding commands.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | The public docs must explain how downstream repositories use `AGENTS.md` and `INSTRUCTIONS.md` to turn Keel from a CLI into the active project-management engine for human and AI collaborators. | GOAL-01 | must | This is the missing conceptual bridge between Keel itself and project-level adoption. |
| FR-02 | The docs must show the concrete adaptation seams between upstream instruction files and a downstream repository, using `port` as the working example. | GOAL-01 | must | Readers need a real pattern for what changes downstream and what stays canonical. |
| FR-03 | The docs must include an upgrade workflow that explains how to upgrade Keel and sync upstream instruction changes while reapplying repo-specific wrappers, proof contracts, and validation steps. | GOAL-02 | must | Downstream users need a safe maintenance path after initial adoption. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | The new docs must preserve the formal, product-led tone of the public docs site while remaining concrete enough to use as operational guidance. | GOAL-01 GOAL-02 | must | This keeps the material consistent with the rest of the site. |
| NFR-02 | The workflow guidance must remain vendor-neutral and describe adaptation at the repository and harness-contract level rather than for a single AI provider. | GOAL-01 GOAL-02 | must | Downstream teams may use different harnesses while sharing the same Keel board model. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Downstream adoption docs | Manual review of authored docs pages, sidebar integration, and cross-links | Story-level manual evidence linked to the new workflow docs |
| Upgrade and sync workflow | Manual review of the authored checklist and example adaptation guidance | Story-level manual evidence linked to the upgrade docs |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| `port` is representative enough to serve as the first downstream example. | The docs may overfit to one repo and miss broader adaptation seams. | Keep examples focused on instruction and workflow patterns rather than product-specific implementation details. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| How much repo-specific detail from `port` should appear directly in the docs? | Epic owner | Resolved during voyage planning: enough to show the adaptation seams, not enough to become `port` reference docs. |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] Downstream maintainers can point to a public docs section that explains how Keel becomes the project-management engine inside a repo.
- [ ] Downstream maintainers can follow a documented upgrade path to resync upstream instructions without losing local adaptations.
<!-- END SUCCESS_CRITERIA -->
