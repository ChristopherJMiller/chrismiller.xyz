
set export := true

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
