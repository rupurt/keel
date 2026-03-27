# Keel Public Docs

This directory contains the public MDX documentation site for Keel.

## Local Workflow

Use the repo-supported `just` recipes from the repository root:

```bash
just docs-install
just docs-dev
just docs-build
```

These commands use the repository's Nix-supported Node toolchain so the docs workflow stays reproducible in this workspace.

## Deployment Inputs

The site reads these optional environment variables at build time:

- `DOCS_SITE_URL`
- `DOCS_BASE_URL`

If they are not set, the site defaults to `https://spoke.sh` and `/`.
