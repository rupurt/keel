# Dogfood Artifact Board

This secondary board owns the persisted evidence for local dogfood VHS scenarios.

- Tape sources live under `testdata/dogfood/scenarios/`.
- `keel dogfood run --scenario <name>` writes rendered artifacts into the owning story `EVIDENCE/`.
- Generated `EVIDENCE/` content and `manifest.yaml` files are intentionally gitignored.
