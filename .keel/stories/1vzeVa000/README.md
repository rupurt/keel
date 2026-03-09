---
id: 1vzeVa000
title: Mission Refine And Activate
type: feat
status: icebox
created_at: 2026-03-09T10:35:10
updated_at: 2026-03-09T10:35:10
scope: 1vzeJF000/1vzeMq000
index: 10
---

# Mission Refine And Activate

## Summary

Implement `keel mission refine` for iterative CHARTER.md goal elicitation and `keel mission activate` to transition from Defining to Active.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] `keel mission refine <id>` returns next question when CHARTER.md is incomplete <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-02/AC-02] `keel mission refine <id>` returns "ready" signal when CHARTER.md is complete <!-- verify: test --> <!-- SRS-02:start:end -->
- [ ] [SRS-03/AC-01] `keel mission refine <id> --answer "<text>"` records answer into CHARTER.md and returns next question or ready <!-- verify: test --> <!-- SRS-03:start:end -->
- [ ] [SRS-04/AC-01] `keel mission activate <id>` transitions Defining → Active, gated on CHARTER Goals having at least one authored MG-XX row <!-- verify: test --> <!-- SRS-04:start:end -->
