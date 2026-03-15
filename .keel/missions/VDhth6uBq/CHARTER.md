# HEAD Syntax For Show Commands - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Add HEAD-relative selectors to all keel show commands so users can open the current list head and navigate backward with HEAD~, HEAD~~, and HEAD^ using the same stable default ordering that list surfaces expose. | board: VDhtrxgW6 |

## Constraints

- Preserve the current stable list-order semantics for each entity type and resolve HEAD syntax from the same canonical order source that the default list surfaces use.
- Keep selector resolution fully local and deterministic; no shell history, git history, or external state may influence HEAD-relative lookups.
- Support exact IDs alongside HEAD forms without introducing legacy alias fallbacks or per-command syntax drift.
- Keep parser behavior and error guidance consistent across mission, epic, voyage, story, bearing, ADR, and routine show commands.

## Halting Rules

- DO NOT halt while epic `VDhtrxgW6` or its child voyage/story work remains non-terminal.
- HALT when epic `VDhtrxgW6` is done and the show surfaces accept exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ from the stable default list ordering for each supported entity type.
- YIELD to human only if existing list-order semantics conflict across entity types and the canonical ordering cannot be resolved from local code or docs.
