# Keel

[![Keel Board](https://img.shields.io/badge/Keel-Board-blue)](.keel/README.md)
[![CI](https://github.com/rupurt/keel/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/rupurt/keel/actions/workflows/ci.yml)

Agentic SDLC management — minimize drift through planning, execution, and verification.

## The Problem

When AI agents implement features, they drift from specifications. Small misunderstandings compound. By the time code is reviewed, the implementation may solve a different problem than intended.

Traditional project management tools don't help — they're designed for humans reading dashboards, not agents parsing context.

## Verified Spec Driven Development (VSDD)

Keel introduces **Verified Spec Driven Development (VSDD)** — a methodology that treats the specification (SRS/SDD) as an executable contract. 

In VSDD, a feature is not "done" when the code is written; it is done when the **Specification-Evidence Loop** is closed. Every requirement in the spec must be satisfied by a machine-verifiable proof (command output, test result, or LLM-judged signal) recorded as immutable evidence.

This shifts the focus from "tracking tasks" to "verifying outcomes," ensuring that agents and humans remain perfectly aligned with the authored intent.

## How Keel Helps

Keel structures work into a **research → planning → execution → verification → learning** loop that catches drift early and builds long-term intelligence:

| Phase | What Happens | Drift Prevention (VSDD) |
|-------|--------------|-------------------------|
| **Research** | Bearings and play-driven exploration run before planning | Ambiguity is reduced before requirements are frozen |
| **Planning** | Requirements captured in PRD → SRS → Stories | Specifications are explicit and traceable |
| **Execution** | Stories track implementation with acceptance criteria | Work stays scoped to what was planned |
| **Verification** | Evidence recorded for every requirement | Outcome is verified against the spec before transition |
| **Learning** | Navigator surfaces trends and thematic rising patterns | Past mistakes inform future research and ADRs |

**Everything flows down**: Vision → Epic → Voyage → Story → Implementation → Reflection.

**Everything loops back**: Reflection → Knowledge → Patterns → Bearings → Architecture.

## Foundational Document Flow

Use this order when authoring or reviewing decisions:

1. ADRs (`.keel/adrs/`) — binding architectural decisions
2. [CONSTITUTION.md](CONSTITUTION.md) — collaboration philosophy and governance intent
3. [FORMAL_RULES.md](FORMAL_RULES.md) — operational invariants and engine constraints
4. [ARCHITECTURE.md](ARCHITECTURE.md) — implementation structure and technical constraints
5. [CONFIGURATION.md](CONFIGURATION.md) — role-based and config-driven topology
6. [RELEASE.md](RELEASE.md) — release capabilities and overview
7. Planning artifacts (`PRD.md` → `SRS.md`/`SDD.md` → story `README.md`) — scoped executable work

## Core Concepts & Architecture

Keel's architecture is built on formal state machines and a pull-based coordination model. See [ARCHITECTURE.md](ARCHITECTURE.md) for full details and onboarding diagrams (layer dependencies, command execution, queue lifecycle).

### Workflow Lane Dashboard

Keel routes work through configurable workflow lanes using a **pull-based** model. Each role family resolves to a lane based on the `keel.toml` topology, and `keel flow` renders the effective topology in priority order. With no overrides, Keel seeds `management` and `delivery` lanes.

```
┌───────────────────────────────────────┬──────────────────────────────────────┐
│         MANAGEMENT LANE               │           DELIVERY LANE              │
├───────────────────────────────────────┼──────────────────────────────────────┤
│  bearing.*                     ...    │  story.backlog                ...    │
│  story.needs-human-verification ...   │  story.in-progress            ...    │
│  voyage.draft                  ...    │                                      │
└───────────────────────────────────────┴──────────────────────────────────────┘
```

- `keel next --role manager` returns management-lane decisions and never returns implementation `Work`.
- `keel next --role operator` returns implementation work from the delivery lane (`in-progress` then `backlog`).
- `keel next` requires `--role`; there is no implicit manager default.
- `keel flow` uses the same queue policy categories and thresholds as `next` while rendering lane cards from the resolved topology.
- Topology is fully configurable via `keel.toml`. See [CONFIGURATION.md](CONFIGURATION.md) for details.

### Lineage and Proof Chain

Keel is designed to make drift visible by preserving a machine-checkable lineage chain across planning and execution:

- Epic `PRD.md` defines the problem, canonical `GOAL-*` rows, canonical `[SCOPE-*]` bullets, and `FR-*` / `NFR-*` requirement rows.
- Voyage `SRS.md` maps each requirement back to that plan with explicit `Scope` (`SCOPE-*`) and `Source` (`FR-*` / `NFR-*`) lineage.
- Story acceptance criteria link back to voyage requirements (`[SRS-XX/AC-YY]`) so implementation work stays grounded in the authored plan.
- Proofs, verification manifests, and reflections close the loop so every accepted story has evidence and every reflection can feed back into reusable knowledge.

That chain is what powers drift prevention. `keel doctor`, `keel audit`, and the `show` surfaces do not just render prose; they validate and summarize whether goals, scope, requirements, acceptance criteria, and proofs still line up.

### Read Models and Steering Surfaces

Markdown files are the source of truth, but agents should not need to reread the whole board on every step. Keel aggregates authored artifacts into read models and summarized CLI surfaces such as:

- `keel epic show`, `keel voyage show`, and `keel story show` for scoped planning and execution context
- `keel next --role <role>` and `keel flow` for queue steering
- `keel topology` for a board-wide spatial map of strategic and tactical relationships
- `keel audit` for traceability and proof review
- `keel knowledge ...` for institutional memory and repeated implementation signals

This keeps the workflow agent-friendly: the CLI provides compact, deterministic summaries for orientation, while still rendering the underlying artifact paths (`PRD.md`, `SRS.md`, `SDD.md`, story `README.md`, and more) when full authored detail is needed.

### Topology World Map

`keel topology` renders the board as a `subtree-weighted orbit map`.

- The center node is the board world.
- Concentric rings represent progressively deeper entity layers.
- Angular span is allocated by subtree weight, so larger branches claim more space.
- `--zoom` reveals deeper layers from `world` through `story`.
- `--focus <id>` filters the map to one branch while preserving the same spatial metaphor.
- On a TTY, `keel topology` is interactive by default; use `--static` for harnesses, logs, and snapshots.

This is meant to be readable as a systems map, not a list dump: the same branch stays visually coherent as you zoom in, and dense areas of the board naturally occupy more of the orbit.

For recurring-work automation specifically, see [GUIDE.md](GUIDE.md) for the
routine authoring, `next`, `flow`, and `pulse` workflow.

### Detection and Verification Techniques

Verification is modeled as a technique bank rather than a single hardcoded test path. Keel supports built-in and custom verification techniques, and the detection engine evaluates project signals such as files, stack hints, and configured commands to determine which techniques are:

- `detected`: relevant for the current project
- `disabled`: configured off
- `active`: both detected and enabled

The main command surfaces are:

- `keel config show` for the full technique inventory and per-technique status
- `keel verify detect` for detection signals and status inputs
- `keel verify recommend` for advisory-only detected+active techniques
- `keel verify run` for actual proof execution

This separation keeps planning, recommendation, and execution distinct while making it straightforward to extend Keel with additional verifiers over time.

### Throughput and Estimation

Keel also uses board history to reason about delivery pace:

- `keel throughput` shows weekly throughput and timing sparklines
- `keel epic show` uses a recent 4-week throughput window to estimate ETA when enough data exists

That gives planners and agents a lightweight estimation surface without leaving the same markdown-backed workflow.

## Commands

```
$ keel --help
Agentic SDLC management — minimize drift through planning, execution, and verification

Usage: keel

Options:
  -h, --help     Print help
  -V, --version  Print version


These are common Keel commands:

Setup
  init        Initialize a new keel board in the current directory
  config      Configuration commands
  generate    Regenerate all README files

Management
  next        Pull the next item using explicit role-based queue routing
  pulse       Run one non-interactive automation cycle
  topology    Show a zoomable world map of the board
  play        Invite play-driven discovery
  audit       Rich evidence/traceability report
  verify      Execute verification proofs
  knowledge   Manage institutional knowledge
  mission     Mission commands (long-running objectives)
  adr         ADR commands (architecture decisions)
  bearing     Bearing commands (research phase)
  epic        Epic commands
  routine     Routine commands
  voyage      Voyage commands
  story       Story commands (new, start, submit, accept, reject, ice, thaw, show, list, link, unlink, record, audit)

Diagnostics
  doctor      Validate board health and optionally fix issues
  flow        Show workflow lane dashboard from configured topology
  throughput  Show weekly throughput and timing sparklines
```

### Command Groups

#### Setup

| Command | Purpose |
|---------|---------|
| `init` | Initialize a new keel board |
| `config show` | Display current configuration |
| `config mode <name>` | Switch CLI modes (e.g., standard vs agent) |
| `generate` | Regenerate all board-level README files |

#### Management

| Command | Purpose |
|---------|---------|
| `next` | Pull from the lane mapped to `--role`; there is no implicit default role |
| `pulse` | Run one non-interactive automation cycle |
| `topology` | Render the board as a subtree-weighted orbit map with zoom and focus controls |
| `play` | Trigger play-driven discovery for a bearing |
| `audit` | Generate a detailed traceability report for a story |
| `verify run/recommend/detect` | Execute proofs, inspect detection signals, and review detected+active verification guidance |
| `knowledge list/show` | Inventory and details of implementation insights |
| `knowledge explore` | Surface "Rising Patterns" and thematic trends |
| `knowledge graph` | Visualize connections between insights and entities |
| `knowledge impact` | Track drift risk and institutionalization progress |
| `mission show/next/verify` | Mission steering and long-running objective management |
| `adr new/accept/supersede` | Architecture Decision Record lifecycle |
| `bearing new/research/lay` | Research and exploration lifecycle |
| `epic new/done/reopen` | Strategic grouping and PRD management |
| `routine new/show/list` | Recurring work definitions and automation context |
| `voyage new/plan/start` | Tactical planning (SRS/SDD) and execution |
| `story new/start/submit` | Implementation units and acceptance criteria |

Story creation flags:

- `keel story new "<Title>"` creates an unscoped story.
- `keel story new "<Title>" --type feat --epic <epic-id>` creates an epic-scoped story.
- `keel story new "<Title>" --type feat --epic <epic-id> --voyage <voyage-id>` creates a voyage-scoped story.
- `--voyage` requires `--epic`; `--type` defaults to `feat`.

#### Diagnostics

| Command | Purpose |
|---------|---------|
| `doctor` | Validate board health and fix consistency issues |
| `flow` | Real-time dashboard of Human vs. Agent queues |
| `throughput` | Show recent weekly throughput and timing sparklines |

### Harness Guidance Contract

Harness integrations should consume canonical command guidance from management command responses using an optional `guidance` object.

| Field | Type | Meaning |
|-------|------|---------|
| `guidance.next_step.command` | `string` | Single canonical follow-up command for a successful actionable outcome. |
| `guidance.recovery_step.command` | `string` | Single canonical recovery command for a blocked/failed actionable outcome. |

Contract rules:

1. `guidance` is emitted only for actionable commands.
2. Informational commands omit `guidance` entirely.
3. Exactly one step type is emitted when guidance exists: `next_step` or `recovery_step` (never both).
4. Command strings are canonical, copy-paste-ready `keel ...` commands with explicit IDs/flags.
5. Single canonical next-step rule: Keel emits one deterministic command even when multiple follow-ups could be valid.

Capability classification:

- Actionable: ADR transitions (`adr accept/reject/deprecate/supersede`), bearing lifecycle transitions (`bearing new/research/assess/park/decline/lay`), guided play suggestion (`play --suggest`), story-scoped verification (`verify <story-id>`), story-scoped audit (`audit <story-id>`).
- Informational: read/list commands (`adr list/show`, `bearing list/show`) and exploratory play outputs (`play`, `play --list-props`, `play <bearing>`, `play --cross`).

Examples (minimal contract snippets):

```json
{
  "guidance": {
    "next_step": {
      "command": "keel story submit 1vxZ0FtD2"
    }
  }
}
```

```json
{
  "guidance": {
    "recovery_step": {
      "command": "keel story audit 1vxZ0EXHC"
    }
  }
}
```

```json
{
  "type": "informational",
  "result": "no-action-required"
}
```

## Installation

### Using Nix Flakes

If you use Nix, you can add Keel to your `flake.nix` inputs:

```nix
{
  inputs = {
    keel.url = "github:rupurt/keel";
  };

  outputs = { self, nixpkgs, keel, ... }: 
    let
      forAllSystems = nixpkgs.lib.genAttrs [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    in {
      devShells = forAllSystems (system: {
        default = nixpkgs.legacyPackages.${system}.mkShell {
          buildInputs = [
            keel.packages.${system}.default
          ];
        };
      });
    };
}
```

Or run it directly without installing:

```bash
nix run github:rupurt/keel
```

## Development

```bash
just build    # Compile the project
just test     # Run all unit and integration tests
just quality  # Run formatting and linting checks
```
