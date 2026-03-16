---
id: VDupml7OG
---

# Collaborative Cryptographic Primitives Over Adversarial Transport — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | agent:web-fetch | https://github.com/frost-rust/frost | 2026-03-01 | 2026-03-14 | high | high | FROST: Flexible Round-Optimized Schnorr Threshold Signatures implementation in Rust. |
| SRC-02 | web | agent:web-fetch | https://github.com/ZcashFoundation/redjubjub | 2026-02-20 | 2026-03-14 | high | high | RedJubjub signature scheme for circuit-friendly signatures. |

## Technical Research

## Feasibility
Initial research suggests that threshold signatures are highly feasible for Keel. The `frost-rust` library provides a robust implementation of Schnorr threshold signatures, which are computationally efficient and produce compact signatures.

## Key Findings

1. FROST allows for a (t, n) threshold scheme where any `t` out of `n` agents can cooperatively sign a message. [SRC-01]

## Unknowns

- Integration complexity with existing `ActorContext` serialization.
