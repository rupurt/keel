---
source_type: Story
source: stories/1vxH84K5a/REFLECT.md
scope: 1vxGy5tco/1vxGzV3oR
source_story_id: 1vxH84K5a
---

### 1vyDuwGh9: Keep token inventories and CLI `new` surfaces coupled by drift tests

| Field | Value |
|-------|-------|
| **Category** | testing |
| **Context** | Token ownership policy spans templates and command interfaces; either side can drift silently without explicit coupling tests. |
| **Insight** | A two-layer contract works best: template bucket tests catch unknown/out-of-bucket tokens while drift tests lock exact `new` command argument sets for ownership boundaries. |
| **Suggested Action** | When adding new tokenized fields, update bucket inventories and expected `new` arg sets in the same change to keep policy deterministic. |
| **Applies To** | src/infrastructure/templates.rs, src/drift_tests.rs, src/cli/command_tree.rs |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-03T18:50:00+00:00 |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** |  |
