---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzV3oR/KNOWLEDGE.md
created_at: 2026-03-03T08:10:40
---

### Gd2KnPbv1: Keep Token Names Equal To Frontmatter Keys

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Template scaffolds and their renderer replacement maps drift when placeholder names are generic (`date`, `datetime`) instead of schema names. |
| **Insight** | Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward. |
| **Suggested Action** | For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases. |
| **Applies To** | `templates/**`, `src/cli/commands/management/*/new.rs`, `src/infrastructure/templates.rs` |
| **Linked Knowledge IDs** | 1vyDuwE2r |
| **Observed At** |  |
| **Score** | 0.82 |
| **Confidence** | 0.89 |
| **Applied** | yes |
