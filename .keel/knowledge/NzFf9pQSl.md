---
source_type: Voyage
source: epics/1vxGy5tco/voyages/1vxGzV3oR/KNOWLEDGE.md
created_at: 2026-03-03T08:10:40
---

### NzFf9pQSl: Keep token inventories and CLI `new` surfaces coupled by drift tests

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Token ownership policy spans templates and command interfaces; either side can drift silently without explicit coupling tests. |
| **Insight** | A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries. |
| **Suggested Action** | When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic. |
| **Applies To** | src/infrastructure/templates.rs, src/drift_tests.rs, src/cli/command_tree.rs |
| **Linked Knowledge IDs** | 1vyDuwGh9 |
| **Observed At** |  |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** |  |
