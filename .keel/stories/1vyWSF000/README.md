---
id: 1vyWSF000
title: Define Artifact Judge Bundle Contract
type: feat
status: done
created_at: 2026-03-06T06:47:03
updated_at: 2026-03-06T08:50:51
scope: 1vyWLl000/1vyWNV000
index: 1
started_at: 2026-03-06T08:47:30
completed_at: 2026-03-06T08:50:51
---

# Define Artifact Judge Bundle Contract

## Summary

Define the machine-readable artifact bundle that semantic judges will consume so tape-driven evidence can be evaluated without tying keel to any one model provider.

## Acceptance Criteria

- [x] [SRS-01/AC-01] The artifact bundle schema captures story metadata, acceptance-criterion text, and references to tape-driven evidence artifacts needed for judging. <!-- verify: cargo test -p keel artifact_judge_bundle_schema_captures_story_context, SRS-01:start:end, proof: ac-1.log-->
- [x] [SRS-NFR-01/AC-01] The artifact bundle schema serializes deterministically for equivalent inputs. <!-- verify: cargo test -p keel artifact_judge_bundle_schema_captures_story_context, SRS-NFR-01:start:end, proof: ac-2.log-->
