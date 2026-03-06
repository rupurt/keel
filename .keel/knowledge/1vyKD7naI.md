---
source_type: Story
source: stories/1vyGZet0Z/REFLECT.md
scope: 1vyFgR2MA/1vyFlAgHB
source_story_id: 1vyGZet0Z
created_at: 2026-03-05T17:42:28
---

### 1vyKD7naI: Keep scaffold templates compliant with day-zero doctor contracts

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Changing epic creation inputs made freshly generated PRDs fail doctor even though the validator logic was correct. |
| **Insight** | If newly scaffolded planning artifacts are expected to be immediately doctor-clean, the fix belongs in the template seed content rather than in weaker validation rules. |
| **Suggested Action** | When creation inputs or placeholder semantics change, regenerate a fresh artifact in tests and run doctor against it before changing any diagnostic gates. |
| **Applies To** | `templates/epic/[name]/PRD.md`, `src/cli/commands/management/epic/new.rs`, doctor scaffold checks |
| **Linked Knowledge IDs** | 1vyDuwBAC |
| **Observed At** | 2026-03-06T01:42:41+00:00 |
| **Score** | 0.87 |
| **Confidence** | 0.95 |
| **Applied** | yes |
