# Reflection - Canonicalize Template Tokens To Schema Names

## Knowledge

### 1vyDuwE2r: Keep Token Names Equal To Frontmatter Keys

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Template scaffolds and their renderer replacement maps drift when placeholder names are generic (`date`, `datetime`) instead of schema names. |
| **Insight** | Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward. |
| **Suggested Action** | For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases. |
| **Applies To** | `templates/**`, `src/cli/commands/management/*/new.rs`, `src/infrastructure/templates.rs` |
| **Observed At** | 2026-03-03T05:45:00Z |
| **Score** | 0.82 |
| **Confidence** | 0.89 |
| **Applied** | yes |

## Observations

- Updating template files alone is insufficient; callsite replacement maps and embedded-template tests must move in lockstep.
- A direct regression test on legacy placeholder strings (`date`, `datetime`) provided a clear hard-cutover guardrail.
