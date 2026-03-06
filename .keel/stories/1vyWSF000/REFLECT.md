---
created_at: 2026-03-06T08:50:03
---

# Reflection - Define Artifact Judge Bundle Contract

## Knowledge

- [1vyYK1g00](../../knowledge/1vyYK1g00.md) Judge Bundles Should Carry References And Hashes

## Observations

The useful seam was smaller than the full judge execution path. Defining the bundle as a reusable schema plus builder let this story stay focused on contract shape and determinism, while leaving external command execution and evidence persistence for the follow-on stories that explicitly own them.

The bundle only became clean once it treated the acceptance criterion itself as first-class data instead of inferring everything from the proof file. Normalizing the `proof:` annotation into canonical `EVIDENCE/...` paths and tagging criterion-proof versus supporting evidence gives the next execution story a provider-neutral handoff surface.
