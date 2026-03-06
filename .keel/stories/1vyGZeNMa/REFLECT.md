---
created_at: 2026-03-05T16:32:46
---

# Reflection - Parse Canonical Goal Lineage

## Knowledge

- [1vyH1gD7p](../../knowledge/1vyH1gD7p.md) Preserve Empty Markdown Table Cells In Planning Parsers

## Observations

The canonical goal parser was straightforward once the markdown row handling stopped filtering empty cells. Reusing a shared splitter that trims only boundary pipes kept column indexes stable for both goal rows and requirement rows.

Cutting the PRD fixtures and scaffolds over in the same slice mattered. Leaving the old three-column goal table anywhere in tests or generators would have masked whether the new parser path was actually the only contract in use.
