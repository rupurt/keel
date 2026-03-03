---
id: 1vxH84k3U
title: Escalate Unresolved Scaffold Checks To Doctor Errors
type: feat
status: done
created_at: 2026-03-02T20:13:04
updated_at: 2026-03-02T21:55:09
scope: 1vxGy5tco/1vxGzVpw5
index: 1
started_at: 2026-03-02T21:46:34
submitted_at: 2026-03-02T21:50:30
completed_at: 2026-03-02T21:55:09
---

# Escalate Unresolved Scaffold Checks To Doctor Errors

## Summary

Promote unresolved scaffold/default text findings from warning-level to error-level in covered doctor checks.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Doctor emits errors (not warnings) for unresolved scaffold/default patterns in covered planning/coherency docs. <!-- verify: manual, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Doctor error output includes artifact path and offending pattern for remediation. <!-- verify: manual, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Enforcement remains hard-cutover and does not downgrade unresolved scaffold/default findings to warnings. <!-- verify: manual, SRS-01:end, proof: ac-3.log-->
