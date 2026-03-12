# VOYAGE REPORT: Evidence Capture and Provider Signals

## Voyage Metadata
- **ID:** 1vzQu0000
- **Epic:** 1vzQpr000
- **Status:** done
- **Goal:** -

## Execution Summary
**Progress:** 3/3 stories complete

## Implementation Narrative
### Model Canonical Evidence Records And Parsing Rules
- **ID:** 1vzQwm000
- **Status:** done

#### Summary
Define the canonical evidence record schema and parsing rules for `EVIDENCE.md` so every research source carries stable IDs, provenance, dates, and quality metadata that downstream scoring and rendering can trust.

#### Acceptance Criteria
- [x] [SRS-01/AC-01] `EVIDENCE.md` supports canonical source records with stable IDs plus required metadata for source class, provenance, publication or observation date, retrieval date, authority, and freshness. <!-- verify: cargo test -p keel evidence_record_schema_is_canonical, SRS-01:start, proof: ac-1.log-->
- [x] [SRS-01/AC-02] The evidence parser and doctor checks reject malformed or unresolved scaffold entries instead of accepting partially structured research notes. <!-- verify: cargo test -p keel evidence_parser_rejects_malformed_records, SRS-01:end, proof: ac-2.log-->
- [x] [SRS-01/AC-03] [SRS-NFR-01/AC-01] Equivalent evidence fixtures normalize into deterministic record ordering and metadata output. <!-- verify: cargo test -p keel evidence_record_parsing_is_deterministic, SRS-NFR-01:start:end, SRS-01:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzQwm000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzQwm000/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/1vzQwm000/EVIDENCE/ac-3.log)

### Add Research Provider Configuration And Weighting Controls
- **ID:** 1vzQwn000
- **Status:** done

#### Summary
Add provider configuration and weighting controls to `keel.toml` so research sources can be enabled, disabled, and ranked explicitly without hiding unavailable-provider states.

#### Acceptance Criteria
- [x] [SRS-03/AC-01] `keel.toml` exposes provider enablement and weighting controls for the supported research source classes. <!-- verify: cargo test -p keel research_provider_config_parses_enablement_and_weights, SRS-03:start, proof: ac-1.log-->
- [x] [SRS-04/AC-01] Disabled, unavailable, or unsupported providers render explicit status in config and research command output instead of silently disappearing. <!-- verify: cargo test -p keel research_provider_status_is_explicit, SRS-04:start, proof: ac-2.log-->
- [x] [SRS-04/AC-02] [SRS-NFR-02/AC-01] Provider failures or gaps never fall back to uncited model-memory findings masquerading as captured evidence. <!-- verify: cargo test -p keel research_provider_failures_do_not_fabricate_evidence, SRS-NFR-02:start:end, SRS-04:end, SRS-03:end, proof: ac-3.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzQwn000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzQwn000/EVIDENCE/ac-2.log)
- [ac-3.log](../../../../stories/1vzQwn000/EVIDENCE/ac-3.log)

### Capture Web Academic Social And Manual Evidence Through One Workflow
- **ID:** 1vzQwp000
- **Status:** done

#### Summary
Implement one canonical research ingestion workflow that can capture web, academic, social, and manual evidence into the shared evidence contract while preserving provider provenance for each source.

#### Acceptance Criteria
- [x] [SRS-02/AC-01] The research workflow accepts web, academic or prior-art, social or trend, and manual or internal evidence through one canonical command and service path. <!-- verify: cargo test -p keel research_workflow_supports_all_signal_classes, SRS-02:start, proof: ac-1.log-->
- [x] [SRS-02/AC-02] Evidence captured from each source class persists through the shared canonical source schema with provider provenance attached to every stored record. <!-- verify: cargo test -p keel research_capture_persists_provenance_for_all_signal_classes, SRS-02:end, proof: ac-2.log-->

#### Verified Evidence
- [ac-1.log](../../../../stories/1vzQwp000/EVIDENCE/ac-1.log)
- [ac-2.log](../../../../stories/1vzQwp000/EVIDENCE/ac-2.log)


