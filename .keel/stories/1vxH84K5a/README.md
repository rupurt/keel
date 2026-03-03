---
id: 1vxH84K5a
title: Codify Token Bucket Contract Tests
type: chore
status: backlog
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzV3oR
index: 4
---

# Codify Token Bucket Contract Tests

## Summary

Create deterministic contract tests for token bucket policy so unknown tokens or ownership violations fail immediately.

## Acceptance Criteria

- [ ] [SRS-05/AC-02] Define and test the allowed token bucket contract (CLI-owned, system-owned, generated markers).
- [ ] [SRS-05/AC-03] Fail tests when templates contain unknown tokens or out-of-bucket token usage.
- [ ] [SRS-05/AC-04] Add/adjust drift tests to keep token inventory and CLI contract aligned over time.
