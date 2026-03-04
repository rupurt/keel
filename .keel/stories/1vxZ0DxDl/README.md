---
id: 1vxZ0DxDl
title: Align Next Command Guidance In Human And Json Output
type: feat
status: backlog
created_at: 2026-03-03T15:18:09
updated_at: 2026-03-03T15:18:10
scope: 1vxYzSury/1vxYzrwma
---

# Align Next Command Guidance In Human And Json Output

## Summary

Align `keel next` guidance rendering across human-readable and JSON outputs so both surfaces expose the same canonical next or recovery command.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Ensure every actionable `keel next` decision renders the same canonical command in human and JSON outputs.
- [ ] [SRS-01/AC-02] Ensure blocked `keel next` decisions expose canonical recovery guidance consistently across output modes.
- [ ] [SRS-01/AC-03] Add parity tests that fail if human formatter and JSON payload diverge for next/recovery guidance.
