---
source_type: Voyage
source: epics/1vwq96cpt/voyages/1vwq9wpT7/KNOWLEDGE.md
scope: null
source_story_id: null
---

### erV4YlxVN: Production-only import checks reduce false positives

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Architecture contract tests scanning modules that also contain `#[cfg(test)]` helper imports |
| **Insight** | Import-boundary checks should target production sections to avoid test-only imports triggering invalid architectural failures. |
| **Suggested Action** | Split source at `#[cfg(test)]` and enforce forbidden-edge patterns only on production content for adapter boundary tests. |
| **Applies To** | `src/architecture_contract_tests.rs`, `src/commands/diagnostics/*.rs`, `src/main.rs`, `src/next/algorithm.rs` |
| **Linked Knowledge IDs** | 1vyDuwvt7 |
| **Observed At** |  |
| **Score** | 0.87 |
| **Confidence** | 0.95 |
| **Applied** | story `1vwqCfdUl` |
