---
source_type: Story
source: stories/1vyGZeEI7/REFLECT.md
scope: 1vyFgR2MA/1vyFmfjA9
source_story_id: 1vyGZeEI7
created_at: 2026-03-05T16:57:14
---

### 1vyJXGpcM: Keep Goal Lineage Parsing On One Canonical Path

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Adding PRD goal-lineage rules that must appear consistently in doctor diagnostics and planning show projections |
| **Insight** | Goal table parsing and requirement goal-link parsing need one shared invariant layer; duplicating that logic across read models and doctor checks causes drift between what the validator enforces and what planning surfaces render |
| **Suggested Action** | Route new PRD lineage rules through shared invariant helpers first, then consume those helpers from doctor and show commands instead of introducing parser copies |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/read_model/planning_show.rs`, epic doctor checks |
| **Linked Knowledge IDs** | 1vyH1gD7p |
| **Observed At** | 2026-03-06T00:57:43+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** |  |
