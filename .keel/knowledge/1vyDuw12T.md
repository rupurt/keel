---
source_type: Story
source: stories/1vwqCd6wg/REFLECT.md
scope: 1vwq96cpt/1vwq9Pj97
source_story_id: 1vwqCd6wg
created_at: 2026-03-01T17:12:38
---

### 1vyDuw12T: Matrix Contracts Need Both Narrative and Table Forms

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a dependency contract that engineers can read quickly and tests can consume with minimal interpretation |
| **Insight** | A single layer table is not enough for review; adding a compact `From \\ To` matrix reduces ambiguity about allowed and forbidden dependencies. |
| **Suggested Action** | Keep layer contracts in two forms: descriptive per-layer rules plus a normalized matrix that can be translated directly into architecture tests. |
| **Applies To** | ARCHITECTURE.md, upcoming architecture contract tests |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T01:11:07+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | yes |
