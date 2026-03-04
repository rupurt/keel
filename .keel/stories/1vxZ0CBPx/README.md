---
id: 1vxZ0CBPx
title: Add Canonical Recovery Guidance To Story Lifecycle Errors
type: feat
status: backlog
created_at: 2026-03-03T15:18:08
updated_at: 2026-03-03T15:18:08
scope: 1vxYzSury/1vxYzjiwH
---

# Add Canonical Recovery Guidance To Story Lifecycle Errors

## Summary

Add canonical recovery guidance for story lifecycle failures so blocked transitions return one deterministic remediation command.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Emit canonical `recovery_step` guidance for common story lifecycle failures (for example invalid transition state, unmet preconditions, or missing required artifacts).
- [ ] [SRS-01/AC-02] Ensure recovery guidance commands are executable and directly address the corresponding blocking condition.
- [ ] [SRS-01/AC-03] Add regression tests that validate story lifecycle error-to-recovery mapping in human and JSON outputs.
