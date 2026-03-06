---
source_type: Story
source: stories/1vyGZflfJ/REFLECT.md
scope: 1vyFgR2MA/1vyFn0OuN
source_story_id: 1vyGZflfJ
created_at: 2026-03-05T17:54:24
---

### 1vyKOWjfv: Canonical Scope Contracts Need Explicit Activation Markers

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Introducing canonical scope-lineage validation while existing voyages still contain prose-only scope sections |
| **Insight** | New planning contracts are safest when they activate off explicit markers like `SCOPE-*`; otherwise doctor treats historical prose as invalid structure and turns a targeted validator into migration noise. |
| **Suggested Action** | Define an activation marker whenever a new authored planning contract is introduced, and only enforce the stricter validator once that marker appears in the relevant artifacts. |
| **Applies To** | `src/domain/state_machine/invariants.rs`, `src/cli/commands/diagnostics/doctor/checks/voyages.rs`, PRD and SRS scope sections |
| **Linked Knowledge IDs** | 1vyIA4sQm, 1vyJXGpcM |
| **Observed At** | 2026-03-06T01:56:29+00:00 |
| **Score** | 0.84 |
| **Confidence** | 0.92 |
| **Applied** | yes |
