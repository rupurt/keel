---
source_type: Story
source: stories/1vxyMsbOj/REFLECT.md
scope: 1vxyM0hvn/1vxyMT6nz
source_story_id: 1vxyMsbOj
created_at: 2026-03-04T18:52:05
---

### 1vyDuwzyf: Keep Blocker Schema Shared Across Human and JSON Paths

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Rendering pairwise confidence blockers in CLI and machine-readable output |
| **Insight** | A single blocker model (`story_id`, `blocked_by_story_id`, `reasons`, `confidence`) makes it easy to keep human and JSON outputs in sync |
| **Suggested Action** | Build future blocker explanations from the same canonical blocker payload and only vary presentation |
| **Applies To** | `src/cli/commands/management/next.rs`, `src/cli/commands/management/next_support/parallel_threshold.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T02:50:59+00:00 |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |
