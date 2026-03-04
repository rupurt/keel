---
id: 1vxZ0Eoj9
title: Add Canonical Guidance To Play Command Outcomes
type: feat
status: backlog
created_at: 2026-03-03T15:18:10
updated_at: 2026-03-03T15:18:10
scope: 1vxYzSury/1vxYzrwma
---

# Add Canonical Guidance To Play Command Outcomes

## Summary

Add canonical guidance handling to play command outcomes so deterministic results emit one next action while exploratory outputs remain non-prescriptive.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Emit canonical `next_step` guidance for play outcomes that have a deterministic follow-up command.
- [ ] [SRS-01/AC-02] Keep exploratory play outputs non-prescriptive when no deterministic command can be recommended.
- [ ] [SRS-01/AC-03] Add tests that cover both prescriptive and non-prescriptive play outcomes across human and JSON output modes.
