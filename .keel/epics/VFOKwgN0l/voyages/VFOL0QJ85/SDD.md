# Txt-scene Framing - SDD

## Overview

This voyage implements the visual "Theater Mode" for artifact playback using the `txt-scene` library.

## Architecture

The `TheaterScene` will wrap the `atxt` playback buffer, providing a high-fidelity visual container.

## Components

- `TheaterScene`: The primary scene container.
- `DoubleBorder`: A layout component for cinematic framing.
- `TitleBar`: Displays mission metadata.

## Data Flow

`atxt` Frames -> `TheaterScene` -> Terminal Buffer.
