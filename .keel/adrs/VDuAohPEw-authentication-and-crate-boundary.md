---
id: VDuAohPEw
index: 2
title: Authentication and Crate Boundary
status: proposed
context: null
applies-to: []
supersedes: []
superseded-by: null
decided_at: 2026-03-14T20:58:02
---

# Authentication and Crate Boundary

## Status

**Proposed** — Awaiting human acceptance. Work in governed context is blocked.

## Context

As Keel evolves from a single-player CLI into a multi-agent, multi-tenant engine, we need a robust way to establish identity ("Who is asking?") and enforce authorization ("Are they allowed to do this?") before allowing mutations to the board state. We also want to decouple terminal parsing from the core domain logic so that the engine can be embedded in daemons, language servers, or remote agents. Furthermore, the authentication mechanisms we develop for Keel are likely applicable to other projects in the `spoke-sh` workspace.

## Decision

1. **Crate Split:** We will split the Keel project into multiple crates:
   - `keel-core` (Library): Contains the domain, application, and infrastructure layers. It knows nothing about the CLI or auth parsing.
   - `keel-cli` (Binary): Contains the Clap parsing, terminal rendering, and auth ingestion.
   - `spoke-auth` (or similar shared crate): We will extract the authentication logic into its own crate so it can be reused across the `~/workspace/spoke-sh` ecosystem.
2. **Execution Context:** We will introduce an `ExecutionContext` (or `ActorContext`) to the Keel core reactor. Every mutating application service in `keel-core` will require this context to perform audit logging and authorization.
3. **Authentication Scheme:** We will use JWTs (JSON Web Tokens) to represent authenticated identities (e.g., automated agents). Local OS execution will default to an unauthenticated/local system bypass unless strict auth is configured.
4. **Token Ingestion:** The CLI will ingest the JWT via a file path (e.g., `keel ping "hello" --auth-file path/to/token.jwt`), rather than an environment variable, to accommodate complex agent environments and file-based secret mounts.

## Constraints

- **MUST:** Extract authentication parsing and validation into a separate, reusable crate.
- **MUST:** Require an `ExecutionContext` for all mutating calls crossing the `keel-core` application boundary.
- **MUST:** Default to a "Local System" unauthenticated context if no `--auth-file` is provided, ensuring human workflows remain frictionless.
- **MUST NOT:** Bleed CLI argument parsing (Clap) or HTTP logic into `keel-core`.
- **SHOULD:** Log the actor's identity from the `ExecutionContext` when performing state mutations (like `keel mission achieve`).

## Consequences

### Positive
- Strict compiler boundaries prevent the CLI from accidentally bypassing domain authorization rules.
- The `keel-core` engine becomes embeddable in non-CLI contexts.
- Other `spoke-sh` projects get a reusable JWT auth crate for free.
- Agents can authenticate securely via file-mounted tokens.

### Negative
- Significant refactoring required to thread the `ExecutionContext` through all existing application services.
- Managing workspace dependencies across multiple local crates increases build complexity slightly.

## Verification

| Check | Type | Description |
|-------|------|-------------|
| Domain Boundary | automated | `keel-core` does not depend on `clap` or `spoke-auth` parsing tools. |
| Application Context | manual | All functions in `src/application/` require an `ExecutionContext` as their first argument. |

## References

- Initial discussion around `ping` and `poke` multi-agent asynchronous communication.
