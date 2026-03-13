# VOYAGE REPORT: Resolve HEAD Show Selectors

## Voyage Metadata
- **ID:** VDhtzKSNF
- **Epic:** VDhtrxgW6
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Add HEAD Selector Parsing And Stable Order Resolution
- **ID:** VDhu6JN89
- **Status:** done

#### Summary
Add the shared HEAD-selector parser and the stable ordering providers that convert HEAD-relative selectors into concrete entity IDs without changing existing exact-ID lookups.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] Introduce a shared selector parser that accepts exact IDs plus HEAD, HEAD~, HEAD~~, and HEAD^ and normalizes unsupported forms into deterministic errors. <!-- verify: cargo test --lib head_selector_parser, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-02/AC-01] Expose canonical ordered ID providers for mission, epic, voyage, story, bearing, ADR, and routine entities using the same stable default ordering semantics as their list surfaces. <!-- verify: cargo test --lib head_selector_ordering, SRS-02:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-01/AC-01] Equivalent fixture boards resolve the same HEAD-relative selectors across repeated runs. <!-- verify: cargo test --lib head_selector_determinism, SRS-NFR-01:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDhu6JN89/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDhu6JN89/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VDhu6JN89/EVIDENCE/ac-3.log)

### Wire HEAD Syntax Into Show Commands
- **ID:** VDhu6M89X
- **Status:** done

#### Summary
Adopt the shared HEAD-selector path in the supported show commands so users can navigate by relative position instead of only by exact IDs.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] Mission, epic, voyage, and story show commands resolve HEAD-relative selectors through the shared selector path while preserving exact-ID behavior. <!-- verify: cargo test --bin keel head_show_commands_resolve_management_entities, SRS-03:start:end, proof: ac-1.log-->
- [x] [SRS-03/AC-02] Bearing, ADR, and routine show commands resolve HEAD-relative selectors through the same shared selector path while preserving exact-ID behavior. <!-- verify: cargo test --bin keel head_show_commands_resolve_governance_entities, SRS-03:start:end, proof: ac-2.log-->
- [x] [SRS-04/AC-01] Empty-set and out-of-range failures surface actionable, deterministic errors for the affected show commands. <!-- verify: cargo test --bin keel head_show_commands_report_selector_errors, SRS-04:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDhu6M89X/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDhu6M89X/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VDhu6M89X/EVIDENCE/ac-3.log)

### Lock HEAD Show Contracts With Regressions
- **ID:** VDhu6Mh9V
- **Status:** done

#### Summary
Lock the HEAD-relative selector contract with regression coverage and CLI guidance so the supported syntax and ordering semantics do not drift.

#### Acceptance Criteria
- [x] [SRS-04/AC-02] Unsupported selector syntax is rejected with canonical guidance that points users back to exact IDs or supported HEAD forms. <!-- verify: cargo test --bin keel head_show_commands_reject_invalid_syntax, SRS-04:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-02] Regression coverage proves the show-command head target matches the corresponding canonical default list ordering for every supported entity type. <!-- verify: cargo test --bin keel head_show_contract_matches_default_list_order, SRS-NFR-01:start:end, proof: ac-2.log-->
- [x] [SRS-NFR-02/AC-01] Help text or user-facing guidance touched by the change stays aligned with the supported selector forms and entity coverage. <!-- verify: cargo test --bin keel head_show_guidance_contract, SRS-NFR-02:start:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/VDhu6Mh9V/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/VDhu6Mh9V/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/VDhu6Mh9V/EVIDENCE/ac-3.log)


