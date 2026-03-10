---
id: VDUGfu8bf
title: Update Accept Role Authorization
type: feat
status: backlog
created_at: 2026-03-10T10:38:14
updated_at: 2026-03-10T13:16:09
scope: VDTpFlMKc/VDUG60pcX
index: 4
---

# Update Accept Role Authorization

## Summary

Require manager roles to accept manually verified stories.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] `keel story accept` accepts `--role <TAXONOMY>` instead of `--human` <!-- verify: test --> <!-- SRS-05:start:end -->
- [ ] [SRS-05/AC-02] If story has manual verification, require a `manager/*` role to accept <!-- verify: test --> <!-- SRS-05:start:end -->
