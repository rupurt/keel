---
id: 1vxZ0BXgR
title: Add Canonical Next Guidance To Story Lifecycle Commands
type: feat
status: done
created_at: 2026-03-03T15:18:07
updated_at: 2026-03-03T16:52:45
scope: 1vxYzSury/1vxYzjiwH
started_at: 2026-03-03T16:41:46
completed_at: 2026-03-03T16:52:45
---

# Add Canonical Next Guidance To Story Lifecycle Commands

## Summary

Apply canonical deterministic guidance to story lifecycle commands so each successful transition returns one clear next action.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to successful story lifecycle commands (`start`, `reflect`, `record`, `submit`, `accept`) when a deterministic follow-up exists. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::start_action_suggests_submit_for_in_progress, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure suggested commands align with valid story state-machine transitions for the resulting lifecycle state. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::record_action_suggests_start_for_rejected_story, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests for lifecycle guidance output in both human and JSON modes. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::guidance_serializes_with_canonical_next_step_shape, SRS-01:end, proof: ac-3.log-->
