---
id: VDupml7OG
---

# Collaborative Cryptographic Primitives Over Adversarial Transport — Evidence

## Sources

| ID | Class | Provenance | Location | Observed / Published | Retrieved | Authority | Freshness | Notes |
|----|-------|------------|----------|----------------------|-----------|-----------|-----------|-------|
| SRC-01 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9591 | 2026-04-06 | 2026-04-06 | high | high | FROST threshold Schnorr specification, round structure, ciphersuites, and security considerations. |
| SRC-04 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9162 | 2026-04-06 | 2026-04-06 | high | high | Inclusion and consistency proofs for append-only transparency logs. |
| SRC-05 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9180 | 2026-04-06 | 2026-04-06 | high | high | HPKE for recipient-sealed payloads and connector-secret content boundaries. |
| SRC-06 | web | agent:web-fetch | https://datatracker.ietf.org/doc/html/rfc9496 | 2026-04-06 | 2026-04-06 | high | high | Prime-order group abstractions including `ristretto255`; relevant to FROST ciphersuite choice. |
| SRC-07 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/docs/architecture/keeper.md | 2026-04-06 | 2026-04-06 | high | high | Keeper architecture: Keel planning authority, reactor inbox/outbox model, connector ingress, and GitHub-first routing. |
| SRC-08 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper/src/lib.rs | 2026-04-06 | 2026-04-06 | medium | high | Current keeper HTTP surface is minimal and lacks mission-request ingestion. |
| SRC-09 | manual | workspace | /home/alex/workspace/spoke-sh/spoke/crates/keeper-cli/src/main.rs | 2026-04-06 | 2026-04-06 | medium | high | Current keeper CLI is scriptable but only exposes missions, start, and status operations. |

## Technical Research

## Key Findings

1. FROST is the right family for Keel quorum attestation because it supports
   `t-of-n` threshold signing with a compact final Schnorr signature. [SRC-01]
2. FROST should be used on checkpoints and high-consequence lifecycle
   transitions, not every event. The main operational hazard is nonce misuse,
   and the RFC explicitly separates signing from key-generation setup. [SRC-01]
3. Append-only inclusion and consistency proofs are the portability layer that
   let Keeper remain backend-agnostic while still gaining strong auditability.
   Transit can implement this well, but the proof model should not be
   Transit-only. [SRC-04][SRC-07]
4. HPKE-style sealed payloads are a strong fit for `reactor-private` and
   `connector-secret` content described in Keeper architecture, because privacy
   needs are concentrated at inbox, outbox, and connector boundaries rather than
   public mission metadata. [SRC-05][SRC-07]
5. `ristretto255` is the best current default for threshold work because it
   gives a prime-order abstraction and is directly aligned with the main FROST
   ciphersuite guidance. [SRC-01][SRC-06]
6. Keeper already defines the right control-plane boundary: Keel is canonical
   for planning truth, Keeper is canonical for ingress, routing, and execution.
   That means mission request parsing belongs at the Keel CLI boundary while
   provider polling belongs in Keeper. [SRC-07][SRC-08][SRC-09]
7. A provider-neutral mission request flow is missing today. Keeper's current
   service and CLI surfaces are too thin to normalize GitHub issues or any other
   provider payload into a replayable Keel command flow. [SRC-08][SRC-09]

## Feasibility

This direction is feasible if the work is staged:

1. Canonicalize mission-request and planning-boundary events.
2. Add parse, validate, draft, and apply commands to Keel.
3. Add provider polling and acknowledgement in Keeper.
4. Add append-only checkpoint proofs.
5. Add threshold attestation only after key lifecycle and policy questions are
   settled.

## Unknowns

- Which DKG or resharing approach fits Keeper's project and reactor lifecycle
  without centralizing trust
- Which transitions should truly require threshold attestation versus ordinary
  signed or hashed evidence
- How to represent provider revisions so retries and edits are idempotent but
  still auditable
- Whether mission request application should stop at mission creation or also
  seed a research bearing when intake is exploratory
