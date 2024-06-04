# syntax=docker/dockerfile:1.4

FROM python:3.11.4-slim-buster as builder
EXPOSE 8000
WORKDIR /server

# set environment variables
ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

# install system dependencies
RUN apt-get update && apt-get install -y netcat

COPY . /server
RUN pip install -r requirements.txt --no-cache-dir

ENTRYPOINT ["/server/docker-entrypoint.sh"]
