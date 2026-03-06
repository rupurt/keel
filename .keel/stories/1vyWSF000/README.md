---
id: 1vyWSF000
title: Define Artifact Judge Bundle Contract
type: feat
status: backlog
created_at: 2026-03-06T06:47:03
updated_at: 2026-03-06T06:50:33
scope: 1vyWLl000/1vyWNV000
index: 1
---

# Define Artifact Judge Bundle Contract

## Summary

Define the machine-readable artifact bundle that semantic judges will consume so tape-driven evidence can be evaluated without tying keel to any one model provider.

## Acceptance Criteria

- [ ] [SRS-01/AC-01] [SRS-NFR-01/AC-01] The artifact bundle schema captures story metadata, acceptance-criterion text, and references to tape-driven evidence artifacts needed for judging, and it serializes deterministically for equivalent inputs. <!-- verify: cargo test -p keel artifact_judge_bundle_schema_captures_story_context, SRS-01:start:end, SRS-NFR-01:start:end -->
