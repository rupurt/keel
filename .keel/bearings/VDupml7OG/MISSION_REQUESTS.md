# Spoke Mission Request Ingress

This document formalizes a provider-neutral mission request that Keeper can
ingest and lower into Keel commands.

## Goal

Mission requests should be:

- human-readable at the provider boundary
- machine-parseable and machine-validatable
- replayable from stored provider evidence
- provider-neutral once normalized
- composable from CLI tools so Keeper, scripts, or other programs can call the
  same interface

GitHub issues should be the default provider, not the only provider.

## Canonical Mission Request Envelope

Every provider payload should normalize into one envelope with these fields.

| Field | Required | Purpose |
|-------|----------|---------|
| `version` | yes | Schema version for forward-compatible parsing |
| `request_id` | yes | Stable canonical request id |
| `provider` | yes | Source provider such as `github-issues` |
| `provider_ref` | yes | Stable provider reference such as `spoke-sh/keel#123` |
| `provider_event_ref` | no | Event or revision id when the provider exposes one |
| `requested_at` | yes | Timestamp from the provider or ingestion layer |
| `requester` | yes | Human or system requesting the mission |
| `project_ref` | yes | Target project or repository boundary |
| `title` | yes | Candidate mission title |
| `problem` | yes | Why the mission exists |
| `desired_outcome` | yes | What success should change visibly |
| `constraints` | no | Hard boundaries or non-goals |
| `artifacts` | no | Repos, apps, deployments, docs, or systems in scope |
| `acceptance_signals` | yes | Observable signals that the request is satisfied |
| `priority` | no | Intake priority hint |
| `visibility` | no | Public, project-private, or reactor-private |
| `attachments` | no | Supporting links, issue refs, docs, or evidence |
| `idempotency_key` | yes | Stable replay key derived from provider identity and payload revision |

## Default GitHub Issue Activation

The default provider is GitHub issues.

### Activation rule

The issue title must start with:

```text
Keel Mission Request:
```

The suffix becomes the candidate mission title.

Example:

```text
Keel Mission Request: Add Keel Mission Request Feature
```

### Default issue body format

The body should be structured as Markdown with required YAML front matter.

```md
---
version: 1
project: spoke-sh/keel
requester: "@alex"
priority: p2
visibility: project-private
artifacts:
  - repo: spoke-sh/keel
  - repo: spoke-sh/spoke
---

## Problem
Keeper needs a formal mission request ingress that can start from GitHub issues
and lower into native Keel commands.

## Desired Outcome
Keeper can detect, parse, validate, draft, and apply mission requests from
provider payloads without hardcoding GitHub-only logic into Keel.

## Constraints
- Default provider is GitHub issues.
- The flow must remain backend-agnostic and Keeper-managed.
- Other programs should be able to call the same CLI interface.

## Acceptance Signals
- A formal request schema exists.
- Keeper can activate requests from GitHub issue titles with the required prefix.
- Keel exposes CLI commands for template, parse, validate, draft, and apply.
```

## Validation Rules

A valid mission request should satisfy all of the following:

- title begins with the required provider activation prefix
- YAML front matter is present and parses cleanly
- `version` is supported
- `project` is present and maps to an allowed Keeper project boundary
- `Problem`, `Desired Outcome`, and `Acceptance Signals` sections are present
  and non-empty
- the normalized `title` is non-empty after trimming the prefix
- `idempotency_key` can be derived deterministically
- provider evidence is retained by reference, digest, or stored copy

Validation should fail hard on missing required structure. It may warn on:

- missing `priority`
- missing `constraints`
- ambiguous artifact references
- broad wording that likely needs operator refinement

## Proposed Keel Command Family

The request flow should live under a native Keel namespace so Keeper can call
Keel instead of reimplementing parser logic:

```text
keel mission request template
keel mission request parse
keel mission request validate
keel mission request draft
keel mission request apply
keel mission request ack
```

### Command responsibilities

`keel mission request template`
- prints the default provider template
- supports `--provider github-issues`
- supports `--format markdown` and `--format json`

`keel mission request parse`
- accepts raw provider payload fields
- normalizes them into the canonical mission request envelope
- emits machine-readable JSON

`keel mission request validate`
- checks schema and policy rules
- exits non-zero on invalid input
- emits diagnostics for operators and Keeper logs

`keel mission request draft`
- shows what Keel would create or mutate
- prints the planned mission title, linked evidence, and candidate commands
- should not mutate the board

`keel mission request apply`
- creates the mission through native Keel lifecycle surfaces
- links provider evidence back into authored artifacts
- records the request id and provider ref for replay and audit

`keel mission request ack`
- prepares the provider-facing acknowledgement payload
- should let Keeper write back a canonical comment or status update

## Recommended CLI Shape

The interface should be pipeline-friendly.

Example parse and validate flow:

```bash
keel mission request parse \
  --provider github-issues \
  --provider-ref spoke-sh/keel#123 \
  --title "Keel Mission Request: Add Keel Mission Request Feature" \
  --body-file issue.md \
  --requested-at 2026-04-06T14:30:00Z \
| keel mission request validate --stdin
```

Example draft flow:

```bash
keel mission request parse \
  --provider github-issues \
  --provider-ref spoke-sh/keel#123 \
  --title "Keel Mission Request: Add Keel Mission Request Feature" \
  --body-file issue.md \
  --requested-at 2026-04-06T14:30:00Z \
| keel mission request validate --stdin \
| keel mission request draft --stdin
```

Example apply flow from Keeper:

```bash
keel mission request parse \
  --provider github-issues \
  --provider-ref spoke-sh/keel#123 \
  --title "Keel Mission Request: Add Keel Mission Request Feature" \
  --body-file issue.md \
  --requested-at 2026-04-06T14:30:00Z \
| keel mission request validate --stdin \
| keel mission request apply --stdin
```

## Lowering Into Existing Planning Surfaces

The default apply behavior should be conservative.

It should:

- create the mission
- attach provider evidence to the authored mission artifacts
- record the canonical request id and provider ref
- optionally attach an existing bearing when the request is clearly exploratory

It should not assume that every request should immediately create epics,
voyages, or stories.

## Keeper Runtime Flow

The Keeper side should look like this:

1. Poll configured providers.
2. Select provider artifacts that match activation rules.
3. Fetch title, body, metadata, and revision identity.
4. Invoke `keel mission request parse`.
5. Invoke `keel mission request validate`.
6. Invoke `keel mission request draft` or `apply` depending on policy.
7. Record evidence and acknowledge the source provider.

This keeps provider-specific logic in Keeper and request semantics in Keel.

## Security Notes

- Provider payloads are input, not truth.
- Request ids and idempotency keys must be stable across retries.
- Provider edits should create a new reviewed revision, not silently replace the
  previously normalized request.
- The raw provider artifact should be preserved by reference, digest, or stored
  copy so humans can audit what Keeper saw.
- Threshold attestation is likely unnecessary for initial request parsing, but
  useful when a request is promoted into high-consequence mission state.
