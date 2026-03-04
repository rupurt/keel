---
id: 1vxZ0C7OF
title: Add Canonical Guidance To ADR Transition Commands
type: feat
status: backlog
created_at: 2026-03-03T15:18:08
updated_at: 2026-03-03T15:18:09
scope: 1vxYzSury/1vxYzjVMv
---

# Add Canonical Guidance To ADR Transition Commands

## Summary

Add canonical guidance output to ADR lifecycle transitions so successful and recoverable outcomes expose one deterministic follow-up command.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Add canonical `next_step` guidance to actionable ADR transition outputs (`accept`, `reject`, `deprecate`, `supersede`) where a deterministic follow-up exists.
- [ ] [SRS-01/AC-02] Emit canonical `recovery_step` guidance for ADR transition failures that require explicit user remediation.
- [ ] [SRS-01/AC-03] Add tests covering ADR command output guidance in both human-readable and JSON modes.
