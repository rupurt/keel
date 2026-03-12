---
id: VDcFgtsLv
title: Pulse Command Surface
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:45
operator-signal: 
scope: VDakmG8cH/VDcFd5Sop
index: 2
---

# Pulse Command Surface

## Summary

Provide the non-interactive `keel pulse` entry point and the first automation
cycle summary contract for schedulers and operators.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `keel pulse` runs one automation cycle without interactive prompts. <!-- verify: test, SRS-01:start -->
- [ ] [SRS-01/AC-02] Pulse output reports evaluated, triggered, and skipped routines for the cycle. <!-- verify: test, SRS-01:end -->
- [ ] [SRS-NFR-02/AC-01] Pulse emits structured and human-readable output suitable for scheduler logs and regression checks. <!-- verify: test, SRS-NFR-02:start -->
