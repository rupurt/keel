---
id: 1vxZ0DxDl
title: Align Next Command Guidance In Human And Json Output
type: feat
status: done
created_at: 2026-03-03T15:18:09
updated_at: 2026-03-03T19:50:46
scope: 1vxYzSury/1vxYzrwma
started_at: 2026-03-03T19:41:26
completed_at: 2026-03-03T19:50:46
---

# Align Next Command Guidance In Human And Json Output

## Summary

Align `keel next` guidance rendering across human-readable and JSON outputs so both surfaces expose the same canonical next or recovery command.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Ensure every actionable `keel next` decision renders the same canonical command in human and JSON outputs. <!-- verify: cargo test --lib cli::commands::management::next::tests::actionable_decisions_keep_human_and_json_guidance_in_sync, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Ensure blocked `keel next` decisions expose canonical recovery guidance consistently across output modes. <!-- verify: cargo test --lib cli::commands::management::next::tests::blocked_and_empty_decisions_keep_human_and_json_guidance_in_sync, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add parity tests that fail if human formatter and JSON payload diverge for next/recovery guidance. <!-- verify: cargo test --lib cli::commands::management::next::tests, SRS-01:end, proof: ac-3.log-->
