---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9rycE/KNOWLEDGE.md
created_at: 2026-03-02T10:34:57
---

### IxTCT6ayY: Keep Operational Metrics In A Single Read Model

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Consolidating repeated queue/flow/status metrics used across diagnostics and next-decision logic |
| **Insight** | A canonical projection DTO that embeds both flow metrics and status metrics removes drift and lets adapters format output without recalculating business metrics |
| **Suggested Action** | Add read-model projection services first, then migrate every consumer to the projection API before deleting local metric structs |
| **Applies To** | src/read_model/flow_status.rs; src/commands/diagnostics/{flow,status}.rs; src/next/algorithm.rs |
| **Linked Knowledge IDs** | 1vyDuwXBN |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.89 |
| **Applied** | 1vwqCfS0F |
