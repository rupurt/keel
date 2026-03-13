# Strict Deterministic Board Generation - Charter

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Make `keel generate` strictly deterministic and scoped so repeated runs on unchanged input never introduce unrelated whitespace churn or incidental edits outside the generated frontier. | board: VDiHwGwe5 |

## Constraints

- Keep generation deterministic across filesystem ordering, repeated runs, and equivalent board states.
- Confine rewrites to generated surfaces and avoid incidental markdown normalization outside ownership boundaries.
- Prefer graph-scoped regeneration over whole-board rewrites wherever ownership is knowable.
- Back the behavior with doctor and regression coverage rather than relying on human inspection alone.

## Halting Rules

- DO NOT halt while `keel generate` can still rewrite unrelated artifacts or introduce whitespace-only churn on stable inputs.
- HALT when generation is deterministic, scoped, and free from incidental edits across repeated runs.
- YIELD to human if reaching strict determinism requires a one-time repo-wide artifact normalization pass.
