---
id: 1vv7YqiJv
title: Enforce Strict Datetime Parsing in All Frontmatter
type: feat
status: done
created_at: 2026-02-24T21:35:41
updated_at: 2026-02-24T21:46:57
scope: 1vv7YWzw2/1vv7YeGDR
index: 2
submitted_at: 2026-02-24T21:46:57
completed_at: 2026-02-24T21:46:57
---

# Enforce Strict Datetime Parsing in All Frontmatter

## Summary

Enforce strict datetime parsing for all timestamp fields, removing support for date-only formats.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Update `src/model/mod.rs` to remove `deserialize_flexible_datetime` and use `deserialize_strict_datetime` everywhere. <!-- verify: manual, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Update all tests to use the strict `YYYY-MM-DDTHH:MM:SS` format. <!-- verify: manual, SRS-02:end, proof: ac-2.log-->
