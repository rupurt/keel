---
id: 1vxZ0FZaN
title: Implement Command Capability Classification Map
type: feat
status: done
created_at: 2026-03-03T15:18:11
updated_at: 2026-03-03T20:42:23
scope: 1vxYzSury/1vxYzsAxT
started_at: 2026-03-03T20:33:23
completed_at: 2026-03-03T20:42:23
---

# Implement Command Capability Classification Map

## Summary

Implement a single command-capability classification map so guidance rendering can consistently distinguish actionable from informational commands.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Introduce a canonical classification map that labels management commands as actionable or informational. <!-- verify: cargo test --lib capability_map::tests::classification_map_labels_representative_commands, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Use the classification map in guidance rendering paths to control when `next_step` or `recovery_step` guidance is emitted. <!-- verify: cargo test --lib capability_map::tests::informational_commands_suppress_guidance_payload, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Add tests covering representative commands in both categories to ensure deterministic classification behavior. <!-- verify: cargo test --lib capability_map::tests::actionable_commands_emit_canonical_next_or_recovery_payload, SRS-01:end, proof: ac-3.log-->
