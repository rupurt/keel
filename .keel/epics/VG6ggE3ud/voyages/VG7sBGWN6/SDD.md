# Stabilize Mission Request Command Semantics - Software Design Description

> Stabilize the mission request command contract for automation callers

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines the machine-facing contract for the `keel mission request`
command family. The design treats the request envelope as the stable interface
and keeps provider transport details outside the command surface so Keeper and
non-Keeper automation can invoke the same semantics.

## Context & Boundaries

### In Scope

- the request envelope shape
- stdin/file loading behavior
- validation semantics
- output/result contract

### Out of Scope

- provider polling
- GitHub issue revision rules
- Keeper runtime scheduling

### External Actors

- Keeper workers
- local automation or scripts
- future provider adapters that emit canonical envelopes

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Mission request envelope | Internal contract | Shared request payload across commands | CLI contract |
| Keel planning commands | Internal surface | Apply validated requests into board mutations | Existing CLI |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Provider neutrality | Keep provider-specific transport metadata optional and non-authoritative | Prevents GitHub-first behavior from becoming the core API |
| Single envelope | Use one canonical request envelope across all mission-request subcommands | Keeps piping and replay behavior stable |
| Explicit result modes | Separate success, validation failure, and execution failure semantics | Lets automation decide retry, repair, or escalate behavior deterministically |

## Architecture

The command family is modeled in three layers:
- input normalization: read stdin/file payloads into the canonical envelope
- validation and drafting semantics: determine whether the request is structurally and operationally ready
- execution surface: expose `apply` and `ack` contracts without embedding provider adapters

## Components

- Envelope parser: accepts stdin/file input and produces the canonical request model.
- Validator: checks required fields, derivable fields, and readiness constraints.
- Result renderer: emits stable human and machine outcomes for success and failure cases.
- Apply bridge: translates validated requests into native planning mutations.

## Interfaces

- `template`: emits a canonical request scaffold.
- `parse`: loads raw input and returns normalized request content.
- `validate`: returns readiness plus actionable errors.
- `draft`: returns a proposed planning slice without mutating the board.
- `apply`: performs native Keel mutations from a valid request.
- `ack`: emits canonical acknowledgement content for upstream transport owners.

## Data Flow

1. Caller submits a mission request payload.
2. The parser normalizes the payload into the canonical envelope.
3. Validation checks required fields, derivations, and readiness state.
4. `draft` or `apply` consumes the validated envelope.
5. The result renderer returns deterministic success or failure output.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Missing required mission fields | Envelope validation | Return structured validation failure | Caller repairs payload and retries |
| Provider-only data supplied without canonical fields | Parse/validate mismatch | Return actionable normalization error | Provider adapter supplies canonical fields or derivations |
| Request is structurally valid but not execution-ready | Draft/apply readiness check | Return non-terminal validation failure | Caller uses `draft` or requests more detail |
| Native mutation fails after validation | Apply execution path | Return execution failure separate from validation | Caller can retry or escalate without redefining the contract |
