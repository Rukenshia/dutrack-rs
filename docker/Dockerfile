FROM ubuntu:16.04
MAINTAINER Jan Christophersen <jan@ruken.pw>

WORKDIR /dutrack
ADD dutrack-rs ./dutrack
ADD templates ./templates
ADD assets ./assets
ADD .env ./.env
ADD migrations ./migrations

RUN chmod +x dutrack

RUN apt-get update && apt-get install -y libpq-dev
EXPOSE 8000

ENTRYPOINT ./dutrack