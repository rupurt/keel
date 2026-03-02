---
id: 1vuz9XRK3
title: Add Regression Tests for Gate Runtime and Reporting Modes
type: feat
status: done
created_at: 2026-02-24T12:37:07
updated_at: 2026-02-24T19:10:37
scope: 1vuz8K4NM/1vuz8dYT5
index: 5
submitted_at: 2026-02-24T19:07:55
completed_at: 2026-02-24T19:10:37
---

# Add Regression Tests for Gate Runtime and Reporting Modes

## Summary

Add regression coverage that proves strict runtime blocking and reporting-mode visibility remain coherent for shared gate rules.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Add tests that verify runtime mode blocks on errors for representative story and voyage transitions. <!-- verify: manual, SRS-05:start -->
- [x] [SRS-05/AC-02] Add tests that verify reporting mode surfaces non-blocking findings for the same scenarios when expected. <!-- verify: manual, SRS-05:continues -->
- [x] [SRS-05/AC-03] Add parity tests that compare runtime/reporting outputs to ensure they originate from one rule source. <!-- verify: manual, SRS-05:end -->
