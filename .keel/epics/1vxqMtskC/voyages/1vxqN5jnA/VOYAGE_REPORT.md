# VOYAGE REPORT: Technique Catalog Configuration And Autodetection

## Voyage Metadata
- **ID:** 1vxqN5jnA
- **Epic:** 1vxqMtskC
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 4/4 stories complete

## Implementation Narrative
### Surface Technique Recommendations In Planning Shows
- **ID:** 1vxqNFHpk
- **Status:** done

#### Summary
Expose technique recommendations in planning read commands so teams can see which automated verification approaches are available, configured, and currently underused.

#### Acceptance Criteria
- [x] [SRS-04/AC-01] `keel epic show`, `keel voyage show`, and `keel story show` render a recommendation section with ranked techniques and rationale. <!-- verify: cargo test --lib show_recommendation_sections, SRS-04:start, proof: ac-1.log-->
- [x] [SRS-04/AC-02] Recommendation output identifies whether techniques like `vhs` and `llm-judge` are configured/unused and provides adoption guidance snippets. <!-- verify: cargo test --lib show_recommendation_usage_status, SRS-04:end, proof: ac-2.log-->
- [x] [SRS-04/AC-03] Show rendering remains advisory-only and does not trigger execution of recommended techniques. <!-- verify: cargo test --lib show_recommendations_do_not_execute, SRS-NFR-02:start:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyDuwLvf: Centralized recommendation projection keeps show commands coherent**
  - Insight: A shared recommendation report model plus per-command input extraction avoids drift between epic/voyage/story rendering.
  - Suggested Action: Add new recommendation behavior in `verification_techniques` first, then wire each show command through the same renderer helper.
  - Applies To: `src/read_model/verification_techniques.rs`, `src/cli/commands/management/*/show.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxqNFHpk/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFHpk/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFHpk/EVIDENCE/ac-2.log)

### Implement Keel.toml Technique Configuration Overrides
- **ID:** 1vxqNFJOf
- **Status:** done

#### Summary
Allow projects to configure the technique bank through `keel.toml`, including enabling/disabling built-ins and defining local custom technique entries.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] Parse `keel.toml` technique configuration into a typed override model with validation for schema and required fields. <!-- verify: cargo test --lib technique_override_config_parse, SRS-02:start, proof: ac-1.log -->
- [x] [SRS-02/AC-02] Merge overrides with built-ins using deterministic precedence and support local enable/disable/customize behavior. <!-- verify: cargo test --lib technique_override_merge_precedence, SRS-02:end, proof: ac-2.log -->
- [x] [SRS-02/AC-03] Invalid overrides never trigger technique execution and produce explicit diagnostics. <!-- verify: cargo test --lib technique_override_invalid_is_advisory_only, SRS-NFR-02:start:end, proof: ac-3.log -->

#### Implementation Insights
- **1vyDuwSon: Advisory parser keeps keel.toml resilient**
  - Insight: Parsing overrides from raw TOML with per-field diagnostics allows invalid entries to be ignored safely without blocking normal command behavior.
  - Suggested Action: Keep optional/advanced config surfaces advisory by default, then merge validated entries into canonical models with explicit diagnostics.
  - Applies To: `src/read_model/verification_techniques.rs`
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxqNFJOf/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFJOf/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFJOf/EVIDENCE/ac-2.log)

### Implement Project Autodetection And Recommendation Engine
- **ID:** 1vxqNFNdN
- **Status:** done

#### Summary
Build the autodetection and ranking pipeline that infers project stack signals and recommends the highest-value automated verification techniques with rationale.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Detect project stack signals from repository artifacts (for example Rust CLI and browser test stack markers) and compute confidence scores. <!-- verify: cargo test --lib technique_project_signal_detection, SRS-03:start, proof: ac-1.log -->
- [x] [SRS-03/AC-02] Produce ranked recommendations from the merged catalog with rationale and applicability metadata per recommendation. <!-- verify: cargo test --lib technique_recommendation_ranking, SRS-03:end, proof: ac-2.log -->
- [x] [SRS-03/AC-03] Recommendation ranking is deterministic for equivalent repository inputs. <!-- verify: cargo test --lib technique_recommendation_deterministic, SRS-NFR-01:start:end, proof: ac-3.log -->

#### Implementation Insights
- **1vyDuwiA5: Deterministic ranking requires total-order tie breaks**
  - Insight: Deterministic ordering is guaranteed only when ranking sorts by score and then by stable id as a total-order tie breaker.
  - Suggested Action: Keep recommendation outputs sorted by `(score desc, id asc)` and normalize lists/sets before scoring.
  - Applies To: `src/read_model/verification_techniques.rs`
  - Category: code


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxqNFNdN/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFNdN/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFNdN/EVIDENCE/ac-2.log)

### Define Verification Technique Catalog Model
- **ID:** 1vxqNFaR9
- **Status:** done

#### Summary
Define the canonical automated-verification technique model and built-in catalog entries so advanced techniques like `vhs` and `llm-judge` are first-class and discoverable.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Introduce a `TechniqueDefinition` model with fields required for configuration, applicability detection, recommendation ranking, and rendering. <!-- verify: cargo test --lib technique_definition_model, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] Seed a built-in catalog that includes `vhs` and `llm-judge` plus baseline command-based techniques for Rust and browser stacks. <!-- verify: cargo test --lib builtin_technique_catalog_contains_vhs_llm_judge, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-01/AC-03] [SRS-NFR-01/AC-01] Built-in technique ordering is deterministic across runs and fixtures. <!-- verify: cargo test --lib builtin_technique_catalog_deterministic, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Implementation Insights
- **1vyDuwZW6: Catalog Entries Should Be Declarative And Sorted By ID**
  - Insight: A stable schema plus ID-sorted built-ins gives deterministic output and a predictable merge base for later override/ranking stages.
  - Suggested Action: Keep all built-ins in one constructor and enforce sort-by-ID before returning catalog vectors.
  - Applies To: `src/read_model/verification_techniques.rs`, upcoming config merge/recommendation modules
  - Category: architecture


#### Verified Evidence
- [ac-1.log](../../../../stories/1vxqNFaR9/EVIDENCE/ac-1.log)
- [ac-3.log](../../../../stories/1vxqNFaR9/EVIDENCE/ac-3.log)
- [ac-2.log](../../../../stories/1vxqNFaR9/EVIDENCE/ac-2.log)


