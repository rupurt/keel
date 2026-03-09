---
id: 1vzQwn000
title: Add Research Provider Configuration And Weighting Controls
type: feat
status: backlog
created_at: 2026-03-08T20:06:21
updated_at: 2026-03-08T20:10:04
scope: 1vzQpr000/1vzQu0000
index: 2
---

# Add Research Provider Configuration And Weighting Controls

## Summary

Add provider configuration and weighting controls to `keel.toml` so research sources can be enabled, disabled, and ranked explicitly without hiding unavailable-provider states.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `keel.toml` exposes provider enablement and weighting controls for the supported research source classes. <!-- verify: cargo test -p keel research_provider_config_parses_enablement_and_weights, SRS-03:start, proof: ac-1.log-->
- [ ] [SRS-04/AC-01] Disabled, unavailable, or unsupported providers render explicit status in config and research command output instead of silently disappearing. <!-- verify: cargo test -p keel research_provider_status_is_explicit, SRS-04:start, proof: ac-2.log-->
- [ ] [SRS-04/AC-02] [SRS-NFR-02/AC-01] Provider failures or gaps never fall back to uncited model-memory findings masquerading as captured evidence. <!-- verify: cargo test -p keel research_provider_failures_do_not_fabricate_evidence, SRS-NFR-02:start:end, SRS-04:end, SRS-03:end, proof: ac-3.log-->
