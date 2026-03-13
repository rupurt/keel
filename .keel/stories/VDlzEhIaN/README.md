---
id: VDlzEhIaN
title: Build Play Theater Command Flag
type: feat
status: done
created_at: 2026-03-13T11:21:52
updated_at: 2026-03-13T11:26:55
operator-signal: 
scope: VDlzCqxr9/VDlzEF2OP
index: 1
started_at: 2026-03-13T11:24:42
completed_at: 2026-03-13T11:26:55
---

# Build Play Theater Command Flag

## Summary

Add `keel play --theater` command surface and session bootstrap so operators can launch the theater flow without changing existing default play behavior.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel play --help` documents `--theater` and `--theme` flags with clear examples. <!-- verify: cargo run --quiet -- play --help, SRS-01:start:end, proof: ac-1.log -->
- [x] [SRS-01/AC-02] `keel play --theater` launches theater mode and renders a startup frame with selected theme and persona. <!-- verify: cargo run --quiet -- play --theater --theme comedy --persona shakespeare, SRS-01:start:end, proof: ac-2.log -->
