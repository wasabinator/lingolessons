### LingoLessons server development

Currently development under Fedora or a Debian based Linux OS is supported.

1) Requirements:

- Python up to version 3.13 & pip.
- The following libs

Fedora:

> postgresql-devel python3-devel

Debian/Ubuntu:

> libpq-dev python3-dev

2) Create the virtual environment:

python -m venv .venv
. .venv/bin/activate
pip install -r requirements.txt

3) Start the server:

./run.sh

The server should now be up, listening on 127.0.0.1:8000
