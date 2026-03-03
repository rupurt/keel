---
id: 1vxH84nTQ
title: Enforce Terminal Story Coherency In Doctor
type: feat
status: done
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-03T09:34:26
scope: 1vxGy5tco/1vxGzVpw5
index: 2
started_at: 2026-03-03T08:14:15
submitted_at: 2026-03-03T09:13:53
completed_at: 2026-03-03T09:34:26
---

# Enforce Terminal Story Coherency In Doctor

## Summary

Add stage-aware story/reflection coherency checks so unresolved default scaffold text blocks terminal workflow states in diagnostics.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Doctor fails `needs-human-verification` and `done` stories that retain default story scaffold text. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-03/AC-01] Doctor fails `needs-human-verification` and `done` stories that retain default reflection scaffold text. <!-- verify: manual, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-02/AC-02] Non-terminal stories are excluded from these terminal coherency checks. <!-- verify: manual, SRS-02:end, proof: ac-3.log-->
