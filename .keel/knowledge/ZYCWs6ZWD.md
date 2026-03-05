---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzVpw5/KNOWLEDGE.md
scope: null
source_story_id: null
---

### ZYCWs6ZWD: Report Pattern And Severity From One Shared Placeholder Extractor

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Multiple doctor checks were independently detecting TODO/tokens and emitting inconsistent warning messages. |
| **Insight** | A shared unresolved-pattern extractor enables deterministic detection and allows every check to emit the same actionable `pattern: ...` output while enforcing error severity. |
| **Suggested Action** | Route all new scaffold/default-text checks through the shared extractor and assert severity/message structure in unit tests. |
| **Applies To** | `src/infrastructure/validation/structural.rs`, `src/cli/commands/diagnostics/doctor/checks/*.rs` |
| **Linked Knowledge IDs** | 1vyDuwBAC |
| **Observed At** |  |
| **Score** | 0.84 |
| **Confidence** | 0.91 |
| **Applied** | yes |
