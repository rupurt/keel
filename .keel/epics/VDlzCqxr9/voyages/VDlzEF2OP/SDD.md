# Theater Play Runtime and Themes - Software Design Description

> Launch a themed interactive theater mode for keel play with genre and persona flavor.

**SRS:** [SRS.md](SRS.md)

## Overview

The voyage introduces a dedicated interactive layer inside `keel play` that chooses an interaction mode and theme-aware copy/presenter pipeline.

Execution order:

1. Parse command arguments (`--theater`, optional `--theme`, optional persona selector).
2. Resolve theme and persona configuration.
3. Enter TUI runtime and stream structured prompts/events.
4. Reuse the existing play command loop for core progression and session controls.

## Data Model

Define three small internal models in `play` command layer:

- `TheaterMode`: identifies standard runtime style (`comedy`, `drama`, `action`, etc.).
- `SessionTheme`: copy and color/style profile for a named session.
- `PersonaProfile`: output persona that alters narration tone (`standup`, `shakespeare`, `broadway`).

## Components

| Component | Responsibility |
|-----------|----------------|
| `theater_mode` | Flag parsing, theme/persona resolution, startup dispatch. |
| `theme_registry` | Registry of built-in themes and default fallbacks. |
| `persona_renderer` | Converts generic events into style-specific narration text. |
| `play_tui` | Input loop and terminal draw/update loop for theater sessions. |

## Flow

`play` command

- New CLI branch routes `--theater` to theater runner.
- Runner resolves active theme/persona from flags or defaults.
- Runner emits `SessionEvent`s consumed by TUI and persona renderer.
- Renderer emits deterministic text with explicit fallback text if theme data is unavailable.

## Interface Notes

- CLI flags:
  - `--theater`: enable TUI interaction mode.
  - `--theme <theme-id>`: set theme at startup.
  - `--persona <persona-id>`: set voice/persona style (`standup`, `shakespeare`, `broadway`).

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Unknown theme/persona | Invalid ID from args or session state | Show supported values and selected fallback | Continue with default theme/persona |
| Unsupported terminal size | Terminal capability check before renderer setup | Notify in TUI footer | Continue with compact layout |
| Unknown rendering panic | Runtime exception or render error path | Abort safely with context | Exit to standard output mode |
