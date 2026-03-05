---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9Pj97/KNOWLEDGE.md
created_at: 2026-03-02T07:36:50
---

### 1jCvlheVL: Matrix Contracts Need Both Narrative and Table Forms

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Defining a dependency contract that engineers can read quickly and tests can consume with minimal interpretation |
| **Insight** | A single layer table is not enough for review; adding a compact `From \\ To` matrix reduces ambiguity about allowed and forbidden dependencies. |
| **Suggested Action** | Keep layer contracts in two forms: descriptive per-layer rules plus a normalized matrix that can be translated directly into architecture tests. |
| **Applies To** | ARCHITECTURE.md, upcoming architecture contract tests |
| **Linked Knowledge IDs** | 1vyDuw12T |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | yes |
