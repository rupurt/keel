# Keel: The Agentic SDLC Simulator

> **Minimize drift through planning, execution, and verification.**

Welcome to Keel. This isn't just a project management tool; it’s an engine designed for the era of human-agent collaboration. Keel treats software development as a high-fidelity simulation where **Formal Rules** act as the physics and **Play** is the primary mode of discovery.

---

## 🎮 The Ramping Path

Keel is designed to meet you where you are in your journey of comprehension. As you interact with the engine, you naturally level up through three distinct roles:

### 1. The Fixer (Learning by Healing)
**Comprehension:** Low
**The Move:** `keel doctor`
Start here. The engine will tell you exactly where the board is "broken" (Scaffold Drift, Structural Drift). By fixing these objective issues, you learn the structural invariants of the system without needing to understand the full architecture yet.

### 2. The Operator (Learning by Building)
**Comprehension:** Medium
**The Move:** `keel mission next --status`
Once the board is healthy, you move into implementation. You pull stories, record evidence, and close the **Specification-Evidence Loop**. You learn how requirements flow from planning into verified code.

### 3. The Architect (Learning by Constraining)
**Comprehension:** High
**The Move:** `keel adr new` / `keel voyage plan`
At the highest level, you define the physics of the sandbox. You author the Architecture Decision Records (ADRs) and tactical plans (SRS/SDD) that constrain how agents and other operators execute.

---

## 🎭 Discovery through Play

We believe that planning should be preceded by exploration. Keel encourages **Play** as a first-class citizen to reduce the "fog of war" before requirements are frozen.

- Use `keel play --theater` to launch narrative discovery sessions.
- Cross-pollinate ideas with `keel play --cross <id1> <id2>`.
- Let the **Theater Personas** (Shakespeare, Stand-up, Action) help you look at a technical problem through a different mask.

---

## ⚖️ The Physics: Formal Rules

Keel is governed by a strict set of operational invariants. These rules ensure that as the simulation grows in complexity, it never drifts into chaos.

### Document Hierarchy
Use this order when authoring or reviewing decisions:
1. ADRs (`.keel/adrs/`) — binding architectural decisions
2. [CONSTITUTION.md](CONSTITUTION.md) — collaboration philosophy and governance intent
3. [FORMAL_RULES.md](FORMAL_RULES.md) — operational invariants and engine constraints
4. [ARCHITECTURE.md](ARCHITECTURE.md) — implementation structure and technical constraints
5. [CONFIGURATION.md](CONFIGURATION.md) — role-based and config-driven topology
6. [RELEASE.md](RELEASE.md) — release capabilities and overview
7. Planning artifacts (`PRD.md` → `SRS.md`/`SDD.md` → story `README.md`) — scoped executable work

- **[AGENTS.md](AGENTS.md)**: The tactical loop for AI contributors.

---

## 🚀 Quick Start

1. **Install:** `nix run github:spoke-sh/keel`
2. **Orient:** `keel mission next --status`
3. **Heal:** `keel doctor`
4. **Play:** `keel play --theater`

**Everything flows down:** Vision → Epic → Voyage → Story → Implementation.
**Everything loops back:** Reflection → Knowledge → Patterns → Bearings → Architecture.

---

*"The goal is not to automate humans out of the loop, but to place human judgment where it is irreplaceable."*
