---
created_at: 2026-03-10T16:03:58
---

# Reflection - Add Workflow Topology Config Model

## Knowledge

- [123abcDEF](../../knowledge/123abcDEF.md) Existing knowledge title
- [1vyDuwl5B](../../knowledge/1vyDuwl5B.md) Canonical read models remove adapter drift

## Observations

Centralizing seeded defaults and selector compilation in `read_model::workflow_topology`
kept config parsing and `keel config show` on one effective topology surface. That
should make the next routing, flow, and doctor slices safer because they can consume
the same resolved model instead of reimplementing lane logic.
