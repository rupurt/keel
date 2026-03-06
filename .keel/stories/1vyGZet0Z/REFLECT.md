---
created_at: 2026-03-05T17:42:28
---

# Reflection - Keep Fresh Epic Scaffolds Doctor Clean

## Knowledge

- [1vyKD7naI](../../knowledge/1vyKD7naI.md) Keep scaffold templates compliant with day-zero doctor contracts

## Observations

The failing regression initially looked like a doctor drift bug, but the real issue was that the fresh epic PRD template no longer satisfied the authored-content contract after the problem-only CLI cutover.

Fixing the scaffold at the source kept doctor strict and made the regression coverage more valuable because it now exercises the exact generated artifact shape users see.
