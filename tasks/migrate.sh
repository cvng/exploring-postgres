# https://www.postgresql.org/docs/current/app-psql.html

set -e
source .env

host="$DATABASE_URL"
name="bits"

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="\connect postgres" \
    --command="drop database if exists $name with (force);" \
    --command="create database $name;" \

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --single-transaction \
    --file="docs/schema.sql" \
    --quiet \

psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --file="docs/es.sql" \
    --quiet \

PGOPTIONS='--client-min-messages=warning' psql "$host" \
    --no-psqlrc \
    --variable=ON_ERROR_STOP=1 \
    --command="create table temp (row jsonb);" \
    --command="\copy temp (row) from tasks/seed.ndjson;" \
    --command="insert into cqrs.event (type, data) select (row->>'type')::cqrs.event_type, (row->'data')::jsonb from temp;" \

# --file="tasks/seed.sql" \
