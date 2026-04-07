# Mission Request Command Contract

## Purpose

Define the first canonical contract for `keel mission request` so Keeper and
other automation can compose mission request workflows without embedding Keel
internals or provider-specific parsing logic.

## Command Family

| Command | Input | Output | Side Effects |
|---------|-------|--------|--------------|
| `template` | optional format selection | canonical mission request template | none |
| `parse` | raw request payload via stdin or file | normalized mission request envelope | none |
| `validate` | normalized request envelope | validation result and diagnostics | none |
| `draft` | validated request envelope | proposed planning slice and acknowledgement preview | none |
| `apply` | validated request envelope | created or updated planning artifact summary | mutates `.keel` planning state |
| `ack` | validated request plus planning result | provider-facing acknowledgement payload | none |

## Canonical Mission Request Envelope

```yaml
version: 1
request:
  title: Add Keel Mission Request Feature
  summary: Add a native mission request workflow for Keeper and automation.
  problem: Operators need a canonical request intake path.
  outcome: Mission creation and acknowledgement are scriptable and replayable.
  constraints:
    - Keep Keel provider-neutral.
    - Preserve deterministic stdin/stdout composition.
  requested_scope:
    in_scope:
      - Native mission request subcommands
      - Provider-neutral request envelope
    out_of_scope:
      - Provider polling workers
  provider:
    kind: github-issue
    source_id: 1234
    source_url: https://github.com/spoke-sh/keel/issues/1234
    revision: 7
```

## Behavioral Contract

### `template`

- Emits the canonical authoring template for the first mission request shape.
- Supports human-authored sources and machine-composed programs equally.

### `parse`

- Converts raw provider or user input into the canonical envelope.
- Rejects malformed structures and missing required sections with explicit
  diagnostics.
- Does not mutate board state.

### `validate`

- Checks canonical field completeness, semantic coherence, and boundary rules.
- Returns a deterministic pass/fail result plus actionable diagnostics.
- Remains provider-neutral after normalization.

### `draft`

- Shows the mission-level planning move that `apply` would make.
- Returns board lineage, authored artifact paths, and acknowledgement preview.
- Never mutates the repository.

### `apply`

- Creates or updates the planning slice produced by `draft`.
- Materializes the mission boundary first and may attach linked exploratory
  artifacts only when the request explicitly authorizes them.
- Returns stable identifiers and artifact locations for downstream automation.

### `ack`

- Renders the provider-facing acknowledgement from the validated request and the
  resulting planning move.
- Emits text that external automation can post directly or transform further.
- Never calls providers directly.

## Pipeline Rules

- All subcommands accept explicit stdin or file input where applicable.
- All subcommands emit deterministic stdout suitable for piping or capture.
- Provider metadata remains part of the envelope, not part of Keel-specific
  command flags.
- Interactive prompts are excluded from the first contract surface.

## Command Boundary

Keel owns the request contract, normalization target shape, diagnostics, and
planning mutation behavior. Provider workers own collection of external payloads
and the decision of when to call this surface.
