# {{project_name}} Architecture

This document is downstream from Keel and should describe the actual technical shape of {{project_name}}. Keel owns the board engine; this file should explain the repo, runtime, and technical seams that agents need to understand before they change behavior. Protocol-level coordination rules, including Mission Stack behavior, belong in `PROTOCOL.md`.

## System Map

Hydrate the high-level architecture of {{project_name}}.

- What is the product or system this repository builds?
- Which directories are the primary entrypoints?
- Which boundaries are stable, and which are currently moving?

## Key Components

Document the major components and their responsibilities.

- Interface layers
- Core domain or business logic
- Persistence or external integrations
- Build, deployment, or runtime surfaces

## Technical Boundaries

- Where should new code go?
- Which modules should remain thin?
- Which dependencies or abstractions are intentionally avoided?

## Protocol Boundaries

- Where do external or Mission Stack requests enter the system?
- Where is repo-local board authority enforced?
- Which surfaces validate or emit stack handoffs, receipts, acknowledgments, or managed-worktree rules?

## Operational Seams

- Note the verification surfaces that matter in this repo.
- Note any architecture decisions enforced by ADRs.
- Note any areas where future agents should be especially conservative.
- Note how formal protocol surfaces intersect with runtime or delivery code.
