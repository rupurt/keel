---
id: 1vxH84k3U
title: Escalate Unresolved Scaffold Checks To Doctor Errors
type: feat
status: backlog
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:05:48
scope: 1vxGy5tco/1vxGzVpw5
index: 1
---

# Escalate Unresolved Scaffold Checks To Doctor Errors

## Summary

Promote unresolved scaffold/default text findings from warning-level to error-level in covered doctor checks.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Doctor emits errors (not warnings) for unresolved scaffold/default patterns in covered planning/coherency docs.
- [ ] [SRS-01/AC-02] Doctor error output includes artifact path and offending pattern for remediation.
- [ ] [SRS-01/AC-03] Enforcement remains hard-cutover and does not downgrade unresolved scaffold/default findings to warnings.
