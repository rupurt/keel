# Release Process

`keel` uses [release-plz](https://github.com/MarcoIeni/release-plz) and [cargo-dist](https://opensource.axodotdev.com/cargo-dist/) to automate cross-platform releases. Binaries and installers for Linux, macOS, and Windows are automatically built and uploaded to GitHub Releases.

## How to Install

### Homebrew (macOS and Linux)

```bash
brew tap rupurt/homebrew-tap
brew install keel
```

### Nix (Anywhere)

```bash
nix run github:rupurt/keel
```

---

## How to Perform a Release

Follow these steps to release a new version of `keel`:

### 1. Version Bump & PR
`keel` is configured with `release-plz`. When you push changes to `main`, the `release-plz` GitHub Action will:
- Check if a new version should be released based on [Conventional Commits](https://www.conventionalcommits.org/).
- Automatically create or update a "Release PR" that bumps the version in `Cargo.toml` and updates `CHANGELOG.md`.

### 2. Merge the Release PR
Merge the PR created by `release-plz`.

### 3. Automated Tagging and Release
Once the Release PR is merged into `main`, the `release-plz` workflow will:
- Tag the commit (e.g., `v0.1.0`).
- Push the tag to GitHub.

Pushing the tag triggers the [Release GitHub Action](.github/workflows/release.yml). This workflow will:
- Plan the release using `cargo dist plan`.
- Build binaries for all supported platforms in parallel.
- Generate supported installers (shell, PowerShell, Homebrew, and `.msi`).
- Create a GitHub Release and upload all artifacts and checksums.

### 4. Verify the Release
Once the GitHub Action completes:
1.  Go to the [Releases](https://github.com/rupurt/keel/releases) page.
2.  Verify that all artifacts (tarballs and installers) are attached.
3.  Ensure the `checksums.txt` file is present.

---

## Supported Platforms & Artifacts

| Platform | Target Triple | Artifacts |
|----------|---------------|-----------|
| **Linux (x86_64, glibc)** | `x86_64-unknown-linux-gnu` | `.tar.gz`, shell installer |
| **Linux (x86_64, static)** | `x86_64-unknown-linux-musl` | `.tar.gz`, shell installer |
| **Linux (ARM64)** | `aarch64-unknown-linux-gnu` | `.tar.gz`, shell installer |
| **macOS (Intel)** | `x86_64-apple-darwin` | `.tar.gz`, shell installer, Homebrew formula |
| **macOS (Apple Silicon)** | `aarch64-apple-darwin` | `.tar.gz`, shell installer, Homebrew formula |
| **Windows (x86_64)** | `x86_64-pc-windows-msvc` | `.zip`, `.msi`, PowerShell installer |

---

## Local Testing

You can simulate the release plan locally (if `cargo-dist` is installed):

```bash
# See what would be built
cargo dist plan

# Build artifacts locally (outputs to target/dist)
cargo dist build
```
