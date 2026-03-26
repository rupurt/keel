# Extract Reusable Speccy Markdown Template Engine - Product Requirements

## Problem Statement

Keel's current markdown template engine is a thin but important layer spread across `keel-core` infrastructure modules. The generic parts, placeholder rendering and markdown body/frontmatter handling, live beside board-specific concerns such as embedded `.keel` template catalogs. That coupling blocks reuse in other `spoke-sh` projects and makes the correct extraction seam ambiguous.

We need a reusable workspace crate named `speccy` that owns the generic markdown template rendering behavior, generic frontmatter mutation, first-class template catalog/loading abstractions, and fallible host hooks so Keel and other projects can adopt it without leaking Keel-specific concepts into the public API.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish `speccy` as the canonical reusable markdown template renderer for Keel and other Rust projects in the workspace. | Shared crate exists with generic rendering/document helpers and no dependency on Keel crates. | `speccy` crate landed and adopted by Keel |
| GOAL-02 | Make host integration explicit instead of Keel-specific. | Another project could provide its own template inventory, loaders, and render hooks without importing `.keel` concepts. | Public API supports first-class template catalogs plus fallible extension hooks |
| GOAL-03 | Cut Keel over without changing current scaffold behavior. | Existing Keel template rendering paths use `speccy` while preserving current output for representative creation flows. | Keel migration complete with regression coverage |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keel Maintainer | Maintains creation and generation commands that currently render markdown scaffolds. | One reusable engine so Keel-specific board logic no longer owns generic rendering behavior. |
| Workspace Adopter | Builds another Rust project in `spoke-sh` that needs simple markdown templating. | A crate they can plug into their own template catalog without importing Keel concepts. |
| Tooling Integrator | Wants to wrap rendering with project-specific processing such as catalog lookup or post-render transforms. | Fallible hook points and generic mutation APIs that avoid forking the core renderer. |

## Scope

### In Scope

- [SCOPE-01] Add a new workspace crate named `speccy` that owns generic double-curly token rendering, markdown document helpers, and generic frontmatter mutation APIs equivalent to Keel's current reusable behavior.
- [SCOPE-02] Define first-class template catalog/loading abstractions plus fallible host hooks for template lookup and optional post-render integration so other projects can supply their own catalogs and transforms.
- [SCOPE-03] Cut Keel's existing template rendering and frontmatter mutation callers over to `speccy` while keeping project-specific template inventory and board semantics at adapter boundaries.
- [SCOPE-04] Document the reusable boundary, including any follow-on extraction candidates that remain intentionally Keel-owned after this slice.

### Out of Scope

- [SCOPE-05] Expanding the template language beyond current placeholder substitution into loops, conditionals, or a larger DSL.
- [SCOPE-06] Building a second production consumer outside Keel in this voyage.
- [SCOPE-07] Extracting Keel-specific board rules unless they can be expressed as generic markdown utilities without leaking domain concepts.

## Requirements

### Functional Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| FR-01 | `speccy` must provide deterministic double-curly token rendering, markdown document helpers, and generic frontmatter mutation without depending on Keel crates or `.keel` board structures. | GOAL-01 GOAL-03 | must | This is the minimum reusable engine Keel is extracting today. |
| FR-02 | `speccy` must expose first-class template catalog/loading abstractions and fallible host hooks for render-time or post-render extension so other projects can adopt it without forking the renderer. | GOAL-02 | must | Reuse requires the crate to stop assuming Keel's embedded template catalog and board-specific processing. |
| FR-03 | Keel must consume `speccy` for its current template rendering and frontmatter mutation paths while leaving only project-specific template inventory and board semantics outside the reusable crate. | GOAL-01 GOAL-03 | must | The extraction is only proven if Keel itself uses the shared crate as the canonical implementation. |
| FR-04 | Planning artifacts must state which responsibilities belong in `speccy` versus the host project, including the final decision to keep template inventory host-owned while frontmatter mutation moves into `speccy`. | GOAL-02 GOAL-03 | should | This prevents the public boundary from drifting back into implicit Keel coupling. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Identical template inputs must render deterministically in `speccy`, regardless of whether the caller uses embedded strings or a host-provided template catalog. | GOAL-01 GOAL-02 | must | Deterministic scaffolding is required for Keel and any future adopters. |
| NFR-02 | Keel scaffold output for representative mission, epic, voyage, story, and bearing paths must remain behaviorally equivalent after the cutover. | GOAL-03 | must | The extraction should not change existing authored artifact contracts. |
| NFR-03 | The reusable API must avoid filesystem assumptions and keep all project-specific template inventory outside `speccy`. | GOAL-02 | must | Host projects need to choose their own catalog strategy. |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

| Area | Method | Evidence |
|------|--------|----------|
| Reusable crate behavior | Unit tests in `speccy` plus compile-time dependency boundaries | Story-level test logs |
| Keel migration | Existing Keel command and regression tests over scaffold generation | Story-level test logs |
| Boundary documentation | Planning artifact review and final voyage report | Story narrative + report artifacts |

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| The current placeholder renderer, markdown body helpers, and frontmatter mutation cover the reusable seam Keel needs today. | The voyage may need to expand into adjacent markdown-processing logic before another adopter can cut over cleanly. | Validate during implementation against current callers. |
| A first-class catalog trait plus fallible hooks are sufficient for the first external adoption path. | `speccy` may need a wider public API than planned. | Confirm through crate docs and non-Keel adoption guidance. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Are there additional generic markdown transforms beyond frontmatter mutation that belong in `speccy`, or should the crate stay narrow after this extraction? | Epic owner | Open |
| Are there non-Rust adoption expectations that would change the public API shape, naming, or packaging strategy? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `speccy` exists as a reusable workspace crate with generic rendering/document helpers, generic frontmatter mutation, and no dependency on Keel crates.
- [ ] Keel consumes `speccy` for current template rendering flows without behavior regressions on representative scaffold paths.
- [ ] The public reusable boundary and host-owned concerns are documented well enough to support another project adopting the crate, including first-class catalogs and fallible hooks.
<!-- END SUCCESS_CRITERIA -->
