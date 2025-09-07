{ pkgs, lib, config, inputs, ... }:

let
  nightly = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [
      "rustfmt"
    ];
  };
in
{
  cachix.enable = false;

  overlays = [ inputs.rust-overlay.overlays.default ];

  packages = with pkgs; [
  ];

  languages.rust = {
    enable = true;
    toolchain.rustfmt = nightly;
  };

  enterShell = ''
    cp local.properties client/android/local.properties
  '';

  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  git-hooks.hooks = {
    #rustfmt.enable = true;
    clippy = {
    #  enable = true;
      settings = {
        allFeatures = true;
        offline = false;
        denyWarnings = true;
        extraArgs = "--all-targets";
      };
    };
    check-added-large-files.enable = true;
    check-json.enable = true;
    check-merge-conflicts.enable = true;
    check-toml.enable = true;
    check-yaml.enable = true;
    end-of-file-fixer.enable = true;
  };
}
