---
id: 1vxqNFJOf
title: Implement Keel.toml Technique Configuration Overrides
type: feat
status: backlog
created_at: 2026-03-04T09:51:05
updated_at: 2026-03-04T09:51:12
scope: 1vxqMtskC/1vxqN5jnA
---

# Implement Keel.toml Technique Configuration Overrides

## Summary

Allow projects to configure the technique bank through `keel.toml`, including enabling/disabling built-ins and defining local custom technique entries.

## Acceptance Criteria

- [ ] [SRS-02/AC-01] Parse `keel.toml` technique configuration into a typed override model with validation for schema and required fields. <!-- verify: cargo test --lib technique_override_config_parse, SRS-02:start -->
- [ ] [SRS-02/AC-02] Merge overrides with built-ins using deterministic precedence and support local enable/disable/customize behavior. <!-- verify: cargo test --lib technique_override_merge_precedence, SRS-02:end -->
- [ ] [SRS-NFR-02/AC-01] Invalid overrides never trigger technique execution and produce explicit diagnostics. <!-- verify: cargo test --lib technique_override_invalid_is_advisory_only, SRS-NFR-02:start:end -->
