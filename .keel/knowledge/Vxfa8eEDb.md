---
source_type: Voyage
source: epics/1vxYzSury/voyages/1vxqEChvp/KNOWLEDGE.md
created_at: 2026-03-04T10:27:26
---

### Vxfa8eEDb: Intake Stage Must Be Canonical In Creation Code

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story creation had stage branching by voyage state |
| **Insight** | Canonical defaults are safest when enforced once in creation code and not left to template/state-condition combinations. |
| **Suggested Action** | Keep `story new` stage assignment unconditional and test it across unscoped/draft/planned scopes. |
| **Applies To** | `src/cli/commands/management/story/new.rs`, story template/frontmatter defaults |
| **Linked Knowledge IDs** | 1vyDuwoz6 |
| **Observed At** |  |
| **Score** | 0.80 |
| **Confidence** | 0.90 |
| **Applied** | yes |
