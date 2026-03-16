# URL-Aware Capture Defaults - Software Design Description

> Auto-derive capture defaults from URL location

**SRS:** [SRS.md](SRS.md)

## Overview

Modify `ResearchCaptureArgs::into_request()` in `crates/keel-cli/src/cli/commands/management/bearing/mod.rs` to detect URL locations and apply smart defaults before requiring explicit flags.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| URL detection | `location.starts_with("http://") \|\| location.starts_with("https://")` | Simple, no external deps, covers all web URLs |
| Domain extraction | Split on `://`, take host before first `/` | Avoids URL crate dependency; handles standard URLs |
| Default priority | Explicit flag > URL-derived default > required-error | Operators retain full control |

## Components

### URL defaults logic

A helper function `apply_url_defaults(args: &mut ResearchCaptureArgs)` called at the start of `into_request()`. If `location` is a URL:
- Sets `class` to `"web"` if None
- Sets `retrieved_at` to today (`chrono::Local::today()`) if None
- Sets `provider` to `"manual:<domain>"` if None

### Domain extraction

Extract hostname from URL: strip scheme prefix (`https://`), take everything before the first `/`, strip `www.` prefix for cleaner provenance labels.

## Data Flow

```
ResearchCaptureArgs
  → apply_url_defaults (if location is URL, fill missing fields)
  → into_request (existing validation + conversion)
  → ResearchCaptureRequest
```

## Error Handling

| Error Condition | Detection | Response | Recovery |
|-----------------|-----------|----------|----------|
| Malformed URL (no host after scheme) | Split produces empty host | Skip defaults, let existing validation handle | User provides explicit flags |
| Non-URL location | No `http://` or `https://` prefix | No defaults applied, all flags required as before | Existing behavior |
