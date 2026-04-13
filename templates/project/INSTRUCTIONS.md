# INSTRUCTIONS.md

Procedural instructions for humans and agents working with {{project_name}} through Keel.

## Downstream Contract

This file is downstream from Keel and should keep the engine's turn-loop discipline recognizable while describing how {{project_name}} actually operates.

Mission Stack coordination is part of the formal downstream protocol surface. Hydrate `PROTOCOL.md` instead of inventing ad hoc cross-repo rules in chat or commit lore.

When syncing from a newer Keel version, preserve the `PROJECT-SPECIFIC` block instead of re-authoring the local operating surface from scratch.

## The Turn Loop

Use Keel's canonical `Orient -> Inspect -> Pull -> Ship -> Close` loop as the basis of daily work:

1. **Orient**: `keel heartbeat`, `keel health --scene`, `keel flow --scene`, `keel doctor --status`
2. **Inspect**: `keel mission next --status`, `keel pulse`, `keel roles`, `keel next --role <role> --explain`
3. **Pull**: choose one bounded slice for the appropriate role or lane
4. **Ship**: execute the slice and record proof while the work is fresh
5. **Close**: land the relevant lifecycle transition and the sealing commit

## Primary Workflows

Define the role-specific procedures for this repository. Each workflow should document the context commands, typical actions, and constraints for that role.

### Operator (Implementation)
Focus on **evidence-backed delivery**.
- **Context**: `keel story show <id>`, `keel voyage show <id>`, `keel next --role operator`
- **Action**: Implement requirements, record proofs with `keel story record`, and `submit`.
- **Constraint**: Every AC must have a proof.

### Manager (Planning)
Focus on **strategic alignment and unblocking**.
- **Context**: `keel epic show <id>`, `keel roles`, `keel next --role manager --explain`, `keel flow`
- **Action**: Author `PRD.md`, `SRS.md`, `SDD.md`, resolve routing, decompose stories, and attach mission children.
- **Constraint**: Move voyages from `draft` to `planned` only when requirements are coherent.

### Explorer (Research)
Focus on **technical discovery and fog reduction**.
- **Context**: `keel bearing list`
- **Action**: Fill `BRIEF.md`, collect `EVIDENCE.md`, and `assess`.
- **Constraint**: Graduate to epics only when research is conclusive.

## Repo-Specific Turn Surfaces

<!-- BEGIN PROJECT-SPECIFIC -->
- Add the exact build, test, runtime, or preview commands this repo expects.
- Add any mandatory local wrappers such as `just check`, `pnpm test`, `cargo test`, or deploy smoke tests.
- Add role-specific expectations if the repo uses designer, marketer, legal, or executive lanes in practice.
- If this repo participates in Mission Stacks, point to the branch, checkpoint, handoff, and managed-worktree rules in `PROTOCOL.md`.
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
- Treat Mission Stack coordination as a formal protocol. Another repo must not mutate this repo's `{{board_dir}}/` state directly.
- Keep downstream wrappers truthful about the real repo commands.
- Update this file when the repo workflow changes materially.
