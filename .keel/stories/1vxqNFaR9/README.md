---
id: 1vxqNFaR9
title: Define Verification Technique Catalog Model
type: feat
status: backlog
created_at: 2026-03-04T09:51:05
updated_at: 2026-03-04T09:51:12
scope: 1vxqMtskC/1vxqN5jnA
---

# Define Verification Technique Catalog Model

## Summary

Define the canonical automated-verification technique model and built-in catalog entries so advanced techniques like `vhs` and `llm-judge` are first-class and discoverable.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] Introduce a `TechniqueDefinition` model with fields required for configuration, applicability detection, recommendation ranking, and rendering. <!-- verify: cargo test --lib technique_definition_model, SRS-01:start -->
- [ ] [SRS-01/AC-02] Seed a built-in catalog that includes `vhs` and `llm-judge` plus baseline command-based techniques for Rust and browser stacks. <!-- verify: cargo test --lib builtin_technique_catalog_contains_vhs_llm_judge, SRS-01:continues -->
- [ ] [SRS-NFR-01/AC-01] Built-in technique ordering is deterministic across runs and fixtures. <!-- verify: cargo test --lib builtin_technique_catalog_deterministic, SRS-NFR-01:start -->
