---
id: 1vxZ0CBPx
title: Add Canonical Recovery Guidance To Story Lifecycle Errors
type: feat
status: done
created_at: 2026-03-03T15:18:08
updated_at: 2026-03-03T19:17:59
scope: 1vxYzSury/1vxYzjiwH
started_at: 2026-03-03T19:03:23
completed_at: 2026-03-03T19:17:59
---

# Add Canonical Recovery Guidance To Story Lifecycle Errors

## Summary

Add canonical recovery guidance for story lifecycle failures so blocked transitions return one deterministic remediation command.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Emit canonical `recovery_step` guidance for common story lifecycle failures (for example invalid transition state, unmet preconditions, or missing required artifacts). <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests::recovery_not_found_maps_to_story_list, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure recovery guidance commands are executable and directly address the corresponding blocking condition. <!-- verify: cargo test --lib cli::commands::management::story::accept::tests::accept_errors_on_manual_verification_without_human_flag, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add regression tests that validate story lifecycle error-to-recovery mapping in human and JSON outputs. <!-- verify: cargo test --lib cli::commands::management::story::guidance::tests, SRS-01:end, proof: ac-3.log-->
