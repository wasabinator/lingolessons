{ pkgs, lib, config, inputs, ... }:

{
  packages = with pkgs; [
    rustup
  ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "rust-std" ];
  };

  enterShell = ''
    ln -sf local.properties client/android/local.properties
    #rustup default stable
  '';

  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  scripts.check.exec = "cargo +nightly clippy";
  scripts.fix.exec = "cargo +nightly clippy --fix";
}
