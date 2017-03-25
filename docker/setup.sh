#!/bin/bash
apt-get update && apt-get install -y redis-server
service postgresql start
sleep 10
su postgres -c 'psql -c "CREATE DATABASE dutrack;"; psql -c "CREATE USER dutrack PASSWORD '"'dutrack'"'; GRANT ALL ON DATABASE dutrack TO dutrack;"; psql -c "CREATE EXTENSION pgcrypto;" dutrack'
cat /var/log/postgresql/postgresql-9.5-main.log
set -a
source .env
set +a
diesel migration run