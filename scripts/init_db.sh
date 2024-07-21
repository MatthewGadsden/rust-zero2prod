set -x
set -eo pipefail

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=postgres}"
# Check if a custom db name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom db name has been set, otherwise default to 'newsletter'
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom db name has been set, otherwise default to 'newsletter'
DB_HOST="${POSTGRES_HOST:=localhost}"

# Lauch postgres with Docker
docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000