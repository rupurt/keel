# Extract Speccy Markdown Template Engine For Cross-Project Reuse - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Extract the current markdown template engine into a reusable workspace crate named `speccy`, prove the crate boundary by cutting Keel over to it, and leave only project-specific template inventory plus explicitly justified adapter logic in Keel. | board: VEzIwU3fh |

## Constraints

- `speccy` must remain reusable and must not depend on `keel-core` or `keel-cli`.
- Preserve the current double-curly token placeholder contract unless a follow-up design decision explicitly expands the language.
- Host projects must be able to provide their own template catalog and integration hooks without depending on `.keel` paths, board frontmatter fields, or Keel command modules.

## Halting Rules

- DO NOT halt while markdown template rendering is still coupled to Keel through reusable logic that belongs in `speccy`.
- HALT when `speccy` exists as a reusable crate, Keel consumes it for its current template rendering paths, and the reusable-vs-host boundary is documented.
- YIELD to human if the desired hook surface requires expanding beyond the current placeholder model or if frontmatter mutation must become part of the reusable public API.
