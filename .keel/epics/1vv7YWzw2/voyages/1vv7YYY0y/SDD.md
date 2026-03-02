# Unified Enforcement Wiring - System Design Document

> Replace fragmented command-side checks with the unified enforcement service and update documentation.

**Epic:** [1vv7YWzw2](../../README.md) | **SRS:** [SRS.md](SRS.md)

## System Overview

This voyage wires the `EnforcementService` into the command actuators to ensure state and gate validation is unified across the CLI.

## Components

### Command Actuators
- `src/commands/story/start.rs`: Refactor to use `enforce_transition`.
- `src/commands/story/submit.rs`: Refactor to use `enforce_transition`.
- `src/commands/story/accept.rs`: Refactor to use `enforce_transition`.
- `src/commands/voyage/plan.rs`: Refactor to use `enforce_transition`.
- `src/commands/voyage/start.rs`: Refactor to use `enforce_transition`.

### Enforcement Mapping
- Map `StoryAction` and `VoyageAction` to `TransitionIntent`.
- Utilize `EnforcementPolicy::RUNTIME` for all destructive/state-changing commands.

## Constraints & Considerations

- Ensure that any command-specific logic (e.g., auto-start of voyages in `story start`) is preserved.
- Use `format_enforcement_error` for consistent CLI feedback.
