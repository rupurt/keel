{
  description = "Keel - Agentic SDLC management";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "llvm-tools" ];
        };
        isLinux = pkgs.stdenv.isLinux;
        isDarwin = pkgs.stdenv.isDarwin;

        keel = pkgs.callPackage ./nix/keel.nix {
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rust;
            rustc = rust;
          };
        };
      in {
        packages = {
          keel = keel;
          default = keel;
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [
            rust
            pkgs.just
            pkgs.cargo-nextest
            # CLI recording and video processing
            pkgs.vhs
            pkgs.ffmpeg
            # Native build deps
            pkgs.pkg-config
          ] ++ pkgs.lib.optionals isLinux [
            # mold is a faster linker (Linux only)
            pkgs.mold
          ];

          shellHook = ''
            # Shared target directory across all worktrees for faster builds
            # WARNING: cargo clean will affect ALL worktrees
            export CARGO_TARGET_DIR="$HOME/.cache/cargo-target/keel"

            # Auto-install cargo-llvm-cov if missing (consistent across all platforms)
            if ! command -v cargo-llvm-cov &> /dev/null; then
              echo "Installing cargo-llvm-cov..."
              cargo install cargo-llvm-cov --quiet
            fi
          '' + pkgs.lib.optionalString isDarwin ''
            # Fix TMPDIR for sccache on macOS (Nix 2.24+ issue)
            # Nix sets TMPDIR to a nix-shell-specific dir that gets cleaned up
            # causing "Failed to create temp dir" errors in sccache
            # See: https://github.com/NixOS/nix/issues/11929
            export TMPDIR=/var/tmp
          '' + pkgs.lib.optionalString isLinux ''
            # Use mold linker on Linux for faster linking
            export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=mold"
            export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUSTFLAGS="-C link-arg=-fuse-ld=mold"
          '';
        };
      });
}
