# Extract Reusable Speccy Markdown Template Engine - Product Requirements

## Problem Statement

Keel's current markdown template engine is a thin but important layer spread across `keel-core` infrastructure modules. The generic parts, placeholder rendering and markdown body/frontmatter handling, live beside board-specific concerns such as embedded `.keel` template catalogs and frontmatter mutation. That coupling blocks reuse in other `spoke-sh` projects and makes the correct extraction seam ambiguous.

We need a reusable workspace crate named `speccy` that owns the generic markdown template rendering behavior, exposes host integration hooks, and lets Keel adopt it without leaking Keel-specific concepts into the public API.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Establish `speccy` as the canonical reusable markdown template renderer for Keel and other Rust projects in the workspace. | Shared crate exists with generic rendering/document helpers and no dependency on Keel crates. | `speccy` crate landed and adopted by Keel |
| GOAL-02 | Make host integration explicit instead of Keel-specific. | Another project could provide its own template inventory and render hooks without importing `.keel` concepts. | Public API supports host-supplied template sources and extension hooks |
| GOAL-03 | Cut Keel over without changing current scaffold behavior. | Existing Keel template rendering paths use `speccy` while preserving current output for representative creation flows. | Keel migration complete with regression coverage |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Keel Maintainer | Maintains creation and generation commands that currently render markdown scaffolds. | One reusable engine so Keel-specific board logic no longer owns generic rendering behavior. |
| Workspace Adopter | Builds another Rust project in `spoke-sh` that needs simple markdown templating. | A crate they can plug into their own template catalog without importing Keel concepts. |
| Tooling Integrator | Wants to wrap rendering with project-specific processing such as catalog lookup or post-render transforms. | Hook points that avoid forking the core renderer. |

## Scope

### In Scope

- [SCOPE-01] Add a new workspace crate named `speccy` that owns generic double-curly token rendering and markdown document helpers equivalent to Keel's current reusable behavior.
- [SCOPE-02] Define a host hook surface for template lookup and optional render/post-render integration so other projects can supply their own catalogs and transforms.
- [SCOPE-03] Cut Keel's existing template rendering callers over to `speccy` while keeping project-specific template inventory and board semantics at adapter boundaries.
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
| FR-01 | `speccy` must provide deterministic double-curly token rendering and markdown document helpers without depending on Keel crates or `.keel` board structures. | GOAL-01 GOAL-03 | must | This is the minimum reusable engine Keel is extracting today. |
| FR-02 | `speccy` must expose host integration hooks for template sourcing and render-time or post-render extension so other projects can adopt it without forking the renderer. | GOAL-02 | must | Reuse requires the crate to stop assuming Keel's embedded template catalog and board-specific processing. |
| FR-03 | Keel must consume `speccy` for its current template rendering paths while leaving only project-specific adapter code outside the reusable crate. | GOAL-01 GOAL-03 | must | The extraction is only proven if Keel itself uses the shared crate. |
| FR-04 | Planning artifacts must state which responsibilities belong in `speccy` versus the host project, including any intentionally deferred extraction points. | GOAL-02 GOAL-03 | should | This prevents the public boundary from drifting back into implicit Keel coupling. |
<!-- END FUNCTIONAL_REQUIREMENTS -->

### Non-Functional Requirements

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Requirement | Goals | Priority | Rationale |
|----|-------------|-------|----------|-----------|
| NFR-01 | Identical template inputs must render deterministically in `speccy`, regardless of whether the caller uses embedded strings or a host-provided template catalog hook. | GOAL-01 GOAL-02 | must | Deterministic scaffolding is required for Keel and any future adopters. |
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
| The current placeholder renderer and markdown body/frontmatter helpers are the right initial extraction seam. | The voyage may need to expand into adjacent markdown-processing logic before Keel can cut over cleanly. | Validate during implementation against current callers. |
| A hook-based API can satisfy external reuse without immediately standardizing every downstream transform as a first-class `speccy` feature. | `speccy` may need a wider public API than planned. | Confirm with design review during implementation. |
| Keel-specific frontmatter mutation can remain in adapter code unless a truly generic markdown transform emerges. | The cutover may leave too much value behind in Keel. | Reassess once the hook surface is concrete. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should `speccy` own generic frontmatter mutation helpers, or should Keel continue to compose that behavior outside the crate via hooks? | Epic owner | Open |
| Does the public hook surface need to standardize template catalogs as traits, closures, or both? | Epic owner | Open |
| Are there non-Rust adoption expectations that would change the public API shape, naming, or packaging strategy? | Epic owner | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `speccy` exists as a reusable workspace crate with generic rendering/document helpers and no dependency on Keel crates.
- [ ] Keel consumes `speccy` for current template rendering flows without behavior regressions on representative scaffold paths.
- [ ] The public reusable boundary and host-owned concerns are documented well enough to support another project adopting the crate.
<!-- END SUCCESS_CRITERIA -->
