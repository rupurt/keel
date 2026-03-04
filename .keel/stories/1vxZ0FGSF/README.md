---
id: 1vxZ0FGSF
title: Add Drift Tests For Canonical Guidance Contracts
type: feat
status: done
created_at: 2026-03-03T15:18:11
updated_at: 2026-03-03T20:32:05
scope: 1vxYzSury/1vxYzsAxT
started_at: 2026-03-03T20:15:46
completed_at: 2026-03-03T20:32:05
---

# Add Drift Tests For Canonical Guidance Contracts

## Summary

Add drift tests that enforce canonical command guidance contracts and prevent actionable/informational classification regressions.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add drift tests asserting actionable management commands emit canonical guidance with the expected contract shape. <!-- verify: cargo test --lib guidance_contracts::actionable_commands_emit_canonical_guidance_shape, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Add drift tests asserting informational management commands omit canonical guidance fields. <!-- verify: cargo test --lib guidance_contracts::informational_commands_omit_guidance_fields, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Ensure drift tests fail on contract-key changes or classification regressions that would break harness automation. <!-- verify: cargo test --lib guidance_contracts, SRS-01:end, proof: ac-3.log-->
