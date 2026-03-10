---
id: VDUGflIUh
title: Update Next Role Routing
type: feat
status: icebox
created_at: 2026-03-10T10:38:13
updated_at: 2026-03-10T10:38:13
scope: VDTpFlMKc/VDUG60pcX
index: 3
---

# Update Next Role Routing

## Summary

Update `keel next` to route based on `--role` instead of `--agent`/`--human`.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `keel next` accepts `--role <TAXONOMY>` <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-02/AC-02] `--agent` and `--human` are removed or error gracefully (conflict) <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-03/AC-01] `manager/*` role maps to Management queue decisions <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-03/AC-02] `engineer/*` role maps to Execution queue work <!-- verify: test --> <!-- SRS-03:start:end -->
