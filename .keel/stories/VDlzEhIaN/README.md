---
id: VDlzEhIaN
title: Build Play Theater Command Flag
type: feat
status: icebox
created_at: 2026-03-13T11:21:52
updated_at: 2026-03-13T11:21:52
operator-signal: 
scope: VDlzCqxr9/VDlzEF2OP
index: 1
---

# Build Play Theater Command Flag

## Summary

Add `keel play --theater` command surface and session bootstrap so operators can launch the theater flow without changing existing default play behavior.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] `keel play --help` documents `--theater` and `--theme` flags with clear examples. <!-- verify: CLI proof -->
- [ ] [SRS-01/AC-02] `keel play --theater` launches theater mode and renders a startup frame with selected theme and persona. <!-- verify: CLI proof -->
