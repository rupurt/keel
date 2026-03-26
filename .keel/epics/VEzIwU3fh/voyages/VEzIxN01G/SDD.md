# Speccy Foundation And Keel Integration Pilot - Software Design Description

> Land a reusable speccy crate boundary with generic markdown template rendering hooks and cut Keel over to it without keeping Keel-specific logic in the reusable crate.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage extracts the current generic markdown template renderer out of `keel-core` into a standalone workspace crate named `speccy`, then rewires Keel's template-rendering call sites to consume that crate through a narrow adapter layer. The pilot keeps Keel-owned template inventory and explicitly project-specific markdown behavior outside `speccy` while defining reusable hook points for non-Keel adopters.

## Context & Boundaries

```text
┌────────────────────────────────────────────────────────────┐
│         speccy extraction and Keel cutover pilot          │
│                                                            │
│  Keel templates.rs  ─┬─> host catalog / adapter layer      │
│                      │                                      │
│  Keel mutations.rs ──┤                                      │
│                      ├─> speccy core renderer               │
│  other projects   ───┘    + markdown helpers + hooks        │
└────────────────────────────────────────────────────────────┘
```

### In Scope

- Move pure rendering and markdown document helpers into `crates/speccy`.
- Add host hook surfaces for template lookup and optional post-render transforms.
- Cut current Keel template-rendering call sites over to `speccy`.
- Document which responsibilities remain host-owned after the extraction.

### Out of Scope

- A richer template language than the current double-curly token substitution contract.
- A second real production consumer outside Keel.
- Extracting all frontmatter mutation logic into `speccy` before proving it is generic.

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| `regex` | library | Placeholder scanning and optional markdown/frontmatter helpers if needed | existing workspace dependency |
| Keel template inventory | internal consumer | Proves the extracted crate can render embedded project-owned templates | existing `templates.rs` module |
| Keel markdown mutation service | internal adapter | Supplies project-specific post-render transforms where needed | existing `frontmatter_mutation.rs` module |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Crate boundary | `speccy` owns pure rendering and generic markdown helpers; Keel keeps project-specific template inventory and board semantics | This is the narrowest reusable seam already proven by current call sites |
| Hook surface | Support host-provided template lookup and optional post-render integration points | Other projects need reuse without adopting Keel's embedded-template or mutation model |
| Frontmatter mutation | Keep it outside `speccy` unless implementation shows a clearly generic markdown transform worth promoting | The current behavior is useful but still coupled to Keel workflow semantics |
| Hard cutover | Replace Keel's existing generic renderer on migrated call sites in the same slice | Prevents parallel implementations from drifting immediately |

## Architecture

The design splits responsibilities into three layers:

- `speccy` core: generic string-template rendering, placeholder substitution, and markdown document helpers that operate on plain text.
- `speccy` host hooks: narrow traits or closure-based integration points for template retrieval and optional post-render processing.
- host adapter code: project-specific wiring that chooses a template catalog, passes replacement values, and composes any downstream transforms such as frontmatter mutation.

## Components

| Component | Purpose | Interface | Notes |
|-----------|---------|-----------|-------|
| `speccy::render` | Replace double-curly token placeholders deterministically from caller-supplied values | pure functions or a small renderer type | Mirrors current `render` behavior |
| `speccy::markdown` | Provide body-only/full-document helpers around markdown/frontmatter boundaries | pure helper functions | Mirrors current `render_body` behavior |
| `speccy` host hooks | Allow callers to supply template lookup and optional post-render logic | trait or closure boundary | Avoids baking Keel assumptions into the crate |
| Keel adapter layer | Select embedded templates and compose frontmatter mutation where required | small wrapper functions | Keeps `.keel` concerns local to Keel |

## Interfaces

The public API should stay narrow and text-oriented. Candidate capabilities:

- render a template string with caller-provided replacements
- render a named template through a host-supplied catalog hook
- return either full rendered markdown or a body-only form that strips an optional leading frontmatter block
- allow the host to attach optional post-render processing without introducing Keel-specific types into `speccy`

The exact hook shape, trait, closure, or both, is intentionally left open until the implementation confirms which approach keeps the API smallest while still supporting Keel and external adopters.

## Data Flow

1. Host code chooses a template source, embedded string, caller-managed map, or another project-specific catalog.
2. Host code passes template content and replacements into `speccy`.
3. `speccy` renders the template and optionally strips a leading frontmatter block for body-only callers.
4. Host code optionally applies project-specific post-processing such as frontmatter mutation.
5. Keel writes the resulting markdown artifacts and existing tests validate parity.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Hook surface is too narrow for Keel or another adopter | Integration tests cannot express current caller needs cleanly | Revisit the hook boundary before landing the public API | Expand the hook API without importing Keel-specific types |
| `speccy` leaks Keel dependencies or concepts | Build graph or code review shows imports/types from Keel crates | Refactor the reusable logic back behind plain-text abstractions | Keep only host adapter code in Keel |
| Keel behavior changes after cutover | Existing command tests or targeted regression tests fail | Block the cutover until parity is restored | Adjust adapters or widen helper coverage inside `speccy` |
