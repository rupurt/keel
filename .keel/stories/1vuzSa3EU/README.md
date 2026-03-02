---
id: 1vuzSa3EU
title: Gate Voyage Report Artifacts to Done State
type: feat
status: done
created_at: 2026-02-24T12:56:48
updated_at: 2026-02-24T16:58:56
scope: 1vuz8K4NM/1vuz8dYT5
index: 6
submitted_at: 2026-02-24T00:00:00
completed_at: 2026-02-24T00:00:00
started_at: 2026-02-24T06:28:24
---

# Gate Voyage Report Artifacts to Done State

## Summary

Make voyage reporting artifacts lifecycle-aware so `VOYAGE_REPORT.md` and `COMPLIANCE_REPORT.md` are produced only when a voyage reaches `done`, not during draft/planned/in-progress generation.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Update generation and voyage README document-link behavior so `VOYAGE_REPORT.md` and `COMPLIANCE_REPORT.md` are absent/hidden for non-done voyage states. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-02/AC-02] Generate or refresh both report artifacts as part of `voyage done` transition execution, using current story/evidence state. <!-- verify: manual, SRS-02:continues -->
- [x] [SRS-02/AC-03] Add regression tests covering non-done voyages (no report artifacts/links) and done voyages (artifacts present and linked). <!-- verify: manual, SRS-02:end -->
