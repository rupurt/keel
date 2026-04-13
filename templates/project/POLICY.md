# {{project_name}} Policy

This document is downstream from Keel and should describe the operational invariants that govern {{project_name}}. Keel supplies the board engine and lifecycle discipline; this file defines the repo-specific rules that must remain true.

## Engine Contract

{{project_name}} uses Keel as its project-management engine.

- The board lives in `{{board_dir}}/`.
- The canonical tactical rhythm is the Turn Loop exposed by `keel turn`.
- Board mutations, proof, and lifecycle closure happen through the CLI, not manual file edits.
- Cross-repo coordination, including Mission Stacks, follows `PROTOCOL.md`; no peer repo or reactor may mutate this repo's `{{board_dir}}/` state directly.

## The Core Objective: Zero Drift

The primary goal of the engine is to eliminate **Drift** — any gap between what the board describes and what the code actually delivers. Progress is blocked if any of the following are detected by the `doctor`:
- **Structural Drift:** Missing files, invalid IDs, or broken frontmatter.
- **Architectural Drift:** Working in a bounded context with a `proposed` ADR.
- **Requirement Drift:** Stories missing SRS references or acceptance criteria.
- **Scaffold Drift:** Presence of placeholder text (e.g., `{{goal}}`, `Item 1`).

## Entity Invariants

Every entity in the `{{board_dir}}/` directory must adhere to structural rules. Define what must always be true for each entity type in this repository:

- **Missions:** Must have a `CHARTER.md` with at least one `board:`-verifiable goal to be `active`.
- **Epics:** Status is *derived* from voyages. An epic is `draft` until its first voyage is `planned`.
- **Voyages:** Must have an `SRS.md` and `SDD.md` with authored content to transition from `draft` to `planned`.
- **Stories:** Must link to a `voyage/SRS` requirement via the `[SRS-XX/AC-YY]` format.

## Repo Invariants

Hydrate the rules that should always be true in this repository.

- What must pass before a change can land?
- What kinds of changes require explicit human review?
- What evidence is required for code, product, UX, legal, or operational claims?
- Which Mission Stack or external-ingress rules are binding for cross-repo work in this repository?

## Safety Rails

- Define release and rollback expectations.
- Define how secrets, production systems, or customer data may be touched.
- Define what should block autonomous execution.

## Local Exceptions

If {{project_name}} needs exceptions from the default Keel operating model, document them explicitly here rather than letting them live in habit or chat memory.
