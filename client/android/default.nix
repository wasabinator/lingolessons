{ pkgs, devenv, ... }:

{
  packages = with pkgs; [ rustup cargo-ndk ];
  
  android = {
    enable = true;
    ndk = {
      enable = true;
      version = [ "27.1.12297006" ];
    };
    platforms.version = [ "34" ];
    systemImageTypes = [ "google_apis_playstore" ];
    emulator = {
      enable = true;
      version = "34.1.9";
    };
    sources.enable = true;
    systemImages.enable = true;
    android-studio.enable = true;
  };

  languages.java.jdk.package = pkgs.jdk17;

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    targets = [
      # iOS
      #"aarch64-apple-ios"
      #"aarch64-apple-ios-sim"

      # Android
      #"armv7-linux-androideabi"
      #"i686-linux-android"
      "aarch64-linux-android"
      "x86_64-linux-android"

      # Linux
      #"x86_64-unknown-linux-gnu"

      # macOS
      #"x86_64-apple-darwin"
      #"aarch64-apple-darwin"

      # Windows
      #"x86_64-pc-windows-gnu"
      #"x86_64-pc-windows-msvc"
    ];
  };

  enterShell = ''
      echo 'READY';
      export ANDROID_HOME=$(which android | sed -E 's/(.*libexec\/android-sdk).*/\1/')
      export ANDROID_NDK_HOME=$ANDROID_HOME/ndk/27.1.12297006
      export ANDROID_SDK_ROOT=$ANDROID_NDK_HOME
      export PATH=$ANDROID_HOME/cmdline-tools/latest/bin:$PATH
  '';
  
  scripts.create-avd.exec = "avdmanager create avd --force --name 'Pixel6a' --package 'system-images;android-34-ext10;google_apis_playstore;x86_64' --device 'pixel_6a'";
}
