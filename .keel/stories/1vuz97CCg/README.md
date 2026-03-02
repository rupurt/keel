---
id: 1vuz97CCg
title: Update Architecture and Command Docs for Queue Policy
type: feat
status: done
created_at: 2026-02-24T12:36:41
updated_at: 2026-02-24T14:48:19
scope: 1vuz8K4NM/1vuz8VYmc
index: 5
submitted_at: 2026-02-24T14:46:59
completed_at: 2026-02-24T14:48:19
---

# Update Architecture and Command Docs for Queue Policy

## Summary

Update architecture and command documentation so queue policy thresholds, derivation order, and human/agent boundaries are explicitly consistent with implemented behavior.

## Acceptance Criteria

- [x] [SRS-05/AC-01] Update `ARCHITECTURE.md` sections for 2-queue flow and system-state derivation to match canonical policy values. <!-- verify: manual, SRS-05:start -->
- [x] [SRS-05/AC-02] Update command/help documentation to reflect human-mode queue boundaries and policy-driven decision behavior. <!-- verify: manual, SRS-05:continues -->
- [x] [SRS-05/AC-03] Add or update documentation consistency checks/tests that prevent threshold and terminology drift. <!-- verify: manual, SRS-05:end -->
