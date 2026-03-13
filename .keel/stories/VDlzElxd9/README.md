---
id: VDlzElxd9
title: Build Theater Session Theme Registry
type: feat
status: done
created_at: 2026-03-13T11:21:52
updated_at: 2026-03-13T11:28:09
operator-signal: 
scope: VDlzCqxr9/VDlzEF2OP
index: 2
started_at: 2026-03-13T11:27:09
completed_at: 2026-03-13T11:28:09
---

# Build Theater Session Theme Registry

## Summary

Build a local session theme registry and registration model so comedy, drama, and action themes can be configured and selected before running a theater session.

## Acceptance Criteria

- [x] [SRS-02/AC-01] Theme definitions and default registry are represented as structured data with explicit names and fallback theme. <!-- verify: rg -n "THEATER_THEME_REGISTRY|TheaterTheme" ../src/cli/commands/management/play.rs, SRS-02:start:end, proof: ac-1.log -->
- [x] [SRS-02/AC-02] `keel play --theater --theme <id>` validates against registry and surfaces supported values on invalid input. <!-- verify: bash -c "cd .. && cargo run --quiet -- play --theater --theme opera 2>&1 | grep -q 'Supported themes:'", SRS-02:start:end, proof: ac-2.log -->
