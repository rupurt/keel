---
id: 1vxH84M8t
title: Gate Story Submit And Accept On Coherent Artifacts
type: feat
status: backlog
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzVpw5
index: 3
---

# Gate Story Submit And Accept On Coherent Artifacts

## Summary

Enforce submit/accept lifecycle gating so unresolved scaffold/default story and reflection artifacts cannot advance to terminal states.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Story submit is blocked when story README or REFLECT contains unresolved scaffold/default patterns.
- [ ] [SRS-04/AC-02] Story accept is blocked on the same coherency violations.
- [ ] [SRS-05/AC-01] Generated report artifacts remain excluded from unresolved-scaffold enforcement scope.
