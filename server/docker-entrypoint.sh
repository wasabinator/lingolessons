#!/bin/sh

if [ "$DATABASE" = "postgres" ]
then
    echo "Waiting for postgres..."

    while ! nc -z $SQL_HOST $SQL_PORT; do
      sleep 0.1
    done

    echo "PostgreSQL started"
fi

echo "Apply database migrations"
python manage.py flush --no-input
python manage.py migrate
python manage.py createsuperuser --noinput
python manage.py loaddata languages

echo "Collect static files"
#python manage.py collectstatic --noinput

echo "Starting server"
python manage.py runserver 0.0.0.0:8000

exec "$@"

