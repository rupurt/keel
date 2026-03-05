---
id: 1vwqCf53S
title: Refactor Main Dispatch To Interface Adapters
type: feat
status: done
created_at: 2026-03-01T15:28:01
updated_at: 2026-03-01T19:27:06
scope: 1vwq96cpt/1vwq9wpT7
index: 1
submitted_at: 2026-03-01T17:04:57
completed_at: 2026-03-01T17:04:58
started_at: 2026-03-01T16:59:58
---

# Refactor Main Dispatch To Interface Adapters

## Summary

Refactor top-level CLI dispatch into thin interface adapters.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Main CLI dispatch and command handlers are rewritten as thin adapters that delegate to application/read-model APIs. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
