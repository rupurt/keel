---
id: 1vugz2UvY
title: Add Multi-Modal Judge Support to Executor
type: feat
status: done
created_at: 2026-02-23T17:13:04
updated_at: 2026-02-24T10:37:16
scope: 1vugyr0OR/1vugyujor
index: 2
completed_at: 2026-02-24T00:00:00
---

# Add Multi-Modal Judge Support to Executor

## Summary

The verification executor needs to support novel automated judges beyond exit codes. This story adds support for `vhs` terminal recording and `llm-judge` reasoning captures as part of the proof execution cycle.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Executor can trigger `vhs` to record CLI interactions and store them in `EVIDENCE/` <!-- verify: vhs record-cli.tape, SRS-02:start -->
- [x] [SRS-02/AC-02] Executor can package a story's diff and ACs for an `llm-judge` and capture the signed transcript <!-- verify: llm-judge, SRS-02:end -->
