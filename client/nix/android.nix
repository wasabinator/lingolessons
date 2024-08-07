{ pkgs, devenv, ... }:

{
  default = {
    packages = with pkgs; [];

    android = {
      enable = true;
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

    scripts.create-avd.exec = "avdmanager create avd --force --name 'Pixel6a' --package 'system-images;android-34-ext10;google_apis_playstore;x86_64' --device 'pixel_6a'";
  };
}
