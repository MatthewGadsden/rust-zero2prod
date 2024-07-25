set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
        echo >&2 "Error: pql is not installed."
        exit 1
fi

if ! [ -x "$(command --version sqlx)"]; then
        echo >&2 "Error: sqlx is not installed"
        echo >&2 "Use:"
        echo >&2 "      cargo install sqlx-cli --no-default-features --features rustls,postgres"
        echo >&2 "to install it."
        exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"
# Check if a custom db name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom db name has been set, otherwise default to 'newsletter'
DB_PORT="${POSTGRES_PORT:=5433}"
# Check if a custom db name has been set, otherwise default to 'newsletter'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Lauch postgres with Docker
>&2 echo "$(command docker compose -f ./scripts/docker-compose.yaml -p zero-2-prod up -d )"

export PGPASSWORD="${DB_PASSWORD}"
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
        >&2 echo "Postgres is still unavailable - sleeping"
        sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx database create

sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"
