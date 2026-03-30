# Cinematic Verification Playback - Charter

Archetype: Voyage

## Goals

| ID | Description | Verification |
|----|-------------|--------------|
| MG-01 | Integrate atxt-core library with streaming support and frame-accurate timing. | board: VFOKwZazq |
| MG-02 | Implement "Theater Mode" in txt-scene with borders and centered alignment. | board: VFOKwgN0l |
| MG-03 | Add an interactive "Final Review" gate to `keel mission verify` requiring user sign-off after playback. | board: VFOKwnF2e |

## Constraints

- Playback must occur entirely within the terminal buffer (no external processes).
- The playback must be interruptible (e.g., via `q` or `Ctrl-C`).
- Framing must adapt gracefully to terminal resizing.

## Halting Rules

- DO NOT halt while any MG-* goal has unfinished board work
- HALT when all MG-* goals with `board:` verification are satisfied
- YIELD to human sign-off once the high-dimension proof is playing successfully.
