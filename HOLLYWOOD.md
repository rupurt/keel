# HOLLYWOOD.md

> "A broken workshop is a messy workshop!"

This document codifies the design philosophy and storytelling expertise required to build and maintain high-fidelity visual metaphors within the Keel CLI. 

## The Philosophy of Tactile Diagnostics

In Keel, a diagnostic report is not just data—it is a **Physical State**. By projecting system metrics into tangible metaphors, we reduce cognitive load and create an emotional connection to the board's health.

### 1. Metaphor Mapping
Connect abstract system metrics to physical objects that share their properties:
*   **Liquidity/Ready Work** → Power/Batteries (The Circuit).
*   **Blocked Dependencies** → Tension/Clamps (The Vice).
*   **Structural Drift** → Entropy/Dust (The Sawdust).
*   **Remediation Effort** → Lubrication/Maintenance (The Oil Can).
*   **Architectural Tools** → Storage/Inventory (The Pegboard).

### 2. The Three Dimensions of Fidelity
When creating a `--scene`, aim for these layers of detail:
1.  **Structural**: Use ASCII borders and boxes to define the space (The Vault, The Workbench).
2.  **Functional**: Map data to dynamic elements (Battery bars, WIP occupancy).
3.  **Atmospheric**: Add "furniture" that builds personality but remains grounded (Oil cans, Anvils, Combination locks).

### 3. State-Driven Dynamics
Visuals MUST reflect the data accurately. A "High Fidelity" scene that lies to the user is a failure of integrity.
*   **Energized**: The "Light" should only be ON if the system has a recent heartbeat.
*   **Healthy**: Blown capacitors or sparks should only appear if the `doctor` reports errors.
*   **Idle**: Dim the visuals (Grey/Dim) when the system is off the clock or unplugged.

## Technical Standards

### Color Palette
*   **Yellow/Cyan**: Active energy, heat, and progress.
*   **Green**: Assets, solvency, and health.
*   **Red**: Blockages, debt, and broken tools.
*   **Dim/Grey**: Background furniture, sawdust, and idle states.

### Composition
*   **Width**: Target a maximum of **60-80 characters** to fit in terminal splits.
*   **Height**: Limit to **10-15 lines** to avoid scrolling.
*   **Transitions**: Ensure the scene appears at the *top* of the output, followed by actionable directives.

## The Creator's Checklist
When adding a new metaphor:
1.  Is the object intuitively linked to the metric?
2.  Does the visual state change based on real board data?
3.  Is there "sawdust" or "noise" that reflects system entropy?
4.  If the user's lights are on, is the scene bright?

---

*Keep the shop clean, the circuit hot, and the story grounded in reality.*
