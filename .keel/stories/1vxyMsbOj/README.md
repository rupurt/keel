---
id: 1vxyMsbOj
title: Pairwise Blocker Rendering For Parallel Next
type: feat
status: backlog
created_at: 2026-03-04T18:23:14
updated_at: 2026-03-04T18:25:50
scope: 1vxyM0hvn/1vxyMT6nz
---

# Pairwise Blocker Rendering For Parallel Next

## Summary

Render pairwise blocker explanations so operators can see exactly which story pairs are blocked and why.

## Acceptance Criteria

- [ ] [SRS-04/AC-01] Human output displays pairwise blocker entries with `story -> blocked_by` and concrete reasons. <!-- verify: cargo test --lib next_parallel_pairwise_blockers_render_human, SRS-04:start:end -->
- [ ] [SRS-04/AC-02] JSON output includes the same pairwise blocker semantics with stable field names. <!-- verify: cargo test --lib next_parallel_pairwise_blockers_render_json, SRS-04:start:end -->
