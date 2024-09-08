{ pkgs, ... }:

{
  packages = with pkgs; [];

  languages.rust = {
    enable = true;
    channel = "stable";

    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };
}
