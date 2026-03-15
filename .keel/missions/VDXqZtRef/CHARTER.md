# Keel Lib Interface and Hexagonal Refactor - Charter

Archetype: Strategic
## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Define Storage Ports (traits) for Board and Entity operations. | board: VDXBSiFXW |
| MG-02 | Refactor Application Services to depend on Ports rather than concrete paths. | board: VDXBU7W4O |
| MG-03 | Implement FileSystem adapter for Storage Ports. | board: VDXBUAn7a |
| MG-04 | Expose stable library API in `lib.rs` for embedding. | board: VDXBUEBAG |
| MG-05 | Add `keel.toml` configuration for storage backend selection. | board: VDXBUHZB0 |

## Constraints

- Maintain zero-config local filesystem defaults.
- No breaking changes to the existing `.keel/` directory format.
- `lib.rs` must not leak CLI dependencies (e.g., `clap`).
- Application services must remain agnostic of the underlying storage implementation.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human when only `metric:` or `manual:` goals remain
