# PRESS RELEASE: Planning Read Surfaces And Evidence Visibility

## Overview

## Narrative Summary
### Render Concrete Evidence In Story Show
Rework `keel story show` evidence output to display real proof details (metadata, excerpts, artifact files, and media assets) so human acceptance can happen directly from command output.

### Implement Voyage Show Requirement Progress
Upgrade `keel voyage show` so it reports voyage intent, scope boundaries, and requirement-level completion/verification progress instead of dumping raw markdown.

### Implement Epic Show Planning Summary
Upgrade `keel epic show` to render an actionable planning report: authored summary, requirement/verification readiness, artifact visibility, and completion progress with ETA.

### Define Planning Show Output Contracts
Define a shared planning/evidence projection contract that all three `show` commands consume, including deterministic section ordering and missing-data placeholders.

## Key Insights
### Insights from Render Concrete Evidence In Story Show
- **L001: Evidence UX Needs Structured Inventory Layers**
  - Insight: A clear split between linked proofs, supplementary artifacts, and media playback hints makes acceptance decisions possible without opening files manually.
  - Suggested Action: Keep evidence rendering model-driven and test each layer (metadata, excerpts, missing warnings, placeholders) independently.


### Insights from Implement Voyage Show Requirement Progress
- **L001: Voyage Requirement Views Need Both AC And Verify Mapping**
  - Insight: Requirement linkage should combine AC references and verify requirement IDs; relying on one source undercounts coverage/verification state.
  - Suggested Action: Build requirement matrices from both marker channels, then deterministically sort rows and linked stories.


### Insights from Implement Epic Show Planning Summary
- **L001: Planning Show Parsing Needs Scaffold Filters**
  - Insight: Requirement parsing must explicitly ignore scaffold rows like `TODO`/template defaults or placeholder mode appears complete when it is not.
  - Suggested Action: Keep placeholder filters and add fixture tests that assert empty summaries on scaffold-only PRDs.


### Insights from Define Planning Show Output Contracts
- **L001: Centralized show projections reduce drift**
  - Insight: A shared read-model projection layer stabilizes data contracts, keeps ordering deterministic, and lets renderers remain thin.
  - Suggested Action: Add new planning/read surfaces by extending `read_model::planning_show` first, then adapt renderer output only.


## Verification Proof
### Proof for Render Concrete Evidence In Story Show
- [ac-4.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppkEH9/EVIDENCE/ac-2.log)

### Proof for Implement Voyage Show Requirement Progress
- [ac-4.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppkB6M/EVIDENCE/ac-2.log)

### Proof for Implement Epic Show Planning Summary
- [ac-4.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-4.log)
- [ac-1.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-1.log)
- [ac-5.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-5.log)
- [ac-3.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppkN0w/EVIDENCE/ac-2.log)

### Proof for Define Planning Show Output Contracts
- [ac-1.log](../../../../stories/1vxppk4Oj/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxppk4Oj/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxppk4Oj/EVIDENCE/ac-2.log)

