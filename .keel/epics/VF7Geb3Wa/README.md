---
# system-managed
id: VF7Geb3Wa
created_at: 2026-03-27T17:11:39
# authored
title: Derive Heartbeat From Repository Activity
mission: VF7AnKCka
index: 107
---

# Derive Heartbeat From Repository Activity

> Keel still treats a synthetic .keel/heartbeat file as the pacemaker signal for recent work even though Git state and worktree changes are the real activity source. That file adds ritual, hides the actual governor controls, and makes flow, hooks, and docs tell a less coherent story than the engine now needs.

## Documents

| Document | Description |
|----------|-------------|
| [PRD.md](PRD.md) | Product requirements and success criteria |
| `PRESS_RELEASE.md` (optional) | Working-backwards artifact for large user-facing launches; usually skip for incremental/refactor/architecture-only work |

## Voyages

<!-- BEGIN GENERATED -->
**Progress:** 2/2 voyages complete, 6/6 stories done
| Voyage | Status | Stories |
|--------|--------|---------|
| [Introduce Derived Heartbeat Surface And Flow Fallback](voyages/VF7Gfk7zv/) | done | 3/3 |
| [Remove File Heartbeat And Align Pacemaker Operations](voyages/VF7Gfkizo/) | done | 3/3 |
<!-- END GENERATED -->
