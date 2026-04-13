# {{project_name}} Protocol

This document is downstream from Keel and should describe the protocol surfaces that external systems, peer reactors, and operators rely on when coordinating work with {{project_name}}.

## Downstream Contract

Use this file for stable coordination rules and data contracts, not for local implementation details.

Mission Stack coordination is part of the formal protocol surface. It should be documented here as an explicit contract rather than left as chat lore or repo-local habit.

## Protocol Scope

Hydrate the protocol surfaces that matter in this repository:

- ingress requests from humans, agents, or upstream systems
- repo-to-repo handoff receipts
- lifecycle acknowledgments and checkpoint signals
- file, API, webhook, or queue payloads that other systems consume

## Mission Stack Coordination

Mission Stacks are a formal Keel protocol for cross-repo execution.

- Each participating repository remains authoritative for its own `{{board_dir}}/` state.
- Cross-repo work should flow through explicit stack negotiation and handoff rather than direct mutation of another repo's planning artifacts.
- Stack-linked work should use `stack/<id>` as the coordination branch unless this repo intentionally documents a narrower rule here.
- Foreign execution in another member repo should happen through a managed worktree rather than that repo's primary checkout.
- Hydrate any repo-specific checkpoint, receipt, approval, or cleanup expectations here.

## External Ingress

Describe how outside work enters {{project_name}}:

- accepted request shapes
- validation and acknowledgment
- how requests materialize into local mission lineage
- rejection or retry behavior

## Data Contracts

Document stable contracts that other systems rely on:

- input shapes
- output shapes
- receipt or acknowledgment fields
- versioning or migration expectations

## Local Exceptions

If {{project_name}} intentionally narrows or extends Keel's default protocol rules, document those deviations explicitly here.
