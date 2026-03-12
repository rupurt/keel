---
id: VDcFgsiMj
title: Routine Due-State Engine
type: feat
status: backlog
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T19:29:44
operator-signal: 
scope: VDakmCGYi/VDcFd5kmn
index: 1
---

# Routine Due-State Engine

## Summary

Introduce the routine timing evaluator that turns cadence metadata into
deterministic due-state for the pull loop.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Routine cadence metadata resolves to due/not-due state plus next eligible time. <!-- verify: test, SRS-01:start:end -->
- [ ] [SRS-NFR-01/AC-01] Due-state evaluation uses injected or reference time so identical inputs are deterministic. <!-- verify: test, SRS-NFR-01:start:end -->
