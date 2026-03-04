---
id: 1vxZ0Bh0v
title: Add Contract Tests For Canonical Guidance Fields
type: feat
status: done
created_at: 2026-03-03T15:18:07
updated_at: 2026-03-03T18:13:28
scope: 1vxYzSury/1vxYzh8ep
started_at: 2026-03-03T16:54:11
completed_at: 2026-03-03T18:13:28
---

# Add Contract Tests For Canonical Guidance Fields

## Summary

Add regression tests that lock the canonical guidance payload shape so downstream harnesses can rely on stable `next_step` and `recovery_step` semantics.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Add unit tests for canonical guidance serialization covering `next_step`-only, `recovery_step`-only, and omitted guidance cases. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Add mapping tests that validate deterministic command strings for both actionable and blocked decisions. <!-- verify: cargo test --lib decision_to_json_, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Ensure tests fail on contract drift in field names or object shape expected by harness consumers. <!-- verify: cargo test --lib cli::commands::management::guidance::tests::serializes_omitted_guidance_as_empty_object, SRS-01:end, proof: ac-3.log-->
