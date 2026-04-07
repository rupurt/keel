# Keeper-Managed Multiplayer Security Model

This document captures the recommended security stance for multi-player Keel
when the runtime is contained and managed by Keeper.

## Design Stance

- Keel remains authoritative for planning truth under `.keel/`.
- Keeper owns provider ingress, routing, execution, and replayable operational
  evidence.
- Transit is a strong optional backend, not a required substrate.
- High-consequence state should be attested at checkpoints and boundaries, not
  by forcing heavyweight cryptography onto every low-value event.
- The trusted interior is cooperative by policy; the risky edges are storage,
  transport, provider ingress, connector egress, and delegated execution.

## Trust Boundaries

### Keel planning boundary

Keel owns:

- missions, epics, voyages, stories, bearings, and ADR lifecycle
- canonical repo-authored planning artifacts
- workflow validation and board integrity

Keeper may not silently invent planning truth outside Keel commands or
Keel-authored artifacts.

### Keeper runtime boundary

Keeper owns:

- polling providers and normalizing inbound requests
- routing envelopes to reactor inboxes and outboxes
- managing execution leases and guest launches
- emitting operator-visible status and escalation evidence

Keeper should treat every connector and external provider as an untrusted
boundary, even when the underlying system is cooperative.

### Backend boundary

The event backend should satisfy a narrow contract:

- append canonical events
- read event ranges
- produce checkpoint roots
- prove inclusion of an event in a checkpoint
- prove that one checkpoint is an append-only extension of another
- persist sealed checkpoint attestations

Transit can satisfy this contract well, but the contract must remain backend
agnostic so Keeper can also run over simpler stores.

## Recommended Cryptographic Primitives

### 1. Threshold Schnorr via FROST

FROST is the strongest fit for quorum attestation in Keeper-managed Keel.

Use it for:

- mission activation, pause, and closure gates
- story acceptance or override when policy demands quorum
- checkpoint sealing for replayable audit snapshots
- cross-reactor delegation or lease transfer that should require more than one
  actor
- key rotation acknowledgements

Do not use it for:

- every inbox event
- every connector delivery
- every low-risk local planning mutation

The right default is to sign checkpoints or high-consequence lifecycle moves,
not the full raw event stream. See [EVIDENCE.md](EVIDENCE.md) `SRC-01`.

### 2. Merkle inclusion and consistency proofs

Keeper needs append-only auditability regardless of backend. Merkle-backed
inclusion proofs answer "is this event in checkpoint X?" and consistency proofs
answer "is checkpoint Y a valid append-only extension of checkpoint X?".

This should be the main portability layer between Keel/Keeper and storage. See
[EVIDENCE.md](EVIDENCE.md) `SRC-04`.

### 3. DKG / VSS / resharing

Threshold signatures are not enough by themselves. Keeper also needs:

- threshold key setup without a permanent central signer
- safe membership rotation as keepers join or leave
- resharing when project policy changes

This is a prerequisite for taking FROST beyond research and into fleet policy.

### 4. HPKE-style payload sealing

Not every message should be stored in plaintext. Reactor-private inbox items
and connector-secret payloads should support sealed payloads or encrypted
references so that replay is preserved without exposing sensitive content.

This belongs on private payloads and connector edges, not on public mission
metadata. See [EVIDENCE.md](EVIDENCE.md) `SRC-05`.

### 5. Prime-order group defaults

The cleaner default for threshold work is the FROST ciphersuite over
`ristretto255` with `SHA-512`, because the prime-order abstraction removes a
class of group pitfalls and matches the main FROST guidance. See
[EVIDENCE.md](EVIDENCE.md) `SRC-01` and `SRC-06`.

### 6. Optional robustness wrapper

FROST solves threshold signing. It does not by itself solve signer liveness or
robust coordination in partially available fleets. If Keeper uses threshold
signing in production, it should plan for a robustness wrapper around signing
sessions rather than assuming all selected participants will always cooperate.

## Attestation Scope

### Low-consequence

- local planning reads
- mailbox polling
- provider fetches
- advisory diagnostics

These usually need ordinary audit logging, not quorum crypto.

### Medium-consequence

- provider request normalization
- issue-to-mission draft generation
- single-reactor lease changes
- status summaries written back to providers

These should be canonically hashed and attributable to one reactor or operator.

### High-consequence

- checkpoint publication
- mission state transitions with org-wide effect
- cross-reactor handoff that changes stewardship
- policy or key rotation
- irreversible acceptance gates

These are the events that justify threshold attestation.

## What Not To Depend On

- Transit-only semantics in Keel's core event model
- a centralized signing service as the source of truth
- provider-native auth as the only provenance signal
- free-form provider payloads without normalization and validation

## Recommended Backend Contract

```text
append(events[])
read(range)
checkpoint(range) -> checkpoint_id, root
prove_inclusion(event_id, checkpoint_id)
prove_consistency(old_checkpoint_id, new_checkpoint_id)
seal(checkpoint_id, policy_ref, attestation)
```

The checkpoint object should carry:

- canonical event range
- previous checkpoint reference
- root hash
- attestation policy reference
- signer set or signer commitments
- threshold or signer count metadata

## Keeper Implications

- Keeper should normalize provider ingress into canonical mission-request
  envelopes before it tries to mutate planning state.
- Keel should expose a small command surface for parse, validate, draft, and
  apply operations so Keeper can compose it rather than reimplementing it.
- Raw provider payloads should be preserved by reference or digest so operator
  review and replay remain possible.
- The security model should assume multiple keepers can cooperate, but no
  single keeper should be trusted as the only witness for important state.
