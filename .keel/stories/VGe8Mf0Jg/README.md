---
# system-managed
id: VGe8Mf0Jg
status: done
created_at: 2026-04-12T22:34:35
updated_at: 2026-04-12T22:58:00
# authored
title: Surface Mission Stack In Turn Next And Mission Status
type: feat
operator-signal:
scope: VGe7mCcFW/VGe8Ad6Jy
index: 1
started_at: 2026-04-12T22:55:05
completed_at: 2026-04-12T22:58:00
---

# Surface Mission Stack In Turn Next And Mission Status

## Summary

Thread the new Mission Stack projection through the canonical operator surfaces.
`turn`, `next`, and `mission next --status` should explain local stack context,
current gating, and cross-repo dependencies while staying unchanged when no
stack is active.

## Acceptance Criteria

- [x] [SRS-02/AC-01] `keel turn` renders Mission Stack id, branch, member role, mode, checkpoint, and foreign-execution state in both text and JSON surfaces. <!-- verify: cargo test -p keel turn_surfaces_mission_stack_context_in_text_and_json, SRS-02:start:end, proof: ac-1.log -->
- [x] [SRS-03/AC-01] `keel next` emits stack-aware block or yield decisions when local execution is forbidden by the active Mission Stack state. <!-- verify: cargo test -p keel next_emits_stack_aware_decisions, SRS-03:start:end, proof: ac-2.log -->
- [x] [SRS-04/AC-01] `keel mission next --status` reports linked member missions, pending negotiations, and waiting receipts for the local stack. <!-- verify: cargo test -p keel mission_next_status_surfaces_stack_dependencies, SRS-04:start:end, proof: ac-3.log -->
