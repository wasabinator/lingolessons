{
  description = "Flake for ESP32 dev";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
  };

  outputs = inputs@{ flake-parts, nixpkgs, systems, devenv, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        devenv.flakeModule
      ];
      systems = import systems;
      perSystem = { config, self', inputs', pkgs, system, lib, ... }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
        devenv.shells = import ./nix/android.nix {
            inherit config;
            inherit pkgs;
            inherit devenv;
        };
      };
    };
}
