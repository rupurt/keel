---
id: 1vxZ0FtD2
title: Document Command Guidance Contract For Harness Consumers
type: feat
status: done
created_at: 2026-03-03T15:18:11
updated_at: 2026-03-03T20:49:39
scope: 1vxYzSury/1vxYzsAxT
started_at: 2026-03-03T20:43:35
completed_at: 2026-03-03T20:49:39
---

# Document Command Guidance Contract For Harness Consumers

## Summary

Document the canonical command guidance contract for harness consumers so automation can reliably interpret actionable and recovery recommendations.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Document the canonical guidance schema (`next_step`, `recovery_step`, and command string semantics) in CLI-facing documentation. <!-- verify: rg --line-number -e guidance.next_step.command -e guidance.recovery_step.command ../README.md, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Document classification semantics for actionable versus informational commands and the single canonical next-step rule. <!-- verify: rg --line-number -e Actionable: -e Informational: -e next-step ../README.md, SRS-01:continues, proof: ac-2.log-->
- [x] [SRS-01/AC-03] Provide examples for success, blocked recovery, and informational no-guidance cases that harnesses can consume directly. <!-- verify: rg --line-number -e no-action-required -e recovery_step -e 1vxZ0FtD2 ../README.md, SRS-01:end, proof: ac-3.log-->
