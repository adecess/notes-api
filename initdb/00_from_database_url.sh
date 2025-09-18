#!/usr/bin/env sh
set -eu

# This script runs inside the postgres container at first init.
# It parses DATABASE_URL and creates the user and database accordingly.

if [ -z "${DATABASE_URL:-}" ]; then
  echo "DATABASE_URL must be set in .env"
  exit 1
fi

# Expected formats:
#  - postgres://user:pass@host:port/dbname
#  - postgresql://user:pass@host:port/dbname

scheme_removed="${DATABASE_URL#*://}"
credentials="${scheme_removed%@*}"
host_and_db="${scheme_removed#*@}"
host_port="${host_and_db%%/*}"
db_name="${host_and_db#*/}"
user_name="${credentials%%:*}"
user_pass="${credentials#*:}"
user_pass="${user_pass%%@*}"
host_name="${host_port%%:*}"
port_num="${host_port#*:}"

if [ "$host_name" = "$host_port" ]; then
  port_num="5432"
fi

if [ -z "${POSTGRES_PASSWORD:-}" ]; then
  echo "POSTGRES_PASSWORD must be set in .env for secure initialization"
  exit 1
fi

# Use TCP with explicit password for reliability inside init container
export PGPASSWORD="${POSTGRES_PASSWORD}"

# Connect via local socket as superuser. The official entrypoint sets up local trust
# for the superuser when POSTGRES_PASSWORD is provided, so this will succeed.
psql -v ON_ERROR_STOP=1 --username "postgres" <<-EOSQL
  DO \$\$
  BEGIN
    IF NOT EXISTS (
      SELECT FROM pg_catalog.pg_roles WHERE rolname = '${user_name}'
    ) THEN
      CREATE ROLE ${user_name} LOGIN PASSWORD '${user_pass}';
    END IF;
  END\$\$;
EOSQL

# CREATE DATABASE cannot run inside a DO block. Check and create separately.
db_exists=$(psql --username "postgres" -tAc "SELECT 1 FROM pg_database WHERE datname='${db_name}'" || echo "")
if [ "${db_exists}" != "1" ]; then
  psql -v ON_ERROR_STOP=1 --username "postgres" -c "CREATE DATABASE \"${db_name}\" OWNER \"${user_name}\";"
fi

echo "Initialized database '${db_name}' with owner '${user_name}'."


