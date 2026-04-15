# Keel Public Docs

This directory contains the public MDX documentation site for Keel.

## Local Workflow

Use the repo-supported `just` recipes from the repository root:

```bash
just docs-install
just docs-dev
just docs-build
```

The dev server binds to `0.0.0.0` by default. Override the port when `3000` is already in use:

```bash
PORT=3010 just docs-dev
```

These commands use the repository's Nix-supported Node toolchain so the docs workflow stays reproducible in this workspace.

For production publication, the repository-owned
[`publish-docs.yml`](../.github/workflows/publish-docs.yml) workflow is the
preferred lane. It publishes the stable Keel site plus the `main` preview into
the shared `spoke-previews` bucket through the infra-managed OIDC role. The
checked-in [`publish-docs.sh`](../scripts/publish-docs.sh) script is the local
repair and CI execution surface for that contract. Published docs objects use
`Cache-Control: no-cache` so browsers revalidate with `ETag` and
`Last-Modified`.

## Deployment Inputs

The site reads these optional environment variables at build time:

- `DOCS_SITE_URL`
- `DOCS_BASE_URL`

If they are not set, the site defaults to `https://spoke.sh` and `/`.

The publish script also accepts:

- `DOCS_APP_NAME`
- `DOCS_PREVIEW_BUCKET`
- `DOCS_BRANCH`
- `DOCS_PUBLISH_STABLE`
- `DOCS_SKIP_SYNC`
