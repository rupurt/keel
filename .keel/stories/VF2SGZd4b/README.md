---
# system-managed
id: VF2SGZd4b
status: done
created_at: 2026-03-26T21:26:25
updated_at: 2026-03-26T21:32:14
# authored
title: Document Downstream Project Engine Contracts
type: feat
operator-signal:
scope: VF2RJfiKo/VF2RKxjt7
index: 1
started_at: 2026-03-26T21:27:29
submitted_at: 2026-03-26T21:32:11
completed_at: 2026-03-26T21:32:14
---

# Document Downstream Project Engine Contracts

## Summary

Document how downstream repositories use `AGENTS.md` and `INSTRUCTIONS.md` to make Keel the active project-management engine, and use `port` to show the concrete seams between upstream canonical guidance and local adaptation.

## Acceptance Criteria

- [x] [SRS-01/AC-01] A workflow page explains how `AGENTS.md` and `INSTRUCTIONS.md` act as the downstream operating contract when a repository adopts Keel as its project-management engine. <!-- verify: manual, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] The page uses `port` to show what remains canonical from upstream Keel and what gets adapted inside a downstream project. <!-- verify: manual, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] The guidance stays vendor-neutral by focusing on repository contracts, command surfaces, and operating patterns rather than any single harness provider. <!-- verify: manual, SRS-NFR-02:start:end, proof: ac-3.log-->
