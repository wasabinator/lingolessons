{ pkgs, devenv, ... }:

{
  packages = with pkgs; [ rustup ];
  
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

  languages.java.jdk.package = pkgs.jdk17;

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    targets = [
      # iOS
      "aarch64-apple-ios"
      "aarch64-apple-ios-sim"

      # Android
      #"armv7-linux-androideabi"
      #"i686-linux-android"
      "aarch64-linux-android"
      "x86_64-linux-android"

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

  enterShell = ''
      echo 'READY';
      export ANDROID_HOME=$(which android | sed -E 's/(.*libexec\/android-sdk).*/\1/')
      export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/28.0.13004108/
      export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
      export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$PATH
  '';
  
  scripts.avd-create.exec = if builtins.currentSystem == "aarch64-linux" || builtins.currentSystem == "aarch64-darwin"
  then "avdmanager create avd --force --name 'Pixel' --package 'system-images;android-35;google_apis_playstore;arm64-v8a' --device 'pixel_6a';echo 'hw.keyboard=no' >> .android/avd/Pixel.ini"
  else "avdmanager create avd --force --name 'Pixel' --package 'system-images;android-35;google_apis_playstore;x86_64' --device 'pixel_6a';echo 'hw.keyboard=no' >> .android/avd/Pixel.ini";

  scripts.avd.exec = "nohup $ANDROID_HOME/emulator/emulator -avd Pixel &";
  scripts.avd-cold.exec = "nohup $ANDROID_HOME/emulator/emulator -avd Pixel -no-snapshot &";
}
