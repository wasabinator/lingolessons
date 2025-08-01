#!/bin/sh
python manage.py loaddata languages
python manage.py loaddata lessons
python manage.py runserver
