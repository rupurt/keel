# INSTRUCTIONS.md

Procedural instructions for humans and agents working with {{project_name}} through Keel.

## Downstream Contract

This file is downstream from Keel and should keep the engine's turn-loop discipline recognizable while describing how {{project_name}} actually operates.

When syncing from a newer Keel version, preserve the `PROJECT-SPECIFIC` block instead of re-authoring the local operating surface from scratch.

## The Turn Loop

Use Keel's canonical `Orient -> Inspect -> Pull -> Ship -> Close` loop as the basis of daily work:

1. **Orient**: `keel heartbeat`, `keel health --scene`, `keel flow --scene`, `keel doctor --status`
2. **Inspect**: `keel mission next --status`, `keel pulse`, `keel roles`, `keel next --role <role> --explain`
3. **Pull**: choose one bounded slice for the appropriate role or lane
4. **Ship**: execute the slice and record proof while the work is fresh
5. **Close**: land the relevant lifecycle transition and the sealing commit

## Repo-Specific Turn Surfaces

<!-- BEGIN PROJECT-SPECIFIC -->
- Add the exact build, test, runtime, or preview commands this repo expects.
- Add any mandatory local wrappers such as `just check`, `pnpm test`, `cargo test`, or deploy smoke tests.
- Add role-specific expectations if the repo uses designer, marketer, legal, or executive lanes in practice.
<!-- END PROJECT-SPECIFIC -->

## Hydration Checklist

Before relying on this file as a real operating contract, fill in:

- the repo-specific proof bar
- the local verification commands
- the release or review gates
- the runtime or UX checks that matter for this project

## Hygiene Rules

- Use the CLI as the canonical lifecycle surface.
- Prefer board-backed proof over memory or chat summaries.
- Keep downstream wrappers truthful about the real repo commands.
- Update this file when the repo workflow changes materially.
