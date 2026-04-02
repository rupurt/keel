# Keel Code Walkthrough

This document orients contributors and agents to the source layout, key abstractions, and data flows in the keel codebase. For governance philosophy see [CONSTITUTION.md](CONSTITUTION.md); for architectural contracts see [ARCHITECTURE.md](ARCHITECTURE.md).

## Workspace Layout

Keel is a Rust workspace with five crates:

| Crate | Path | Purpose |
|-------|------|---------|
| **keel-cli** | `crates/keel-cli/` | Binary entry point, command adapters, presentation layer |
| **keel-core** | `crates/keel-core/` | Domain model, application services, infrastructure, read-model projections |
| **speccy** | `crates/speccy/` | Reusable markdown template engine (`{{placeholder}}` rendering, frontmatter mutation) |
| **txt-scene** | `crates/txt-scene/` | Fixed-width terminal scene primitives (ANSI-aware measurement, framed rows) |
| **spoke-auth** | `crates/spoke-auth/` | Actor types (`LocalSystem`, `Authenticated`) and execution context |

`keel-cli` depends on `keel-core`, `txt-scene`, and `spoke-auth`. `keel-core` depends on `speccy` and `spoke-auth`. The two utility crates have no keel-specific dependencies.

## keel-core: Five-Layer Architecture

`crates/keel-core/src/lib.rs` exposes four public module roots that, together with the CLI layer in `keel-cli`, form five architecture layers:

```
CLI  →  Application  →  Domain  ←  Infrastructure
                          ↑
                      Read Model
```

| Layer | Location | Responsibility |
|-------|----------|----------------|
| **Domain** | `keel-core/src/domain/` | Entities, value objects, state machines, transition rules, policies, hexagonal ports |
| **Application** | `keel-core/src/application/` | Use-case orchestration (`story_lifecycle`, `voyage_epic_lifecycle`, `mission_lifecycle`), process manager, domain events |
| **Infrastructure** | `keel-core/src/infrastructure/` | Filesystem board loader/storage, YAML/TOML parsing, template rendering, validation, code generation, verification |
| **Read Model** | `keel-core/src/read_model/` | Query-side projections: flow status, capacity, lane routing, traceability, evidence, knowledge graph, scheduled routines |
| **CLI** | `keel-cli/src/cli/` | Command tree (clap), runtime dispatch, terminal presentation (themes, tables, scenes, typography) |

### Domain Layer Internals

```
domain/
├── model/          # Entity structs: Board, Story, Voyage, Epic, Bearing, Mission, Routine, Watch, ADR
├── state_machine/  # Formal state machines, enforcement policies, gating, preconditions, invariants
├── transitions/    # Transition execution engine with frontmatter mutations and side effects
├── policy/         # Queue policy and pull-system rules
└── port/           # Hexagonal ports: BoardStore, EntityStore<T>, DocumentServicePort
```

**Board** is the aggregate root — a struct holding `HashMap` collections of all eight entity types plus a `root: PathBuf` pointing to the `.keel/` directory.

### Key Domain Entities

| Entity | Identity | States | Role |
|--------|----------|--------|------|
| **Story** | Nanoid | Backlog → InProgress → NeedsHumanVerification → Done (or Icebox, Rejected) | Atomic implementation unit |
| **Voyage** | Nanoid | Draft → Planned → InProgress → Done | Story container scoped to one delivery arc |
| **Epic** | Nanoid | Draft / Active / Done (derived from voyage states) | Strategic value shift containing voyages |
| **Bearing** | Nanoid | Exploring → Evaluating → Ready → Laid (or Parked, Declined) | Pre-epic research artifact |
| **Mission** | Nanoid | Defining → Active → Achieved → Verified (or Cancelled) | Autonomous harness session with charter and goals |
| **Routine** | Nanoid | (stateless blueprint) | Recurring work pattern with cron cadence |
| **Watch** | Nanoid | (active constraint) | Time-boxed capacity slot (12-hour analog dial) |
| **ADR** | Nanoid | Proposed → Accepted / Rejected / Deprecated / Superseded | Binding architectural decision |

### State Machine Infrastructure

State transitions are enforced through a layered system in `domain/state_machine/`:

1. **`enforcement.rs`** — `enforce_transition(board, entity, intent, policy)` evaluates blocking rules at three policy levels: `RUNTIME` (permissive), `STRICT` (all rules), `PREVIEW` (dry-run).
2. **`gating.rs`** — Validates preconditions per entity type (e.g., epic completion requires all voyages done).
3. **`preconditions.rs`** — Captures `TransitionContext` with blocking constraints and role requirements.

Transitions are executed in `domain/transitions/` which applies the state change, mutates YAML frontmatter (timestamps, status), and emits domain events to the `DomainProcessManager` for cross-aggregate coordination.

## keel-cli: Command Tree and Presentation

### Entry Point

`crates/keel-cli/src/main.rs` → `cli::run()` → clap command tree built in `cli/command_tree.rs`.

### Command Organization

```
cli/commands/
├── diagnostics/   # doctor, flow, health, heartbeat, throughput, workshop, turn, screen, topology, play
├── management/    # story/*, voyage/*, epic/*, bearing/*, mission/*, routine/*, watch/*, adr/*, knowledge/*
├── setup/         # new, upgrade, generate, hooks, config
└── comms/         # ping, poke, inbox, outbox, notify
```

Key command families:

