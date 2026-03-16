---
id: VE23JvauZ
title: Populate PRD Risks from Brief Open Questions
type: feat
status: backlog
created_at: 2026-03-16T05:18:26
updated_at: 2026-03-16T05:19:29
operator-signal:
scope: VDiHwPniZ/VE22wWzeD
index: 2
blocked_by:
  - VE23HpS3U
---

# Populate PRD Risks from Brief Open Questions

## Summary

Extend `create_prd_from_bearing` to extract open questions from BRIEF.md and populate the PRD Open Questions & Risks table with them as rows. When no open questions exist, the existing boilerplate risk row is used.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Generated PRD Open Questions & Risks table contains rows from BRIEF.md open questions. <!-- verify: cargo test -p keel bearing_lay_prd_includes_brief_open_questions, SRS-02:start:end -->
- [ ] [SRS-04/AC-01] Generated PRD falls back to boilerplate risk row when BRIEF.md has no open questions. <!-- verify: cargo test -p keel bearing_lay_prd_falls_back_without_open_questions, SRS-04:start:end -->
