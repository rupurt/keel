---
created_at: 2026-03-05T17:29:24
---

# Reflection - Parse Canonical Scope Lineage

## Knowledge

- [1vyJXGpcM](../../knowledge/1vyJXGpcM.md) Keep Goal Lineage Parsing On One Canonical Path

## Observations

Keeping the new scope parser in `domain::state_machine::invariants` avoided another split between validation rules and future planning surfaces. The stable contract is to parse one canonical `SCOPE-*` namespace plus section-specific disposition (`in` vs `out`) while preserving the authored prose after the ID instead of trying to recover lineage from free-form text later.
