{ pkgs, lib, config, inputs, ... }:

let
  nightly = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [
      "rustfmt"
    ];
  };
in
{
  overlays = [ inputs.rust-overlay.overlays.default ];

  packages = with pkgs; [
  ];

  languages.rust = {
    enable = true;
    #toolchain.rustfmt = nightly;
    channel = "stable";
    targets = [
      # Android
      "armv7-linux-androideabi"
      "i686-linux-android"
      "aarch64-linux-android"
      "x86_64-linux-android"

      # iOS
      "aarch64-apple-ios"
      "x86_64-apple-ios"
      "aarch64-apple-ios-sim"

      # Linux
      "x86_64-unknown-linux-gnu"

      # macOS
      "x86_64-apple-darwin"
      "aarch64-apple-darwin"

      # Windows
      #"x86_64-pc-windows-gnu"
      #"x86_64-pc-windows-msvc"
    ];
  };

  git-hooks.hooks = {
    rustfmt = {
      #enable = true;
      settings.files-with-diff = true;
    };
    check-added-large-files.enable = true;
    check-json.enable = true;
    check-merge-conflicts.enable = true;
    check-toml.enable = true;
    check-yaml.enable = true;
    end-of-file-fixer.enable = true;
  };
}
