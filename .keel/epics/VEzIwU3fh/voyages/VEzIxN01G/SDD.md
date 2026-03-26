# Speccy Foundation And Keel Integration Pilot - Software Design Description

> Land a reusable speccy crate boundary with generic markdown template rendering hooks and cut Keel over to it without keeping Keel-specific logic in the reusable crate.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extracts the current generic markdown template renderer out of `keel-core` into a standalone workspace crate named `speccy`, then rewires Keel's template-rendering and generic frontmatter-mutation call sites to consume that crate through a narrow adapter layer. The pilot keeps Keel-owned template inventory and explicitly project-specific markdown behavior outside `speccy` while defining reusable hook points for non-Keel adopters.

## Context & Boundaries

```text
┌────────────────────────────────────────────────────────────┐
│         speccy extraction and Keel cutover pilot          │
│                                                            │
│  Keel templates.rs  ─┬─> host catalog / adapter layer      │
│                      │                                      │
│  other projects   ───┼─> speccy render + mutation core      │
│                      │      + catalogs + hooks              │
│  Keel board rules  ──┘                                      │
└────────────────────────────────────────────────────────────┘
```

### In Scope

- Move pure rendering, markdown document helpers, and generic frontmatter mutation into `crates/speccy`.
- Add first-class template catalogs plus fallible host hook surfaces for template lookup and optional post-render transforms.
- Cut current Keel template-rendering and generic frontmatter-mutation call sites over to `speccy`.
- Document which responsibilities remain host-owned after the extraction.

### Out of Scope

- A richer template language than the current double-curly token substitution contract.
- A second real production consumer outside Keel.
- Extracting project-specific board transforms beyond generic frontmatter mutation.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `anyhow` | library | Fallible catalog loading and hook APIs | existing workspace dependency |
| Keel template inventory | internal consumer | Proves the extracted crate can render embedded project-owned templates through a host-owned catalog | existing `templates.rs` module |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Crate boundary | `speccy` owns pure rendering, generic markdown helpers, and generic frontmatter mutation; Keel keeps project-specific template inventory and board semantics | This captures the reusable seam without dragging `.keel` rules into the crate |
| Catalog abstraction | Standardize template lookup as a first-class trait with a simple in-memory implementation | Other projects need a reusable loading abstraction without inheriting Keel's embedded constants |
| Hook surface | Support fallible token-resolution and post-render hooks from the start | Keel and external adopters both need extension points that can fail cleanly |
| Frontmatter mutation | Promote the generic mutation service into `speccy` and use it from Keel | The mutation logic is text-oriented and reusable across host projects |
| Hard cutover | Replace Keel's existing generic renderer on migrated call sites in the same slice | Prevents parallel implementations from drifting immediately |

## Architecture

The design splits responsibilities into three layers:

- `speccy` core: generic string-template rendering, placeholder substitution, markdown document helpers, and generic frontmatter mutation that operate on plain text.
- `speccy` integration surface: a first-class `TemplateCatalog` trait plus fallible hook callbacks for token resolution and post-render processing.
- host adapter code: project-specific wiring that chooses a template catalog, passes replacement values, and composes any downstream project rules that remain outside the crate.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| `speccy::render` / `render_body` | Replace double-curly token placeholders deterministically from caller-supplied values and expose body-only helpers | pure functions | Mirror current `render` and `render_body` behavior |
| `speccy::TemplateCatalog` | Load templates through a host-owned abstraction | trait | Keeps template inventory out of the reusable crate |
| `speccy::MemoryTemplateCatalog` | Provide a simple in-memory catalog for tests and small adopters | concrete type | Proves the first-class catalog contract |
| `speccy::RenderHooks` | Allow callers to provide fallible token resolution and post-render behavior | closure-backed hook surface | Avoids baking Keel assumptions into the crate |
| `speccy::Mutation` + `apply_frontmatter_mutations` | Apply generic frontmatter updates to rendered markdown | text-mutation API | Replaces the Keel-owned generic mutation implementation |
| Keel adapter layer | Select embedded templates and project-specific board wiring | small wrapper functions | Keeps `.keel` concerns local to Keel |

## Interfaces

The public API stays narrow and text-oriented:

- `TemplateCatalog::load(&self, template_id) -> anyhow::Result<String>`
- `MemoryTemplateCatalog` for simple embedded or test catalogs
- `RenderHooks` for fallible token resolution and post-render processing
- `render`, `render_with_hooks`, `render_from_catalog`, and `render_body` helpers
- `Mutation` plus `apply_frontmatter_mutations` and the convenience render-and-mutate helpers

Keel keeps only the embedded template inventory and project-specific board semantics outside this interface.

## Data Flow

1. Host code chooses a template source, typically an embedded or caller-managed `TemplateCatalog`.
2. Host code passes template content and replacements into `speccy`, optionally with `RenderHooks`.
3. `speccy` renders the template and optionally strips a leading frontmatter block for body-only callers.
4. When the host needs generic markdown frontmatter updates, it uses `speccy::Mutation` and `apply_frontmatter_mutations` or the render-and-mutate helpers.
5. Keel writes the resulting markdown artifacts and existing tests validate parity.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Hook surface is too narrow for Keel or another adopter | Integration tests cannot express current caller needs cleanly | Revisit the hook boundary before landing the public API | Expand the hook API without importing Keel-specific types |
| Catalog abstraction is too weak for external adopters | Another project cannot model its template inventory cleanly | Add another host-owned catalog implementation, not a Keel-specific shortcut | Preserve `TemplateCatalog` as the public loading contract |
| `speccy` leaks Keel dependencies or concepts | Build graph or code review shows imports/types from Keel crates | Refactor the reusable logic back behind plain-text abstractions | Keep only host adapter code in Keel |
| Keel behavior changes after cutover | Existing command tests or targeted regression tests fail | Block the cutover until parity is restored | Adjust adapters or widen helper coverage inside `speccy` |
