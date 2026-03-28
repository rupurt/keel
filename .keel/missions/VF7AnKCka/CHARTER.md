# Replace File-Based Heartbeat With Derived Git Activity Model - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Replace the synthetic `.keel/heartbeat` file as Keel's primary activity signal with a derived heartbeat model exposed through `keel heartbeat`, and have `keel flow --scene` use that derived activity to decide when the system is unplugged. | board: VF7Geb3Wa |

## Constraints

- Land the migration in two passes: first add the derived heartbeat read model and CLI surface while keeping file-based heartbeat only as a bounded compatibility fallback, then remove the file-based heartbeat path and update the surrounding engine surfaces.
- Derive activity from Git and worktree state first: dirty tracked files and HEAD commit time are canonical signals; board frontmatter timestamps may remain a secondary signal when useful.
- Keep the public heartbeat contract platform-stable. Avoid making inode details part of the user-facing semantics even if lower-level invalidation uses filesystem metadata internally.
- Preserve the current warning-level pacemaker semantics during the transition: the system may be idle or unplugged without being structurally unhealthy.
- Finish the migration by updating hooks, diagnostics, and documentation so downstream projects no longer depend on committing a synthetic heartbeat file.

## Halting Rules

- DO NOT halt while `keel flow --scene` still depends primarily on `.keel/heartbeat` mtime to decide whether the system is energized.
- HALT when `keel heartbeat` is the canonical activity surface, `keel flow --scene` consumes that derived signal, and the file-based heartbeat path is either removed or explicitly constrained to transitional compatibility only.
- YIELD to human if the derived heartbeat model forces a change in the meaning of "recent work" beyond Git/worktree activity and board timestamp synthesis.
