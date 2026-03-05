---
id: 1vugz2Kx9
title: Implement High-Fidelity Verification Manifest
type: feat
status: done
created_at: 2026-02-23T17:13:04
updated_at: 2026-02-24T08:31:29
scope: 1vugyr0OR/1vugyujor
index: 1
started_at: 2026-02-24T01:13:04
completed_at: 2026-02-24T01:18:04
---

# Implement High-Fidelity Verification Manifest

## Summary

Instead of simply executing scripts, the `verify` command must generate a `manifest.yaml` containing the Git SHA, proof artifact hashes, and LLM-Judge signatures. This story implements the manifest structure and integrity checking.

## Acceptance Criteria

- [x] [SRS-01/AC-01] `keel verify` generates a signed manifest linking artifacts to the current Git SHA <!-- verify: true, SRS-01:start -->
- [x] [SRS-01/AC-02] `keel doctor` detects if a manifest is missing or if artifacts have been tampered with (hash mismatch) <!-- verify: true, SRS-01:end -->
