# Adaptive Resizing - SDD

## Overview

Ensures cinematic playback remains visually stable and correctly scaled during terminal window resizing.

## Architecture

The system will leverage `crossterm` resize events to trigger a re-render of the `TheaterScene` layout.

## Components

- `ResizeListener`: Listens for TTY window change signals.
- `ScaleProvider`: Recalculates `atxt` intent dimensions.

## Data Flow

`ResizeEvent` -> `atxt::plan_render` -> `TheaterScene::update`.
