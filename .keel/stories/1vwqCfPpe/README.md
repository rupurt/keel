---
id: 1vwqCfPpe
title: Rewire Command Handlers To Use Cases
type: feat
status: done
created_at: 2026-03-01T15:28:01
updated_at: 2026-03-02T09:47:14
scope: 1vwq96cpt/1vwq9Zf67
index: 4
submitted_at: 2026-03-02T09:33:26
completed_at: 2026-03-02T09:47:14
started_at: 2026-03-02T00:30:43
---

# Rewire Command Handlers To Use Cases

## Summary

Rewire command handlers to use application services and enforcement entrypoints.

## Acceptance Criteria

- [x] [SRS-03/AC-01] Command handlers call application use-case services and no longer orchestrate cross-command workflows directly. <!-- verify: manual, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Transition enforcement policies are invoked through application orchestration paths and covered by service-level tests. <!-- verify: manual, SRS-04:start:end, proof: ac-2.log-->
