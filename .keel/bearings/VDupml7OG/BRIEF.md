# Collaborative Cryptographic Primitives Over Adversarial Transport — Brief

## Hypothesis

Keel's multi-player security model should be centered on Keeper-managed
coordination, backend-agnostic audit proofs, and narrow use of strong
cryptographic primitives at the boundaries that matter.

Transit remains a strong optional backend because it can provide append-only
lineage and replay, but Keel should not depend on Transit semantics in its core
planning model. The load-bearing design is:

- Keel owns planning truth
- Keeper owns provider ingress, routing, execution, and replay
- cryptography seals checkpoints, high-consequence transitions, and private
  payload boundaries
- external mission requests normalize into a provider-neutral envelope before
  they lower into Keel commands

## Problem Space

Traditional distributed systems overfit to adversarial consensus. Keel and
Keeper operate in a different regime: cooperative actors inside a workflow
engine, with adversarial or unreliable boundaries at storage, transport,
provider ingress, and delegated execution.

That changes what is actually load-bearing:

- **Not required on the hot path:** consensus, BFT, per-event threshold
  signatures, provider-specific planning logic
- **Required at the boundary:** canonical event encoding, append-only proofs,
  stable provenance, replayable ingress, and strong attestation for important
  lifecycle moves
- **Required for private operation:** explicit handling of reactor-private and
  connector-secret payloads

Keeper in `spoke` sharpens the problem. Keeper is the multi-player Keel
runtime, with reactor inboxes, outboxes, connector ingress, execution leases,
and typed provider gateways. The security model therefore has to answer two
questions together:

1. Which cryptographic primitives should protect multiplayer Keel under Keeper?
2. How should external mission requests enter the system in a way that is
   provider-neutral, auditable, and reducible to Keel commands?

## Success Criteria

- [ ] Define a Keeper-managed security boundary that keeps Keel authoritative for
      planning truth and keeps provider logic outside of Keel core
- [ ] Specify a backend-agnostic audit model based on canonical events,
      checkpoints, and inclusion/consistency proofs
- [ ] Identify which lifecycle transitions justify threshold attestation and
      which should remain lightweight
- [ ] Specify the role of FROST, DKG/VSS, HPKE-style sealing, and append-only
      proof systems in multi-player Keel
- [ ] Formalize a provider-neutral mission request envelope that Keeper can
      normalize from GitHub issues first and other providers later
- [ ] Define a native `keel mission request ...` command family that other
      programs can compose without embedding Keeper or GitHub-specific logic

## Open Questions

- What is the minimal high-consequence set that should require threshold
  signatures instead of ordinary audit logging?
- Which DKG or resharing flow fits Keeper fleet membership without introducing a
  central permanent signer?
- Should raw connector-secret payloads be stored encrypted, stored by reference,
  or redacted after normalization?
- How should provider edits, retries, and webhook or polling races affect
  mission request idempotency?
- Should `keel mission request apply` create only the mission by default, or can
  it safely attach a research bearing and seed initial evidence?
- How much provider identity should be preserved in the canonical request versus
  normalized into repo-authored mission artifacts?
