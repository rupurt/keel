# PROTOCOL.md: The Keel Communications Protocol

The Keel CLI implements an asynchronous communication layer through the `ping` and `poke` commands. This system allows the workflow engine to field requests, route messages, and provide either synchronous automated responses or facilitate asynchronous offline interactions.

This document defines the expected message structures, the routing logic, and the lifecycle of an inbox message.

## 1. The Inbox Lifecycle

All communication is routed through the `.keel/inbox/` directory.

1. **Submission:** A user or agent invokes `keel ping "<message>"`.
2. **Evaluation:** The engine evaluates the message against its **Routing Rules**.
3. **Response (Sync vs. Async):**
   - **Synchronous (`pong`):** If a routing rule matches, the engine immediately responds via `stdout`, marks the message as `ponged`, and saves the state to the inbox.
   - **Asynchronous (Pending):** If no routing rule matches, the engine returns only the generated **Ping ID** (e.g., `VDtzUxoCp`) to `stdout`. The message is saved to the inbox in a `pending` state.
4. **Resolution (`poke`):** A pending message can be resolved later using `keel poke <id> "[message]"`.
   - If a manual message is provided, it acts as the response, marking the ping as `ponged`.
   - If no manual message is provided, the engine re-evaluates the original message against the current routing rules (which may have been updated).

## 2. Message Format

Messages are persisted as JSON files within `.keel/inbox/<id>.json`.

**Schema (`PingMessage`):**
```json
{
  "id": "VDtzUxoCp",
  "message": "The original message content.",
  "timestamp": "2026-03-15T03:00:00Z",
  "status": "pending | ponged",
  "pong_message": "The response message, or null if pending."
}
```
- **`id`**: A globally unique identifier generated using Keel's standard ID generation (e.g., `VD...`).
- **`status`**: The current state of the interaction (`pending` or `ponged`).
- **`pong_message`**: The recorded response, ensuring historical traceability of the communication.

## 3. Routing Rules

When a `ping` is executed, the engine attempts to match the message content to a set of predefined rules to trigger a synchronous **auto-pong**.

Currently, the routing rules are simple substring matches (case-insensitive):

| Match Condition (contains word) | Synchronous Response (Pong) |
| :--- | :--- |
| `"ping"` | `pong` |
| `"hello"` or `"hi"` | `Hello! I am keel. How can I help?` |
| `"help"` | `I am a workflow engine. Try running `keel doctor` or `keel flow`.` |

*If a message does not match any of these conditions, it falls through to the Asynchronous track and requires a future `poke`.*

## 4. JSONIN and JSONOUT Data Contracts

As a primary interface for autonomous agents and external scripts, Keel implements strict `JSONOUT` and potential `JSONIN` data contracts. These are separate from human-readable CLI outputs and guarantee structured predictability.

### JSONOUT: Engine to Agent
Whenever a command is invoked with `--json` (e.g., `keel pulse --json`, `keel next --json`, `keel verify run --json`), the standard out is guaranteed to be a single, parseable JSON payload representing the complete return state.

- **Predictable Schemas**: Changes to `--json` output schemas must be treated as breaking changes. 
- **Example (`keel pulse --json`)**:
  ```json
  {
    "mode": "materialize",
    "evaluated": 3,
    "created": 1,
    "skipped": 0,
    "deferred": 2,
    "routines": [
      {
        "id": "routine-due",
        "outcome": "created",
        "story_id": "VDtx8IW2K"
      }
    ]
  }
  ```

