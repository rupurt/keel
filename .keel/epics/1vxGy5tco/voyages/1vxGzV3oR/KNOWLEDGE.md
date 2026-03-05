---
created_at: 2026-03-03T08:10:40
---

# Knowledge - 1vxGzV3oR

> Automated synthesis of story reflections.

## Story Knowledge

## Story: Codify Token Bucket Contract Tests (1vxH84K5a)

### 1vyDuwGh9: Keep token inventories and CLI `new` surfaces coupled by drift tests

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Token ownership policy spans templates and command interfaces; either side can drift silently without explicit coupling tests. |
| **Insight** | A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries. |
| **Suggested Action** | When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic. |
| **Applies To** | src/infrastructure/templates.rs, src/drift_tests.rs, src/cli/command_tree.rs |
| **Applied** |  |



---

## Story: Extend Adr Creation Inputs For Context Ownership (1vxH84Xh8)

### 1vyDuwxcg: Keep CLI parser, runtime mapping, and template tokens aligned

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Adding new `adr new` flags required updates across clap action enums, command tree wiring, runtime ArgMatches extraction, and template rendering inputs. |
| **Insight** | Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set. |
| **Suggested Action** | For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior. |
| **Applies To** | src/cli/commands/management/adr/mod.rs, src/cli/runtime.rs, src/cli/command_tree.rs, templates/adrs/ADR.md |
| **Applied** |  |



---

## Story: Canonicalize Template Tokens To Schema Names (1vxH83JcY)

### 1vyDuwE2r: Keep Token Names Equal To Frontmatter Keys

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Template scaffolds and their renderer replacement maps drift when placeholder names are generic (`date`, `datetime`) instead of schema names. |
| **Insight** | Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward. |
| **Suggested Action** | For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases. |
| **Applies To** | `templates/**`, `src/cli/commands/management/*/new.rs`, `src/infrastructure/templates.rs` |
| **Applied** | yes |



---

## Story: Align CLI Contracts For Creation Commands (1vxH83MOO)

### 1vyDuwuNj: Keep CLI contract updates end-to-end

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | When changing creation command flags and required inputs |
| **Insight** | Command tree flags, runtime mappers, and user-facing suggestion strings drift unless updated in the same slice. |
| **Suggested Action** | Pair every CLI contract edit with parser rejection tests for removed flags and updates to generated command hints. |
| **Applies To** | src/cli/command_tree.rs, src/cli/runtime.rs, src/cli_tests.rs, src/cli/presentation/flow/next_up.rs |
| **Applied** | yes |



---

## Synthesis

### NzFf9pQSl: Keep token inventories and CLI `new` surfaces coupled by drift tests

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Token ownership policy spans templates and command interfaces; either side can drift silently without explicit coupling tests. |
| **Insight** | A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries. |
| **Suggested Action** | When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic. |
| **Applies To** | src/infrastructure/templates.rs, src/drift_tests.rs, src/cli/command_tree.rs |
| **Linked Knowledge IDs** | 1vyDuwGh9 |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** |  |

### Mb1Dyko1K: Keep CLI parser, runtime mapping, and template tokens aligned

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Adding new `adr new` flags required updates across clap action enums, command tree wiring, runtime ArgMatches extraction, and template rendering inputs. |
| **Insight** | Parser and persistence changes stay reliable when command tests cover both parse-time option capture and file-level frontmatter serialization in one change set. |
| **Suggested Action** | For every new CLI flag, add tests for command parsing and persisted artifact output before changing runtime behavior. |
| **Applies To** | src/cli/commands/management/adr/mod.rs, src/cli/runtime.rs, src/cli/command_tree.rs, templates/adrs/ADR.md |
| **Linked Knowledge IDs** | 1vyDuwxcg |
| **Score** | 0.82 |
| **Confidence** | 0.91 |
| **Applied** |  |

### Gd2KnPbv1: Keep Token Names Equal To Frontmatter Keys

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Template scaffolds and their renderer replacement maps drift when placeholder names are generic (`date`, `datetime`) instead of schema names. |
| **Insight** | Canonical token names that mirror frontmatter fields (`created_at`, `updated_at`) remove ambiguity and make drift detection/test assertions straightforward. |
| **Suggested Action** | For any new template token, require a matching model/frontmatter key name (or explicit documented exception) and add a regression guard against legacy aliases. |
| **Applies To** | `templates/**`, `src/cli/commands/management/*/new.rs`, `src/infrastructure/templates.rs` |
| **Linked Knowledge IDs** | 1vyDuwE2r |
| **Score** | 0.82 |
| **Confidence** | 0.89 |
| **Applied** | yes |

### 60OXnfaXF: Keep CLI contract updates end-to-end

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | When changing creation command flags and required inputs |
| **Insight** | Command tree flags, runtime mappers, and user-facing suggestion strings drift unless updated in the same slice. |
| **Suggested Action** | Pair every CLI contract edit with parser rejection tests for removed flags and updates to generated command hints. |
| **Applies To** | src/cli/command_tree.rs, src/cli/runtime.rs, src/cli_tests.rs, src/cli/presentation/flow/next_up.rs |
| **Linked Knowledge IDs** | 1vyDuwuNj |
| **Score** | 0.82 |
| **Confidence** | 0.90 |
| **Applied** | yes |

