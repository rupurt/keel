# Mission Request Command Contract

## Purpose

Define the stabilized machine-facing contract for `keel mission request` so
Keeper and other automation can compose request intake without depending on
provider-specific parsing logic or hidden Keel internals.

## Stability Rules

- The mission request envelope is the canonical exchange shape.
- Provider transport details normalize into the envelope instead of becoming
  Keel-specific flags.
- `template`, `parse`, `validate`, `draft`, and `ack` are side-effect free.
- `apply` is the only stage allowed to mutate `.keel` planning state.
- Validation failures are recoverable caller errors; execution failures are
  runtime or policy failures that must not be misreported as invalid input.

## Command Family

| Command | Accepted Input | Stdout Contract | Failure Class | Side Effects |
|---------|----------------|-----------------|---------------|--------------|
| `template` | optional format selector | canonical request scaffold | usage or runtime only | none |
| `parse` | stdin or file carrying raw request content | normalized envelope | validation or runtime | none |
| `validate` | stdin or file carrying normalized envelope | readiness result plus diagnostics | validation or runtime | none |
| `draft` | validated envelope from stdin or file | proposed planning move and acknowledgement preview | validation or runtime | none |
| `apply` | validated envelope from stdin or file | stable planning result summary with created identifiers and paths | validation or execution | mutates `.keel` |
| `ack` | validated request plus planning result from stdin or file | provider-facing acknowledgement payload | validation or runtime | none |

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

## Caller Field Responsibility

| Field | Responsibility | Rule |
|-------|----------------|------|
| `version` | caller-supplied | Required. Identifies the envelope contract version. |
| `request.title` | caller-supplied | Required. Must express the requested planning move in operator-readable form. |
| `request.summary` | caller-supplied | Required. Short statement of the request. |
| `request.problem` | caller-supplied | Required. Explains why the request exists. |
| `request.outcome` | caller-supplied | Required. Defines the desired result. |
| `request.constraints` | caller-supplied | Optional. Defaults to an empty list when omitted. |
| `request.requested_scope.in_scope` | caller-supplied | Required. Must contain at least one concrete requested item. |
| `request.requested_scope.out_of_scope` | caller-supplied | Optional but recommended. Defaults to an empty list when omitted. |
| `request.provider.kind` | derivable | Required after normalization when the request came from an external provider. |
| `request.provider.source_id` | derivable | Required after normalization for provider-sourced requests. |
| `request.provider.source_url` | derivable | Optional. Included when the provider exposes a stable URL. |
| `request.provider.revision` | derivable | Required when the provider supports revision or edit tracking. |

## Success And Failure Semantics

### Success

- The command accepts the declared input shape.
- The command returns deterministic stdout for the selected stage.
- `apply` returns the created or updated planning identifiers and artifact paths.
- Side effects occur only for `apply`.

### Validation Failure

- The payload was readable but failed structural or semantic checks.
- The command returns actionable diagnostics that the caller can repair.
- No planning state is mutated.
- The caller may safely retry after changing the request payload.

### Execution Failure

- The payload passed validation but the requested action could not complete due
  to policy, board state, or runtime constraints.
- The command returns an error that is distinct from validation feedback.
- The caller should treat this as escalation, retry, or policy review work, not
  as an invitation to rewrite the envelope blindly.

## Stage Notes

### `parse`

- Accepts raw provider or human-authored content.
- Produces the canonical envelope without mutating planning state.
- Rejects malformed structures with validation diagnostics.

### `validate`

- Checks field completeness, semantic coherence, and boundary rules.
- Distinguishes between actionable payload issues and runtime failures.

### `draft`

- Uses the validated envelope to preview the planning move that `apply` would
  take.
- Returns lineage and acknowledgement previews without mutating the board.

### `apply`

- Materializes the same planning move described by `draft`.
- Returns stable identifiers and artifact locations for downstream automation.
- Must not create hidden side effects outside the reported planning mutation.

### `ack`

- Renders provider-facing acknowledgement content from the normalized request
  and planning result.
- Never delivers the acknowledgement to the provider directly.

## Pipeline Rules

- All stages must be pipe-friendly for stdin/stdout composition.
- Automation callers should be able to chain `parse | validate | draft|apply`
  without provider-specific branches after normalization.
- The same normalized envelope should produce the same semantic result across
  repeated invocations unless board state has changed.
