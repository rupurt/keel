# AGENTS.md

Shared guidance for AI agents working with this repository.

## Operational Guidance

Keel is an engine with strict constraints (see [FORMAL_RULES.md](FORMAL_RULES.md)). Your primary responsibility is to execute tactical moves that advance the board state while maintaining 100% integrity.

### Procedural Instructions
Follow the formal procedural loops and checklists defined in:
👉 **[INSTRUCTIONS.md](INSTRUCTIONS.md)**

## Decision Resolution Hierarchy

When faced with ambiguity, resolve decisions in this descending order:
1.  **ADRs**: Binding architectural constraints.
2.  **CONSTITUTION**: The philosophy of collaboration.
3.  **FORMAL RULES**: The engine's operational invariants.
4.  **ARCHITECTURE**: Source layout and technical boundaries.
5.  **PLANNING**: PRD/SRS/SDD authored for the current mission.

## Foundational Documents

These define the constraints and workflow of the Keel environment:

- `README.md` — Entrypoint and canonical document navigation.
- `INSTRUCTIONS.md` — Step-by-step procedural loops and checklists.
- `FORMAL_RULES.md` — Operational invariants and engine constraints.
- `CONSTITUTION.md` — Collaboration philosophy and decision hierarchy.
- `ARCHITECTURE.md` — Implementation architecture and flow model.
- `STAGE.md` — Visual philosophy and scene rendering.
- `PROTOCOL.md` — Communications protocol and data contracts.
- `CONFIGURATION.md` — Role-based and config-driven topology.
- `RELEASE.md` — Release process and artifacts.
- `.keel/adrs/` — Binding architecture decisions.

Use this order when interpreting constraints: ADRs → Constitution → Formal Rules → Architecture → Configuration → Planning artifacts.
