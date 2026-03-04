---
id: 1vxZ0Bh0v
title: Add Contract Tests For Canonical Guidance Fields
type: feat
status: backlog
created_at: 2026-03-03T15:18:07
updated_at: 2026-03-03T15:18:07
scope: 1vxYzSury/1vxYzh8ep
---

# Add Contract Tests For Canonical Guidance Fields

## Summary

Add regression tests that lock the canonical guidance payload shape so downstream harnesses can rely on stable `next_step` and `recovery_step` semantics.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add unit tests for canonical guidance serialization covering `next_step`-only, `recovery_step`-only, and omitted guidance cases.
- [ ] [SRS-01/AC-02] Add mapping tests that validate deterministic command strings for both actionable and blocked decisions.
- [ ] [SRS-01/AC-03] Ensure tests fail on contract drift in field names or object shape expected by harness consumers.
