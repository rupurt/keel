# Rich Evidence Capture - Software Design Description

> Streamline the evidence recording process with editor integration and multi-modal attachments.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage enhances `keel story record` to support multi-modal evidence. It moves beyond text logs to include terminal recordings and AI-generated validation transcripts.

## Components

### 1. VHS Adapter
- **Purpose**: Wraps the `vhs` CLI to record terminal sessions.
- **Behavior**: Generates a temporary `.tape` file if none exists, runs `vhs`, and moves the resulting GIF into the story's `EVIDENCE/` directory.

### 2. Judge Client
- **Purpose**: Interfaces with LLMs to evaluate story work.
- **Behavior**: Packages story context (diff, ACs, code) and sends it to a configured "Judge" (e.g., local model or API). Captures the reasoning and signature.

### 3. Provenance Generator
- **Purpose**: Generates YAML headers for all evidence.
- **Behavior**: Injects Git SHA, Author, and Timestamp into every recorded artifact.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Editor | System Default | Respects user preference via `$EDITOR` |
| Video | GIF/MP4 | Widely supported in Markdown viewers and Git hosts |
