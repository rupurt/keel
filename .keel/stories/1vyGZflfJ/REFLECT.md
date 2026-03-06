---
created_at: 2026-03-05T17:54:24
---

# Reflection - Detect Scope Drift During Planning

## Knowledge

- [1vyIA4sQm](../../knowledge/1vyIA4sQm.md) Scope Planning Diagnostics To Transition-Relevant Voyages
- [1vyJXGpcM](../../knowledge/1vyJXGpcM.md) Keep Goal Lineage Parsing On One Canonical Path
- [1vyKOWjfv](../../knowledge/1vyKOWjfv.md) Canonical Scope Contracts Need Explicit Activation Markers

## Observations

The scope-lineage evaluator fit cleanly into `invariants.rs` once the PRD and SRS parsing paths were normalized, so the doctor check could consume one canonical result instead of growing a second interpretation layer.

The non-obvious part was rollout. Enforcing the new validator across every existing voyage immediately converted ordinary prose-only scope sections into failures. Treating canonical `SCOPE-*` markers as the activation signal kept the feature aligned with hard-cutover semantics for newly authored contracts without turning this story into a repo-wide migration pass.
