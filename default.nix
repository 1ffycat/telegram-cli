{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package; in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;

  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;

  meta = {
    inherit (manifest) description;
    homepage = manifest.repository;
    license = pkgs.lib.licenses.mpl20;
    mainProgram = "telegram-cli";
  };
}
