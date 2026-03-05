---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vx8TLqpp/KNOWLEDGE.md
created_at: 2026-03-02T12:03:53
---

### Q8pKzHiNH: Path-Wide Module Moves Need Import Rewrite First

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Relocating top-level module families (`commands`, `flow`, `next`) to a new root (`cli`) while preserving behavior. |
| **Insight** | Bulk file moves are low-risk only when import rewrites and architecture path fixtures are updated in the same slice; otherwise compile passes but contract tests drift. |
| **Suggested Action** | For physical normalization stories, perform move + import rewrite + fixture path updates atomically before running full test and doctor checks. |
| **Applies To** | src/main.rs, src/cli/**, src/architecture_contract_tests.rs |
| **Linked Knowledge IDs** | 1vyDuwLDX |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** |  |
