# Release Process

This project uses [cargo-dist](https://opensource.axodotdev.com/cargo-dist/) to automate cross-platform releases. Binaries and installers for Linux, macOS, and Windows are automatically built and uploaded to GitHub Releases when a version tag is pushed.

## How to Install

### Homebrew (macOS and Linux)

```bash
brew tap spoke-sh/tap
brew install keel
```

### Shell Installer (macOS and Linux)

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/spoke-sh/keel/releases/latest/download/keel-installer.sh | sh
```

### Nix (Anywhere)

```bash
nix run github:spoke-sh/keel
```

### Upgrade an Existing Install

```bash
keel upgrade
keel upgrade --ref v0.2.1
keel upgrade --ref 533fa5b8
```

`keel upgrade` uses the published release installer by default. `keel upgrade --ref <tag-or-sha>` refreshes a cached source checkout under `~/.cache/keel`, looks for a supported Rust toolchain, builds a host-local installer bundle, and then installs that build through the generated installer script.

---

## Homebrew Publishing Setup

Keel publishes its Homebrew formula into the public tap repo `spoke-sh/homebrew-tap`. Homebrew users install that tap with the short form `spoke-sh/tap`.

Before cutting a release that should update Homebrew, configure these settings on the `spoke-sh/keel` GitHub repository:

- Actions variable: `HOMEBREW_TAP_REPO=spoke-sh/homebrew-tap`
- Actions secret: `HOMEBREW_TAP_TOKEN=<token with write access to spoke-sh/homebrew-tap>`

The token must be able to clone, commit, and push to the tap repository. A fine-grained token scoped to `spoke-sh/homebrew-tap` with repository contents write access is sufficient.

---

## How to Perform a Release

Follow these steps to release a new version of `keel`:

### 1. Update Version
Bump the version number in `Cargo.toml`. We follow [Semantic Versioning](https://semver.org/).

```toml
# Cargo.toml
[workspace.package]
version = "0.2.1" # Update this
```

### 2. Commit and Push
Commit the version bump to the `main` branch.

```bash
git add Cargo.toml
git commit -m "chore: bump version to 0.2.1"
git push origin main
```

### 3. Create and Push a Tag
Create a git tag corresponding to the new version (must start with `v`).

```bash
git tag v0.2.1
git push origin v0.2.1
```

### 4. Automated Workflow
Pushing the tag triggers the [Release GitHub Action](.github/workflows/release.yml). This workflow will:
- Plan the release using `cargo dist plan`.
- Build binaries for all supported platforms in parallel.
- Generate supported installers (shell, PowerShell, Homebrew, and `.msi`).
- Create a GitHub Release and upload all artifacts and checksums.
- Publish `Formula/keel.rb` into `spoke-sh/homebrew-tap` when `HOMEBREW_TAP_REPO` and `HOMEBREW_TAP_TOKEN` are configured.

### 5. Verify the Release
Once the GitHub Action completes:
1.  Go to the [Releases](https://github.com/spoke-sh/keel/releases) page.
2.  Verify that all artifacts (tarballs and installers) are attached.
3.  Ensure the `sha256.sum` file is present.
4.  If Homebrew publishing was enabled, verify `Formula/keel.rb` was updated in `spoke-sh/homebrew-tap`.

---

## Supported Platforms & Artifacts

| Platform | Target Triple | Artifacts |
|----------|---------------|-----------|
| **Linux (x86_64, glibc)** | `x86_64-unknown-linux-gnu` | `.tar.gz`, shell installer, `.deb`, `.rpm` |
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
