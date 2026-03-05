---
created_at: 2026-02-24T14:18:41
---

### Note 001: Policy categories should drive messaging, not only hard-stop thresholds
Using `VerificationQueueCategory` for flow summaries captures `attention`, `human-blocked`, and `flow-blocked` states explicitly, which keeps user-facing guidance aligned with `next` behavior.

### Note 002: Cross-module boundary tests catch semantic drift early
Asserting `next` human decisions and flow summaries on threshold boundaries (`<5`, `>=5`, `>20`) prevents silent divergence when queue-policy semantics evolve.
