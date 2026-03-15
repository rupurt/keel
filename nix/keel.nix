{ lib, rustPlatform, pkg-config, zstd, git, ... }:

let
  cargoToml = lib.importTOML ../Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = "keel";
  version = cargoToml.workspace.package.version;

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
    outputHashes = {
      "txtplot-0.1.0" = "sha256-bC6zo1yhJg41iz69XbXqwIKOfNVXwFke0vzcSMbqvFE=";
    };
  };

  doCheck = false;

  nativeBuildInputs = [
    pkg-config
  ];

  nativeCheckInputs = [
    git
  ];

  buildInputs = [
    zstd
  ];

  meta = with lib; {
    description = "Fast CLI for project board management";
    homepage = "https://github.com/rupurt/keel";
    license = licenses.mit;
    maintainers = [ ];
  };
}
