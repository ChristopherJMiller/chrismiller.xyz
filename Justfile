set export := true

# ============================================
# LOCAL DEVELOPMENT COMMANDS (Docker Compose)
# ============================================

# Local database URL for docker-compose PostgreSQL (use 127.0.0.1 to force TCP)
LOCAL_DATABASE_URL := "postgres://postgres:postgres@127.0.0.1/postgres"

# Start local PostgreSQL container
dev-db-up:
  docker-compose up -d postgres
  @echo "Waiting for PostgreSQL to be ready..."
  @until docker-compose exec -T postgres pg_isready -U postgres > /dev/null 2>&1; do sleep 1; done
  @echo "PostgreSQL is ready!"

# Stop local PostgreSQL container
dev-db-down:
  docker-compose down

# Stop local PostgreSQL and remove data volume
dev-db-reset:
  docker-compose down -v
  docker-compose up -d postgres
  @echo "Waiting for PostgreSQL to be ready..."
  @until docker-compose exec -T postgres pg_isready -U postgres > /dev/null 2>&1; do sleep 1; done
  @echo "PostgreSQL is ready with fresh data!"

# Run diesel setup with local database
dev-db-setup:
  DATABASE_URL={{LOCAL_DATABASE_URL}} diesel setup --config-file ./diesel.toml --migration-dir ./migrations

# Run diesel migrations with local database
dev-db-migrate:
  DATABASE_URL={{LOCAL_DATABASE_URL}} diesel migration run --config-file ./diesel.toml --migration-dir ./migrations

# Revert last migration on local database
dev-db-rollback:
  DATABASE_URL={{LOCAL_DATABASE_URL}} diesel migration revert --config-file ./diesel.toml --migration-dir ./migrations

# Run the application with local database
dev: dev-db-up
  DATABASE_URL={{LOCAL_DATABASE_URL}} cargo run

# Run with CSS watch (start yarn watch separately or use a terminal multiplexer)
dev-full: dev-db-up
  @echo "Start 'yarn watch' in another terminal for CSS rebuilding"
  DATABASE_URL={{LOCAL_DATABASE_URL}} cargo run

# ============================================
# PRODUCTION/K8S DATABASE COMMANDS
# ============================================

db:
  USER=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  echo postgres://$USER_D:$PASSWORD_D@localhost/postgres

db-setup:
  USER=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  export DATABASE_URL=postgres://$USER_D:$PASSWORD_D@localhost/postgres && \
  PGSSLMODE=disable diesel setup --config-file ./diesel.toml --migration-dir ./migrations

db-rollback:
  USER=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  export DATABASE_URL=postgres://$USER_D:$PASSWORD_D@localhost/postgres && \
  PGSSLMODE=disable diesel migration revert --config-file ./diesel.toml --migration-dir ./migrations

db-migrate: fix-psql-table
  USER=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  export DATABASE_URL=postgres://$USER_D:$PASSWORD_D@localhost/postgres && \
  PGSSLMODE=disable diesel migration run --config-file ./diesel.toml --migration-dir ./migrations

fix-psql-table:
  USER=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.username}}) && \
  PASSWORD=$(kubectl -n chrismillerxyz get secret postgres.acid-chrismillerxyz.credentials.postgresql.acid.zalan.do --template={{{{.data.password}}) && \
  USER_D=$(echo $USER | base64 -d) && \
  PASSWORD_D=$(echo $PASSWORD | base64 -d) && \
  export DATABASE_URL=postgres://$USER_D:$PASSWORD_D@localhost/postgres && \
  PGSSLMODE=disable psql $DATABASE_URL -c "DROP TABLE IF EXISTS postgres_log CASCADE;"

db-pf:
  kubectl -n chrismillerxyz port-forward services/psql 5432

minio-pf:
  kubectl -n chrismillerxyz port-forward service/minio 9000
