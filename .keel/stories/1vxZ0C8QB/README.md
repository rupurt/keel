---
id: 1vxZ0C8QB
title: Add Canonical Guidance To Voyage Lifecycle Commands
type: feat
status: done
created_at: 2026-03-03T15:18:08
updated_at: 2026-03-03T19:00:57
scope: 1vxYzSury/1vxYzjiwH
started_at: 2026-03-03T18:47:55
completed_at: 2026-03-03T19:00:57
---

# Add Canonical Guidance To Voyage Lifecycle Commands

## Summary

Apply canonical deterministic guidance to voyage lifecycle commands so planning and execution transitions expose one clear next command.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to successful voyage lifecycle commands (`plan`, `start`, `done`) where a deterministic follow-up action exists. <!-- verify: cargo test --lib cli::commands::management::voyage::guidance::tests::serializes_plan_guidance_with_canonical_next_step_shape, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure voyage guidance command suggestions align with lifecycle guards and resulting voyage states. <!-- verify: cargo test --lib cli::commands::management::voyage::guidance::tests::start_action_suggests_done_transition, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests covering voyage lifecycle guidance in both human-readable and JSON output. <!-- verify: cargo test --lib voyage::guidance, SRS-01:end, proof: ac-3.log-->
