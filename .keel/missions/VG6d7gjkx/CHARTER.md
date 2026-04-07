# Keel Mission Request Command Surface - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define the canonical `keel mission request` CLI surface so Keeper and other programs can parse, validate, draft, apply, and acknowledge mission requests through native Keel commands. | board: VG6ggE3ud |

## Constraints

- Keep the command model provider-neutral even though GitHub issues are the first activation source.
- Keep the surface pipeline-friendly so automation can compose commands over stdin/stdout without embedding Keel internals.

## Halting Rules

- Halt after the command family, request envelope, and CLI composition contract are captured in a bearing or graduated epic with actionable next steps.
- Yield to human review before expanding the surface beyond the initial `template`, `parse`, `validate`, `draft`, `apply`, and `ack` command set.
