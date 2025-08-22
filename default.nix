{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package; in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  nativeBuildInputs = with pkgs; [ pkg-config openssl ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}