| Family | Examples | Purpose |
|--------|----------|---------|
| **Diagnostics** | `doctor`, `flow`, `health`, `heartbeat`, `turn` | Board health, flow state, system orientation |
| **Story lifecycle** | `story new/start/submit/accept/reject/ice/thaw/record` | Full story state machine traversal |
| **Planning** | `epic new`, `voyage new/plan/start/done`, `bearing new/assess/lay` | Strategic pipeline management |
| **Missions** | `mission new/attach/next` | Autonomous session lifecycle |
| **Setup** | `new`, `upgrade`, `generate`, `hooks install` | Project scaffold, install maintenance, and artifact sync |
| **Comms** | `ping`, `poke`, `inbox`, `notify` | Human-agent message exchange |

### Presentation Layer

```
cli/presentation/
├── flow/          # Lane dashboard: box components, capacity bars, bottleneck detection, throughput
├── topology.rs    # Zoomable world-map visualization
├── show.rs        # Entity detail display
├── scene.rs       # Scene composition for --scene flag
├── theme.rs       # Color/styling themes with no-color support
├── audio.rs       # Audio feedback
└── markdown.rs    # Syntax-highlighted markdown rendering
```

The `--scene` flag on diagnostic commands switches from tabular output to ASCII art scenes (battery packs, circuit diagrams, workbenches) rendered via `txt-scene` primitives.

## Template System

Templates live in `templates/` and are embedded at compile time via `include_str!()` in `keel-core/src/infrastructure/templates.rs`.

```
templates/
├── project/       # Bootstrap templates for `keel new` (AGENTS.md, CLAUDE.md, keel.toml, etc.)
├── epic/          # Epic + nested voyage templates (README, PRD, PRESS_RELEASE, SRS, SDD)
├── stories/       # Story README + REFLECT templates
├── bearings/      # BRIEF, EVIDENCE, ASSESSMENT templates
├── missions/      # CHARTER, LOG templates
├── voyage/        # VOYAGE_REPORT, COMPLIANCE_REPORT templates
├── routines/      # Routine README template
├── watches/       # Watch README template
└── adrs/          # ADR template
```

Templates use `{{placeholder}}` tokens in three ownership buckets (validated by tests in `templates.rs`):
- **CLI-owned**: user-supplied during creation (`title`, `goal`, `problem`, `type`, etc.)
- **System-owned**: engine-managed (`id`, `status`, `created_at`, `updated_at`, etc.)
- **Generated**: computed during artifact sync (`done_count`, `total_count`, `matrix`, etc.)

Rendering flows through `keel-core/src/infrastructure/template_rendering.rs` which delegates to the `speccy` crate for substitution and frontmatter mutation.

## Board Artifact Generation

`keel-core/src/infrastructure/generate/` handles idempotent README regeneration:

- **`board_readme.rs`** — Top-level `.keel/README.md` with bearings/epics/stories tables
- **`epic_readme.rs`** — Per-epic summary with voyage progress (uses `<!-- BEGIN/END GENERATED -->` markers)
- **`voyage_readme.rs`** — Voyage overview with story list
- **`voyage_report.rs`** — Delivery metrics and compliance status

`sync_board_artifacts()` is called during `keel new` and `keel generate`. It is deterministic and idempotent — running it twice produces identical output.

## Typical Command Flow

Example: `keel story start <id>`

1. **Parse** — clap matches `story start` subcommand in `command_tree.rs`
2. **Dispatch** — `runtime.rs` routes to `commands::management::story::start::run()`
3. **Load** — Board loaded from `.keel/` via `infrastructure::loader::load_board()`
4. **Enforce** — `enforce_transition(board, story, Start, RUNTIME)` checks preconditions
5. **Execute** — `StoryLifecycleService::start()` applies transition, mutates frontmatter (`started_at`), emits `StoryStarted` event
6. **Persist** — Updated story written back to `.keel/stories/<id>/README.md`
7. **Present** — CLI formats success message with updated state

## Verification Pipeline

The verification system in `infrastructure/verification/` supports Verified Spec Driven Development (VSDD):

- **`parser.rs`** — Extracts `<!-- verify: ... -->` markers from story acceptance criteria
- **`executor.rs`** — Runs verification commands and captures output
- **`comparator.rs`** — Compares expected vs actual results
- **`reporter.rs`** — Formats verification evidence for `EVIDENCE/` directories

Evidence is stored alongside stories and rolled up into voyage compliance reports during `keel generate`.

## Configuration

`keel.toml` at the project root defines workflow topology:

- **Lanes** — Named queues with priority, inclusion patterns, and `manual_accept` flag
- **Roles** — Named actors mapped to default lanes and operational contracts
- **Workflow** — Working hours, battery capacity, heartbeat decay, auto-staging
- **Scoring** — Constrained or unconstrained mode for story prioritization

See [CONFIGURATION.md](CONFIGURATION.md) for the full reference.

## Where to Look

| I want to... | Start here |
|---------------|-----------|
| Understand an entity's lifecycle | `keel-core/src/domain/state_machine/` |
| Add a new CLI command | `keel-cli/src/cli/command_tree.rs` + `commands/` |
| Change how flow/health renders | `keel-cli/src/cli/presentation/` + `commands/diagnostics/` |
| Modify board validation rules | `keel-core/src/infrastructure/validation/` |
| Add a new entity template | `templates/` + `keel-core/src/infrastructure/templates.rs` |
| Change transition behavior | `keel-core/src/domain/transitions/` |
| Add a new read-model projection | `keel-core/src/read_model/` |
| Modify the scaffold pipeline | `keel-cli/src/cli/commands/setup/new.rs` |
