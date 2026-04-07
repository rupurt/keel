# GitHub Mission Request Ingress Contract

## Purpose

Define the first Keeper ingress path for formal mission requests sourced from
GitHub issues.

## Activation Rule

A GitHub issue is a mission request candidate only when:

- the title begins with `Keel Mission Request:`
- the body conforms to the canonical mission request template
- the issue remains readable through the configured Keeper provider credentials

Issues that fail any of these checks remain ordinary issue traffic and are not
normalized into Keel requests.

## GitHub Issue Template

```md
# Summary
<one paragraph mission summary>

# Problem
<what is broken, missing, or strategically required>

# Desired Outcome
<what must become true if the mission is accepted>

# Constraints
- <constraint 1>
- <constraint 2>

# Requested Scope
## In Scope
- <item>

## Out of Scope
- <item>
```

## Normalization Mapping

| GitHub Source | Canonical Envelope Field | Notes |
|---------------|--------------------------|-------|
| issue title suffix | `request.title` | prefix is stripped before normalization |
| `# Summary` section | `request.summary` | required |
| `# Problem` section | `request.problem` | required |
| `# Desired Outcome` section | `request.outcome` | required |
| `# Constraints` bullets | `request.constraints[]` | optional but preserved |
| `Requested Scope / In Scope` bullets | `request.requested_scope.in_scope[]` | required for structured requests |
| `Requested Scope / Out of Scope` bullets | `request.requested_scope.out_of_scope[]` | required for boundary clarity |
| issue number and URL | `provider.source_id` / `provider.source_url` | required for replay |
| latest body edit marker | `provider.revision` | increments on edit or refetch reconciliation |

## Keeper Invocation Boundary

Keeper is responsible for:

- polling or receiving GitHub issue changes
- filtering for the mission request title prefix
- parsing the structured issue body
- attaching provider identity, repository reference, and revision metadata
- invoking `keel mission request parse`, `validate`, `draft`, `apply`, and `ack`
- posting the acknowledgement payload back to GitHub when appropriate

Keel is responsible for:

- the canonical request schema
- validation and diagnostics
- planning mutation semantics
- acknowledgement rendering

## Revision and Replay Rules

- Every processed issue revision is normalized with an explicit provider
  revision number.
- Keeper retries re-run the same normalized payload when the provider revision
  is unchanged.
- Edited issues produce a new normalized revision and a fresh Keel evaluation.
- Acknowledgements refer back to the exact issue number and revision that
  produced the planning result.

## Failure Modes

| Failure | Keeper Response |
|---------|-----------------|
| Missing required section in issue body | emit validation diagnostics and keep issue un-applied |
| Title does not match activation prefix | ignore as non-request issue |
| Provider fetch conflict during edit | normalize the latest revision and rerun |
| Keel rejects validated payload | publish or log the returned diagnostics for operator review |
