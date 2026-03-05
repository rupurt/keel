---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vx8TLqpp/KNOWLEDGE.md
created_at: 2026-03-02T12:03:53
---

### Pa8P1V8dA: Relocated Source Files May Break Compile-Time Template Paths

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Moving `templates.rs` under `src/infrastructure` changed relative path depth for `include_str!` macro calls. |
| **Insight** | Compile-time embedded asset paths are location-sensitive; module moves must include immediate path rebasing for `include_str!` constants. |
| **Suggested Action** | After moving infrastructure modules, run a targeted compile/test immediately and patch relative asset paths before broader refactors. |
| **Applies To** | src/infrastructure/templates.rs |
| **Linked Knowledge IDs** | 1vyDuwGDS |
| **Observed At** |  |
| **Score** | 0.86 |
| **Confidence** | 0.96 |
| **Applied** |  |
