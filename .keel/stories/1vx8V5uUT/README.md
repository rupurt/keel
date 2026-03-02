---
id: 1vx8V5uUT
title: Relocate Cli Command Surface Into Src Cli
type: feat
status: done
created_at: 2026-03-02T11:00:15
updated_at: 2026-03-02T11:21:14
scope: 1vwq96cpt/1vx8TLqpp
index: 2
submitted_at: 2026-03-02T11:19:10
completed_at: 2026-03-02T11:21:14
started_at: 2026-03-02T11:09:42
---

# Relocate Cli Command Surface Into Src Cli

## Summary

Move command parsing, command adapter modules, and flow/next terminal presentation
code into `src/cli/**` so the physical layout matches the DDD interface boundary.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `main.rs` dispatches through `src/cli/**` and active command adapter modules live under `src/cli/**`. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log -->
- [x] [SRS-01/AC-02] Legacy top-level CLI families (`src/commands/**`, `src/flow/**`, `src/next/**`) are removed from active module declarations. <!-- verify: manual, SRS-01:continues, proof: ac-2.log -->
- [x] [SRS-04/AC-01] Root module declarations expose `cli` as the interface entrypoint layer without regressing command behavior. <!-- verify: manual, SRS-04:start:end, proof: ac-3.log -->
- [x] [SRS-04/AC-02] `just test` remains green after CLI relocation. <!-- verify: manual, SRS-04:end, proof: ac-4.log -->
