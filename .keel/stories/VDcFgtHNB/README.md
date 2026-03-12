---
id: VDcFgtHNB
title: Pulse Routine Materialization
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T21:44:48
operator-signal: 
scope: VDakmG8cH/VDcFd5Sop
index: 1
started_at: 2026-03-11T21:32:50
completed_at: 2026-03-11T21:44:48
---

# Pulse Routine Materialization

## Summary

Materialize due routine work safely so recurring automation creates board work
exactly once per eligible window.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Due routines materialize work exactly once per eligible window under their target scope. <!-- verify: cargo test pulse_materializes_due_routine_once_per_eligible_window --bin keel, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Already materialized or no-longer-eligible routines are skipped without duplicate work creation. <!-- verify: cargo test pulse_materializes_due_routine_once_per_eligible_window --bin keel, SRS-02:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Repeated pulse runs remain idempotent and safe for frequent cron or systemd execution. <!-- verify: cargo test pulse_materializes_due_routine_once_per_eligible_window --bin keel, SRS-NFR-01:start:end, proof: ac-3.log-->
- [x] [SRS-03/AC-01] Pulse records enough diagnostic state to explain why a routine was created, skipped, or deferred. <!-- verify: cargo test pulse_json_output_is_structured_for_created_skipped_and_deferred_state --bin keel, SRS-03:start:end, proof: ac-4.log-->
