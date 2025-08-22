{
  description = "Telegram bot CLI";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      telegramCli = pkgs.callPackage ./default.nix { };
    in
    {
      packages.telegram-cli = telegramCli;
      defaultPackage = telegramCli;
    }) // {
      homeManagerModules.telegram-cli = import ./home-manager/module.nix;
      homeManagerModules.default = self.homeManagerModules.telegram-cli;
    };
}