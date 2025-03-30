{
  description = "Flake for client dev";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/default";
    devenv.url = "github:cachix/devenv";
    fenix.url = "github:nix-community/fenix";
    fenix.inputs = { nixpkgs.follows = "nixpkgs"; };
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
        devenv.shells = { 
          default = {
          };
          client = import ./client/default.nix {
            inherit config pkgs devenv;
          };
          server = import ./server/default.nix {
            inherit config pkgs devenv;
          };
        };
      };
    };
}
