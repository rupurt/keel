---
source_type: Story
source: stories/1vxH84k3U/REFLECT.md
scope: 1vxGy5tco/1vxGzVpw5
source_story_id: 1vxH84k3U
created_at: 2026-03-02T21:50:30
---

### 1vyDuwBAC: Report Pattern And Severity From One Shared Placeholder Extractor

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple doctor checks were independently detecting TODO/tokens and emitting inconsistent warning messages. |
| **Insight** | A shared unresolved-pattern extractor enables deterministic detection and allows every check to emit the same actionable `pattern: ...` output while enforcing error severity. |
| **Suggested Action** | Route all new scaffold/default-text checks through the shared extractor and assert severity/message structure in unit tests. |
| **Applies To** | `src/infrastructure/validation/structural.rs`, `src/cli/commands/diagnostics/doctor/checks/*.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T05:58:00+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |
