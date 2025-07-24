{ pkgs, devenv, ... }:

{
  packages = with pkgs; [
    python311
    python311Packages.pip
  ];

  languages = {
    javascript = {
      enable = true;
      npm = {
        enable = true;
        install.enable = true;
      };
    };
  };

  enterShell = 
    ''
    python -m venv .venv
    source .venv/bin/activate
    pip install -r requirements.txt
    '';

  scripts = {
    tailwind.exec = "python manage.py tailwind start";
    run.exec = "python manage.py runserver";
  };
}
