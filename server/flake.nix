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

      perSystem = { config, self', inputs', pkgs, system, ... }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        devenv.shells.default = {
          packages = with pkgs; [
          ];

          languages = {
            javascript = {
              enable = true;
              npm = {
                enable = true;
                install.enable = true;
              };
            };

            python = {
              enable = true;
              venv = {
                enable = true;
                requirements = ./requirements.txt;
              };
            };
          };

          scripts = {
            tailwind.exec = "python manage.py tailwind start";
            run.exec = "python manage.py runserver";
          };
        };
      };
    };
}
