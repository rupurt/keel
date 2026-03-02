---
id: 1vuzUa2mf
title: Fix Scaffold and Story Timestamp Doctor Findings at Source
type: feat
status: done
created_at: 2026-02-24T12:58:52
updated_at: 2026-02-24T19:33:54
scope: 1vuz8K4NM/1vuz8jNo3
index: 6
submitted_at: 2026-02-24T19:33:08
completed_at: 2026-02-24T19:33:54
---

# Fix Scaffold and Story Timestamp Doctor Findings at Source

## Summary

Fix the real causes behind current doctor findings by generating datetime fields where state transitions/scaffolding write timestamps. This includes epic/voyage scaffold `created_at`, story lifecycle `submitted_at`/`completed_at`, and removal of default TODO placeholders from newly scaffolded planning artifacts.

## Acceptance Criteria

- [x] [SRS-06/AC-01] Update epic/voyage scaffolding paths so generated frontmatter uses datetime `created_at` format (`YYYY-MM-DDTHH:MM:SS`) instead of date-only values. <!-- verify: manual, SRS-06:start -->
- [x] [SRS-06/AC-02] Replace generated TODO placeholders in default epic/voyage planning artifacts with non-placeholder baseline content that does not trigger doctor placeholder warnings on creation. <!-- verify: manual, SRS-06:continues -->
- [x] [SRS-06/AC-03] Ensure `keel story submit` writes `submitted_at` using datetime format (`YYYY-MM-DDTHH:MM:SS`) and never date-only values. <!-- verify: manual, SRS-06:continues -->
- [x] [SRS-06/AC-04] Ensure `keel story accept` writes `completed_at` using datetime format (`YYYY-MM-DDTHH:MM:SS`) and never date-only values. <!-- verify: manual, SRS-06:continues -->
- [x] [SRS-06/AC-05] Add regression coverage showing fresh scaffolded epic+voyage and story submit/accept flows pass relevant doctor datetime and placeholder checks. <!-- verify: manual, SRS-06:end -->
