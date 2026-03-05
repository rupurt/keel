---
source_type: Voyage
source: epics/1vxYzSury/voyages/1vxpomgnN/KNOWLEDGE.md
created_at: 2026-03-04T11:56:17
---

### F2f7FWwpy: Voyage Requirement Views Need Both AC And Verify Mapping

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Building requirement-level voyage progress from story artifacts |
| **Insight** | Requirement linkage should combine AC references and verify requirement IDs; relying on one source undercounts coverage/verification state. |
| **Suggested Action** | Build requirement matrices from both marker channels, then deterministically sort rows and linked stories. |
| **Applies To** | `src/cli/commands/management/voyage/show.rs`, planning-read projections |
| **Linked Knowledge IDs** | 1vyDuwuY1 |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | yes |
