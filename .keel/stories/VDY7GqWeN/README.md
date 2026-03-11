---
id: VDY7GqWeN
title: Add Storage Section To Configuration Schema
type: feat
status: backlog
created_at: 2026-03-10T23:30:00
scope: VDXBUHZB0/VDY7AlCLy
index: 1
updated_at: 2026-03-11T02:26:38
---

# Add Storage Section To Configuration Schema

## Summary

Update the `Config` struct and related TOML parsing logic to include a new `[storage]` section.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `Config` struct has a `storage` field. <!-- verify: inspection, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] Default storage backend is set to `filesystem`. <!-- verify: inspection, SRS-02:start:end -->
