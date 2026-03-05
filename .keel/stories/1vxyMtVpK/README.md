---
id: 1vxyMtVpK
title: Command And Projection Tests For Parallel Safety
type: feat
status: backlog
created_at: 2026-03-04T18:23:15
updated_at: 2026-03-04T18:25:50
scope: 1vxyM0hvn/1vxyMT6nz
---

# Command And Projection Tests For Parallel Safety

## Summary

Add command-level and projection-level contract tests to keep human/JSON parallel outputs synchronized and deterministic.

## Acceptance Criteria

- [ ] [SRS-06/AC-02] Parallel recommendation order and candidate selection are deterministic across repeated runs for the same state. <!-- verify: cargo test --lib next_parallel_output_is_deterministic, SRS-06:start:end -->
- [ ] [SRS-06/AC-03] Human and JSON projections expose consistent pairwise blocker semantics for selected and blocked candidates. <!-- verify: cargo test --lib next_parallel_pairwise_blockers_render_consistently, SRS-06:start:end -->
