{ pkgs, devenv, ... }:

{
#  default = {
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
#  };
}
