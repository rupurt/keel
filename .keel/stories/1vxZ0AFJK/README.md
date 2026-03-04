---
id: 1vxZ0AFJK
title: Define Canonical Guidance Output Contract
type: feat
status: in-progress
created_at: 2026-03-03T15:18:06
updated_at: 2026-03-03T15:55:59
scope: 1vxYzSury/1vxYzh8ep
started_at: 2026-03-03T15:55:59
---

# Define Canonical Guidance Output Contract

## Summary

Define a canonical command-guidance contract that can represent one deterministic next step or one deterministic recovery step, and wire it into `keel next` JSON output as the baseline for broader command adoption.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Introduce a shared canonical guidance contract type with explicit `next_step` and `recovery_step` fields for machine-readable command output. <!-- verify: cargo test --lib serializes_next_step_only, SRS-01:start, proof: ac-1.log -->
- [x] [SRS-01/AC-02] Make `keel next --json` emit the canonical guidance contract for actionable decisions (next step on success paths, recovery step on blocked paths). <!-- verify: cargo test --lib decision_to_json_work_includes_next_step_guidance, SRS-01:continues, proof: ac-2.log -->
- [x] [SRS-01/AC-03] Add regression tests covering guidance contract serialization and decision-to-guidance mapping behavior. <!-- verify: cargo test --lib decision_to_json_blocked_includes_recovery_guidance, SRS-01:end, proof: ac-3.log -->
