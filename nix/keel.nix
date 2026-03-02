{ lib, rustPlatform, pkg-config, zstd, git, ... }:

let
  cargoToml = lib.importTOML ../Cargo.toml;
in
rustPlatform.buildRustPackage {
  pname = "keel";
  version = cargoToml.package.version;

  src = ../.;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

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
