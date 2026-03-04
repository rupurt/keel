---
id: 1vxZ0FZaN
title: Implement Command Capability Classification Map
type: feat
status: backlog
created_at: 2026-03-03T15:18:11
updated_at: 2026-03-03T15:18:11
scope: 1vxYzSury/1vxYzsAxT
---

# Implement Command Capability Classification Map

## Summary

Implement a single command-capability classification map so guidance rendering can consistently distinguish actionable from informational commands.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Introduce a canonical classification map that labels management commands as actionable or informational.
- [ ] [SRS-01/AC-02] Use the classification map in guidance rendering paths to control when `next_step` or `recovery_step` guidance is emitted.
- [ ] [SRS-01/AC-03] Add tests covering representative commands in both categories to ensure deterministic classification behavior.
