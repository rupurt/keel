# PRESS RELEASE: Verification Technique Command Surface Cutover

## Overview

## Narrative Summary
### Refactor Config Show Into Technique Flag Matrix
Refactor `keel config show` to present verification techniques as a canonical flag matrix and machine-readable payload, while keeping `keel config mode` unchanged.

### Implement Verify Recommend For Active Detected Techniques
Introduce `keel verify recommend` as the recommendation surface, filtered to detected and active techniques only, with advisory-only behavior and machine-readable output.

### Remove Planning Show Recommendations And Update Planning Guidance
Remove recommendation sections from planning read commands and update architect planning guidance to rely on `config show` and `verify recommend` for verification technique planning.

### Hard Cutover Verify Command To Subcommands
Perform a hard cutover of verification execution to `keel verify run`, preserving execution semantics while making legacy `keel verify` fail fast with migration guidance.

## Key Insights
### Insights from Refactor Config Show Into Technique Flag Matrix
- **L001: Prefer direct status flags over aggregated recommendation blocks**
  - Insight: A per-technique flag matrix (`detected`, `disabled`, `active`) is a better contract boundary than mixed narrative sections because it cleanly separates inventory from recommendation logic.
  - Suggested Action: Keep config/read commands focused on canonical state and move advisory ranking/commentary to dedicated recommend commands.


### Insights from Implement Verify Recommend For Active Detected Techniques
- **L001: Centralize technique status before rendering**
  - Insight: A shared status report API in the read model removes duplicated filtering logic and keeps recommendation output consistent across surfaces.
  - Suggested Action: Route all verification-technique render paths through `resolve_technique_status_report` rather than command-local detection code.


### Insights from Remove Planning Show Recommendations And Update Planning Guidance
- **L001: Keep recommendation sourcing decoupled from planning read surfaces**
  - Insight: Moving recommendation concerns to dedicated commands (`config show` inventory + `verify recommend`) keeps planning show outputs focused on planning state and avoids mixed concerns.
  - Suggested Action: Keep epic/voyage/story show projections limited to planning progress/evidence summaries; centralize recommendation logic in verification/config read models.


### Insights from Hard Cutover Verify Command To Subcommands
- **L001: Parse legacy forms but block execution paths**
  - Insight: Keeping hidden legacy root args allows deterministic migration errors without relying on generic clap parse failures, while still forcing execution through the new subcommand path.
  - Suggested Action: For future command cutovers, preserve temporary parse compatibility only for guidance and route all execution through explicit new subcommands.


## Verification Proof
### Proof for Refactor Config Show Into Technique Flag Matrix
- [ac-4.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIZRXy/EVIDENCE/ac-2.log)

### Proof for Implement Verify Recommend For Active Detected Techniques
- [ac-1.log](../../../../stories/1vxvIaM4w/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIaM4w/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIaM4w/EVIDENCE/ac-2.log)

### Proof for Remove Planning Show Recommendations And Update Planning Guidance
- [ac-1.log](../../../../stories/1vxvIa2RC/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIa2RC/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIa2RC/EVIDENCE/ac-2.log)

### Proof for Hard Cutover Verify Command To Subcommands
- [ac-1.log](../../../../stories/1vxvIaPe8/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxvIaPe8/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxvIaPe8/EVIDENCE/ac-2.log)

