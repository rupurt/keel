---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzVpw5/KNOWLEDGE.md
created_at: 2026-03-03T11:50:46
---

### W1jACJhp8: Reuse the structural placeholder detector in runtime gates

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Story submit/accept transitions need coherence checks aligned with doctor enforcement. |
| **Insight** | Reusing `first_unfilled_placeholder_pattern` keeps runtime and doctor behavior consistent while avoiding duplicate marker logic. |
| **Suggested Action** | Add lifecycle gate checks by composing existing structural validators before adding new regex or scanners. |
| **Applies To** | src/domain/state_machine/gating.rs, src/infrastructure/validation/structural.rs |
| **Linked Knowledge IDs** | 1vyDuwoFf |
| **Observed At** |  |
| **Score** | 0.86 |
| **Confidence** | 0.93 |
| **Applied** |  |
