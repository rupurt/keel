set shell := ["bash", "-euo", "pipefail", "-c"]

# Show all available commands
default:
  @just --list --unsorted

# Install necessary Cargo tools for testing and coverage
setup:
  cargo install --locked --force cargo-nextest cargo-llvm-cov

# Build the project in debug mode
build:
  @just build-debug

# Compile the project in debug mode and copy binary to target/debug
build-debug:
  cargo build
  mkdir -p target/debug target/release
  cp -f "${CARGO_TARGET_DIR:-target}/debug/keel" target/debug/keel

# Compile the project in release mode and copy binary to target/release
build-release:
  cargo build --release
  mkdir -p target/debug target/release
  cp -f "${CARGO_TARGET_DIR:-target}/release/keel" target/release/keel

# Run tests using cargo nextest
test *args:
  cargo nextest run {{args}}

# Run doc tests (nextest does not support these)
doctest:
  cargo test --doc

# Check formatting and run clippy
quality:
  cargo fmt --all -- --check
  cargo clippy --all-targets --all-features -- -D warnings

# Generate an LCOV coverage report at coverage/lcov.info (pass custom args to override)
coverage args="":
  mkdir -p coverage
  if [[ -n "{{args}}" ]]; then cargo llvm-cov nextest {{args}}; else cargo llvm-cov nextest --lcov --output-path ./coverage/lcov.info; fi

# Install the docs site dependencies using the repo-supported Node toolchain
docs-install:
  nix shell nixpkgs#nodejs_22 -c sh -lc 'cd website && npm install'

# Run the public docs site locally
docs-dev:
  nix shell nixpkgs#nodejs_22 -c sh -lc 'cd website && npm run start -- --host "${HOST:-0.0.0.0}" --port "${PORT:-3000}"'

# Build the public docs site
docs-build:
  nix shell nixpkgs#nodejs_22 -c sh -lc 'cd website && npm run build'

# Run the keel binary via cargo with arguments
keel *args:
  cargo run {{args}}

# Run quality checks and tests
pre-commit: quality test doctest
  @echo "✓ All pre-commit checks passed"
