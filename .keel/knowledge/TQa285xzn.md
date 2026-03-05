---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxqN5jnA/KNOWLEDGE.md
scope: null
source_story_id: null
---

### TQa285xzn: Centralized recommendation projection keeps show commands coherent

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple read commands need consistent recommendation output while using different local data sources. |
| **Insight** | A shared recommendation report model plus per-command input extraction avoids drift between epic/voyage/story rendering. |
| **Suggested Action** | Add new recommendation behavior in `verification_techniques` first, then wire each show command through the same renderer helper. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/management/*/show.rs` |
| **Linked Knowledge IDs** | 1vyDuwLvf |
| **Observed At** |  |
| **Score** | 0.85 |
| **Confidence** | 0.93 |
| **Applied** | yes |
