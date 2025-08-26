{ pkgs, devenv, ... }:

{
  packages = with pkgs; [
  ];

  languages = {
    javascript = {
      enable = true;
      directory = "./server";
      npm = {
        enable = true;
        install.enable = true;
      };
    };

    python = {
      enable = true;
      directory = "./server";
      venv.enable = true;
      uv = {
        enable = true;
        sync.enable = true;
      };
    };
  };

  git-hooks.hooks = {
    pylint.enable = true;
    pyright.enable = true;
  };

  scripts = {
    tailwind.exec = "python manage.py tailwind start";
    run.exec = "python manage.py runserver";
  };
}
