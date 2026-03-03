### Note 001: Documentation should encode policy names and boundaries, not just numbers
Using canonical policy constants (`HUMAN_NEXT_VERIFY_BLOCK_THRESHOLD`, `FLOW_VERIFY_BLOCK_THRESHOLD`) in architecture docs keeps behavior and narrative aligned through future threshold changes.

### Note 002: Drift tests should pin command docs to mode contracts
Asserting README/help text for `keel next` human vs agent semantics prevents subtle messaging drift that can reintroduce queue-boundary confusion.
