# Atxt Core Streaming Client - SDD

## Overview

Integrates the foundational atxt-core library into Keel for artifact processing.

## Architecture

Leverages atxt's environment detection and planning engine to select the best terminal renderer.

## Components

- `AtxtClient`: High-level wrapper for atxt APIs.
- `TerminalScanner`: Uses `atxt::TerminalEnvironment::capture`.

## Data Flow

`ArtifactPath` -> `probe_path` -> `plan_render` -> `render_to_text`.
