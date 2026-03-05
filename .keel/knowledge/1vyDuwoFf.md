---
source_type: Story
source: stories/1vxH84M8t/REFLECT.md
scope: 1vxGy5tco/1vxGzVpw5
source_story_id: 1vxH84M8t
---

### 1vyDuwoFf: Reuse the structural placeholder detector in runtime gates

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story submit/accept transitions need coherence checks aligned with doctor enforcement. |
| **Insight** | Reusing `first_unfilled_placeholder_pattern` keeps runtime and doctor behavior consistent while avoiding duplicate marker logic. |
| **Suggested Action** | Add lifecycle gate checks by composing existing structural validators before adding new regex or scanners. |
| **Applies To** | src/domain/state_machine/gating.rs, src/infrastructure/validation/structural.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T18:05:00+00:00 |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** |  |
