# PRESS RELEASE: Technique Catalog Configuration And Autodetection

## Overview

## Narrative Summary
### Define Verification Technique Catalog Model
Define the canonical automated-verification technique model and built-in catalog entries so advanced techniques like `vhs` and `llm-judge` are first-class and discoverable.

### Implement Keel.toml Technique Configuration Overrides
Allow projects to configure the technique bank through `keel.toml`, including enabling/disabling built-ins and defining local custom technique entries.

### Surface Technique Recommendations In Planning Shows
Expose technique recommendations in planning read commands so teams can see which automated verification approaches are available, configured, and currently underused.

### Implement Project Autodetection And Recommendation Engine
Build the autodetection and ranking pipeline that infers project stack signals and recommends the highest-value automated verification techniques with rationale.

## Key Insights
### Insights from Define Verification Technique Catalog Model
- **L001: Catalog Entries Should Be Declarative And Sorted By ID**
  - Insight: A stable schema plus ID-sorted built-ins gives deterministic output and a predictable merge base for later override/ranking stages.
  - Suggested Action: Keep all built-ins in one constructor and enforce sort-by-ID before returning catalog vectors.


### Insights from Implement Keel.toml Technique Configuration Overrides
- **L001: Advisory parser keeps keel.toml resilient**
  - Insight: Parsing overrides from raw TOML with per-field diagnostics allows invalid entries to be ignored safely without blocking normal command behavior.
  - Suggested Action: Keep optional/advanced config surfaces advisory by default, then merge validated entries into canonical models with explicit diagnostics.


### Insights from Surface Technique Recommendations In Planning Shows
- **L001: Centralized recommendation projection keeps show commands coherent**
  - Insight: A shared recommendation report model plus per-command input extraction avoids drift between epic/voyage/story rendering.
  - Suggested Action: Add new recommendation behavior in `verification_techniques` first, then wire each show command through the same renderer helper.


### Insights from Implement Project Autodetection And Recommendation Engine
- **L001: Deterministic ranking requires total-order tie breaks**
  - Insight: Deterministic ordering is guaranteed only when ranking sorts by score and then by stable id as a total-order tie breaker.
  - Suggested Action: Keep recommendation outputs sorted by `(score desc, id asc)` and normalize lists/sets before scoring.


## Verification Proof
### Proof for Define Verification Technique Catalog Model
- [ac-1.log](../../../../stories/1vxqNFaR9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFaR9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFaR9/EVIDENCE/ac-2.log)

### Proof for Implement Keel.toml Technique Configuration Overrides
- [ac-1.log](../../../../stories/1vxqNFJOf/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFJOf/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFJOf/EVIDENCE/ac-2.log)

### Proof for Surface Technique Recommendations In Planning Shows
- [ac-1.log](../../../../stories/1vxqNFHpk/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFHpk/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFHpk/EVIDENCE/ac-2.log)

### Proof for Implement Project Autodetection And Recommendation Engine
- [ac-1.log](../../../../stories/1vxqNFNdN/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFNdN/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFNdN/EVIDENCE/ac-2.log)

