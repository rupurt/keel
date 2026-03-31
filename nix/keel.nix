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
      "atxt-0.1.0" = "sha256-tTgEGGX1+Kiw5dJR1IWmOIPKrRjJ6xGipv0pfltASfM=";
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
