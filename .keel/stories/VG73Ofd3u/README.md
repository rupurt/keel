---
# system-managed
id: VG73Ofd3u
status: backlog
created_at: 2026-04-07T06:47:52
updated_at: 2026-04-07T07:07:38
# authored
title: Specify GitHub Mission Request Detection And Normalization
type: feat
operator-signal:
scope: VG73ONWxt/VG73OZ01E
index: 1
---

# Specify GitHub Mission Request Detection And Normalization

## Summary

Define the first Keeper ingress slice for GitHub issue mission requests so issue
detection, normalization, Keel invocation, and provider acknowledgement all run
through one replayable contract.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The story defines the GitHub mission request activation rule using the formal issue-title prefix and structured body template. <!-- verify: manual, SRS-01:start -->
- [ ] [SRS-02/AC-01] The story defines how Keeper normalizes GitHub issue metadata and request content into the canonical mission request envelope. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] The story defines the boundary where Keeper invokes the native `keel mission request` commands and captures acknowledgement outputs. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-NFR-01/AC-01] The story preserves deterministic replay inputs for retries, edits, and acknowledgement decisions. <!-- verify: manual, SRS-NFR-01:start:end -->
