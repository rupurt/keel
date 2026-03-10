---
created_at: 2026-03-10T13:52:40
---

# Reflection - Update Next Role Routing

## Knowledge

## Observations

The command/runtime cutover was spread across parser help text, runtime interception, queue-lane selection, and downstream guidance text. The tests were already close to the intended behavior; this slice mostly completed the hard cutover by removing the legacy `next` flags from the CLI contract and aligning docs, guidance, and proof annotations with the role-based path.
