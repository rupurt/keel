---
created_at: 2026-03-05T16:57:14
---

# Reflection - Validate Goal Links In PRD Requirements

## Knowledge

- [1vyH1gD7p](../../knowledge/1vyH1gD7p.md) Preserve Empty Markdown Table Cells In Planning Parsers
- [1vyJXGpcM](../../knowledge/1vyJXGpcM.md) Keep Goal Lineage Parsing On One Canonical Path

## Observations

Collapsing the new goal-lineage parser into `invariants.rs` removed the duplicated `GoalEntry` parsing path in `planning_show.rs` and made the doctor check read from the same contract as the CLI projections.

The repo-level `just keel doctor` failure after the code changes was existing queue drift in `.keel` voyage status, not a regression from the validator work; the story-specific proof commands and the full test suite both passed cleanly.
