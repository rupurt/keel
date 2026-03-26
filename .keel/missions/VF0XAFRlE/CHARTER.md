# Refactor Speccy Crate Surface And Module Layout - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Refactor `speccy` in two passes so its module layout is explicit, its public rendering API is smaller and more stable, and Keel consumes the reduced surface without reintroducing Keel-specific concerns into the reusable crate. | board: VF0XAFqlF |

## Constraints

- Keep `speccy` reusable and free of dependencies on `keel-core`, `keel-cli`, `.keel` paths, or board domain types.
- Land the refactor in two passes: first a behavior-preserving module split, then a public API reduction plus Keel cutover.
- Preserve the current double-curly placeholder contract and current supported frontmatter mutation behavior unless a concrete regression or boundary violation forces a design change.

## Halting Rules

- DO NOT halt while `speccy` still mixes catalog, rendering, and frontmatter concerns in a way that keeps the reusable boundary ambiguous.
- HALT when the two-pass refactor is complete, Keel depends on the smaller `speccy` API, and the stable extension points are documented.
- YIELD to human if the API reduction requires changing template semantics beyond the current placeholder and frontmatter contracts.
