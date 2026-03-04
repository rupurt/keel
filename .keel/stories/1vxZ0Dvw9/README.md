---
id: 1vxZ0Dvw9
title: Keep Informational Governance Commands Non Prescriptive
type: feat
status: backlog
created_at: 2026-03-03T15:18:09
updated_at: 2026-03-03T15:18:09
scope: 1vxYzSury/1vxYzjVMv
---

# Keep Informational Governance Commands Non Prescriptive

## Summary

Ensure informational governance commands remain non-prescriptive by omitting canonical next-step guidance when no deterministic action is required.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Identify governance read-only commands (for example `adr list/show` and `bearing list/show`) and ensure they do not emit canonical guidance.
- [ ] [SRS-01/AC-02] Preserve prescriptive guidance behavior for actionable governance transitions while keeping informational outputs non-prescriptive.
- [ ] [SRS-01/AC-03] Add regression tests asserting informational command outputs omit guidance fields in JSON and avoid imperative next-step text in human output.