### JSONIN: Agent to Engine
While most inputs to Keel are simple strings and flags (like `keel ping "hello"`), the protocol anticipates `JSONIN` payloads for complex configurations, bulk operations, or structured responses (like piping an LLM's structured JSON output directly to a state-mutating command).

- **The Inbox as JSONIN**: The `.keel/inbox/<id>.json` file itself represents our first JSONIN construct. When an agent creates or modifies a ping file directly, it must conform to the `PingMessage` schema defined above.
- **Future Support**: We plan to support piping JSON directly into commands (e.g., `cat payload.json | keel poke <id> --json-in`) to allow agents to pass rich contextual data structures instead of flat strings.

## 5. The System Pacemaker

The system's **Pacemaker** is derived from repository activity rather than a dedicated file. `keel heartbeat` is the inspection surface for that derived signal.

### The Heartbeat
- **Activation**: Dirty worktree activity is the primary signal; a clean repository falls back to the latest commit timestamp.
- **Inspection**: `keel heartbeat` reports the source, age, and whether the worktree is carrying uncommitted energy.
- **Turn Loop**: `Orient` uses `keel heartbeat` as the canonical charge surface before `flow --scene` and other visual checks are interpreted.
- **Idle State**: If the derived heartbeat decays beyond `battery_decay_minutes` (default: 10m), the engine transitions to **IDLE**, dimming the visual scenes and pausing autonomous backlog discharge. Idle heartbeat is a flow-state signal, not structural drift.
- **Flow Override Gate**: `keel flow` may keep the circuit open during transitional mission intake only while `keel heartbeat` still proves recent activity. An idle heartbeat removes that exception.

### Pace-setting
To maintain board integrity, the pacemaker should still be synchronized with the commit boundary.
- **The Protocol**: Land sealing commits to close dirty worktree energy, and rely on the installed hooks to keep quality checks and tests attached to that boundary.
- **Consistency**: A dirty worktree is warning-level evidence that the loop has been opened but not yet sealed. The commit itself clears that warning by aligning the repository state with the resulting board state.

## 6. Formal Mission Request Boundary

`ping` and `poke` are conversational surfaces. Mission requests are structured
planning ingress.

This is the documented boundary for external providers and runtimes such as
Keeper. Direct planning commands remain the normal path today, but the
cross-runtime protocol should already be treated as stable in shape.

### First Provider Shape

The first documented provider is GitHub issues.

A GitHub issue is a formal mission request candidate when:
- the title begins with `Keel Mission Request:`
- the body contains the required structured sections
- the provider metadata can be preserved for replay and acknowledgement

### Canonical Envelope

Mission requests should normalize into a provider-neutral envelope before they
touch planning state:

```yaml
version: 1
request:
  title: Add Keel Mission Request Feature
  summary: Add a native mission request workflow for Keeper and automation.
  problem: Operators need a canonical intake path that preserves board truth.
  outcome: Mission creation and acknowledgement become scriptable and replayable.
  constraints:
    - Keep Keel provider-neutral.
    - Preserve deterministic stdin/stdout composition.
  requested_scope:
    in_scope:
      - Native mission request command surface
      - Provider-neutral normalization rules
    out_of_scope:
      - Provider polling internals
  provider:
    kind: github-issue
    source_id: 1234
    source_url: https://github.com/spoke-sh/keel/issues/1234
    revision: 7
```

### Caller Field Responsibility

- Required caller-supplied fields: `version`, `request.title`,
  `request.summary`, `request.problem`, `request.outcome`, and
  `request.requested_scope.in_scope`
- Optional caller-supplied fields: `request.constraints` and
  `request.requested_scope.out_of_scope`
- Derivable provider fields after normalization: `request.provider.kind`,
  `source_id`, `source_url`, and `revision`

### Processing Contract

The documented direction is a native `keel mission request ...` namespace with
parse, validate, draft, apply, and acknowledgement stages.

Until that surface ships, external runtimes should still preserve the same
contract:
1. Detect the formal provider request.
2. Normalize it into the canonical envelope.
3. Validate the request before mutating planning state.
4. Lower the validated request into ordinary Keel planning commands.
5. Render acknowledgement content separately from provider delivery.

### Stage IO And Failure Semantics

- `template`, `parse`, `validate`, `draft`, and `ack` are side-effect free.
- `apply` is the only stage that mutates `.keel` planning state.
- Validation failures are recoverable caller errors with actionable diagnostics
  and no mutation.
- Execution failures happen after validation and should be treated as runtime or
  policy failures rather than malformed input.
- The same normalized envelope should yield deterministic semantic results for
  automation callers unless the board state itself has changed.

### Provider Revision And Acknowledgement Rules

- Provider runtimes should bind every normalized request to an explicit replay
  identity, at minimum provider kind, source identity, and provider revision.
- Duplicate deliveries for an unchanged replay identity must not create a second
  planning mutation.
- Edited provider requests should produce a new normalized revision and a fresh
  Keel evaluation.
- Keeper or another runtime owns acknowledgement delivery and retries.
- Keel owns acknowledgement payload rendering from the normalized request and
  planning result.
- Acknowledgement retry failure must not be treated as license to replay an
  already-applied planning mutation.

### Responsibility Split

- **Keel** owns request semantics, planning mutation rules, and acknowledgement
  content contracts.
- **Keeper or another runtime** owns provider polling, authentication, revision
  tracking, retries, and provider-facing response delivery.

### Security Boundary

Mission request ingress is part of the multiplayer security model:
- replay metadata should stay attached to the normalized request
- stronger auditability should come from backend-agnostic append, checkpoint,
  inclusion-proof, and consistency-proof operations
- threshold attestation should be reserved for high-consequence transitions and
  published checkpoints, not for every local move

## 7. Mission Stack Coordination

Mission requests solve structured ingress into another board. A **Mission Stack**
defines what happens after that request is accepted when one outcome spans
multiple repositories, each with its own Keel board and reactor.

### Federated Board Rule

- A Mission Stack is a federation of independent Keel boards, not a shared
  multi-repo board.
- One reactor may act as the stack steward for coordination, but stewardship
  does not grant authority to mutate another repo's board directly.
- Each member repository remains authoritative for its own `.keel` state.
- One repository MUST NOT mutate another repository's planning artifacts
  directly from outside that repository's reactor.
- Cross-repo work begins with a mission request and reactor negotiation.
- A target reactor materializes its own local mission lineage after negotiation
  rather than accepting external `.keel` mutations from another repo.

### Stack Modes

Mission Stack uses explicit coordination modes:

- `exclusive(<repo>)` allows one active member repo at a time.
- `shared([repos...])` opens an explicit parallel execution window.
- `checkpoint(<name>, required_members...)` pauses forward progress until the
  named members acknowledge the integration boundary.

### Stack Branch And Handoff Rule

- Every stack member uses branch `stack/<id>` for stack-linked work.
- Local closure still happens at commit time inside the member repo.
- Inter-reactor handoff happens at push time and may remain git-native in the
  first version of the protocol.
- The minimum pushed receipt is stack id, member repo identity, branch, head
  sha, and optional checkpoint or role context.

### Coordination Sequence

The canonical Mission Stack handoff is:

1. local work proceeds inside the current member repo
2. local closure seals that work at commit time
3. the member pushes `stack/<id>` to publish the sealed result
4. the pushed receipt exposes stack id, repo identity, branch, head sha, and
   optional checkpoint or role context
5. the target reactor negotiates the next move and materializes any local
   mission lineage inside its own board
6. the target reactor acknowledges or integrates the receipt and either
   continues locally or yields at the next stack mode boundary

### Foreign Worktree Rule

When a reactor needs to execute in another stack member repository from the
outside, that work MUST happen in a managed git worktree.

- Foreign execution MUST NOT run in the member repo's primary checkout.
- The managed worktree MUST target `stack/<id>` or an explicitly approved
  stack-derived head from that branch.
- The worktree lifecycle MUST define create, reuse, inspection, and cleanup
  behavior for the lifetime of the open stack.
- Managed foreign worktrees SHOULD be garbage-collected when the stack closes.
- Cleanup MUST fail safe by reporting ambiguous leftovers rather than silently
  deleting uncertain state.

### Command And Hook Enforcement

The Mission Stack worktree rule is enforced at the same seams that already
govern turn execution and closure:

- command surfaces such as `turn`, `next`, `mission next`, `story start`, and
  `doctor` should explain or reject unsupported foreign execution
- git hooks should reject commits or pushes that violate the approved stack
  worktree and branch contract
- diagnostics should surface wrong-branch, unsupported-primary-checkout, or
  missing-cleanup states as stack violations rather than silent drift

## 8. Expanding the Protocol

As Keel's capabilities grow, the routing rules will be expanded to support more complex interactions:
- **Regex/Semantic Matching:** Moving beyond simple word inclusion to understand intent.
- **Action Triggers:** Allowing a `ping` to synchronously trigger engine operations (e.g., "ping: status" running `keel flow`).
- **Agent Handoffs:** Routing pending messages to specific sub-agents or workflow lanes for evaluation during `keel pulse`.
