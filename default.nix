{ pkgs ? import <nixpkgs> { } }:
let manifest = (pkgs.lib.importTOML ./Cargo.toml).package; in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;

  nativeBuildInputs = with pkgs; [ pkg-config openssl ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
}