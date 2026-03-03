---
id: 1vxH84jzB
title: Add Hard Cutover Regression Coverage
type: chore
status: backlog
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzVpw5
index: 4
---

# Add Hard Cutover Regression Coverage

## Summary

Add regression coverage that enforces hard-cutover behavior across doctor and transition gates with no warning-oriented legacy expectations.

## Acceptance Criteria

- [ ] [SRS-06/AC-01] Add regression tests proving doctor and transition paths enforce hard errors for unresolved scaffold/default text.
- [ ] [SRS-06/AC-02] Replace legacy warning-oriented expectations with hard-failure assertions.
- [ ] [SRS-06/AC-03] Ensure updated suites remain green under `just quality` and `just test`.
