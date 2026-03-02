---
id: 1vuz9Xrx9
title: Route Story and Voyage Commands Through Unified Enforcer
type: feat
status: done
created_at: 2026-02-24T12:37:07
updated_at: 2026-02-24T15:07:06
scope: 1vuz8K4NM/1vuz8dYT5
index: 2
submitted_at: 2026-02-24T15:00:08
completed_at: 2026-02-24T15:07:06
---

# Route Story and Voyage Commands Through Unified Enforcer

## Summary

Route runtime story and voyage command handlers through the shared transition enforcer so gate behavior is applied consistently.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Update story lifecycle commands to invoke the unified enforcer for transition validation before execution. <!-- verify: manual, SRS-02:start -->
- [x] [SRS-02/AC-02] Update voyage lifecycle commands to invoke the unified enforcer for transition and completion validation. <!-- verify: manual, SRS-02:continues -->
- [x] [SRS-02/AC-03] Remove duplicated command-level blocking checks replaced by enforcer outputs while preserving expected command behavior. <!-- verify: manual, SRS-02:end -->
