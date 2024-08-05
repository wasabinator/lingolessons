{
  description = "Flake for LingoLessons client";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
  };

  outputs = inputs @ {flake-parts, nixpkgs, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.devenv.flakeModule
      ];

      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        devenv.shells.default = {
          packages = [];

          languages = {
            python = {
              enable = true;
              venv = {
                enable = true;
                requirements = ./requirements.txt;
              };
            };
          };

          #scripts.create-avd.exec =
          #  "avdmanager create avd --force --name 'Pixel6a' --package 'system-images;android-34-ext10;google_apis_playstore;x86_64' --device 'pixel_6a'";

          processes = {
          };

          enterShell = ''
          '';
        };
      };
    };
}
