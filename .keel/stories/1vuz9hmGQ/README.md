---
id: 1vuz9hmGQ
title: Remove Legacy State and Status Deserializers
type: feat
status: done
created_at: 2026-02-24T12:37:17
updated_at: 2026-02-24T15:44:14
scope: 1vuz8K4NM/1vuz8jNo3
index: 2
submitted_at: 2026-02-24T15:44:00
completed_at: 2026-02-24T15:44:14
---

# Remove Legacy State and Status Deserializers

## Summary

Remove legacy deserializer aliases after migration support is available so only canonical schema values are accepted at runtime.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Remove legacy alias parsing for story, voyage, and epic status values from canonical deserializers. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-02/AC-02] Ensure parser errors for legacy tokens clearly identify canonical replacements. <!-- verify: manual, SRS-02:continues -->
- [x] [SRS-02/AC-03] Add tests that assert legacy inputs are rejected after migration cutover. <!-- verify: manual, SRS-02:end -->
