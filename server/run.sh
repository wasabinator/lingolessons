#!/bin/sh
python manage.py migrate

export DJANGO_SUPERUSER_USERNAME="admin"
export DJANGO_SUPERUSER_EMAIL="admin@lingolessons.com"
export DJANGO_SUPERUSER_PASSWORD="admin"
python manage.py createsuperuser --noinput

python manage.py loaddata languages
python manage.py loaddata lessons

python manage.py runserver
