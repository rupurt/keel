---
id: VDlzElxd9
title: Build Theater Session Theme Registry
type: feat
status: icebox
created_at: 2026-03-13T11:21:52
updated_at: 2026-03-13T11:21:52
operator-signal: 
scope: VDlzCqxr9/VDlzEF2OP
index: 2
---

# Build Theater Session Theme Registry

## Summary

Build a local session theme registry and registration model so comedy, drama, and action themes can be configured and selected before running a theater session.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Theme definitions and default registry are represented as structured data with explicit names and fallback theme. <!-- verify: inspection -->
- [ ] [SRS-02/AC-02] `keel play --theater --theme <id>` validates against registry and surfaces supported values on invalid input. <!-- verify: CLI proof -->
