{pkgs, ...}:
pkgs.rustPlatform.buildRustPackage
{
  name = "hypryaml";
  src = ./.;
  cargoLock = {lockFile = ./Cargo.lock;};
}
