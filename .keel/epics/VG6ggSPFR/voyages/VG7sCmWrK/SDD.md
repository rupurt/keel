# Define Mission Request Ingress Replay And Acknowledgement - Software Design Description

> Define Keeper replayable ingress and acknowledgement behavior for formal mission requests

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines Keeper’s first provider-ingress lifecycle for formal mission
requests. The design makes GitHub issue activation the first provider path while
keeping the normalized ingress model replayable and cleanly separated from the
native Keel command surface.

## Context & Boundaries

### In Scope

- GitHub issue activation detection
- normalized ingress record and revision behavior
- retry and acknowledgement ownership boundaries

### Out of Scope

- non-GitHub providers
- cryptographic transport hardening
- direct mutation of planning state by provider adapters

### External Actors

- GitHub issues as provider artifacts
- Keeper ingress worker
- native `keel mission request` commands

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| GitHub issue activation format | External contract | First provider activation signal | Title/body schema |
| Canonical mission request commands | Internal contract | Native Keel mutation and validation surface | CLI contract |
| Keeper ingress runtime | Internal service | Polling, normalization, retry, and acknowledgement orchestration | Keeper architecture |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Versioned ingress records | Represent provider edits and retries as canonical revisions | Keeps replay and deduplication explicit |
| Provider-owned acknowledgements | Keeper owns transport-facing comments/status updates | Preserves the Keel/Keeper split |
| GitHub-first, not GitHub-only | Use GitHub issue activation as the first adapter on top of a generic ingress model | Avoids locking the model to one provider |

## Architecture

The ingress design has three layers:
- provider detection: recognize formal GitHub mission request activations
- normalization and revisioning: convert provider payloads into canonical request revisions
- acknowledgement orchestration: decide when Keeper emits provider-facing acknowledgements versus calling native Keel commands

## Components

- Activation detector: matches the formal title prefix and required body sections.
- Revision normalizer: produces canonical request revisions and deduplicates retries.
- Keel command bridge: calls native mission-request commands for validation, draft, and apply behavior.
- Acknowledgement renderer: prepares provider-facing acknowledgement content from canonical outcomes.

## Interfaces

- Provider input: GitHub issue title, body, metadata, and edit history.
- Normalized ingress record: canonical request plus revision metadata and replay identity.
- Command bridge output: success, validation failure, or execution failure from native Keel commands.
- Provider acknowledgement output: comment/status payloads owned by Keeper.

## Data Flow

1. Keeper reads a GitHub issue candidate.
2. Activation detection validates the formal mission request prefix and body shape.
3. The normalizer emits a canonical request revision and deduplicates retries.
4. Keeper invokes native mission-request commands with the canonical payload.
5. Keeper renders a provider-facing acknowledgement from the canonical result.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Issue title/body do not satisfy activation format | Activation detector | Ignore or mark as invalid request candidate | Provider edits request into valid form |
| Provider edit changes a previously ingested request | Revision normalizer | Emit a new canonical revision | Keeper re-runs validation/draft/apply as configured |
| Duplicate delivery or retry arrives | Replay identity check | Reuse the existing canonical revision outcome | No duplicate planning mutation |
| Provider acknowledgement fails after Keel processing | Transport response failure | Preserve canonical command outcome and surface retryable provider error | Keeper retries acknowledgement without replaying planning mutation |
