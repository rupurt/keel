---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzVpw5/KNOWLEDGE.md
created_at: 2026-03-03T11:50:46
---

### uvyKHWaQn: Stage-gate scaffold checks to avoid noisy early warnings

| Field | Value |
|-------|-------|
| **Category** | code |
| **Context** | Adding scaffold/default text diagnostics to doctor checks |
| **Insight** | Stage filtering is critical: terminal-only checks avoid penalizing in-progress drafting while still hard-failing review-complete states. |
| **Suggested Action** | Reuse a shared unresolved-pattern detector and explicitly gate by story stage (`needs-human-verification`, `done`) for terminal coherency rules. |
| **Applies To** | src/cli/commands/diagnostics/doctor/checks/stories.rs, src/infrastructure/validation/structural.rs |
| **Linked Knowledge IDs** | 1vyDuwdbL |
| **Observed At** |  |
| **Score** | 0.85 |
| **Confidence** | 0.91 |
| **Applied** | yes |
