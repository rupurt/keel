---
id: 1vyWSD000
title: Link Tape Evidence Into Verification Manifests
type: feat
status: backlog
created_at: 2026-03-06T06:47:01
updated_at: 2026-03-06T06:50:33
scope: 1vyWLl000/1vyWNL000
index: 5
---

# Link Tape Evidence Into Verification Manifests

## Summary

Close the loop between tape execution and keel's proof model by storing rendered artifacts, companion transcripts, and manifest hashes under the dogfood stories that own each scenario.

## Acceptance Criteria

- [ ] [SRS-05/AC-01] Dogfood runs persist rendered VHS outputs and companion transcript/log artifacts under story `EVIDENCE/` and record them in verification manifests. <!-- verify: cargo test -p keel dogfood_vhs_evidence_enters_manifest, SRS-05:start:end -->
- [ ] [SRS-06/AC-01] Dogfood planning artifacts and story annotations document the tape/transcript/manifest proof chain clearly enough for `voyage plan` and `keel doctor` to pass. <!-- verify: just keel doctor, SRS-06:start:end -->
