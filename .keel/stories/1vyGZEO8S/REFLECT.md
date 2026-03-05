---
created_at: 2026-03-05T15:54:30
---

# Reflection - Gate Voyage Planning On PRD Lineage

## Knowledge

- [1vyH1gD7p](../../knowledge/1vyH1gD7p.md) Preserve Empty Markdown Table Cells In Planning Parsers

## Observations

The critical bug was not in lineage validation itself but in markdown-table parsing: dropping empty cells makes `Verification` slide into the `Source` slot and hides missing-source errors. For contract-style planning docs, table parsing has to preserve interior empty columns before any semantic validation runs.
