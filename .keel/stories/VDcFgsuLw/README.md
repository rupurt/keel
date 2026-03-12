---
id: VDcFgsuLw
title: Next Temporal Countdown Rendering
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:44
operator-signal: 
scope: VDakmCGYi/VDcFd5kmn
index: 1
---

# Next Temporal Countdown Rendering

## Summary

Expose routine due-state through `keel next` so scheduled work is reviewable in
both human and JSON pull surfaces.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Human-readable `keel next` output shows due-now or next-run countdown context for routine work. <!-- verify: test, SRS-02:start -->
- [ ] [SRS-02/AC-02] JSON `keel next` output includes structured gating rationale and next eligible time for scheduled work. <!-- verify: test, SRS-02:end -->
- [ ] [SRS-NFR-02/AC-01] Countdown and gating text stay stable enough for CLI regression assertions. <!-- verify: test, SRS-NFR-02:start:end -->
- [ ] [SRS-03/AC-01] Non-due routines are filtered out of actionable work selection before ranking. <!-- verify: test, SRS-03:start -->
- [ ] [SRS-03/AC-02] Due routines participate in existing prioritization semantics without reordering unrelated actionable work. <!-- verify: test, SRS-03:end -->
