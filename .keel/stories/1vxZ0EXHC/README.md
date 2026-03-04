---
id: 1vxZ0EXHC
title: Add Canonical Guidance To Verify And Audit Commands
type: feat
status: done
created_at: 2026-03-03T15:18:10
updated_at: 2026-03-03T20:07:26
scope: 1vxYzSury/1vxYzrwma
started_at: 2026-03-03T19:52:02
completed_at: 2026-03-03T20:07:26
---

# Add Canonical Guidance To Verify And Audit Commands

## Summary

Add canonical guidance output to verification and audit command outcomes so success and failure paths provide deterministic next or recovery actions.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add canonical `next_step` guidance for successful `verify` and `story audit` outcomes when a deterministic follow-up action exists. <!-- verify: cargo test --lib verification_guidance::tests::verify_success_guidance_maps_to_story_lifecycle_next_step, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Add canonical `recovery_step` guidance for failed verification and audit outcomes with actionable remediation commands. <!-- verify: cargo test --lib verification_guidance::tests::verify_failed_report_maps_to_story_audit_recovery, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering verify/audit guidance parity across human-readable and JSON command outputs. <!-- verify: cargo test --lib verification_guidance::tests::verify_and_audit_guidance_preserve_human_json_parity, SRS-01:end, proof: ac-3.log-->
