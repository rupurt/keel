---
source_type: Voyage
source: epics/1vxyM0hvn/voyages/1vxyMT6nz/KNOWLEDGE.md
created_at: 2026-03-04T19:11:59
---

### 4DVX5dewJ: Keep Blocker Schema Shared Across Human and JSON Paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering pairwise confidence blockers in CLI and machine-readable output |
| **Insight** | A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync |
| **Suggested Action** | Build future blocker explanations from the same canonical blocker payload and only vary presentation |
| **Applies To** | `src/cli/commands/management/next.rs`, `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Linked Knowledge IDs** | 1vyDuwzyf |
| **Observed At** |  |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |
