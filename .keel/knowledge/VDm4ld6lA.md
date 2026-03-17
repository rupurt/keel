---
source_type: Story
source: stories/VDlzEqbZk/REFLECT.md
scope: VDlzCqxr9/VDlzEF2OP
source_story_id: VDlzEqbZk
created_at: 2026-03-13T11:48:54
---

### VDm4ld6lA: Fail-safes should produce progressive recovery actions

| Field | Value |
|-------|-------|
| **Category** | process |
| **Context** | Command surfaces returning hard errors when a user asks for a missing artifact (e.g., prop/theater data). |
| **Insight** | Hard failures (`exit 1`) are avoidable for recoverable user mistakes; a staged fallback (prompt alternatives, then fallback mode) preserves flow and reduces repeated support friction. |
| **Suggested Action** | For interactive commands, implement recovery branches that keep the previous state, surface actionable options, and only fail on explicit user opt-out. |
| **Applies To** | `src/cli/commands/management/play.rs` and other user-facing error paths |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-13T07:00:00+00:00 |
| **Score** | 0.89 |
| **Confidence** | 0.95 |
| **Applied** | [x] |
