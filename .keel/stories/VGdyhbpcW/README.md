---
# system-managed
id: VGdyhbpcW
status: backlog
created_at: 2026-04-12T21:56:12
updated_at: 2026-04-12T21:57:09
# authored
title: Enforce Managed Foreign Worktree Lifecycle For Stack Execution
type: feat
operator-signal:
scope: VGdxE0lFe/VGdyGssEu
index: 1
---

# Enforce Managed Foreign Worktree Lifecycle For Stack Execution

## Summary

Define the first foreign-reactor isolation contract: outside execution in
another member repo must use a managed git worktree on the stack branch, validate
ownership before work starts, and clean up or report leftovers when the stack
closes.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] The lifecycle requires foreign reactor execution to happen in a managed worktree rather than the member repo's primary checkout. <!-- verify: manual, SRS-01:start:end -->
- [ ] [SRS-02/AC-01] The lifecycle validates `stack/<id>` branch or approved stack-derived head state before foreign execution begins. <!-- verify: manual, SRS-02:start:end -->
- [ ] [SRS-03/AC-01] The lifecycle defines create, reuse, and inspection behavior for managed foreign worktrees while a stack is open. <!-- verify: manual, SRS-03:start:end -->
- [ ] [SRS-04/AC-01] The lifecycle defines stack-close garbage collection and fail-safe reporting for ambiguous leftovers. <!-- verify: manual, SRS-04:start:end -->
- [ ] [SRS-05/AC-01] The lifecycle names the command and hook enforcement points that reject unsupported foreign execution. <!-- verify: manual, SRS-05:start:end -->
- [ ] [SRS-NFR-01/AC-01] Managed worktree operations avoid perturbing the member repo's primary checkout. <!-- verify: manual, SRS-NFR-01:start:end -->
