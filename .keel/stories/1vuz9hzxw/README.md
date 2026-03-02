---
id: 1vuz9hzxw
title: Implement Hard Migration Command for Canonical Schema
type: feat
status: done
created_at: 2026-02-24T12:37:17
updated_at: 2026-02-24T15:27:53
scope: 1vuz8K4NM/1vuz8jNo3
index: 1
submitted_at: 2026-02-24T15:26:41
completed_at: 2026-02-24T15:27:53
---

# Implement Hard Migration Command for Canonical Schema

## Summary

Implement the hard migration command that rewrites legacy schema values to canonical forms and provides a deterministic migration report.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Implement a migration command that rewrites legacy states and legacy date field keys to canonical schema values. <!-- verify: manual, SRS-01:start -->
- [x] [SRS-01/AC-02] Ensure migration is idempotent and reports changed files with actionable output. <!-- verify: manual, SRS-01:continues -->
- [x] [SRS-01/AC-03] Add fixture-based integration tests covering successful migration and unknown-token failure paths. <!-- verify: manual, SRS-01:end -->
