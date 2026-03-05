---
id: 1vuxZYutW
title: Add LLM-Judge Command Integration to Record
type: feat
status: done
created_at: 2026-02-24T10:55:52
updated_at: 2026-02-24T11:06:15
scope: 1vugyr0OR/1vugyuhks
index: 3
started_at: 2026-02-24T18:55:52
completed_at: 2026-02-24T19:00:52
---

# Add LLM-Judge Command Integration to Record

## Summary

This story adds the `--judge` flag to the `keel story record` command, allowing humans to explicitly trigger an LLM-Judge verification and capture the resulting transcript as evidence.

## Acceptance Criteria

- [x] [SRS-03/AC-01] `keel story record --judge` triggers LLM-Judge and stores transcript in EVIDENCE/ <!-- verify: true, SRS-03:start:end -->
