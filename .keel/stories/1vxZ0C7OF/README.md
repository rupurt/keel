---
id: 1vxZ0C7OF
title: Add Canonical Guidance To ADR Transition Commands
type: feat
status: done
created_at: 2026-03-03T15:18:08
updated_at: 2026-03-03T18:30:08
scope: 1vxYzSury/1vxYzjVMv
started_at: 2026-03-03T18:22:50
completed_at: 2026-03-03T18:30:08
---

# Add Canonical Guidance To ADR Transition Commands

## Summary

Add canonical guidance output to ADR lifecycle transitions so successful and recoverable outcomes expose one deterministic follow-up command.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add canonical `next_step` guidance to actionable ADR transition outputs (`accept`, `reject`, `deprecate`, `supersede`) where a deterministic follow-up exists. <!-- verify: cargo test --lib cli::commands::management::adr::guidance::tests::success_guidance_for_transitions_is_canonical_next_step, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Emit canonical `recovery_step` guidance for ADR transition failures that require explicit user remediation. <!-- verify: cargo test --lib cli::commands::management::adr::guidance::tests::error_with_recovery_embeds_recovery_command_block, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering ADR command output guidance in both human-readable and JSON modes. <!-- verify: cargo test --lib adr::guidance, SRS-01:end, proof: ac-3.log-->
