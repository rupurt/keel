---
id: VDUGfu8bf
title: Update Accept Role Authorization
type: feat
status: done
created_at: 2026-03-10T10:38:14
updated_at: 2026-03-10T14:39:51
scope: VDTpFlMKc/VDUG60pcX
index: 4
started_at: 2026-03-10T14:23:56
completed_at: 2026-03-10T14:39:51
---

# Update Accept Role Authorization

## Summary

Require manager roles to accept manually verified stories.

## Acceptance Criteria

- [x] [SRS-05/AC-01] `keel story accept` accepts `--role <TAXONOMY>` instead of `--human` <!-- verify: cargo test --lib story_accept, SRS-05:start, proof: ac-1.log -->
- [x] [SRS-05/AC-02] If story has manual verification, require a `manager/*` role to accept <!-- verify: cargo test --lib manager_role, SRS-05:end, proof: ac-2.log -->
