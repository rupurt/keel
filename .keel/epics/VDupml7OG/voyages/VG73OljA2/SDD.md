# Define Keeper Trust Boundaries And Audit Checkpoints - Software Design Description

> Define the first implementation-facing security slice around Keeper trust boundaries, append-only audit checkpoints, and threshold attestation scope.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage turns the security epic into an implementation-facing contract. The
design separates planning truth, provider ingress, execution control, append-only
audit checkpoints, and threshold attestation so Keel remains backend-agnostic
while Keeper can enforce stronger multiplayer guarantees at the runtime
boundary.

## Context & Boundaries

Keel owns planning state, mission request semantics, and artifact mutation.
Keeper owns provider ingress, execution routing, payload handling, and the
runtime coordination needed to produce or verify audit evidence. Backend
adapters sit beneath Keeper and supply append/checkpoint/proof operations.

```
┌──────────────────┐   ┌──────────────────┐   ┌──────────────────────┐
│       Keel       │   │      Keeper      │   │   Backend adapter    │
│ planning truth   │<->│ ingress/execution│<->│ append/prove/checkpt │
└──────────────────┘   └──────────────────┘   └──────────────────────┘
          ↑                       ↑
    mission requests       attestation / secrecy
```

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Keeper runtime architecture | internal | Defines ingress, routing, and execution boundaries | current workspace |
| Keel planning model | internal | Remains authoritative for board state and mission request semantics | current workspace |
| Append-only proof backend | external/internal | Supplies checkpoint and proof primitives | backend-specific |
| Threshold signature and sealing primitives | external design dependency | Protect high-consequence transitions and private payloads | FROST / HPKE class |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Authority split | Keel for planning truth, Keeper for runtime ingress and execution | Preserves a narrow trust boundary |
| Audit granularity | Sign checkpoints and high-consequence transitions, not every event | Keeps operations lighter while retaining strong evidence |
| Backend contract | Append/checkpoint/prove interface instead of Transit-specific semantics | Keeps storage swappable |
| Payload privacy | Treat sealed payloads as a separate concern from attestation | Avoids collapsing secrecy and authorization into one mechanism |

## Architecture

The security slice has four layers: canonical Keel events and mission requests,
Keeper-managed ingress and execution coordination, backend audit checkpoints,
and optional cryptographic sealing or attestation at high-consequence
boundaries. Each layer exposes an explicit contract so later implementation can
swap backends or strengthen policy without changing Keel planning semantics.

## Components

- Planning authority: Keel commands and board artifacts that remain the source
  of truth for missions, epics, voyages, and stories.
- Ingress coordinator: Keeper provider workers that normalize external requests
  and record replay metadata.
- Audit checkpoint manager: backend-facing component that appends events,
  materializes checkpoints, and exposes inclusion or consistency proofs.
- Attestation policy engine: decides which transitions need ordinary logging,
  threshold signatures, or sealed payload handling.
- Secret boundary handler: manages connector-private or reactor-private payloads
  without leaking raw secrets into public planning artifacts.

## Interfaces

- Mission request envelope between provider ingress and Keel.
- Checkpoint interface with append, checkpoint, inclusion proof, and consistency
  proof operations.
- Attestation policy interface that maps lifecycle moves to audit-only or
  threshold-attested handling.
- Payload handling interface for sealed content versus public planning evidence.

## Data Flow

1. A provider request enters Keeper and is normalized into the canonical mission
   request envelope.
2. Keel evaluates and applies planning mutations as appropriate.
3. Keeper records the resulting lifecycle events into the audit backend.
4. The backend materializes checkpoints and exposes inclusion or consistency
   proofs for later verification.
5. High-consequence transitions are mapped through the attestation policy and
   receive threshold signatures or equivalent quorum evidence when required.
6. Private payloads are sealed or referenced according to the secret boundary
   rules while public planning artifacts remain reviewable.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Provider payload cannot be normalized safely | Ingress validation | Refuse the request and preserve diagnostics | Request is corrected and replayed as a new revision |
| Backend cannot produce append-only proof material | Checkpoint manager | Mark audit path degraded and stop claiming strong proof semantics | Retry or fail over to a healthy backend |
| Threshold attestation quorum unavailable | Policy engine | Defer the protected transition or fall back to review-only status where policy allows | Retry when quorum is restored |
| Secret payload would leak into public artifacts | Secret boundary handler | Redact or seal the payload before publication | Store by reference or re-run through sealed path |
