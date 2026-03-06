# Keel

[![Keel Board](https://img.shields.io/badge/Keel-Board-blue)](.keel/README.md)
[![CI](https://github.com/rupurt/keel/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/rupurt/keel/actions/workflows/ci.yml)

Agentic SDLC management — minimize drift through planning, execution, and verification.

## The Problem

When AI agents implement features, they drift from specifications. Small misunderstandings compound. By the time code is reviewed, the implementation may solve a different problem than intended.

Traditional project management tools don't help — they're designed for humans reading dashboards, not agents parsing context.

## How Keel Helps

Keel structures work into a **research → planning → execution → verification → learning** loop that catches drift early and builds long-term intelligence:

| Phase | What Happens | Drift Prevention |
|-------|--------------|------------------|
| **Research** | Bearings and play-driven exploration run before planning | Ambiguity is reduced before requirements are frozen |
| **Planning** | Requirements captured in PRD → SRS → Stories | Specifications are explicit and traceable |
| **Execution** | Stories track implementation with acceptance criteria | Work stays scoped to what was planned |
| **Verification** | Doctor validates health, story reflections capture drift | Drift is detected before it compounds |
| **Learning** | Navigator surfaces trends and thematic rising patterns | Past mistakes inform future research and ADRs |

**Everything flows down**: Vision → Epic → Voyage → Story → Implementation → Reflection.

**Everything loops back**: Reflection → Knowledge → Patterns → Bearings → Architecture.

## Foundational Document Flow

Use this order when authoring or reviewing decisions:

1. ADRs (`.keel/adrs/`) — binding architectural decisions
2. [CONSTITUTION.md](CONSTITUTION.md) — collaboration philosophy and governance intent
3. [ARCHITECTURE.md](ARCHITECTURE.md) — implementation structure and technical constraints
4. Planning artifacts (`PRD.md` → `SRS.md`/`SDD.md` → story `README.md`) — scoped executable work

## Core Concepts & Architecture

Keel's architecture is built on formal state machines and a pull-based coordination model. See [ARCHITECTURE.md](ARCHITECTURE.md) for full details and onboarding diagrams (layer dependencies, command execution, queue lifecycle).

### The 2-Queue Pull System

Keel coordinates work between humans and agents using a **pull-based** model. Each actor pulls when ready — no push coordination needed.

```
┌───────────────────────────────────────┬──────────────────────────────────────┐
│           HUMAN QUEUE                 │            AGENT QUEUE               │
├───────────────────────────────────────┼──────────────────────────────────────┤
│  accept    → stories to review        │  backlog     → ready to start        │
│  start     → voyages to begin         │  in-progress → being worked          │
│  decompose → drafts need stories      │                                      │
│  research  → bearings to explore      │                                      │
└───────────────────────────────────────┴──────────────────────────────────────┘
```

- `keel next` (human mode) only returns human-queue decisions and never returns implementation `Work`.
- `keel next --agent` returns implementation work from the agent queue (`in-progress` then `backlog`).
- `keel flow` uses the same queue policy categories and thresholds as `next` (`>= 5` human block, `> 20` flow block).

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
- `keel next` and `keel flow` for queue steering
- `keel audit` for traceability and proof review
- `keel knowledge ...` for institutional memory and repeated implementation signals

This keeps the workflow agent-friendly: the CLI provides compact, deterministic summaries for orientation, while still rendering the underlying artifact paths (`PRD.md`, `SRS.md`, `SDD.md`, story `README.md`, and more) when full authored detail is needed.

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
  next        Surface the single most important thing to work on
  play        Invite play-driven discovery
  audit       Rich evidence/traceability report
  verify      Execute verification proofs
  knowledge   Manage institutional knowledge
  adr         ADR commands (architecture decisions)
  bearing     Bearing commands (research phase)
  epic        Epic commands
  voyage      Voyage commands
  story       Story commands

Diagnostics
  doctor      Validate board health and optionally fix issues
  status      Show board status summary
  flow        Show two-actor flow dashboard (human queue vs agent queue)
  throughput  Show weekly throughput and timing sparklines
  capacity    Show per-epic capacity breakdown with parallel potential
  gaps        Show gap classification summary (runs doctor, shows only gap counts)
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
| `next` | Pull from the human queue by default; use `--agent` to pull implementation work |
| `play` | Trigger play-driven discovery for a bearing |
| `audit` | Generate a detailed traceability report for a story |
| `verify run/recommend/detect` | Execute proofs, inspect detection signals, and review detected+active verification guidance |
| `knowledge list/show` | Inventory and details of implementation insights |
| `knowledge explore` | Surface "Rising Patterns" and thematic trends |
| `knowledge graph` | Visualize connections between insights and entities |
| `knowledge impact` | Track drift risk and institutionalization progress |
| `adr new/accept/supersede` | Architecture Decision Record lifecycle |
| `bearing new/survey/lay` | Research and exploration lifecycle |
| `epic new/done/reopen` | Strategic grouping and PRD management |
| `voyage new/plan/start` | Tactical planning (SRS/SDD) and execution |
| `story new/start/submit` | Implementation units and acceptance criteria |

#### Diagnostics

| Command | Purpose |
|---------|---------|
| `doctor` | Validate board health and fix consistency issues |
| `status` | High-level summary of entity counts and blockers |
| `flow` | Real-time dashboard of Human vs. Agent queues |
| `throughput` | Show recent weekly throughput and timing sparklines |
| `capacity` | Analyze epic-level bandwidth and parallel potential |
| `gaps` | Identify missing requirements or design coverage |

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

- Actionable: ADR transitions (`adr accept/reject/deprecate/supersede`), bearing lifecycle transitions (`bearing new/survey/assess/park/decline/lay`), guided play suggestion (`play --suggest`), story-scoped verification (`verify <story-id>`), story-scoped audit (`audit <story-id>`).
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
