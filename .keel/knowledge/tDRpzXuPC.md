---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxvFrNta/KNOWLEDGE.md
scope: null
source_story_id: null
---

### tDRpzXuPC: Centralize technique status before rendering

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Multiple commands (`config show`, `verify recommend`) need the same detected/disabled/active evaluation. |
| **Insight** | A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces. |
| **Suggested Action** | Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code. |
| **Applies To** | `src/read_model/verification_techniques.rs`, `src/cli/commands/setup/config.rs`, `src/cli/commands/management/verify.rs` |
| **Linked Knowledge IDs** | 1vyDuwmNc |
| **Observed At** |  |
| **Score** | 0.83 |
| **Confidence** | 0.92 |
| **Applied** | yes |
