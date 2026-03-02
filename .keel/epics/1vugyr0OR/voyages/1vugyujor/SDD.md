# Continuous Verification - Software Design Description

> Implement board-wide automated proof re-validation to prevent regression and evidence drift.

**SRS:** [SRS.md](SRS.md)

## Overview

This voyage shifts verification from script execution to artifact integrity. It introduces a `manifest.yaml` at the story level that cryptographically links evidence to the Git state.

## Components

### 1. Manifest Generator
- **Purpose**: Creates `manifest.yaml` in the story bundle.
- **Behavior**: Captures the current Git SHA and generates SHA-256 hashes for all files in the `EVIDENCE/` directory.

### 2. Integrity Judge
- **Purpose**: Validates existing manifests.
- **Behavior**: Re-calculates hashes of evidence artifacts and compares them against the manifest. Checks if the current HEAD matches the manifest SHA.

## Key Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Format | YAML | Human-readable and consistent with existing board metadata |
| Hashing | SHA-256 | Industry standard for integrity verification |
