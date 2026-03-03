---
id: 1vxH84nTQ
title: Enforce Terminal Story Coherency In Doctor
type: feat
status: backlog
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzVpw5
index: 2
---

# Enforce Terminal Story Coherency In Doctor

## Summary

Add stage-aware story/reflection coherency checks so unresolved default scaffold text blocks terminal workflow states in diagnostics.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Doctor fails `needs-human-verification` and `done` stories that retain default story scaffold text.
- [ ] [SRS-03/AC-01] Doctor fails `needs-human-verification` and `done` stories that retain default reflection scaffold text.
- [ ] [SRS-02/AC-02] Non-terminal stories are excluded from these terminal coherency checks.
