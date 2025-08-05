{ pkgs, devenv, ... }:

{
  packages = with pkgs; [
    ktfmt
    ktlint
    rustup
  ];

  android = {
    enable = true;
    ndk = {
      enable = true;
      version = [ "28.0.13004108" ]; # available versions defined in https://github.com/NixOS/nixpkgs/blob/master/pkgs/development/androidndk-pkgs/default.nix
    };
    platforms.version = [ "35" ];
    buildTools.version = [ "35.0.0" ];
    systemImageTypes = [ "google_apis_playstore" ];
    emulator = {
      enable = true;
      version = "35.5.2";
    };
    sources.enable = true;
    systemImages.enable = true;
    #android-studio.enable = true;
  };

  languages.java.jdk.package = pkgs.jdk21;

  enterShell = ''
      echo 'READY';
      export ANDROID_HOME=$(which android | sed -E 's/(.*libexec\/android-sdk).*/\1/')
      export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/28.0.13004108/
      export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
      export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$PATH

      rustup default stable
  '';

  scripts.create-avd.exec = if builtins.currentSystem == "aarch64-linux" || builtins.currentSystem == "aarch64-darwin"
  then "avdmanager create avd --force --name 'Pixel' --package 'system-images;android-35;google_apis_playstore;arm64-v8a' --device 'pixel_6a'"
  else "avdmanager create avd --force --name 'Pixel' --package 'system-images;android-35;google_apis_playstore;x86_64' --device 'pixel_6a'";

  scripts.avd.exec = "nohup $ANDROID_HOME/emulator/emulator -avd Pixel &";
  scripts.avd-cold.exec = "nohup $ANDROID_HOME/emulator/emulator -avd Pixel -no-snapshot &";

  scripts.check.exec = "cargo +nightly clippy";
  scripts.fix.exec = "cargo +nightly clippy --fix"; 
}
