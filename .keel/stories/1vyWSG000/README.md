---
id: 1vyWSG000
title: Wire Provider Agnostic Llm Judge Execution
type: feat
status: backlog
created_at: 2026-03-06T06:47:04
updated_at: 2026-03-06T06:50:33
scope: 1vyWLl000/1vyWNV000
index: 3
---

# Wire Provider Agnostic Llm Judge Execution

## Summary

Replace the current diff-only judge stub with an external contract that accepts an artifact bundle path, leaving provider-specific prompting and transport outside the keel crate.

## Acceptance Criteria

- [ ] [SRS-03/AC-01] `llm-judge` is invoked through a provider-agnostic external contract that receives an artifact-bundle path instead of relying on `git diff` text alone. <!-- verify: cargo test -p keel llm_judge_uses_artifact_bundle_contract, SRS-03:start:end -->
- [ ] [SRS-NFR-02/AC-01] The judge integration introduces no vendor-specific SDK or transport dependency into keel. <!-- verify: cargo test -p keel llm_judge_uses_artifact_bundle_contract, SRS-NFR-02:start:end -->
