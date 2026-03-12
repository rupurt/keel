---
id: VDcFgtsLv
title: Pulse Command Surface
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T21:30:30
operator-signal: 
scope: VDakmG8cH/VDcFd5Sop
index: 2
started_at: 2026-03-11T21:24:01
completed_at: 2026-03-11T21:30:30
---

# Pulse Command Surface

## Summary

Provide the non-interactive `keel pulse` entry point and the first automation
cycle summary contract for schedulers and operators.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel pulse` runs one automation cycle without interactive prompts. <!-- verify: cargo run -- pulse, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Pulse output reports evaluated, triggered, and skipped routines for the cycle. <!-- verify: cargo test pulse_human_output_reports_evaluated_would_trigger_and_skipped_routines --bin keel, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] Pulse emits structured and human-readable output suitable for scheduler logs and regression checks. <!-- verify: cargo test pulse_json_output_is_structured_for_scheduler_logs --bin keel, SRS-NFR-02:start, proof: ac-3.log-->
