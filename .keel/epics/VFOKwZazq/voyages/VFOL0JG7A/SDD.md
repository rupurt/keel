# Frame Accurate Scheduler - SDD

## Overview

Manages the real-time playback timing for multi-frame artifacts like GIFs.

## Architecture

A non-blocking loop that calculates sleep durations based on frame timestamp metadata.

## Components

- `HighFidelityScheduler`: Controls loop timing.
- `DeltaEncoder`: Minimizes terminal throughput.

## Data Flow

`FrameSequence` -> `Scheduler` -> `DeltaEncoder` -> `STDOUT`.
