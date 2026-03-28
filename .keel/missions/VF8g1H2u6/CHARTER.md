# Converge Product Narrative And Engine Contracts - Charter

Archetype: Strategic

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Establish a canonical command catalog that makes the CLI taxonomy in the public docs authoritative for help output, capability guidance, scene discovery, and future docs generation. | board: VF8hiVofm |
| MG-02 | Introduce first-class turn and scene projections so Keel's visual surfaces render canonical engine state instead of duplicating product logic across CLI commands. | board: VF8hiVofm |
| MG-03 | Promote roles and lanes from mostly internal routing logic to explicit, inspectable product surfaces with explainable next-step behavior. | board: VF8hiVofm |
| MG-04 | Convert the strongest narrative claims in the docs into executable invariants so product drift becomes test failure instead of silent divergence. | board: VF8hiVofm |

## Constraints

- Treat the current docs narrative as the intended product contract unless an implementation constraint forces a deliberate design decision.
- Prefer single metadata and projection sources that multiple surfaces can consume over parallel taxonomies in help text, docs, and guidance code.
- Keep scene semantics honest: visual surfaces may compress state, but they must not invent or reinterpret canonical engine signals.
- Preserve the onboarding-first docs story while strengthening the engine underneath it; this mission is about convergence, not expanding scope into a second documentation rewrite.
- Decompose the work into implementation boards that can land incrementally without breaking existing commands, docs navigation, or current scene affordances.

## Halting Rules

- DO NOT halt while command taxonomy, turn/scene semantics, roles-and-lanes explainability, or narrative invariants remain only implied in prose with no concrete implementation path.
- HALT when epic `VF8hiVofm` closes with implemented command catalog convergence, turn/scene projections, role-and-lane surfaces, and narrative contract tests.
- YIELD to human if making the docs narrative authoritative would require changing the public meaning of turns, scenes, or role routing rather than clarifying the current system.
