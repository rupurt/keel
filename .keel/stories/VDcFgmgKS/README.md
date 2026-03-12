---
id: VDcFgmgKS
title: Routine Bundle Contract
type: feat
status: done
created_at: 2026-03-11T19:24:30
updated_at: 2026-03-11T20:28:21
operator-signal: 
scope: VDakm8eVW/VDcFd11nc
index: 1
started_at: 2026-03-11T20:22:35
completed_at: 2026-03-11T20:28:21
---

# Routine Bundle Contract

## Summary

Define the first canonical routine bundle contract so recurring work blueprints
have one authored representation that later automation can consume.

## Acceptance Criteria

- [x] [SRS-01/AC-01] Define routine frontmatter and bundle fields for identity, cadence metadata, target scope, and authored blueprint content. <!-- verify: cargo test domain::model::routine::tests --lib, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-02/AC-01] Introducing routine bundles does not require changes to existing story frontmatter or lifecycle parsing contracts. <!-- verify: cargo test routine_contract_does_not_change_story_frontmatter_parsing, SRS-NFR-02:start:end, proof: ac-2.log-->
