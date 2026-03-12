---
id: VDcFgsiMj
title: Routine Due-State Engine
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T21:03:08
operator-signal: 
scope: VDakmCGYi/VDcFd5kmn
index: 1
started_at: 2026-03-11T20:56:31
completed_at: 2026-03-11T21:03:08
---

# Routine Due-State Engine

## Summary

Introduce the routine timing evaluator that turns cadence metadata into
deterministic due-state for the pull loop.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Routine cadence metadata resolves to due/not-due state plus next eligible time. <!-- verify: cargo test routine_due_state --lib, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-01] Due-state evaluation uses injected or reference time so identical inputs are deterministic. <!-- verify: cargo test evaluate_routine_due_state_is_deterministic_for_identical_reference_time --lib, SRS-NFR-01:start:end, proof: ac-2.log-->
