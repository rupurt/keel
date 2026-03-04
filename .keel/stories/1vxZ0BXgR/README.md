---
id: 1vxZ0BXgR
title: Add Canonical Next Guidance To Story Lifecycle Commands
type: feat
status: backlog
created_at: 2026-03-03T15:18:07
updated_at: 2026-03-03T15:18:07
scope: 1vxYzSury/1vxYzjiwH
---

# Add Canonical Next Guidance To Story Lifecycle Commands

## Summary

Apply canonical deterministic guidance to story lifecycle commands so each successful transition returns one clear next action.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add canonical `next_step` guidance to successful story lifecycle commands (`start`, `reflect`, `record`, `submit`, `accept`) when a deterministic follow-up exists.
- [ ] [SRS-01/AC-02] Ensure suggested commands align with valid story state-machine transitions for the resulting lifecycle state.
- [ ] [SRS-01/AC-03] Add regression tests for lifecycle guidance output in both human and JSON modes.
