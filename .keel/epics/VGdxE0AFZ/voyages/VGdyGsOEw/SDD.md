# Define Mission Stack Stewardship And Handoff Protocol - Software Design Description

> Define Mission Stack identity, steward/member coordination, stack modes, and git-backed pushed receipts for cross-reactor handoff.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage defines the protocol contract that sits between mission-request
ingress and stack-aware execution. The design keeps each repo authoritative for
its own board, gives one reactor the stewardship role for coordination, and uses
git-native pushed receipts as the handoff boundary between reactors.

## Context & Boundaries

### In Scope

- stack identity and stewardship model
- stack modes and checkpoint semantics
- pushed-receipt handoff contract
- member-side local mission materialization rules

### Out of Scope

- CLI surface rendering
- managed worktree mechanics
- stronger cryptographic or non-git receipt layers

### External Actors

- stack steward reactor
- member reactors
- git remote/branch state used for push receipts
- local Keel mission-request and planning commands

## Dependencies

| Dependency | Type | Purpose | Version/API |
|------------|------|---------|-------------|
| Mission request boundary | Internal contract | Preserve target-reactor ingress ownership | Existing Keel mission request direction |
| Git branch/head state | External contract | Carry the first pushed receipt and handoff identity | `stack/<id>` plus head sha |
| Local planning commands | Internal contract | Materialize local mission lineage after negotiation | Mission, epic, voyage commands |

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Board ownership | Each repo owns its own Keel board | Prevents foreign direct mutation and preserves local truth |
| Stewardship | One reactor coordinates the stack while members record local lineage and receipts | Gives the protocol one coordination source without centralizing planning state |
| Handoff boundary | Push is the inter-reactor handoff boundary | Distinguishes local closure from remote execution clearly |
| Receipt shape | First receipts stay git-native | Keeps the protocol implementable before stronger audit layers are required |

## Architecture

The protocol has four layers:

1. stack declaration: establish stack id, branch naming, steward, and members
2. local closure: commit local work and prepare a pushed receipt
3. reactor negotiation: send a mission request to the target reactor and decide
   whether the next move is allowed under the current stack mode
4. member materialization: let the target repo create or link its own local
   mission lineage after acceptance

## Components

- Stack identity record: names the stack, steward, members, and current mode.
- Receipt issuer: derives pushed handoff data from repo, branch, and head sha.
- Negotiation gate: determines whether a target reactor may accept and execute
  the next stack-linked turn.
- Local materializer: lowers accepted stack work into the target repo's own
  mission or epic lineage.

## Interfaces

- Stack declaration interface: stack id, member repos, steward repo, branch
  convention, and optional checkpoint metadata.
- Pushed receipt interface: stack id, repo identity, branch, head sha, and
  optional checkpoint or handoff context.
- Negotiation interface: mission request plus current receipt and stack-mode
  context delivered to the target reactor.

## Data Flow

1. A steward declares or extends a Mission Stack.
2. A member repo performs local work and seals it with a commit.
3. The member pushes `stack/<id>` and emits a git-native receipt.
4. Another reactor negotiates against that receipt through a mission request.
5. The target reactor evaluates stack mode, acceptance rules, and local board
   needs.
6. If accepted, the target reactor creates or links local planning lineage and
   becomes eligible for its own stack turn.

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Foreign board mutation is attempted directly | Negotiation or policy review | Reject the protocol path as invalid | Re-route work through a mission request to the target reactor |
| Receipt is missing required git identity fields | Receipt validation | Treat the handoff as incomplete | Re-push with a valid `stack/<id>` branch and head sha |
| Stack mode forbids the requested handoff | Stack mode evaluation | Block acceptance and surface the active gate | Wait for the active member to yield or for the checkpoint to clear |
