---
source_type: Story
source: stories/1vxvIaM4w/REFLECT.md
scope: 1vxqMtskC/1vxvFrNta
source_story_id: 1vxvIaM4w
created_at: 2026-03-04T16:13:11
---

### 1vyDuwmNc: Centralize technique status before rendering

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple commands (`config show`, `verify recommend`) need the same detected/disabled/active evaluation. |
| **Insight** | A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces. |
| **Suggested Action** | Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs` |
| **Linked Knowledge IDs** |  |
| **Observed At** | 2026-03-05T00:10:00+00:00 |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |
