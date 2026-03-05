---
source_type: Story
source: stories/1vxH83JcY/REFLECT.md
scope: 1vxGy5tco/1vxGzV3oR
source_story_id: 1vxH83JcY
---

### 1vyDuwE2r: Keep Token Names Equal To Frontmatter Keys

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Template scaffolds and their renderer replacement maps drift when placeholder names are generic (`date`, `datetime`) instead of schema names. |
| **Insight** | Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward. |
| **Suggested Action** | For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases. |
| **Applies To** | `templates/**`, `src/cli/commands/management/*/new.rs`, `src/infrastructure/templates.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T05:45:00+00:00 |
| **Score** | 0.82 |
| **Confidence** | 0.89 |
| **Applied** | yes |
