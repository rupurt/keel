# Normalize GitHub Issues Into Mission Requests - Software Design Description

> Define the first GitHub issue ingestion slice that detects formal mission requests, normalizes them, and invokes native Keel commands.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines the first provider ingress worker for mission requests.
GitHub issues are the default source. Keeper detects the formal title prefix,
parses the structured body template, normalizes provider metadata into the
canonical mission request envelope, and then delegates request semantics to Keel
through the native `keel mission request` commands.

## Context & Boundaries

Keeper owns provider polling, issue fetches, metadata capture, retries, and the
posting of acknowledgements. Keel stays responsible for mission-request
semantics and planning mutation. The boundary is the normalized request
envelope plus deterministic command invocation inputs and outputs.

```
┌─────────────────────────────────────────────┐
│                 Keeper ingress              │
│ issue -> detect -> normalize -> invoke Keel│
│                              └──> ack post  │
└─────────────────────────────────────────────┘
             ↑                    ↑
         GitHub issue        Keel command surface
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| GitHub issue provider | external | Supplies title, body, issue identity, and comment channel | GitHub issues |
| Keeper ingress worker | internal | Polls or receives issue updates and performs normalization | current workspace |
| `keel mission request` CLI surface | internal | Handles request semantics after normalization | planned companion mission |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Activation trigger | Title prefix `Keel Mission Request:` | Keeps activation explicit and human-readable |
| Request normalization | Keeper emits the canonical envelope before calling Keel | Preserves a clean boundary between provider and planning semantics |
| Retry model | Replays reuse provider metadata and normalized revisions | Makes polling and edits auditable |

## Architecture

The ingress worker has four phases: issue detection, structured-body parsing,
normalization, and Keel command invocation. Each phase records enough metadata
to replay the same decision later without fetching hidden state from Keel.

## Components

- Issue detector: filters candidate GitHub issues using the formal title prefix.
- Body parser: extracts the structured mission request fields from the issue
  body template.
- Envelope normalizer: adds provider identity, repository reference, issue
  number, and revision information to the canonical request shape.
- Command invoker: runs `parse`, `validate`, `draft`, `apply`, and `ack` as
  needed and records the resulting outputs.
- Acknowledgement publisher: turns the `ack` payload into a GitHub comment or
  equivalent provider-facing response.

## Interfaces

- GitHub issue title format: `Keel Mission Request: <mission title>`.
- GitHub issue body template: canonical mission request sections that Keeper can
  parse deterministically.
- Normalized envelope: the provider-neutral payload passed to Keel.
- Invocation contract: explicit stdin/stdout interaction with `keel mission
  request` subcommands.

## Data Flow

1. Keeper receives or polls a GitHub issue.
2. The detector confirms the formal mission request title prefix.
3. The parser extracts the required sections from the issue body.
4. The normalizer attaches provider identity, issue reference, and revision
   metadata to the canonical request envelope.
5. Keeper invokes the Keel mission request commands in sequence.
6. Keeper records the outputs and posts the rendered acknowledgement back to the
   provider channel.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Title does not match activation rule | Issue detection | Ignore as non-request traffic | No recovery needed |
| Body template missing required sections | Parse stage | Produce validation failure and acknowledgement guidance | User edits issue body and Keeper retries |
| Provider payload changed during processing | Revision metadata mismatch | Treat as a new normalized revision | Re-run normalization and command invocation |
| Keel rejects the normalized request | Command invocation | Capture diagnostics and publish structured acknowledgement | Human corrects request or board state before retry |
