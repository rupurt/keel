# Operational Routine Infrastructure - Software Design Description

> Deliver audio feedback, artifact auto-sync, and report tail elimination for routine operations

**SRS:** [SRS.md](SRS.md)

## Overview

Three operational improvements delivered via routine-materialized stories:
1. **Auto-sync artifacts** — `auto_sync_artifacts()` runs at the CLI runtime exit point before auto-staging, eliminating manual `keel generate` calls
2. **Audio feedback** — `SoundEvent` enum triggers terminal bell or platform-native sound on state transitions
3. **Auto-staging** — CLI commands automatically `git add` the `.keel` directory after mutations

## Components

### Auto-Sync Artifacts (`runtime.rs`)
Calls `sync_board_artifacts()` with default options at the CLI exit point. Idempotent via `write_if_changed()` — read-only commands produce no disk writes.

### Audio Module (`presentation/audio.rs`)
`SoundEvent` enum maps transitions to bell counts (1-3). Playback chain: custom sound file → platform player (`paplay`/`afplay`) → terminal bell (`\x07` via stderr). Non-blocking via `std::thread::spawn`. Configured via `[audio]` section in `keel.toml`.

### Auto-Staging (`runtime.rs`)
`auto_stage_board()` runs `git add .keel/` after every mutating command when `workflow.auto_stage = true`.

## Data Flow

CLI command → execute → auto_sync_artifacts() → auto_stage_board() → return result

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Audio playback fails | Thread catches panic | Silently ignored | Best-effort, never fails |
| Artifact sync fails | Result ignored | No side effect | Next command retries |
| Git staging fails | Result ignored | Warning only | Manual `git add` |
