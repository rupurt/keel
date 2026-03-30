# Verification Sign-off Gate - SDD

## Overview

Integrates the artifact playback directly into the `keel mission verify` lifecycle.

## Architecture

The verification service will now include a `ProofReviewStage` between gate evaluation and state mutation.

## Components

- `ProofReviewer`: Orchestrates the `TheaterScene` playback.
- `SignOffPrompt`: An interactive TTY prompt for user confirmation.

## Data Flow

`Achieved` Status -> `evaluate_gates` -> `play_proof` -> `human_sign_off` -> `Verified` Status.
