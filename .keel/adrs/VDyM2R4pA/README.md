---
id: VDyM2R4pA
index: 3
title: Engine Physics Metaphor
status: proposed
context: null
applies-to: []
supersedes: []
superseded-by: null
decided_at: 2026-03-15T14:07:42
---

# Engine Physics Metaphor

## Status

**Proposed** — Awaiting human acceptance. Work in governed context is blocked.

## Context

As Keel evolves into a multi-agent workflow engine, we need an intuitive mental model to describe the state of the system, its capacity for work, its health, and its readiness to interact with the outside world. This model needs to go beyond standard queues and backlogs to represent the "energy" and "safety" of the autonomous engine. We've begun using ASCII visual scenes (like `keel flow --scene` and `keel doctor --scene`) to communicate this, and we need a formal contract for what those physical metaphors represent.

## Decision

We adopt an **Electrical Circuit & Physiological** metaphor for Keel's systemic state. The core primitives are:

1. **The Battery (Strategic Energy):**
   - Recharged by completing strategic capacity (e.g., closing Voyages and Epics) or by any state-mutating activity in the board.
   - Provides the energy required to run the automated engine loop. 
   - A single battery decays over time (configurable `battery_decay_minutes`), returning the system to idle if it is not receiving active attention.

2. **Battery Packs (Queues):**
   - Queues of ready work represent additional battery packs plugged into the engine. 
   - If too many battery packs (unbounded queues) get plugged in simultaneously, it creates a risk of **Circuit Overload**, requiring system governors to shed load or increase limits.

3. **The Capacitor (Human Attention / Burst Capacity):**
   - Represents the buffer for human verification and interaction.
   - When the project accumulates excess energy (more batteries than it can hold), it discharges to power external signals (see Lighthouse).

3. **The Circuit Breaker (WIP Limit):**
   - Represents a Mission Work-In-Progress (WIP) limit. Currently set to 1, acting as a safety switch.
   - If the system attempts to run too many concurrent, uncoordinated missions, the breaker trips to prevent systemic drift.

4. **The Main Switch (Intent):**
   - An "open for work" concept. If the switch is off, the circuit is physically broken, and no autonomous action will take place regardless of battery charge.

5. **The Illuminators (Signaling):**
   - **Lightbulb (Local):** Illuminates when the engine is operating autonomously (`keel flow --scene`). It dims to indicate an "afterglow" (idle, but recently completed work), and turns off when human input is blocking the circuit.
   - **Lighthouse (External/Network):** Powered when the project has excess capacity and is ready to collaborate. Signals through Keel's comms layer (`ping`/`poke`) that this node is available to assist or coordinate with other projects.

6. **The EKG (System Health):**
   - Grounded in `keel doctor`. If the heart stops (errors in the board state), the electrical circuit instantly blows its capacitors (sparks), physically halting the flow. When work volume increases, the EKG registers Tachycardia.

## Constraints

- **MUST:** Connect visual output in `keel flow --scene` and `keel doctor --scene` directly to these measurable domain metrics.
- **MUST:** Require human intervention (or automated self-healing) to reset the circuit if the Breaker trips or the Capacitors blow due to health errors.
- **SHOULD:** Integrate the Lighthouse concept into the `ping`/`poke` communications protocol to allow inter-project signaling based on internal capacity.

## Consequences

### Positive
- Provides a visceral, easily understandable abstraction for complex queuing and workflow metrics.
- Naturally bridges local autonomy (Lightbulb) with multi-agent network collaboration (Lighthouse).
- Tightens the coupling between system health (Doctor) and system action (Flow).

### Negative
- Requires maintaining abstract visual presentation logic alongside raw diagnostic data.

## Verification

| Check | Type | Description |
|-------|------|-------------|
| Scene Alignment | automated | Unit tests ensure the `flow --scene` correctly maps metrics to the appropriate Lightbulb/Capacitor ASCII representation. |
| Health Coupling | automated | Tests verify that a failing `keel doctor` state reliably produces a "blown circuit" visualization. |

## References

- Discussion on `keel flow --scene` and asynchronous comms.
- `STAGE.md` - The Visual Philosophy of Keel.
