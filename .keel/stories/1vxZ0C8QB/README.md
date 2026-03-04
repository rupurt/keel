---
id: 1vxZ0C8QB
title: Add Canonical Guidance To Voyage Lifecycle Commands
type: feat
status: backlog
created_at: 2026-03-03T15:18:08
updated_at: 2026-03-03T15:18:08
scope: 1vxYzSury/1vxYzjiwH
---

# Add Canonical Guidance To Voyage Lifecycle Commands

## Summary

Apply canonical deterministic guidance to voyage lifecycle commands so planning and execution transitions expose one clear next command.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add canonical `next_step` guidance to successful voyage lifecycle commands (`plan`, `start`, `done`) where a deterministic follow-up action exists.
- [ ] [SRS-01/AC-02] Ensure voyage guidance command suggestions align with lifecycle guards and resulting voyage states.
- [ ] [SRS-01/AC-03] Add regression tests covering voyage lifecycle guidance in both human-readable and JSON output.
