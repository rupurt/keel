---
source_type: Voyage
source: epics/1vxqMtskC/voyages/1vxvFrNta/KNOWLEDGE.md
scope: null
source_story_id: null
---

### ng7SivrJS: Keep recommendation sourcing decoupled from planning read surfaces

| Field | Value |
|-------|-------|
| **Category** | architecture |
| **Context** | Planning read commands and verification-technique discovery can drift when both surfaces try to rank/recommend techniques. |
| **Insight** | Moving recommendation concerns to dedicated commands (`config show` inventory + `verify recommend`) keeps planning show outputs focused on planning state and avoids mixed concerns. |
| **Suggested Action** | Keep epic/voyage/story show projections limited to planning progress/evidence summaries; centralize recommendation logic in verification/config read models. |
| **Applies To** | `src/read_model/planning_show.rs`, `src/cli/commands/management/verify.rs`, `AGENTS.md` |
| **Linked Knowledge IDs** | 1vyDuwUUO |
| **Observed At** |  |
| **Score** | 0.79 |
| **Confidence** | 0.89 |
| **Applied** | yes |
