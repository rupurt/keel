# GitHub Mission Request Ingress Contract

## Purpose

Define the replayable Keeper ingress policy for formal mission requests sourced
from GitHub issues.

## Activation Rule

A GitHub issue becomes a formal mission request candidate only when:

- the title begins with `Keel Mission Request:`
- the body conforms to the canonical mission request template
- Keeper can read the issue and its current revision through the configured
  provider credentials

Anything that fails these checks remains ordinary issue traffic and must not be
lowered into a Keel mission request.

## Normalized Ingress Record

Every accepted GitHub candidate lowers into a canonical ingress record with:

- the provider-neutral mission request envelope
- repository and issue identity
- an explicit provider revision value
- a replay identity used to distinguish new revisions from duplicate deliveries

## Revision Rules

- Keeper must treat the tuple `(provider kind, repository, issue number,
  provider revision)` as the canonical replay identity for the normalized
  request.
- A provider event that resolves to the same replay identity is a duplicate
  delivery and must not create a second planning mutation.
- An issue edit that changes the provider revision creates a new normalized
  revision and triggers a fresh Keel evaluation.
- Re-fetching the same revision is a retry path, not a semantic change.

## Invocation Boundary

Keeper is responsible for:

- polling or receiving GitHub issue changes
- detecting the mission request activation rule
- normalizing issue content and provider metadata into the canonical request
- deciding whether the current event is a new revision or a duplicate retry
- invoking native `keel mission request` stages
- delivering provider-facing acknowledgements

Keel is responsible for:

- the canonical request schema
- validation and diagnostics
- planning mutation semantics
- acknowledgement payload rendering

## Acknowledgement Rules

- Acknowledgements must refer to the exact issue identity and revision that
  produced the current Keel outcome.
- Keeper owns posting or retrying provider-facing acknowledgements.
- Keel owns rendering the canonical acknowledgement payload from the validated
  request and planning result.
- A failed acknowledgement retry must not cause Keeper to replay the planning
  mutation when the replay identity is unchanged.

## Failure Modes

| Failure | Classification | Response |
|---------|----------------|----------|
| Title/body do not satisfy activation rule | non-request traffic | ignore as ordinary issue activity |
| Required mission request sections are missing | validation failure | retain issue identity and surface repairable diagnostics |
| Duplicate delivery for unchanged revision | replay duplicate | reuse the prior normalized result, no new planning mutation |
| Provider edit races with fetch | revision change | normalize the latest revision and re-evaluate once |
| Keel rejects a normalized request | execution or validation failure | preserve diagnostics and bind them to the triggering revision |
| Provider acknowledgement fails after Keel succeeds | transport failure | retry acknowledgement only, not planning mutation |
