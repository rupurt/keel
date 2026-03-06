---
created_at: 2026-03-05T18:04:07
---

# Reflection - Render Scope Lineage In Planning Surfaces

## Knowledge

- [1vyH1gD7p](../../knowledge/1vyH1gD7p.md) Preserve Empty Markdown Table Cells In Planning Parsers
- [1vyJXGpcM](../../knowledge/1vyJXGpcM.md) Keep Goal Lineage Parsing On One Canonical Path
- [1vyKXvBA1](../../knowledge/1vyKXvBA1.md) Lineage Surfaces Need IDs And Prose Together

## Observations

This slice stayed small because the doctor story already pushed scope parsing and drift classification into shared invariants. Once the read model consumed those helpers directly, the show commands only needed to render projection rows instead of re-deriving planning state.

The main design choice was where to preserve human context. Keeping the raw authored scope bullets, then adding separate lineage rows that pair IDs with disposition context, made the output reviewable without losing the original prose that planners actually wrote.
