---
source_type: Story
source: stories/1vxH84nTQ/REFLECT.md
scope: 1vxGy5tco/1vxGzVpw5
source_story_id: 1vxH84nTQ
---

### 1vyDuwdbL: Stage-gate scaffold checks to avoid noisy early warnings

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding scaffold/default text diagnostics to doctor checks |
| **Insight** | Stage filtering is critical: terminal-only checks avoid penalizing in-progress drafting while still hard-failing review-complete states. |
| **Suggested Action** | Reuse a shared unresolved-pattern detector and explicitly gate by story stage (`needs-human-verification`, `done`) for terminal coherency rules. |
| **Applies To** | src/cli/commands/diagnostics/doctor/checks/stories.rs, src/infrastructure/validation/structural.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T17:13:33+00:00 |
| **Score** | 0.85 |
| **Confidence** | 0.91 |
| **Applied** | yes |
