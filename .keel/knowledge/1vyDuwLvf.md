---
source_type: Story
source: stories/1vxqNFHpk/REFLECT.md
scope: 1vxqMtskC/1vxqN5jnA
source_story_id: 1vxqNFHpk
created_at: 2026-03-04T13:05:08
---

### 1vyDuwLvf: Centralized recommendation projection keeps show commands coherent

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple read commands need consistent recommendation output while using different local data sources. |
| **Insight** | A shared recommendation report model plus per-command input extraction avoids drift between epic/voyage/story rendering. |
| **Suggested Action** | Add new recommendation behavior in `verification_techniques` first, then wire each show command through the same renderer helper. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/management/*/show.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-04T12:55:00+00:00 |
| **Score** | 0.85 |
| **Confidence** | 0.93 |
| **Applied** | yes |
