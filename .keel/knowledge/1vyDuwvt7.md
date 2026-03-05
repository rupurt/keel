---
source_type: Story
source: stories/1vwqCfdUl/REFLECT.md
scope: 1vwq96cpt/1vwq9wpT7
source_story_id: 1vwqCfdUl
created_at: 2026-03-02T08:54:40
---

### 1vyDuwvt7: Production-only import checks reduce false positives

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Architecture contract tests scanning modules that also contain `#[cfg(test)]` helper imports |
| **Insight** | Import-boundary checks should target production sections to avoid test-only imports triggering invalid architectural failures. |
| **Suggested Action** | Split source at `#[cfg(test)]` and enforce forbidden-edge patterns only on production content for adapter boundary tests. |
| **Applies To** | `src/architecture_contract_tests.rs`, `src/commands/diagnostics/*.rs`, `src/main.rs`, `src/next/algorithm.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-02T16:48:53+00:00 |
| **Score** | 0.87 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfdUl` |
