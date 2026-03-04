---
id: 1vxZ0EXHC
title: Add Canonical Guidance To Verify And Audit Commands
type: feat
status: backlog
created_at: 2026-03-03T15:18:10
updated_at: 2026-03-03T15:18:10
scope: 1vxYzSury/1vxYzrwma
---

# Add Canonical Guidance To Verify And Audit Commands

## Summary

Add canonical guidance output to verification and audit command outcomes so success and failure paths provide deterministic next or recovery actions.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add canonical `next_step` guidance for successful `verify` and `story audit` outcomes when a deterministic follow-up action exists.
- [ ] [SRS-01/AC-02] Add canonical `recovery_step` guidance for failed verification and audit outcomes with actionable remediation commands.
- [ ] [SRS-01/AC-03] Add tests covering verify/audit guidance parity across human-readable and JSON command outputs.
