---
id: VDY7GqWeN
title: Add Storage Section To Configuration Schema
type: feat
status: done
created_at: 2026-03-10T23:30:00
updated_at: 2026-03-11T02:36:07
scope: VDXBUHZB0/VDY7AlCLy
index: 1
started_at: 2026-03-11T03:00:00
submitted_at: 2026-03-11T02:36:01
completed_at: 2026-03-11T02:36:07
---

# Add Storage Section To Configuration Schema

## Summary

Update the `Config` struct and related TOML parsing logic to include a new `[storage]` section.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `Config` struct has a `storage` field. <!-- verify: manual, SRS-01:start:end -->
- [x] [SRS-02/AC-01] Default storage backend is set to `filesystem`. <!-- verify: manual, SRS-02:start:end -->
