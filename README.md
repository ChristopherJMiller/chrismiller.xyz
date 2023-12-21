# Setting Up

The `Justfile` assumes a very specific environment setup that isn't very friendly to external usage. The site needs access a postgres instance, which can be provided as a full connection url via `DATABASE_URL`.

```sh

yarn install

# In a separate terminal, make the postgres db available
just db-pf

# In a separate terminal, get the connection url and run the site with it. If running through a k8s port forward you need to disable postgres ssl (https://github.com/kubernetes/kubectl/issues/1169)

just db | PGSSLMODE=disable cargo run

# In a separate terminal, run the styling watcher for handle any frontend updates

yarn watch

```
