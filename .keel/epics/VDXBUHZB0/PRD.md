---
id: VDXBUHZB0
title: Storage Configuration
mission: VDXqZtRef
created_at: 2026-03-10T22:36:20
---

# Storage Configuration - PRD

## Problem Statement

With the introduction of multiple storage adapters (FileSystem initially, then others), the Keel CLI needs a way to select and initialize the appropriate backend. This choice should be driven by configuration, allowing a single CLI binary to operate either locally or as a client to a remote Keel server.

## Goals & Objectives

| ID | Goal | Success Metric | Target |
|----|------|----------------|--------|
| GOAL-01 | Add storage backend selection to `keel.toml`. | Users can specify `storage.backend = "filesystem"` or other values. | 100% |
| GOAL-02 | Support environment variable overrides for storage settings. | `KEEL_STORAGE_BACKEND` can override the config file. | 100% |

## Users

| Persona | Description | Primary Need |
|---------|-------------|--------------|
| Power User | User operating Keel in complex environments. | Easy switching between local and remote boards. |
| DevOps | Person configuring CI/CD pipelines. | Non-interactive backend selection via env vars. |

## Scope

### In Scope
- [SCOPE-01] Updating `src/infrastructure/config.rs` to include storage-related fields.
- [SCOPE-02] Implementation of a storage factory that initializes the correct port based on config.
- [SCOPE-03] CLI wiring to use the factory during startup.

### Out of Scope
- [SCOPE-04] Actually implementing the remote/HTTP backend (only the configuration for it is in scope).

## Requirements

<!-- BEGIN FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| FR-01 | `keel.toml` must support a `[storage]` section with `backend` and backend-specific options. | Strategic | GOAL-01 |
| FR-02 | The CLI must default to `filesystem` storage if no configuration is provided. | Strategic | GOAL-01 |
| FR-03 | Environment variables must be able to override storage settings. | Strategic | GOAL-02 |
<!-- END FUNCTIONAL_REQUIREMENTS -->

<!-- BEGIN NON_FUNCTIONAL_REQUIREMENTS -->
| ID | Description | Source | Goals |
|----|-------------|--------|-------|
| NFR-01 | Configuration errors (e.g., unknown backend) must provide clear error messages. | Strategic | GOAL-01 |
<!-- END NON_FUNCTIONAL_REQUIREMENTS -->

## Verification Strategy

- Unit tests for the configuration parsing and validation.
- Integration tests simulating different configurations and verifying backend initialization.

## Assumptions

| Assumption | Impact if Wrong | Validation |
|------------|-----------------|------------|
| Backend-specific options (like URL or API Key) can be mapped easily in the TOML structure. | Config schema may need to be more complex. | TOML mapping tests. |

## Open Questions & Risks

| Question/Risk | Owner | Status |
|---------------|-------|--------|
| Should the storage backend be project-local only or can it be global? | Architect | Open |

## Success Criteria

<!-- BEGIN SUCCESS_CRITERIA -->
- [ ] `keel config show` displays the active storage backend and its settings.
- [ ] Changing the backend in `keel.toml` correctly changes the storage port used by the application.
<!-- END SUCCESS_CRITERIA -->
